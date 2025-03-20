use crate::common::{GpuError, SurfaceId, BufferId, TextureId, ShaderId, PipelineId};
use crate::buffer::BufferDescriptor;
use crate::texture::{TextureDescriptor, TextureFormat, TextureUpdateDescriptor};
use crate::pipeline::PipelineDescriptor;
use crate::shader::ShaderDescriptor;

/// Surface descriptor for creating a render target
#[derive(Debug, Clone)]
pub struct SurfaceDescriptor {
    pub handle: *mut std::ffi::c_void,
    pub width: u32,
    pub height: u32,
}

/// GPU backend configuration
#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub application_name: String,
    pub enable_debug: bool,
    pub preferred_format: Option<TextureFormat>,
    pub vsync: bool,
}

/// Backend capabilities
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub max_texture_size: u32,
    pub supports_compute: bool,
    pub supports_storage_buffers: bool,
    pub supports_float_textures: bool,
    pub max_uniform_buffer_size: usize,
    pub max_color_attachments: u32,
}

/// Base render command enum
#[derive(Debug, Clone)]
pub enum RenderCommand {
    SetPipeline(PipelineId),
    SetVertexBuffer {
        slot: u32,
        buffer: BufferId,
        offset: usize,
    },
    SetIndexBuffer {
        buffer: BufferId,
        offset: usize,
        index_format: IndexFormat,
    },
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    },
    SetViewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    },
    SetScissor {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    ClearColor {
        attachment_index: u32,
        color: [f32; 4],
    },
    ClearDepthStencil {
        depth: Option<f32>,
        stencil: Option<u32>,
    },
}

/// Index format for index buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

/// Base GPU backend trait
pub trait GpuBackend {
    /// Initialize the backend with the given configuration
    fn init(&mut self, config: BackendConfig) -> Result<(), GpuError>;
    
    /// Create a render surface from a platform surface
    fn create_surface(&mut self, desc: SurfaceDescriptor) -> Result<SurfaceId, GpuError>;
    
    /// Destroy a render surface
    fn destroy_surface(&mut self, id: SurfaceId);
    
    /// Resize a surface
    fn resize_surface(&mut self, id: SurfaceId, width: u32, height: u32) -> Result<(), GpuError>;
    
    /// Create a pipeline
    fn create_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError>;
    
    /// Create a buffer
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<BufferId, GpuError>;
    
    /// Update buffer data
    fn update_buffer(&mut self, id: BufferId, data: &[u8], offset: usize) -> Result<(), GpuError>;
    
    /// Create a texture
    fn create_texture(&mut self, desc: TextureDescriptor) -> Result<TextureId, GpuError>;
    
    /// Update texture data
    fn update_texture(&mut self, id: TextureId, data: &[u8], desc: TextureUpdateDescriptor) -> Result<(), GpuError>;
    
    /// Create a shader
    fn create_shader(&mut self, desc: ShaderDescriptor) -> Result<ShaderId, GpuError>;
    
    /// Begin rendering to a surface
    fn begin_frame(&mut self, surface_id: SurfaceId) -> Result<(), GpuError>;
    
    /// Submit render commands
    fn submit_commands(&mut self, commands: &[RenderCommand]) -> Result<(), GpuError>;
    
    /// End rendering and present
    fn end_frame(&mut self) -> Result<(), GpuError>;
    
    /// Get backend capabilities
    fn get_capabilities(&self) -> BackendCapabilities;
    
    /// Shutdown the backend
    fn shutdown(&mut self);
}
