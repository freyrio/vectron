use std::sync::Arc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Adapter, Backends, CreateSurfaceError, Device, Instance, InstanceDescriptor, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration};
use vectron_render::{backend::{
    commands::{CommandBuffer, CommandEncoder}, device::{BackendDevice, BindGroupDesc, BindGroupLayoutDesc, ShaderCode, ShaderDesc}, resource::*, state::*
}, core::error::BackendError, core::types::common::Uuid};

// Module structure
mod conversion;
mod device;
mod commands;
mod resources;

// Re-exports
pub use device::WgpuBackendDevice;
pub use commands::WgpuCommandBuffer;

/// Creates a new WGPU backend device.
/// 
/// This is the main entry point for using the WGPU backend.
pub fn create_wgpu_backend() -> WgpuBackendDevice<'static> {
    WgpuBackendDevice::new()
}

/// Creates a WGPU backend device with a surface for a window.
/// 
/// This is a convenience function for creating a backend with a surface attached to a window.
pub fn create_wgpu_backend_for_window<W>(window: &W, width: u32, height: u32) -> Result<WgpuBackendDevice, BackendError>
where
    W: HasWindowHandle + HasDisplayHandle + Send + Sync,
{
    let mut backend = WgpuBackendDevice::new();
    backend.init_surface_from_window(window, width, height)?;
    Ok(backend)
}

/// A texture view wrapper
#[derive(Debug)]
pub struct TextureView(pub(crate) wgpu::TextureView);

/// Unique identifier for WGPU backend resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WgpuResourceId(Uuid);

// Public types that don't expose wgpu directly
pub struct RenderDevice(wgpu::Device);
pub struct RenderQueue(wgpu::Queue);
pub struct Buffer(wgpu::Buffer);
pub struct RenderPipeline(wgpu::RenderPipeline);
pub struct ShaderModule(wgpu::ShaderModule);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
}

impl From<TextureFormat> for wgpu::TextureFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
}

impl From<wgpu::TextureFormat> for TextureFormat {
    fn from(format: wgpu::TextureFormat) -> Self {
        match format {
            wgpu::TextureFormat::Rgba8Unorm => TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rgba8UnormSrgb => TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Bgra8Unorm => TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Bgra8UnormSrgb => TextureFormat::Bgra8UnormSrgb,
            _ => TextureFormat::Bgra8UnormSrgb, // Default
        }
    }
}

// Buffer utilities
pub fn create_buffer_with_data(device: &RenderDevice, data: &[u8], usage: BufferUsage) -> Buffer {
    let buffer = device.0.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: data.len() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::from_bits_truncate(usage.bits()),
        mapped_at_creation: false,
    });
    
    Buffer(buffer)
}

pub fn write_buffer(queue: &RenderQueue, buffer: &Buffer, offset: u64, data: &[u8]) {
    queue.0.write_buffer(&buffer.0, offset, data);
}

#[derive(Debug, Clone, Copy)]
pub struct BufferUsage(u32);

impl BufferUsage {
    pub const VERTEX: Self = Self(wgpu::BufferUsages::VERTEX.bits());
    pub const INDEX: Self = Self(wgpu::BufferUsages::INDEX.bits());
    pub const UNIFORM: Self = Self(wgpu::BufferUsages::UNIFORM.bits());
    pub const STORAGE: Self = Self(wgpu::BufferUsages::STORAGE.bits());
    pub const COPY_SRC: Self = Self(wgpu::BufferUsages::COPY_SRC.bits());
    pub const COPY_DST: Self = Self(wgpu::BufferUsages::COPY_DST.bits());
    
    pub fn bits(self) -> u32 {
        self.0
    }
    
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

// Shader loading utility
pub fn create_shader_module(device: &RenderDevice, source: &str) -> ShaderModule {
    let module = device.0.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(source)),
    });
    
    ShaderModule(module)
}

