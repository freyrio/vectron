// Shadow effect implementation
//
// This file implements a shadow effect that can be applied to rendered elements

use std::any::Any;
use std::fmt;
use crate::color::Color;
use crate::core::{Renderable, Effect, RenderError};
use crate::math::vector::Vec2;

/// Shadow effect for 2D elements
///
/// Adds a drop shadow behind the rendered element with specified color,
/// offset, and blur radius.
#[derive(Debug, Clone)]
pub struct Shadow {
    /// The color of the shadow
    pub color: Color,
    
    /// The offset of the shadow relative to the original element
    pub offset: Vec2,
    
    /// The blur radius of the shadow (larger values create softer shadows)
    pub blur_radius: f32,
}

impl Shadow {
    /// Create a new shadow effect
    pub fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius: blur_radius.max(0.0), // Ensure non-negative blur radius
        }
    }
    
    /// Create a simple shadow with default parameters
    pub fn simple(color: Color, distance: f32, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        let offset = Vec2::new(cos * distance, sin * distance);
        
        Self {
            color,
            offset,
            blur_radius: distance * 0.5, // A reasonable default blur radius
        }
    }
}

impl Effect for Shadow {
    fn apply_pre(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // Shadow is typically implemented by rendering the element first with shadow parameters
        // In a real implementation, this would configure the renderer for shadow drawing
        // E.g., create an offscreen buffer, apply blur, etc.
        Ok(())
    }
    
    fn apply_post(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // In a fully implemented renderer, this would composite the shadow with the main render
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn is_batchable(&self) -> bool {
        // Shadows typically require separate render passes and can't be batched
        false
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

impl fmt::Display for Shadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shadow(color: {}, offset: {}, blur: {})", 
               self.color, self.offset, self.blur_radius)
    }
} 