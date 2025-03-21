// In src/backends/directx/dx12.rs
use std::collections::HashMap;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;

use crate::backend::{GpuBackend, BackendConfig, BackendCapabilities};
use crate::common::{GpuError, SurfaceId, BufferId, TextureId, ShaderId, PipelineId};

use super::device::DeviceExt;
use super::resources::{BufferExt, TextureExt, ShaderExt, PipelineExt};
use super::commands::CommandExt;
use super::surface::SurfaceExt;
use super::sync::SyncExt;
use super::debug::DebugExt;

pub struct DirectX12Backend {
    // Core DX12 objects
    instance: Option<HMODULE>,
    device: Option<ID3D12Device>,
    command_queue: Option<ID3D12CommandQueue>,
    dxgi_factory: Option<IDXGIFactory4>,
    
    // Resource tracking
    surfaces: HashMap<SurfaceId, SurfaceResources>,
    buffers: HashMap<BufferId, BufferResources>,
    textures: HashMap<TextureId, TextureResources>,
    shaders: HashMap<ShaderId, ShaderResources>,
    pipelines: HashMap<PipelineId, PipelineResources>,
    
    // Synchronization
    fence: Option<ID3D12Fence>,
    fence_event: HANDLE,
    fence_value: u64,
    
    // Debug
    #[cfg(feature = "validation")]
    debug: Option<DirectX12Debug>,
    
    // Capabilities
    capabilities: BackendCapabilities,
    
    // State tracking
    current_surface_id: Option<SurfaceId>,
    current_pipeline_id: Option<PipelineId>,
    frame_index: u32,
    next_id: u64,
}

impl DirectX12Backend {
    pub fn new() -> Self {
        Self {
            instance: None,
            device: None,
            command_queue: None,
            dxgi_factory: None,
            
            surfaces: HashMap::new(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            pipelines: HashMap::new(),
            
            fence: None,
            fence_event: HANDLE(std::ptr::null_mut()),
            fence_value: 1,
            
            #[cfg(feature = "validation")]
            debug: None,
            
            capabilities: BackendCapabilities::default(),
            
            current_surface_id: None,
            current_pipeline_id: None,
            frame_index: 0,
            next_id: 1,
        }
    }
    
    // Helper to generate unique IDs
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// Implement the main GpuBackend trait by delegating to extension traits
impl GpuBackend for DirectX12Backend {
    fn init(&mut self, config: BackendConfig) -> Result<(), GpuError> {
        self.init_impl(config)
    }
    
    fn create_surface(&mut self, desc: crate::backend::SurfaceDescriptor) -> Result<SurfaceId, GpuError> {
        self.create_surface_impl(desc)
    }
    
    fn destroy_surface(&mut self, id: SurfaceId) {
        self.destroy_surface_impl(id)
    }
    
    fn resize_surface(&mut self, id: SurfaceId, width: u32, height: u32) -> Result<(), GpuError> {
        self.resize_surface_impl(id, width, height)
    }
    
    fn create_buffer(&mut self, desc: crate::buffer::BufferDescriptor) -> Result<BufferId, GpuError> {
        self.create_buffer_impl(desc)
    }
    
    fn update_buffer(&mut self, id: BufferId, data: &[u8], offset: usize) -> Result<(), GpuError> {
        self.update_buffer_impl(id, data, offset)
    }
    
    fn create_texture(&mut self, desc: crate::texture::TextureDescriptor) -> Result<TextureId, GpuError> {
        self.create_texture_impl(desc)
    }
    
    fn update_texture(&mut self, id: TextureId, data: &[u8], desc: crate::texture::TextureUpdateDescriptor) -> Result<(), GpuError> {
        self.update_texture_impl(id, data, desc)
    }
    
    fn create_shader(&mut self, desc: crate::shader::ShaderDescriptor) -> Result<ShaderId, GpuError> {
        self.create_shader_impl(desc)
    }
    
    fn create_pipeline(&mut self, desc: crate::pipeline::PipelineDescriptor) -> Result<PipelineId, GpuError> {
        self.create_pipeline_impl(desc)
    }
    
    fn begin_frame(&mut self, surface_id: SurfaceId) -> Result<(), GpuError> {
        self.begin_frame_impl(surface_id)
    }
    
    fn submit_commands(&mut self, commands: &[crate::backend::RenderCommand]) -> Result<(), GpuError> {
        self.submit_commands_impl(commands)
    }
    
    fn end_frame(&mut self) -> Result<(), GpuError> {
        self.end_frame_impl()
    }
    
    fn get_capabilities(&self) -> BackendCapabilities {
        self.capabilities.clone()
    }
    
    fn shutdown(&mut self) {
        self.shutdown_impl();
    }
}