use crate::core::{BufferHandle, PipelineHandle, BindGroupHandle, BackendError, IndexFormat, TextureHandle};
use std::any::Any;

use super::RenderPassDesc;

/// Command buffer trait for recording rendering commands
pub trait CommandBuffer: Send + Sync {
    /// Returns self as an Any trait object to allow downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Sets the active render pipeline
    fn set_pipeline(&mut self, pipeline: PipelineHandle) -> Result<(), BackendError>;
    
    /// Sets a vertex buffer at the specified slot
    fn set_vertex_buffer(&mut self, slot: u32, buffer: BufferHandle, offset: u64) -> Result<(), BackendError>;
    
    /// Sets the index buffer
    fn set_index_buffer(&mut self, buffer: BufferHandle, offset: u64, format: IndexFormat) -> Result<(), BackendError>;
    
    /// Sets a bind group at the specified index
    fn set_bind_group(&mut self, index: u32, bind_group: BindGroupHandle) -> Result<(), BackendError>;
    
    /// Sets push constants
    fn set_push_constants(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError>;
    
    /// Draws vertices without indexing
    fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> Result<(), BackendError>;
    
    /// Draws vertices with indexing
    fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32) -> Result<(), BackendError>;
    
    /// Begins a render pass
    fn begin_render_pass(&mut self, desc: &RenderPassDesc) -> Result<(), BackendError>;
    
    /// Ends the current render pass
    fn end_render_pass(&mut self) -> Result<(), BackendError>;
    
    /// Sets the viewport
    fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) -> Result<(), BackendError>;
    
    /// Sets the scissor rect
    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), BackendError>;
    
    /// Sets the blend constant color
    fn set_blend_constant(&mut self, color: [f32; 4]) -> Result<(), BackendError>;
    
    /// Sets the stencil reference value
    fn set_stencil_reference(&mut self, reference: u32) -> Result<(), BackendError>;
    
    /// Copies data from one buffer to another
    fn copy_buffer_to_buffer(
        &mut self,
        source: BufferHandle,
        source_offset: u64,
        destination: BufferHandle,
        destination_offset: u64,
        size: u64,
    ) -> Result<(), BackendError>;
    
    /// Copies data from a buffer to a texture
    fn copy_buffer_to_texture(
        &mut self,
        source: BufferHandle,
        source_offset: u64,
        destination: TextureHandle,
        destination_offset: [u32; 3],
        extent: [u32; 3],
    ) -> Result<(), BackendError>;
    
    /// Copies data from a texture to a buffer
    fn copy_texture_to_buffer(
        &mut self,
        source: TextureHandle,
        source_offset: [u32; 3],
        destination: BufferHandle,
        destination_offset: u64,
        extent: [u32; 3],
    ) -> Result<(), BackendError>;
    
    /// Copies data from one texture to another
    fn copy_texture_to_texture(
        &mut self,
        source: TextureHandle,
        source_offset: [u32; 3],
        destination: TextureHandle,
        destination_offset: [u32; 3],
        extent: [u32; 3],
    ) -> Result<(), BackendError>;
    
    /// Pushes a debug group with a label
    fn push_debug_group(&mut self, label: &str) -> Result<(), BackendError>;
    
    /// Pops the most recently pushed debug group
    fn pop_debug_group(&mut self) -> Result<(), BackendError>;
    
    /// Inserts a debug marker with a label
    fn insert_debug_marker(&mut self, label: &str) -> Result<(), BackendError>;
}

/// Command encoder trait for creating command buffers
pub trait CommandEncoder: Send + Sync {
    /// Begin encoding commands
    fn begin_encoding(&mut self) -> Result<(), BackendError>;
    
    /// Finish encoding and return the command buffer
    fn finish_encoding(&mut self) -> Result<Box<dyn CommandBuffer>, BackendError>;
} 