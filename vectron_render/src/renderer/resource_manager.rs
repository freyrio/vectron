// Resource manager implementation for Vectron Render
//
// This module handles GPU resource management during rendering.

use std::collections::HashMap;
use crate::core::error::RenderError;
use crate::core::types::{GeometryHandle, TextureHandle, BufferHandle, PipelineHandle, ShaderHandle, BindGroupHandle};
use crate::backend::resource::{BufferUsage, TextureUsage};
use crate::backend::device::BackendDevice;

/// The ResourceManager handles GPU resources during rendering.
/// It tracks which resources are in use, manages uploads, and handles
/// resource lifecycle.
pub struct ResourceManager<'a> {
    /// Reference to the backend device
    device: &'a mut dyn BackendDevice,
    
    /// Map of geometry handles to buffer handles (vertex and index)
    geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>,
    
    /// Pending uploads to be processed
    pending_uploads: Vec<PendingUpload>,
    
    /// Currently bound resources (to avoid redundant binds)
    current_pipeline: Option<PipelineHandle>,
    current_vertex_buffers: HashMap<u32, (BufferHandle, u64)>,
    current_index_buffer: Option<(BufferHandle, u64)>,
    current_bind_groups: HashMap<u32, BindGroupHandle>,
}

/// Represents a resource pending upload to the GPU
enum PendingUpload {
    Buffer {
        handle: BufferHandle,
        data: Vec<u8>,
        offset: u64,
    },
    Texture {
        handle: TextureHandle,
        data: Vec<u8>,
        width: u32,
        height: u32,
        depth: u32,
        offset_x: u32,
        offset_y: u32,
        offset_z: u32,
    },
}

impl<'a> ResourceManager<'a> {
    /// Create a new resource manager with the provided backend device
    pub fn new(device: &'a mut dyn BackendDevice) -> Self {
        Self {
            device,
            geometry_buffers: HashMap::new(),
            pending_uploads: Vec::new(),
            current_pipeline: None,
            current_vertex_buffers: HashMap::new(),
            current_index_buffer: None,
            current_bind_groups: HashMap::new(),
        }
    }
    
    /// Register geometry with vertex and index data
    pub fn register_geometry(&mut self, 
                           vertex_data: &[u8], 
                           index_data: &[u8]) -> Result<GeometryHandle, RenderError> {
        // Create vertex buffer
        let vertex_buffer = self.device.create_buffer(
            vertex_data.len() as u64,
            BufferUsage::VERTEX,
            None,
        )?;
        
        // Create index buffer
        let index_buffer = self.device.create_buffer(
            index_data.len() as u64,
            BufferUsage::INDEX,
            None,
        )?;
        
        // Queue data upload
        self.pending_uploads.push(PendingUpload::Buffer {
            handle: vertex_buffer,
            data: vertex_data.to_vec(),
            offset: 0,
        });
        
        self.pending_uploads.push(PendingUpload::Buffer {
            handle: index_buffer,
            data: index_data.to_vec(),
            offset: 0,
        });
        
        // Create a new handle
        let geometry_handle = GeometryHandle::new();
        
        // Store the mapping
        self.geometry_buffers.insert(geometry_handle, (vertex_buffer, index_buffer));
        
        Ok(geometry_handle)
    }
    
    /// Get the GPU buffers for a geometry handle
    pub fn get_geometry_buffers(&self, handle: GeometryHandle) -> Option<(BufferHandle, BufferHandle)> {
        self.geometry_buffers.get(&handle).cloned()
    }
    
    /// Update geometry data
    pub fn update_geometry(&mut self, 
                         handle: GeometryHandle, 
                         vertex_data: Option<&[u8]>, 
                         index_data: Option<&[u8]>) -> Result<(), RenderError> {
        if let Some((vertex_buffer, index_buffer)) = self.geometry_buffers.get(&handle) {
            // Queue vertex data update if provided
            if let Some(data) = vertex_data {
                self.pending_uploads.push(PendingUpload::Buffer {
                    handle: *vertex_buffer,
                    data: data.to_vec(),
                    offset: 0,
                });
            }
            
            // Queue index data update if provided
            if let Some(data) = index_data {
                self.pending_uploads.push(PendingUpload::Buffer {
                    handle: *index_buffer,
                    data: data.to_vec(),
                    offset: 0,
                });
            }
            
            Ok(())
        } else {
            Err(RenderError::InvalidHandle("Geometry handle not found".to_string()))
        }
    }
    
