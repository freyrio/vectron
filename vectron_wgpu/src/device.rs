use std::collections::HashMap;
use std::sync::Arc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use vectron_render::{backend::{
    commands::{CommandBuffer, CommandEncoder}, device::{BackendDevice, BindGroupDesc, BindGroupLayoutDesc, ShaderCode, ShaderDesc}, resource::*, state::*
}, BackendError};
use crate::conversion::*;
use crate::resources::WgpuResourceManager;
use crate::commands::{WgpuCommandEncoder, WgpuCommandBuffer, WgpuCommandBufferSubmitInfo};

/// WGPU implementation of the backend device
pub struct WgpuBackendDevice<'window> {
    instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'window>>,
    surface_config: Option<SurfaceConfiguration>,
    surface_format: Option<wgpu::TextureFormat>,
    // Resources
    resources: WgpuResourceManager,
    // Device features
    device_features: DeviceFeatures,
}

impl<'window> WgpuBackendDevice<'window> {
    /// Create a new WGPU backend device
    pub fn new() -> Self {
        // Create instance
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Create default device features
        let device_features = DeviceFeatures {
            max_texture_dimension_1d: 8192,
            max_texture_dimension_2d: 8192,
            max_texture_dimension_3d: 2048,
            max_texture_array_layers: 256,
            max_bind_groups: 4,
            max_dynamic_uniform_buffers_per_pipeline_layout: 8,
            max_dynamic_storage_buffers_per_pipeline_layout: 4,
            max_sampled_textures_per_shader_stage: 16,
            max_samplers_per_shader_stage: 16,
            max_storage_buffers_per_shader_stage: 8,
            max_storage_textures_per_shader_stage: 4,
            max_uniform_buffers_per_shader_stage: 12,
            max_uniform_buffer_binding_size: 64 * 1024,
            max_storage_buffer_binding_size: 128 * 1024 * 1024,
            max_vertex_buffers: 8,
            max_vertex_attributes: 16,
            max_vertex_buffer_array_stride: 2048,
            supports_timestamp_query: false,
            supports_pipeline_statistics_query: false,
            supports_multiview: false,
        };
        
        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
            surface: None,
            surface_config: None,
            surface_format: None,
            resources: WgpuResourceManager::new(),
            device_features,
        }
    }
    
    /// Initialize the surface from a window
    pub fn init_surface_from_window<W>(&mut self, window: &'window W, width: u32, height: u32) -> Result<(), BackendError>
    where
    W: HasWindowHandle + HasDisplayHandle + Send + Sync,
    {
        let surface = self.instance.create_surface(window)
            .map_err(|err| BackendError::SurfaceError(format!("Failed to create surface: {}", err)))?;
        
        self.init_adapter_and_device(Some(&surface))?;
        
        self.surface = Some(surface);
        
        let adapter = self.adapter.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();
        
        // Get preferred format
        let surface_caps = self.surface.as_ref().unwrap().get_capabilities(adapter);
        let surface_format = surface_caps.formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        self.surface_format = Some(surface_format);
        
        // Configure surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        self.surface.as_ref().unwrap().configure(device, &config);
        self.surface_config = Some(config);
        
        Ok(())
    }
    
    /// Initialize the adapter and device
    fn init_adapter_and_device(&mut self, compatible_surface: Option<&Surface>) -> Result<(), BackendError> {
        // Request adapter
        let adapter = pollster::block_on(
            self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface,
                force_fallback_adapter: false,
            })
        )
        .map_err(|_| BackendError::DeviceCreation("Failed to create adapter".to_string()))?;
        
        // Request device
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Vectron WGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
        ))
        .map_err(|err| BackendError::DeviceCreation(format!("Failed to create device: {}", err)))?;
        
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        
        // Update device features with actual limits
        if let Some(adapter) = &self.adapter {
            let limits = adapter.limits();
            self.device_features = DeviceFeatures {
                max_texture_dimension_1d: limits.max_texture_dimension_1d,
                max_texture_dimension_2d: limits.max_texture_dimension_2d,
                max_texture_dimension_3d: limits.max_texture_dimension_3d,
                max_texture_array_layers: limits.max_texture_array_layers,
                max_bind_groups: limits.max_bind_groups,
                max_dynamic_uniform_buffers_per_pipeline_layout: limits.max_dynamic_uniform_buffers_per_pipeline_layout,
                max_dynamic_storage_buffers_per_pipeline_layout: limits.max_dynamic_storage_buffers_per_pipeline_layout,
                max_sampled_textures_per_shader_stage: limits.max_sampled_textures_per_shader_stage,
                max_samplers_per_shader_stage: limits.max_samplers_per_shader_stage,
                max_storage_buffers_per_shader_stage: limits.max_storage_buffers_per_shader_stage,
                max_storage_textures_per_shader_stage: limits.max_storage_textures_per_shader_stage,
                max_uniform_buffers_per_shader_stage: limits.max_uniform_buffers_per_shader_stage,
                max_uniform_buffer_binding_size: limits.max_uniform_buffer_binding_size,
                max_storage_buffer_binding_size: limits.max_storage_buffer_binding_size,
                max_vertex_buffers: limits.max_vertex_buffers,
                max_vertex_attributes: limits.max_vertex_attributes,
                max_vertex_buffer_array_stride: limits.max_vertex_buffer_array_stride,
                supports_timestamp_query: adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY),
                supports_pipeline_statistics_query: adapter.features().contains(wgpu::Features::PIPELINE_STATISTICS_QUERY),
                supports_multiview: adapter.features().contains(wgpu::Features::MULTIVIEW),
            };
        }
        
        Ok(())
    }
    
    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), BackendError> {
        if let Some(config) = self.surface_config.as_mut() {
            config.width = width;
            config.height = height;
            
            if let Some(surface) = &self.surface {
                if let Some(device) = &self.device {
                    surface.configure(device, config);
                    return Ok(());
                }
            }
        }
        
        Err(BackendError::InvalidOperation("No surface available to resize".to_string()))
    }
}