// Implement methods for types
impl RenderDevice {
    pub fn new_pipeline(&self, format: TextureFormat, shader: &ShaderModule, vertex_layouts: &[VertexBufferLayout]) -> RenderPipeline {
        // Create default pipeline layout
        let pipeline_layout = self.0.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        // Convert vertex layouts
        let layouts: Vec<wgpu::VertexBufferLayout> = vertex_layouts
            .iter()
            .map(|layout| layout.to_wgpu_layout())
            .collect();
        
        let wgpu_format: wgpu::TextureFormat = format.into();
        
        // Create the render pipeline
        let pipeline = self.0.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader.0,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.0,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
        
        RenderPipeline(pipeline)
    }
}

// Vertex buffer layout abstraction
#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    array_stride: u64,
    step_mode: VertexStepMode,
    attributes: Vec<VertexAttribute>,
}

#[derive(Debug, Clone, Copy)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

impl From<VertexStepMode> for wgpu::VertexStepMode {
    fn from(mode: VertexStepMode) -> Self {
        match mode {
            VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
            VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttribute {
    format: VertexFormat,
    offset: u64,
    shader_location: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Float32x2,
    Float32x3,
    Float32x4,
    // Add more as needed
}

impl From<VertexFormat> for wgpu::VertexFormat {
    fn from(format: VertexFormat) -> Self {
        match format {
            VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
        }
    }
}

impl VertexBufferLayout {
    pub fn new(array_stride: u64, step_mode: VertexStepMode, attributes: Vec<VertexAttribute>) -> Self {
        Self {
            array_stride,
            step_mode,
            attributes,
        }
    }
    
    fn to_wgpu_layout(&self) -> wgpu::VertexBufferLayout<'static> {
        let wgpu_attributes = self.attributes.iter().map(|attr| {
            wgpu::VertexAttribute {
                format: attr.format.into(),
                offset: attr.offset,
                shader_location: attr.shader_location,
            }
        }).collect::<Vec<_>>();
        
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode.into(),
            attributes: wgpu_attributes.leak(), // Note: This is safe for 'static lifetime in this context
        }
    }
}

pub struct VertexAttributeBuilder {
    format: VertexFormat,
    offset: u64,
    shader_location: u32,
}

impl VertexAttributeBuilder {
    pub fn new(format: VertexFormat, shader_location: u32) -> Self {
        Self {
            format,
            offset: 0,
            shader_location,
        }
    }
    
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }
    
    pub fn build(self) -> VertexAttribute {
        VertexAttribute {
            format: self.format,
            offset: self.offset,
            shader_location: self.shader_location,
        }
    }
}

pub struct WgpuBackend<'window> {
    instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'window>>,
    surface_config: Option<SurfaceConfiguration>,
    surface_format: Option<wgpu::TextureFormat>,
}

