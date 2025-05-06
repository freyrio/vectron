use std::any::Any;
use std::fmt::Debug;
use crate::core::types::{Point2D, BoundingVolume, Rect, BoundingBox};
use crate::core::transform::Transform;

/// The foundational trait for any drawable element in the rendering system.
/// This defines the interface that all renderable elements must implement.
pub trait Renderable: Any + Debug + Send + Sync {
    /// Get the local origin point of the element
    fn origin(&self) -> Point2D;
    
    /// Set the local origin point of the element
    fn set_origin(&mut self, origin: Point2D);
    
    /// Get the element's bounding volume in local space (without any transforms applied)
    fn local_bounds(&self) -> BoundingVolume;
    
    /// Get the element's bounding volume in world space (with the given transform applied)
    fn world_bounds(&self, transform: &Transform) -> BoundingVolume;
    
    /// Apply a transform to the element relative to its origin
    fn transform_local(&mut self, transform: &Transform);
    
    /// Check if this renderable is a 2D element
    fn is_2d(&self) -> bool;
    
    /// Check if this renderable is a 3D element
    fn is_3d(&self) -> bool;
    
    /// Create a clone of this renderable
    fn clone_renderable(&self) -> Box<dyn Renderable>;
    
    /// Cast this renderable to Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Cast this renderable to mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A specialized trait for 2D renderable elements
pub trait Renderable2D: Renderable {
    /// Get the 2D bounding rectangle in local space
    fn local_rect(&self) -> Rect;
    
    /// Get the 2D bounding rectangle in world space
    fn world_rect(&self, transform: &Transform) -> Rect;
}

/// A specialized trait for 3D renderable elements
pub trait Renderable3D: Renderable {
    /// Get the 3D bounding box in local space
    fn local_box(&self) -> BoundingBox;
    
    /// Get the 3D bounding box in world space
    fn world_box(&self, transform: &Transform) -> BoundingBox;
}

/// Trait for elements that can be tessellated into geometry
pub trait Tessellable: Renderable {
    /// Convert the element to GPU-ready geometry
    fn tessellate(&self, options: &TessellationOptions) -> TessellationResult;
}

/// Options controlling the tessellation process
#[derive(Debug, Clone)]
pub struct TessellationOptions {
    /// Controls the level of detail for tessellation
    pub quality: TessellationQuality,
    
    /// Maximum error tolerance for tessellation
    pub tolerance: f32,
    
    /// Whether to generate normal vectors
    pub generate_normals: bool,
    
    /// Whether to generate UV coordinates
    pub generate_uvs: bool,
    
    /// Face culling mode
    pub cull_mode: CullMode,
}

/// Quality level for tessellation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TessellationQuality {
    /// Low quality, faster tessellation
    Low,
    
    /// Medium quality, balanced tessellation
    Medium,
    
    /// High quality, slower tessellation
    High,
    
    /// Custom quality with specific settings
    Custom,
}

/// Face culling mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    /// No face culling
    None,
    
    /// Cull front faces
    Front,
    
    /// Cull back faces
    Back,
}

/// Result of tessellation - can be either complete or incremental
#[derive(Debug)]
pub enum TessellationResult {
    /// Complete tessellation result with all data
    Complete(TessellationData),
    
    /// Incremental tessellation result for large meshes
    Incremental(VertexStream),
}

/// Data resulting from tessellation
#[derive(Debug, Clone)]
pub struct TessellationData {
    /// Vertex data
    pub vertices: Vec<Vertex>,
    
    /// Index data
    pub indices: Vec<u32>,
    
    /// Bounding volume of the tessellated geometry
    pub bounds: BoundingVolume,
    
    /// Statistics about the tessellation process
    pub stats: TessellationStats,
}

/// Basic vertex structure
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// Position of the vertex
    pub position: [f32; 3],
    
    /// Normal vector (if generated)
    pub normal: Option<[f32; 3]>,
    
    /// UV texture coordinates (if generated)
    pub uv: Option<[f32; 2]>,
    
    /// Vertex color
    pub color: [f32; 4],
}

/// Stream interface for incremental tessellation
#[derive(Debug)]
pub struct VertexStream {
    // Implementation details would depend on the specific requirements
    // This is a placeholder for the concept
}

/// Statistics about a tessellation operation
#[derive(Debug, Clone, Default)]
pub struct TessellationStats {
    /// Number of vertices in the tessellated geometry
    pub vertex_count: usize,
    
    /// Number of indices in the tessellated geometry
    pub index_count: usize,
    
    /// Number of triangles in the tessellated geometry
    pub triangle_count: usize,
    
    /// Time taken for tessellation (in milliseconds)
    pub tessellation_time_ms: f32,
} 