/// Implementation of the BackendDevice trait for WGPU
impl<'window> BackendDevice for WgpuBackendDevice<'window> {
    fn name(&self) -> &str {
        "WGPU Backend"
    }
    
    fn features(&self) -> &DeviceFeatures {
        &self.device_features
    }
    
    fn begin_frame(&mut self) -> Result<Box<dyn CommandEncoder>, BackendError> {
        let device = self.device.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Device not initialized".to_string()))?;
        
        let encoder = WgpuCommandEncoder::new(device)?;
        Ok(Box::new(encoder))
    }
    
    fn end_frame(&mut self, command_buffer: Box<dyn CommandBuffer>) -> Result<(), BackendError> {
        let wgpu_cmd = match command_buffer.as_any().downcast_ref::<WgpuCommandBuffer>() {
            Some(cmd) => cmd,
            None => return Err(BackendError::InvalidOperation("Invalid command buffer type".to_string())),
        };
        
        let queue = self.queue.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Queue not initialized".to_string()))?;
        
        // Use our helper to get an iterator of owned command buffers
        let submit_info = WgpuCommandBufferSubmitInfo::new(wgpu_cmd);
        queue.submit(submit_info.get_command_buffers());
        
        // Present the frame if we have a surface
        if let Some(surface) = &self.surface {
            // Surface presentation is handled automatically by wgpu when the command buffer is submitted
        }
        
        Ok(())
    }
    
