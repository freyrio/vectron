use crate::error::RenderError;
use crate::resources::GeometryData;
use crate::core::color::Color;
use crate::utils::math::Vec2;

/// Effect trait for special rendering effects
pub trait Effect: Send + Sync {
    /// Apply before the main drawing (e.g., shadows)
    fn apply_pre(&self, renderer: &mut dyn Renderer, drawable: &dyn Drawable) -> Result<(), RenderError> {
        Ok(()) // Default no-op implementation
    }
    
    /// Apply after the main drawing (e.g., glow effects)
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        Ok(()) // Default no-op implementation
    }
}

/// Shadow effect
pub struct Shadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
}

impl Shadow {
    /// Create a new shadow effect
    pub fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }
}

impl Effect for Shadow {
    fn apply_pre(&self, renderer: &mut dyn Renderer, drawable: &dyn Drawable) -> Result<(), RenderError> {
        // Save the original state
        renderer.push_state();
        
        // Apply shadow transform and settings
        renderer.translate(self.offset.x, self.offset.y);
        
        // Create shadow geometry
        let geometry = drawable.to_geometry()?;
        
        // Apply blur if needed
        if self.blur_radius > 0.0 {
            renderer.set_blur(self.blur_radius);
        }
        
        // Draw shadow with the shadow color
        renderer.fill_geometry(&geometry, &Paint::Solid(self.color), FillRule::NonZero)?;
        
        // Restore original state
        renderer.pop_state()?;
        
        Ok(())
    }
}

/// Glow effect
pub struct Glow {
    pub color: Color,
    pub radius: f32,
}

impl Glow {
    /// Create a new glow effect
    pub fn new(color: Color, radius: f32) -> Self {
        Self {
            color,
            radius,
        }
    }
}

impl Effect for Glow {
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        // Save the original state
        renderer.push_state();
        
        // Apply blur
        renderer.set_blur(self.radius);
        
        // Draw a slightly expanded geometry with the glow color
        // (Implementation would depend on ability to expand geometries)
        renderer.fill_geometry(geometry, &Paint::Solid(self.color), FillRule::NonZero)?;
        
        // Restore original state
        renderer.pop_state()?;
        
        Ok(())
    }
}

/// Blur effect
pub struct Blur {
    pub radius: f32,
}

impl Blur {
    /// Create a new blur effect
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
        }
    }
}

impl Effect for Blur {
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        // Set blur radius
        renderer.set_blur(self.radius);
        
        Ok(())
    }
}

// Import dependencies
use crate::core::style::{Renderer, Paint, FillRule};
use crate::core::drawable::Drawable; 