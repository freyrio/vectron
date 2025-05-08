use std::any::Any;
use crate::core::types::{TessellationQuality, CullMode, Dimensionality};
use crate::core::traits::renderable::Renderable;
use crate::core::geometry::bounds::BoundingVolume;

/// Options controlling the tessellation process
#[derive(Debug, Clone)]
pub struct TessellationOptions {
    /// Controls the level of detail for tessellation
    pub quality: TessellationQuality,
    
    /// Maximum error tolerance for curve approximation
    pub tolerance: f32,
    
    /// Whether to generate normal vectors
    pub generate_normals: bool,
    
    /// Whether to generate texture coordinates
    pub generate_uvs: bool,
    
    /// Face culling mode
    pub cull_mode: CullMode,
}

impl Default for TessellationOptions {
    fn default() -> Self {
        Self {
            quality: TessellationQuality::Medium,
            tolerance: 0.1,
            generate_normals: true,
            generate_uvs: true,
            cull_mode: CullMode::Back,
        }
    }
}

/// Statistical information about tessellated geometry
#[derive(Debug, Clone, Default)]
pub struct TessellationStats {
    /// Number of vertices generated
    pub vertex_count: usize,
    
    /// Number of indices generated
    pub index_count: usize,
    
    /// Number of triangles generated
    pub triangle_count: usize,
}

/// Result of tessellation process for 2D or 3D geometry
#[derive(Debug)]
pub enum TessellationResult {
    /// 2D tessellation result
    D2 {
        /// Vertex positions (xy)
        positions: Vec<[f32; 2]>,
        
        /// Optional vertex normals
        normals: Option<Vec<[f32; 2]>>,
        
        /// Optional texture coordinates
        uvs: Option<Vec<[f32; 2]>>,
        
        /// Indices defining triangles
        indices: Vec<u32>,
        
        /// Bounding volume of the tessellated geometry
        bounds: Box<dyn BoundingVolume>,
        
        /// Statistics about the tessellation
        stats: TessellationStats,
    },
    
    /// 3D tessellation result
    D3 {
        /// Vertex positions (xyz)
        positions: Vec<[f32; 3]>,
        
        /// Optional vertex normals
        normals: Option<Vec<[f32; 3]>>,
        
        /// Optional texture coordinates
        uvs: Option<Vec<[f32; 2]>>,
        
        /// Optional vertex tangents
        tangents: Option<Vec<[f32; 4]>>,
        
        /// Indices defining triangles
        indices: Vec<u32>,
        
        /// Bounding volume of the tessellated geometry
        bounds: Box<dyn BoundingVolume>,
        
        /// Statistics about the tessellation
        stats: TessellationStats,
    },
}

impl TessellationResult {
    /// Get the dimensionality of this tessellation result
    pub fn dimensionality(&self) -> Dimensionality {
        match self {
            TessellationResult::D2 { .. } => Dimensionality::D2,
            TessellationResult::D3 { .. } => Dimensionality::D3,
        }
    }
    
    /// Get the bounds of this tessellation result
    pub fn bounds(&self) -> &dyn BoundingVolume {
        match self {
            TessellationResult::D2 { bounds, .. } => bounds.as_ref(),
            TessellationResult::D3 { bounds, .. } => bounds.as_ref(),
        }
    }
    
    /// Get the statistics for this tessellation result
    pub fn stats(&self) -> &TessellationStats {
        match self {
            TessellationResult::D2 { stats, .. } => stats,
            TessellationResult::D3 { stats, .. } => stats,
        }
    }
}

/// Trait for objects that can be tessellated (converted to triangle meshes)
/// 
/// Tessellation is the process of converting high-level geometric shapes into
/// triangle meshes that can be efficiently processed by the GPU.
pub trait Tessellable: Renderable {
    /// Convert the element to GPU-ready geometry
    fn tessellate(&self, options: &TessellationOptions) -> TessellationResult;
    
    /// Returns self as Any for downcast operations
    fn as_tessellable_any(&self) -> &dyn Any;
} 