    fn create_buffer(&mut self, desc: &BufferDesc) -> Result<BufferHandle, BackendError> {
        let device = self.device.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Device not initialized".to_string()))?;
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: desc.label.as_deref(),
            size: desc.size,
            usage: to_wgpu_buffer_usage(desc.usage),
            mapped_at_creation: desc.mapped_at_creation,
        });
        
        let handle = BufferHandle::new();
        self.resources.register_buffer(handle, buffer);
        
        Ok(handle)
    }
    
    fn create_texture(&mut self, desc: &TextureDesc) -> Result<TextureHandle, BackendError> {
        let device = self.device.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Device not initialized".to_string()))?;
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: desc.label.as_deref(),
            size: wgpu::Extent3d {
                width: desc.size[0],
                height: desc.size[1],
                depth_or_array_layers: desc.size[2],
            },
            mip_level_count: desc.mip_level_count,
            sample_count: desc.sample_count,
            dimension: to_wgpu_texture_dimension(desc.dimension),
            format: to_wgpu_texture_format(desc.format),
            usage: to_wgpu_texture_usage(desc.usage),
            view_formats: &[],
        });
        
        let handle = TextureHandle::new();
        self.resources.register_texture(handle, texture);
        
        Ok(handle)
    }
    
    fn create_texture_view(&mut self, texture: TextureHandle, desc: &TextureViewDesc) -> Result<TextureHandle, BackendError> {
        let texture_obj = self.resources.get_texture(&texture)
            .ok_or_else(|| BackendError::InvalidOperation("Texture not found".to_string()))?;
            
        // Use default and override specific fields
        let mut view_desc = wgpu::TextureViewDescriptor::default();
        view_desc.label = desc.label.as_deref();
        view_desc.format = desc.format.map(to_wgpu_texture_format);
        view_desc.dimension = desc.dimension.map(to_wgpu_texture_view_dimension);
        view_desc.aspect = to_wgpu_texture_aspect(desc.aspect);
        view_desc.base_mip_level = desc.base_mip_level;
        view_desc.mip_level_count = desc.mip_level_count;
        view_desc.base_array_layer = desc.base_array_layer;
        view_desc.array_layer_count = desc.array_layer_count;
        
        let view = texture_obj.create_view(&view_desc);
        
        let handle = TextureHandle::new();
        self.resources.register_texture_view(handle, view);
        
        Ok(handle)
    }
    
    fn create_sampler(&mut self, desc: &SamplerDesc) -> Result<SamplerHandle, BackendError> {
        let device = self.device.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Device not initialized".to_string()))?;
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: desc.label.as_deref(),
            address_mode_u: to_wgpu_address_mode(desc.address_mode_u),
            address_mode_v: to_wgpu_address_mode(desc.address_mode_v),
            address_mode_w: to_wgpu_address_mode(desc.address_mode_w),
            mag_filter: to_wgpu_filter_mode(desc.mag_filter),
            min_filter: to_wgpu_filter_mode(desc.min_filter),
            mipmap_filter: to_wgpu_filter_mode(desc.mipmap_filter),
            lod_min_clamp: desc.lod_min_clamp,
            lod_max_clamp: desc.lod_max_clamp,
            compare: desc.compare.map(to_wgpu_compare_function),
            anisotropy_clamp: desc.anisotropy_clamp.unwrap_or(1),
            border_color: None, // TODO: Add border color support to SamplerDesc
        });
        
        let handle = SamplerHandle::new();
        self.resources.register_sampler(handle, sampler);
        
        Ok(handle)
    }
    
    fn create_bind_group(&mut self, desc: &BindGroupDesc) -> Result<BindGroupHandle, BackendError> {
        // TODO: Implement this
        Err(BackendError::UnsupportedFeature("Bind groups not yet implemented".to_string()))
    }
    
    fn create_shader(&mut self, desc: &ShaderDesc) -> Result<ShaderHandle, BackendError> {
        let device = self.device.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Device not initialized".to_string()))?;
        
        let shader_source = match &desc.code {
            ShaderCode::Wgsl(source) => wgpu::ShaderSource::Wgsl(source.clone().into()),
            ShaderCode::Spirv(data) => wgpu::ShaderSource::Wgsl(format!("/* SPIRV Not supported yet */").into()),
            ShaderCode::Glsl(source, _) => {
                return Err(BackendError::UnsupportedFeature("GLSL shaders not directly supported".to_string()));
            },
        };
        
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label.as_deref(),
            source: shader_source,
        });
        
        let handle = ShaderHandle::new();
        self.resources.register_shader(handle, shader);
        
        Ok(handle)
    }
    
    fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<PipelineHandle, BackendError> {
        // TODO: Implement this
        Err(BackendError::UnsupportedFeature("Pipelines not yet implemented".to_string()))
    }
    
    fn update_buffer(&mut self, handle: BufferHandle, data: &[u8], offset: u64) -> Result<(), BackendError> {
        let queue = self.queue.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Queue not initialized".to_string()))?;
        
        let buffer = self.resources.get_buffer(&handle)
            .ok_or_else(|| BackendError::InvalidOperation("Buffer not found".to_string()))?;
        
        queue.write_buffer(buffer, offset, data);
        
        Ok(())
    }
    
    fn update_texture(&mut self, handle: TextureHandle, data: &[u8], offset: [u32; 3], size: [u32; 3]) -> Result<(), BackendError> {
        let queue = self.queue.as_ref()
            .ok_or_else(|| BackendError::InvalidOperation("Queue not initialized".to_string()))?;
        
        let texture = self.resources.get_texture(&handle)
            .ok_or_else(|| BackendError::InvalidOperation("Texture not found".to_string()))?;
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: offset[0],
                    y: offset[1],
                    z: offset[2],
                },
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size[0]), // Assuming RGBA8
                rows_per_image: Some(size[1]),
            },
            wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: size[2],
            },
        );
        
        Ok(())
    }
    
    fn map_buffer(&mut self, handle: BufferHandle, offset: u64, size: u64) -> Result<*mut u8, BackendError> {
        // TODO: Implement buffer mapping
        Err(BackendError::UnsupportedFeature("Buffer mapping not yet implemented".to_string()))
    }
    
    fn unmap_buffer(&mut self, handle: BufferHandle) -> Result<(), BackendError> {
        // TODO: Implement buffer unmapping
        Err(BackendError::UnsupportedFeature("Buffer unmapping not yet implemented".to_string()))
    }
    
    fn destroy_buffer(&mut self, handle: BufferHandle) {
        self.resources.remove_buffer(&handle);
    }
    
    fn destroy_texture(&mut self, handle: TextureHandle) {
        self.resources.remove_texture(&handle);
    }
    
    fn destroy_sampler(&mut self, handle: SamplerHandle) {
        self.resources.remove_sampler(&handle);
    }
    
    fn destroy_bind_group(&mut self, handle: BindGroupHandle) {
        self.resources.remove_bind_group(&handle);
    }
    
    fn destroy_shader(&mut self, handle: ShaderHandle) {
        self.resources.remove_shader(&handle);
    }
    
    fn destroy_pipeline(&mut self, handle: PipelineHandle) {
        self.resources.remove_pipeline(&handle);
    }
    
    fn resize_surface(&mut self, width: u32, height: u32) -> Result<(), BackendError> {
        self.resize(width, height)
    }
    
    fn get_current_surface_texture(&mut self) -> Result<TextureHandle, BackendError> {
        if let Some(surface) = &self.surface {
            let output = surface.get_current_texture()
                .map_err(|err| BackendError::SurfaceError(format!("Failed to get current surface texture: {:?}", err)))?;
            
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            
            let handle = TextureHandle::new();
            self.resources.register_texture_view(handle, view);
            
            Ok(handle)
        } else {
            Err(BackendError::InvalidOperation("No surface available".to_string()))
        }
    }
    
    fn get_surface_format(&self) -> Option<TextureFormat> {
        self.surface_format.map(|format| {
            from_wgpu_texture_format(format).unwrap_or(TextureFormat::Bgra8UnormSrgb)
        })
    }
}

/// Creates a new WGPU backend device.
/// 
/// This is the main entry point for using the WGPU backend.
pub fn create_wgpu_backend() -> WgpuBackendDevice<'static> {
    WgpuBackendDevice::new()
}

/// Creates a WGPU backend device with a surface for a window.
/// 
/// This is a convenience function for creating a backend with a surface attached to a window.
pub fn create_wgpu_backend_for_window<'window, W>(window: &'window W, width: u32, height: u32) -> Result<WgpuBackendDevice<'window>, BackendError>
where
    W: HasWindowHandle + HasDisplayHandle + Send + Sync,
{
    let mut backend = WgpuBackendDevice::new();
    backend.init_surface_from_window(window, width, height)?;
    Ok(backend)
} 