/*!
 * Geometry resource management
 */

use crate::api::bare::resources::BufferHandle;
use crate::error::RenderError;

/// Vertex definition
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// Types of geometry based on update frequency
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GeometryType {
    /// Static geometry that rarely changes
    Static,
    
    /// Dynamic geometry that changes occasionally
    Dynamic,
    
    /// Streamed geometry that changes every frame
    Streamed,
}

/// Geometry descriptor for creating geometry resources
#[derive(Clone, Debug)]
pub struct GeometryDesc {
    /// Vertex data
    pub vertices: Vec<Vertex>,
    
    /// Index data
    pub indices: Vec<u32>,
    
    /// Geometry type
    pub type_: GeometryType,
}

/// Geometry resource for rendering
#[derive(Debug)]
pub struct GeometryResource {
    /// Geometry descriptor
    pub(crate) desc: GeometryDesc,
    
    /// Vertex buffer on the GPU
    pub(crate) vertex_buffer: Option<BufferHandle>,
    
    /// Index buffer on the GPU
    pub(crate) index_buffer: Option<BufferHandle>,
    
    /// Whether the geometry needs to be updated on the GPU
    pub(crate) dirty: bool,
}

impl GeometryResource {
    /// Create a new geometry resource
    pub fn new(desc: GeometryDesc) -> Self {
        Self {
            desc,
            vertex_buffer: None,
            index_buffer: None,
            dirty: true,
        }
    }
    
    /// Update the vertices of the geometry
    pub fn update_vertices(&mut self, vertices: &[Vertex]) {
        self.desc.vertices = vertices.to_vec();
        self.dirty = true;
    }
    
    /// Update the indices of the geometry
    pub fn update_indices(&mut self, indices: &[u32]) {
        self.desc.indices = indices.to_vec();
        self.dirty = true;
    }
    
    /// Get the vertex data
    pub fn vertices(&self) -> &[Vertex] {
        &self.desc.vertices
    }
    
    /// Get the index data
    pub fn indices(&self) -> &[u32] {
        &self.desc.indices
    }
    
    /// Get the geometry type
    pub fn type_(&self) -> GeometryType {
        self.desc.type_
    }
    
    /// Check if the geometry needs to be updated on the GPU
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    
    /// Mark the geometry as clean (uploaded to GPU)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
} 