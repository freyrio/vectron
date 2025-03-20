use crate::texture::TextureFormat;

#[derive(Debug, Clone, Copy)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub format: TextureFormat,
    pub offset: u64,
    pub shader_location: u32,
}

#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

// Vertex attribute format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
    Uint,
    Uint2,
    Uint3,
    Uint4,
    Unorm8x4,
    Snorm8x4,
    Unorm16x2,
    Snorm16x2,
    Unorm16x4,
    Snorm16x4,
}

/// Vertex attribute descriptor
#[derive(Debug, Clone)]
pub struct VertexAttributeDescriptor {
    pub format: VertexFormat,
    pub offset: usize,
    pub shader_location: u32,
}

/// Vertex layout descriptor
#[derive(Debug, Clone)]
pub struct VertexLayoutDescriptor {
    pub stride: usize,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttributeDescriptor>,
}