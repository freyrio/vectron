use crate::backend::commands::{CommandBuffer, CommandEncoder};
use crate::core::error::BackendError;
use crate::backend::resource::*;
use crate::backend::state::*;
use crate::core::{Uuid, BufferHandle, TextureHandle, BindGroupHandle, PipelineHandle, SamplerHandle, ShaderHandle, TextureFormat};

/// Backend device trait for interacting with the GPU
pub trait BackendDevice: Send + Sync {
    /// Returns the device name
    fn name(&self) -> &str;
    
    /// Returns the device features
    fn features(&self) -> &DeviceFeatures;
    
    /// Begins a new frame and returns a command encoder
    fn begin_frame(&mut self) -> Result<Box<dyn CommandEncoder>, BackendError>;
    
    /// Ends the current frame and submits commands for execution
    fn end_frame(&mut self, command_buffer: Box<dyn CommandBuffer>) -> Result<(), BackendError>;
    
    /// Creates a new buffer
    fn create_buffer(&mut self, desc: &BufferDesc) -> Result<BufferHandle, BackendError>;
    
    /// Creates a new texture
    fn create_texture(&mut self, desc: &TextureDesc) -> Result<TextureHandle, BackendError>;
    
    /// Creates a texture view from a texture
    fn create_texture_view(&mut self, texture: TextureHandle, desc: &TextureViewDesc) -> Result<TextureHandle, BackendError>;
    
    /// Creates a new sampler
    fn create_sampler(&mut self, desc: &SamplerDesc) -> Result<SamplerHandle, BackendError>;
    
    /// Creates a new bind group
    fn create_bind_group(&mut self, desc: &BindGroupDesc) -> Result<BindGroupHandle, BackendError>;
    
    /// Creates a new shader module
    fn create_shader(&mut self, desc: &ShaderDesc) -> Result<ShaderHandle, BackendError>;
    
    /// Creates a new render pipeline
    fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<PipelineHandle, BackendError>;
    
    /// Updates buffer data
    fn update_buffer(&mut self, handle: BufferHandle, data: &[u8], offset: u64) -> Result<(), BackendError>;
    
    /// Updates texture data
    fn update_texture(&mut self, handle: TextureHandle, data: &[u8], offset: [u32; 3], size: [u32; 3]) -> Result<(), BackendError>;
    
    /// Maps a buffer for CPU access
    fn map_buffer(&mut self, handle: BufferHandle, offset: u64, size: u64) -> Result<*mut u8, BackendError>;
    
    /// Unmaps a previously mapped buffer
    fn unmap_buffer(&mut self, handle: BufferHandle) -> Result<(), BackendError>;
    
    /// Destroys a buffer
    fn destroy_buffer(&mut self, handle: BufferHandle);
    
    /// Destroys a texture
    fn destroy_texture(&mut self, handle: TextureHandle);
    
    /// Destroys a sampler
    fn destroy_sampler(&mut self, handle: SamplerHandle);
    
    /// Destroys a bind group
    fn destroy_bind_group(&mut self, handle: BindGroupHandle);
    
    /// Destroys a shader
    fn destroy_shader(&mut self, handle: ShaderHandle);
    
    /// Destroys a pipeline
    fn destroy_pipeline(&mut self, handle: PipelineHandle);
    
    /// Resizes the swap chain
    fn resize_surface(&mut self, width: u32, height: u32) -> Result<(), BackendError>;
    
    /// Gets the current surface texture
    fn get_current_surface_texture(&mut self) -> Result<TextureHandle, BackendError>;
    
    /// Gets the current surface format
    fn get_surface_format(&self) -> Option<TextureFormat>;
}

/// Bind group descriptor
#[derive(Debug, Clone)]
pub struct BindGroupDesc {
    pub layout: BindGroupLayoutHandle,
    pub entries: Vec<BindGroupEntry>,
    pub label: Option<String>,
}

/// Bind group entry
#[derive(Debug, Clone)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

/// Binding resource
#[derive(Debug, Clone)]
pub enum BindingResource {
    Buffer {
        buffer: BufferHandle,
        offset: u64,
        size: Option<u64>,
    },
    Texture {
        view: TextureHandle,
    },
    Sampler {
        sampler: SamplerHandle,
    },
}

/// Bind group layout handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutHandle(Uuid);

impl BindGroupLayoutHandle {
    pub fn new() -> Self {
        Self(Uuid::sequential())
    }
}

/// Bind group layout descriptor
#[derive(Debug, Clone)]
pub struct BindGroupLayoutDesc {
    pub entries: Vec<BindGroupLayoutEntry>,
    pub label: Option<String>,
}

/// Bind group layout entry
#[derive(Debug, Clone)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStage,
    pub binding_type: BindingType,
}

/// Shader stage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderStage(u32);

impl ShaderStage {
    pub const VERTEX: Self = Self(0x1);
    pub const FRAGMENT: Self = Self(0x2);
    pub const COMPUTE: Self = Self(0x4);
    
    pub const ALL: Self = Self(Self::VERTEX.0 | Self::FRAGMENT.0 | Self::COMPUTE.0);
    
    pub fn bits(self) -> u32 {
        self.0
    }
    
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

/// Binding type
#[derive(Debug, Clone)]
pub enum BindingType {
    Buffer {
        ty: BufferBindingType,
        has_dynamic_offset: bool,
        min_binding_size: Option<u64>,
    },
    Sampler {
        ty: SamplerBindingType,
    },
    Texture {
        sample_type: TextureSampleType,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: TextureFormat,
        view_dimension: TextureViewDimension,
    },
}

/// Buffer binding type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferBindingType {
    Uniform,
    Storage,
    ReadOnlyStorage,
}

/// Sampler binding type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplerBindingType {
    Filtering,
    NonFiltering,
    Comparison,
}

/// Texture sample type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureSampleType {
    Float,
    UnfilterableFloat,
    Depth,
    Sint,
    Uint,
}

/// Storage texture access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTextureAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// Shader descriptor
#[derive(Debug, Clone)]
pub struct ShaderDesc {
    pub code: ShaderCode,
    pub entry_point: String,
    pub label: Option<String>,
}

/// Shader code
#[derive(Debug, Clone)]
pub enum ShaderCode {
    Spirv(Vec<u8>),
    Wgsl(String),
    Glsl(String, ShaderStage),
} 