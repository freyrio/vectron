use std::fmt::Debug;
use crate::{core::{CompareFunction, Face, FilterMode, FrontFace, IndexFormat, PolygonMode, PrimitiveTopology, ResourceType, ShaderHandle, Uuid}, TextureFormat};

/// Resource descriptor for buffers
#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: BufferUsage,
    pub mapped_at_creation: bool,
    pub label: Option<String>,
}

/// Buffer usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferUsage(u32);

impl BufferUsage {
    pub const VERTEX: Self = Self(0x0001);
    pub const INDEX: Self = Self(0x0002);
    pub const UNIFORM: Self = Self(0x0004);
    pub const STORAGE: Self = Self(0x0008);
    pub const COPY_SRC: Self = Self(0x0010);
    pub const COPY_DST: Self = Self(0x0020);
    pub const MAP_READ: Self = Self(0x0040);
    pub const MAP_WRITE: Self = Self(0x0080);
    pub const INDIRECT: Self = Self(0x0100);
    
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

/// Texture descriptor
#[derive(Debug, Clone)]
pub struct TextureDesc {
    pub size: [u32; 3],
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsage,
    pub label: Option<String>,
}

/// Texture dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
}

/// Texture usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureUsage(u32);

impl TextureUsage {
    pub const COPY_SRC: Self = Self(0x0001);
    pub const COPY_DST: Self = Self(0x0002);
    pub const TEXTURE_BINDING: Self = Self(0x0004);
    pub const STORAGE_BINDING: Self = Self(0x0008);
    pub const RENDER_ATTACHMENT: Self = Self(0x0010);
    
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

/// Texture view descriptor
#[derive(Debug, Clone)]
pub struct TextureViewDesc {
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
    pub label: Option<String>,
}

/// Texture view dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

/// Texture aspect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureAspect {
    All,
    DepthOnly,
    StencilOnly,
}

/// Sampler descriptor
#[derive(Debug, Clone)]
pub struct SamplerDesc {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<u16>,
    pub label: Option<String>,
}

/// Address mode for texture coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
    ClampToBorder,
}

/// Pending resource upload
#[derive(Debug)]
pub struct PendingUpload {
    pub resource_type: ResourceType,
    pub handle: Uuid,
    pub data: Vec<u8>,
    pub offset: u64,
}

/// Pipeline descriptor for render pipeline creation
#[derive(Debug, Clone)]
pub struct PipelineDesc {
    pub vertex_shader: ShaderHandle,
    pub fragment_shader: Option<ShaderHandle>,
    pub vertex_layouts: Vec<VertexBufferLayout>,
    pub color_formats: Vec<Option<TextureFormat>>,
    pub depth_stencil: Option<DepthStencilState>,
    pub primitive: PrimitiveState,
    pub multisample: MultisampleState,
    pub label: Option<String>,
}

/// Vertex buffer layout
#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

/// Vertex step mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

/// Vertex attribute
#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub format: VertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

/// Vertex format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,
    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,
    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,
    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
}

/// Depth stencil state
#[derive(Debug, Clone)]
pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil: StencilState,
    pub bias: DepthBiasState,
}

/// Stencil state
#[derive(Debug, Clone)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
    pub read_mask: u32,
    pub write_mask: u32,
}

/// Stencil face state
#[derive(Debug, Clone)]
pub struct StencilFaceState {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

/// Stencil operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap,
}

/// Depth bias state
#[derive(Debug, Clone)]
pub struct DepthBiasState {
    pub constant: i32,
    pub slope_scale: f32,
    pub clamp: f32,
}

/// Primitive state
#[derive(Debug, Clone)]
pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: Option<Face>,
    pub polygon_mode: PolygonMode,
}

/// Multisample state
#[derive(Debug, Clone)]
pub struct MultisampleState {
    pub count: u32,
    pub mask: u64,
    pub alpha_to_coverage_enabled: bool,
} 