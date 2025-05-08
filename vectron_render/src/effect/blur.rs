// Blur effect implementation
//
// This file implements various blur effects that can be applied to rendered elements

use std::any::Any;
use std::fmt;
use crate::core::{Renderable, Effect, RenderError};

/// Direction for blur effect application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurDirection {
    /// Blur in horizontal direction only
    Horizontal,
    /// Blur in vertical direction only
    Vertical,
    /// Blur in both directions (full blur)
    Both,
}

/// Blur effect for visual elements
///
/// Applies a gaussian blur to the rendered element with a specified radius
/// and direction.
#[derive(Debug, Clone)]
pub struct Blur {
    /// The radius of the blur effect (larger values create more blur)
    pub radius: f32,
    
    /// The direction in which to apply the blur
    pub direction: BlurDirection,
}

impl Blur {
    /// Create a new blur effect
    pub fn new(radius: f32, direction: BlurDirection) -> Self {
        Self {
            radius: radius.max(0.0), // Ensure non-negative radius
            direction,
        }
    }
    
    /// Create a standard blur in both directions
    pub fn standard(radius: f32) -> Self {
        Self {
            radius: radius.max(0.0),
            direction: BlurDirection::Both,
        }
    }
    
    /// Create a horizontal-only blur
    pub fn horizontal(radius: f32) -> Self {
        Self {
            radius: radius.max(0.0),
            direction: BlurDirection::Horizontal,
        }
    }
    
    /// Create a vertical-only blur
    pub fn vertical(radius: f32) -> Self {
        Self {
            radius: radius.max(0.0),
            direction: BlurDirection::Vertical,
        }
    }
}

impl Effect for Blur {
    fn apply_pre(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // In a real implementation, this would set up offscreen buffers
        // and prepare for the blur operation
        Ok(())
    }
    
    fn apply_post(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // In a full implementation, this would apply the gaussian blur algorithm
        // to the rendered content based on radius and direction
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn is_batchable(&self) -> bool {
        // Blur typically requires separate render passes and can't be batched
        false
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

impl fmt::Display for Blur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir_str = match self.direction {
            BlurDirection::Horizontal => "horizontal",
            BlurDirection::Vertical => "vertical",
            BlurDirection::Both => "both",
        };
        
        write!(f, "Blur(radius: {}, direction: {})", self.radius, dir_str)
    }
} 