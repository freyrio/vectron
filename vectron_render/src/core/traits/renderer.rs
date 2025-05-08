// Renderer trait for Vectron Render
//
// This module defines the interface for rendering backends.

use std::any::Any;
use crate::core::error::RenderError;
use crate::core::types::{GeometryHandle, TextureHandle, BufferHandle, PipelineHandle, BindGroupHandle};
use crate::core::geometry::transform::Transform;
use crate::core::operation::render::RenderOperation;
use crate::backend::resource::{BufferUsage, TextureUsage, DepthStencilState, PrimitiveState, MultisampleState};

/// The Renderer trait defines the interface for rendering backends that can 
/// apply styles and draw geometry.
pub trait Renderer: Send + Sync {
    /// Apply a style to tessellated geometry
    fn apply_style(&mut self, style: &dyn Any, geometry_handle: GeometryHandle) -> Result<(), RenderError>;
    
    /// Draw tessellated geometry with the current state
    fn draw_geometry(&mut self, geometry_handle: GeometryHandle) -> Result<(), RenderError>;
    
    /// Draw a full render operation
    fn execute_operation(&mut self, operation: &RenderOperation) -> Result<(), RenderError>;
    
    /// Set the current transform
    fn set_transform(&mut self, transform: &Transform) -> Result<(), RenderError>;
    
    /// Get the current transform
    fn get_transform(&self) -> Transform;
    
    /// Push a new transform to the transform stack
    fn push_transform(&mut self, transform: &Transform) -> Result<(), RenderError>;
    
    /// Pop the transform stack
    fn pop_transform(&mut self) -> Result<(), RenderError>;
    
    /// Push a clip region
    fn push_clip(&mut self, bounds: &dyn Any) -> Result<(), RenderError>;
    
    /// Pop a clip region
    fn pop_clip(&mut self) -> Result<(), RenderError>;
    
    /// Begin a new render pass
    fn begin_render_pass(&mut self, 
                         color_attachments: &[Option<TextureHandle>], 
                         depth_stencil: Option<TextureHandle>) -> Result<(), RenderError>;
    
    /// End the current render pass
    fn end_render_pass(&mut self) -> Result<(), RenderError>;
    
    /// Set the active pipeline
    fn set_pipeline(&mut self, pipeline: PipelineHandle) -> Result<(), RenderError>;
    
    /// Set a bind group
    fn set_bind_group(&mut self, index: u32, bind_group: BindGroupHandle) -> Result<(), RenderError>;
    
    /// Set a vertex buffer
    fn set_vertex_buffer(&mut self, slot: u32, buffer: BufferHandle, offset: u64) -> Result<(), RenderError>;
    
    /// Set an index buffer
    fn set_index_buffer(&mut self, buffer: BufferHandle, offset: u64) -> Result<(), RenderError>;
    
    /// Draw primitives
    fn draw(&mut self, 
            vertex_count: u32, 
            instance_count: u32, 
            first_vertex: u32, 
            first_instance: u32) -> Result<(), RenderError>;
    
    /// Draw indexed primitives
    fn draw_indexed(&mut self, 
                   index_count: u32, 
                   instance_count: u32, 
                   first_index: u32, 
                   base_vertex: i32, 
                   first_instance: u32) -> Result<(), RenderError>;
    
    /// Create a buffer
    fn create_buffer(&mut self, 
                    size: u64, 
                    usage: BufferUsage, 
                    data: Option<&[u8]>) -> Result<BufferHandle, RenderError>;
    
    /// Update buffer data
    fn update_buffer(&mut self, 
                     buffer: BufferHandle, 
                     data: &[u8], 
                     offset: u64) -> Result<(), RenderError>;
    
    /// Create a texture
    fn create_texture(&mut self, 
                     width: u32, 
                     height: u32, 
                     depth: u32, 
                     mip_levels: u32, 
                     format: crate::core::types::TextureFormat, 
                     usage: TextureUsage) -> Result<TextureHandle, RenderError>;
    
    /// Update texture data
    fn update_texture(&mut self, 
                     texture: TextureHandle, 
                     data: &[u8], 
                     width: u32, 
                     height: u32, 
                     depth: u32, 
                     offset_x: u32, 
                     offset_y: u32, 
                     offset_z: u32) -> Result<(), RenderError>;
    
    /// Create a render pipeline
    fn create_pipeline(&mut self, 
                       vertex_shader: crate::core::types::ShaderHandle, 
                       fragment_shader: Option<crate::core::types::ShaderHandle>, 
                       vertex_layouts: &[crate::backend::resource::VertexBufferLayout], 
                       color_formats: &[Option<crate::core::types::TextureFormat>], 
                       depth_stencil: Option<DepthStencilState>, 
                       primitive: PrimitiveState, 
                       multisample: MultisampleState) -> Result<PipelineHandle, RenderError>;
    
    /// Begin a debug group
    fn push_debug_group(&mut self, label: &str) -> Result<(), RenderError>;
    
    /// End a debug group
    fn pop_debug_group(&mut self) -> Result<(), RenderError>;
    
    /// Clear the color attachment
    fn clear_color(&mut self, 
                  attachment: usize, 
                  color: [f32; 4]) -> Result<(), RenderError>;
    
    /// Clear the depth attachment
    fn clear_depth(&mut self, value: f32) -> Result<(), RenderError>;
    
    /// Clear the stencil attachment
    fn clear_stencil(&mut self, value: u32) -> Result<(), RenderError>;
    
    /// Get current viewport dimensions
    fn get_viewport_size(&self) -> (u32, u32);
    
    /// Set the viewport
    fn set_viewport(&mut self, 
                   x: f32, 
                   y: f32, 
                   width: f32, 
                   height: f32, 
                   min_depth: f32, 
                   max_depth: f32) -> Result<(), RenderError>;
    
    /// Set the scissor rect
    fn set_scissor_rect(&mut self, 
                        x: u32, 
                        y: u32, 
                        width: u32, 
                        height: u32) -> Result<(), RenderError>;
} 