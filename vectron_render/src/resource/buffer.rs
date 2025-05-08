// Buffer management for Vectron Render
//
// This module provides functionality for managing vertex and index buffers.

use crate::core::types::{VertexBufferHandle, IndexBufferHandle};

/// Generic geometry buffer containing vertex and index data
#[derive(Debug, Clone)]
pub struct GeometryBuffer<T> {
    /// Vertex data
    pub vertices: Vec<T>,
    /// Index data (optional for non-indexed drawing)
    pub indices: Option<Vec<u32>>,
}

impl<T> GeometryBuffer<T> {
    /// Create a new geometry buffer with the given vertices and indices
    pub fn new(vertices: Vec<T>, indices: Option<Vec<u32>>) -> Self {
        Self { vertices, indices }
    }
    
    /// Create a new geometry buffer with just vertices (no indices)
    pub fn with_vertices(vertices: Vec<T>) -> Self {
        Self { vertices, indices: None }
    }
    
    /// Create a new geometry buffer with vertices and indices
    pub fn with_indices(vertices: Vec<T>, indices: Vec<u32>) -> Self {
        Self { vertices, indices: Some(indices) }
    }
    
    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    
    /// Get the number of indices, or 0 if no indices are present
    pub fn index_count(&self) -> usize {
        self.indices.as_ref().map_or(0, |indices| indices.len())
    }
    
    /// Check if this geometry uses indices
    pub fn is_indexed(&self) -> bool {
        self.indices.is_some()
    }
} 