impl<'window> WgpuBackend<'window> {
    pub fn new() -> Self {
        let backends = if cfg!(windows) {
            Backends::VULKAN
        } else {
            Backends::all()
        };
        
        let instance = Instance::new(&InstanceDescriptor {
            backends,
            ..Default::default()
        });

        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
            surface: None,
            surface_config: None,
            surface_format: None,
        }
    }

    pub fn init_surface<W>(&mut self, window: &'window W, width: u32, height: u32) -> Result<(), String>
    where
        W: HasWindowHandle + HasDisplayHandle + Send + Sync,
    {
        let surface = self.instance.create_surface(window)
            .map_err(|e: CreateSurfaceError| e.to_string())?;

        let adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter: Adapter = pollster::block_on(
            self.instance.request_adapter(&adapter_options)
        )
        .map_err(|e| format!("Failed to request adapter: {}", e))?;
        
        println!("Using adapter: {} (backend: {:?})", adapter.get_info().name, adapter.get_info().backend);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Vectron Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
        ))
        .map_err(|e| e.to_string())?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            view_formats: vec![],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.surface_format = Some(surface_format);

        Ok(())
    }

    pub fn init_surface_from_target(&mut self, target: impl Into<wgpu::SurfaceTarget<'window>>, width: u32, height: u32) -> Result<(), String> {
        let surface = self.instance.create_surface(target)
            .map_err(|e: CreateSurfaceError| e.to_string())?;

        let adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter: Adapter = pollster::block_on(
            self.instance.request_adapter(&adapter_options)
        )
        .map_err(|e| format!("Failed to request adapter: {}", e))?;
        
        println!("Using adapter: {} (backend: {:?})", adapter.get_info().name, adapter.get_info().backend);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Vectron Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
        ))
        .map_err(|e| e.to_string())?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            view_formats: vec![],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.surface_format = Some(surface_format);

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let (Some(surface_config), Some(device), Some(surface)) = 
            (&mut self.surface_config, &self.device, &self.surface) {
            surface_config.width = width;
            surface_config.height = height;
            surface.configure(device, surface_config);
        }
    }

    pub fn render(&self, render_fn: impl FnOnce(&RenderDevice, &RenderQueue, &TextureView)) -> Result<(), String> {
        let (device, queue, surface, _surface_format) = match (&self.device, &self.queue, &self.surface, &self.surface_format) {
            (Some(device), Some(queue), Some(surface), Some(format)) => (device, queue, surface, format),
            _ => return Err("Rendering resources not initialized".to_string()),
        };

        let surface_texture = surface
            .get_current_texture()
            .map_err(|e| format!("Failed to acquire next frame: {:?}", e))?;
        
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        // Wrap the wgpu types in our public types
        let render_device = RenderDevice(device.clone());
        let render_queue = RenderQueue(queue.clone());
        let texture_view = TextureView(view);
        
        render_fn(&render_device, &render_queue, &texture_view);
        
        surface_texture.present();
        
        Ok(())
    }

    pub fn device(&self) -> Option<RenderDevice> {
        self.device.as_ref().map(|d| RenderDevice(d.clone()))
    }

    pub fn queue(&self) -> Option<RenderQueue> {
        self.queue.as_ref().map(|q| RenderQueue(q.clone()))
    }
    
    pub fn surface_format(&self) -> Option<TextureFormat> {
        self.surface_format.map(|f| f.into())
    }
}

// Rendering command encoder
pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.pass.set_pipeline(&pipeline.0);
    }
    
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        self.pass.set_vertex_buffer(slot, buffer.0.slice(..));
    }
    
    pub fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.pass.draw(vertices, instances);
    }
}

// Command encoder for rendering 
pub struct RenderCommandEncoder {
    encoder: Option<wgpu::CommandEncoder>,
}

impl RenderCommandEncoder {
    pub fn begin_render_pass<'a>(
        &'a mut self, 
        view: &'a TextureView, 
        clear_color: [f64; 4]
    ) -> RenderPass<'a> {
        let encoder = self.encoder.as_mut()
            .expect("Encoder already consumed or not initialized");
            
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view.0,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear_color[0],
                        g: clear_color[1],
                        b: clear_color[2],
                        a: clear_color[3],
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        RenderPass { pass }
    }
    
    pub fn finish(mut self) -> wgpu::CommandBuffer {
        // Take ownership of the encoder
        let encoder = self.encoder.take()
            .expect("Encoder already consumed");
            
        encoder.finish()
    }
}

// Extension trait to create a render encoder
pub trait RenderContext {
    fn create_command_encoder(&self, device: &RenderDevice) -> RenderCommandEncoder;
    fn submit_commands(&self, queue: &RenderQueue, commands: wgpu::CommandBuffer);
}

impl RenderContext for TextureView {
    fn create_command_encoder(&self, device: &RenderDevice) -> RenderCommandEncoder {
        let encoder = device.0.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        RenderCommandEncoder {
            encoder: Some(encoder),
        }
    }
    
    fn submit_commands(&self, queue: &RenderQueue, commands: wgpu::CommandBuffer) {
        queue.0.submit(std::iter::once(commands));
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