    /// Create a texture with the provided data
    pub fn create_texture(&mut self, 
                        width: u32, 
                        height: u32, 
                        format: crate::core::types::TextureFormat, 
                        data: Option<&[u8]>) -> Result<TextureHandle, RenderError> {
        // Create the texture
        let texture = self.device.create_texture(
            width,
            height,
            1,
            1,
            format,
            TextureUsage::TEXTURE_BINDING | TextureUsage::COPY_DST,
        )?;
        
        // Queue data upload if provided
        if let Some(data) = data {
            self.pending_uploads.push(PendingUpload::Texture {
                handle: texture,
                data: data.to_vec(),
                width,
                height,
                depth: 1,
                offset_x: 0,
                offset_y: 0,
                offset_z: 0,
            });
        }
        
        Ok(texture)
    }
    
    /// Set the current pipeline
    pub fn set_pipeline(&mut self, pipeline: PipelineHandle) -> Result<(), RenderError> {
        // Check if already bound
        if self.current_pipeline == Some(pipeline) {
            return Ok(());
        }
        
        // Update state and bind
        self.device.set_pipeline(pipeline)?;
        self.current_pipeline = Some(pipeline);
        
        Ok(())
    }
    
    /// Set a vertex buffer
    pub fn set_vertex_buffer(&mut self, 
                           slot: u32, 
                           buffer: BufferHandle, 
                           offset: u64) -> Result<(), RenderError> {
        // Check if already bound at the same offset
        if let Some((current_buffer, current_offset)) = self.current_vertex_buffers.get(&slot) {
            if *current_buffer == buffer && *current_offset == offset {
                return Ok(());
            }
        }
        
        // Update state and bind
        self.device.set_vertex_buffer(slot, buffer, offset)?;
        self.current_vertex_buffers.insert(slot, (buffer, offset));
        
        Ok(())
    }
    
    /// Set an index buffer
    pub fn set_index_buffer(&mut self, 
                          buffer: BufferHandle, 
                          offset: u64) -> Result<(), RenderError> {
        // Check if already bound at the same offset
        if let Some((current_buffer, current_offset)) = self.current_index_buffer {
            if current_buffer == buffer && current_offset == offset {
                return Ok(());
            }
        }
        
        // Update state and bind
        self.device.set_index_buffer(buffer, offset, crate::core::types::IndexFormat::Uint32)?;
        self.current_index_buffer = Some((buffer, offset));
        
        Ok(())
    }
    
    /// Set a bind group
    pub fn set_bind_group(&mut self, 
                        index: u32, 
                        bind_group: BindGroupHandle) -> Result<(), RenderError> {
        // Check if already bound
        if let Some(current_bind_group) = self.current_bind_groups.get(&index) {
            if *current_bind_group == bind_group {
                return Ok(());
            }
        }
        
        // Update state and bind
        self.device.set_bind_group(index, bind_group)?;
        self.current_bind_groups.insert(index, bind_group);
        
        Ok(())
    }
    
    /// Process pending uploads to the GPU
    pub fn process_uploads(&mut self) -> Result<(), RenderError> {
        for upload in self.pending_uploads.drain(..) {
            match upload {
                PendingUpload::Buffer { handle, data, offset } => {
                    self.device.update_buffer(handle, &data, offset)?;
                },
                PendingUpload::Texture { handle, data, width, height, depth, offset_x, offset_y, offset_z } => {
                    self.device.update_texture(handle, &data, width, height, depth, offset_x, offset_y, offset_z)?;
                },
            }
        }
        
        Ok(())
    }
    
    /// Reset the resource manager state for a new frame
    pub fn reset_state(&mut self) {
        self.current_pipeline = None;
        self.current_vertex_buffers.clear();
        self.current_index_buffer = None;
        self.current_bind_groups.clear();
    }
} 