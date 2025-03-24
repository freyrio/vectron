/*!
 * Frame management for the standard rendering API
 */

use crate::api::bare::{DrawList, DrawCommand, DrawCommandType};
use crate::api::bare::resources::ResourceId;
use crate::api::bare::state::Rect;
use crate::error::RenderError;

/// A render frame that can contain multiple draw lists
pub struct Frame {
    /// Width of the frame in pixels
    width: u32,
    
    /// Height of the frame in pixels
    height: u32,
    
    /// Draw lists in this frame, in order of submission
    draw_lists: Vec<DrawList>,
    
    /// Clear color for the frame
    clear_color: Option<[f32; 4]>,
    
    /// Clear depth value
    clear_depth: Option<f32>,
    
    /// Clear stencil value
    clear_stencil: Option<u8>,
}

impl Frame {
    /// Create a new frame
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            draw_lists: Vec::new(),
            clear_color: Some([0.0, 0.0, 0.0, 1.0]),
            clear_depth: Some(1.0),
            clear_stencil: Some(0),
        }
    }
    
    /// Set the clear color for the frame
    pub fn clear_color(&mut self, color: Option<[f32; 4]>) -> &mut Self {
        self.clear_color = color;
        self
    }
    
    /// Set the clear depth for the frame
    pub fn clear_depth(&mut self, depth: Option<f32>) -> &mut Self {
        self.clear_depth = depth;
        self
    }
    
    /// Set the clear stencil for the frame
    pub fn clear_stencil(&mut self, stencil: Option<u8>) -> &mut Self {
        self.clear_stencil = stencil;
        self
    }
    
    /// Add a draw list to the frame
    pub fn add_draw_list(&mut self, draw_list: DrawList) -> &mut Self {
        self.draw_lists.push(draw_list);
        self
    }
    
    /// Set viewport dimensions for the frame
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }
    
    /// Get the width of the frame
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get the height of the frame
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Get all draw lists in the frame
    pub fn draw_lists(&self) -> &[DrawList] {
        &self.draw_lists
    }
    
    /// Generate a clear command for this frame
    pub fn generate_clear_command(&self) -> Option<DrawCommand> {
        // Only generate a clear command if any clear value is set
        if self.clear_color.is_none() && self.clear_depth.is_none() && self.clear_stencil.is_none() {
            return None;
        }
        
        Some(DrawCommand {
            command_type: DrawCommandType::Clear {
                color: self.clear_color.unwrap_or([0.0, 0.0, 0.0, 0.0]),
                depth: self.clear_depth,
                stencil: self.clear_stencil,
            },
            state: Default::default(),
        })
    }
} 