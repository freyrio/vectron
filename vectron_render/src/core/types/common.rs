use std::fmt::Debug;
use std::hash::Hash;
use super::uuid::Uuid;

/// Dimensionality of coordinate space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimensionality {
    D2,
    D3,
}

/// A clipping region for constraining rendering
#[derive(Debug, Clone)]
pub enum ClipRegion {
    /// No clipping (render everything)
    None,
    
    /// Clip to a rectangular region for 2D rendering
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    
    /// Clip to a custom path for 2D rendering
    Path {
        /// Handle to a pre-tessellated path used for clipping
        path_handle: Uuid,
    },
    
    /// Clip to a volume for 3D rendering
    Volume {
        /// Handle to a pre-tessellated volume used for clipping
        volume_handle: Uuid,
    },
}

/// Filter mode for texture sampling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

/// Compare function for depth/stencil operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

/// Index format for indexed drawing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

/// Direction of face culling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Front,
    Back,
}

/// Front face vertex winding order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    Ccw,
    Cw,
}

/// Primitive topology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

/// Polygon fill mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

/// An enum representing different modes of culling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
    All,
}

/// Quality settings for tessellation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TessellationQuality {
    Fast,
    Medium,
    High,
    Ultra,
}

/// Resource type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Buffer,
    Texture,
    Sampler,
    Pipeline,
    BindGroup,
    Shader,
}

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // Color formats with 8 bits per channel
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    // Add more formats as needed
}