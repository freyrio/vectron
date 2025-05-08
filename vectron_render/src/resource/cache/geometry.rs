// Geometry cache for Vectron Render
//
// This module provides caching for geometry resources.

use std::collections::HashMap;
use crate::core::{RenderError, GeometryHandle, VertexBufferHandle, IndexBufferHandle};
use crate::resource::buffer::GeometryBuffer;
use crate::resource::upload::PendingUpload;


/// Cache for geometry resources
#[derive(Debug)]
pub struct GeometryCache<V> {
    /// Stored geometry data mapped by handle
    geometries: HashMap<GeometryHandle, GeometryBuffer<V>>,
    
    /// Mapping from geometry handles to GPU buffer handles (vertex, index)
    gpu_buffers: HashMap<GeometryHandle, (VertexBufferHandle, Option<IndexBufferHandle>)>,
}

impl<V: Clone + 'static> Default for GeometryCache<V> {
    fn default() -> Self {
        Self {
            geometries: HashMap::new(),
            gpu_buffers: HashMap::new(),
        }
    }
}

impl<V: Clone + 'static> GeometryCache<V> {
    /// Create a new geometry cache
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a geometry buffer and get a handle to it
    pub fn register_geometry(&mut self, geometry: GeometryBuffer<V>) -> GeometryHandle {
        let handle = GeometryHandle::new();
        self.geometries.insert(handle, geometry);
        handle
    }
    
    /// Update an existing geometry buffer
    pub fn update_geometry(&mut self, handle: GeometryHandle, geometry: GeometryBuffer<V>) -> Result<(), RenderError> {
        if !self.geometries.contains_key(&handle) {
            return Err(RenderError::ResourceNotFound(format!("Geometry handle {:?} not found", handle)));
        }
        
        self.geometries.insert(handle, geometry);
        Ok(())
    }
    
    /// Get a reference to a geometry buffer
    pub fn get_geometry(&self, handle: GeometryHandle) -> Option<&GeometryBuffer<V>> {
        self.geometries.get(&handle)
    }
    
    /// Get a mutable reference to a geometry buffer
    pub fn get_geometry_mut(&mut self, handle: GeometryHandle) -> Option<&mut GeometryBuffer<V>> {
        self.geometries.get_mut(&handle)
    }
    
    /// Check if a geometry handle is valid
    pub fn contains_geometry(&self, handle: GeometryHandle) -> bool {
        self.geometries.contains_key(&handle)
    }
    
    /// Remove a geometry from the cache
    pub fn remove_geometry(&mut self, handle: GeometryHandle) -> Option<GeometryBuffer<V>> {
        self.gpu_buffers.remove(&handle);
        self.geometries.remove(&handle)
    }
    
    /// Associate GPU buffers with a geometry handle
    pub fn associate_gpu_buffers(
        &mut self, 
        handle: GeometryHandle, 
        vertex_handle: VertexBufferHandle, 
        index_handle: Option<IndexBufferHandle>
    ) -> Result<(), RenderError> {
        if !self.geometries.contains_key(&handle) {
            return Err(RenderError::ResourceNotFound(format!("Geometry handle {:?} not found", handle)));
        }
        
        self.gpu_buffers.insert(handle, (vertex_handle, index_handle));
        Ok(())
    }
    
    /// Get the GPU buffers associated with a geometry handle
    pub fn get_gpu_buffers(&self, handle: GeometryHandle) -> Option<(VertexBufferHandle, Option<IndexBufferHandle>)> {
        self.gpu_buffers.get(&handle).cloned()
    }
    
    /// Generate pending uploads for a geometry
    pub fn generate_uploads(&self, handle: GeometryHandle) -> Result<Vec<PendingUpload>, RenderError> {
        let geometry = self.get_geometry(handle)
            .ok_or_else(|| RenderError::ResourceNotFound(format!("Geometry handle {:?} not found", handle)))?;
        
        let (vertex_handle, index_handle) = self.get_gpu_buffers(handle)
            .ok_or_else(|| RenderError::ResourceNotFound(format!("GPU buffers for geometry {:?} not found", handle)))?;
        
        let mut uploads = Vec::new();
        
        // Create vertex buffer upload
        let vertex_data = unsafe {
            std::slice::from_raw_parts(
                geometry.vertices.as_ptr() as *const u8,
                geometry.vertices.len() * std::mem::size_of::<V>(),
            ).to_vec()
        };
        
        uploads.push(PendingUpload::new_buffer(vertex_handle.id(), vertex_data, 0));
        
        // Create index buffer upload if present
        if let (Some(indices), Some(index_handle)) = (&geometry.indices, index_handle) {
            let index_data = unsafe {
                std::slice::from_raw_parts(
                    indices.as_ptr() as *const u8,
                    indices.len() * std::mem::size_of::<u32>(),
                ).to_vec()
            };
            
            uploads.push(PendingUpload::new_buffer(index_handle.id(), index_data, 0));
        }
        
        Ok(uploads)
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.geometries.clear();
        self.gpu_buffers.clear();
    }
} 