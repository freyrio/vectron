// Post-processing effect implementation
//
// This file implements post-processing effects that can be applied to rendered elements

use std::any::Any;
use std::fmt;
use crate::color::Color;
use crate::core::{Renderable, Effect, RenderError};

/// Common post-processing effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostProcessingType {
    /// Grayscale effect (desaturates colors)
    Grayscale,
    /// Sepia tone effect (applies a brown-yellow tint)
    Sepia,
    /// Invert colors effect
    Invert,
    /// Brightness adjustment
    Brightness,
    /// Contrast adjustment
    Contrast,
    /// Saturation adjustment
    Saturation,
    /// Custom shader effect
    Custom,
}

/// Post-processing effect for rendered elements
///
/// Applies a post-processing filter or effect to the rendered element.
/// These are typically applied after the element has been rendered to a texture.
#[derive(Debug, Clone)]
pub struct PostProcessing {
    /// The type of post-processing effect
    pub effect_type: PostProcessingType,
    
    /// The intensity of the effect (0.0 to 1.0, where 0.0 is no effect)
    pub intensity: f32,
    
    /// Optional color parameter for effects that require it
    pub color: Option<Color>,
    
    /// Custom shader identifier for custom effects
    pub shader_id: Option<String>,
}

impl PostProcessing {
    /// Create a new post-processing effect
    pub fn new(effect_type: PostProcessingType, intensity: f32) -> Self {
        Self {
            effect_type,
            intensity: intensity.max(0.0).min(1.0), // Clamp between 0 and 1
            color: None,
            shader_id: None,
        }
    }
    
    /// Create a new grayscale effect
    pub fn grayscale(intensity: f32) -> Self {
        Self::new(PostProcessingType::Grayscale, intensity)
    }
    
    /// Create a new sepia tone effect
    pub fn sepia(intensity: f32) -> Self {
        Self::new(PostProcessingType::Sepia, intensity)
    }
    
    /// Create a new invert colors effect
    pub fn invert(intensity: f32) -> Self {
        Self::new(PostProcessingType::Invert, intensity)
    }
    
    /// Create a new brightness adjustment
    pub fn brightness(value: f32) -> Self {
        Self::new(PostProcessingType::Brightness, value)
    }
    
    /// Create a new contrast adjustment
    pub fn contrast(value: f32) -> Self {
        Self::new(PostProcessingType::Contrast, value)
    }
    
    /// Create a new saturation adjustment
    pub fn saturation(value: f32) -> Self {
        Self::new(PostProcessingType::Saturation, value)
    }
    
    /// Create a new custom shader effect
    pub fn custom(shader_id: &str, intensity: f32) -> Self {
        let mut effect = Self::new(PostProcessingType::Custom, intensity);
        effect.shader_id = Some(shader_id.to_string());
        effect
    }
    
    /// Set a color parameter for the effect
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl Effect for PostProcessing {
    fn apply_pre(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // Most post-processing effects don't need pre-rendering setup
        Ok(())
    }
    
    fn apply_post(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        // In a full implementation, this would apply the selected post-processing effect
        // based on the effect_type and parameters
        Ok(())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn is_batchable(&self) -> bool {
        // Some post-processing effects might be batchable, but generally
        // they require separate render passes
        match self.effect_type {
            PostProcessingType::Brightness | 
            PostProcessingType::Contrast | 
            PostProcessingType::Saturation => true,
            _ => false,
        }
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

impl fmt::Display for PostProcessing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let effect_name = match self.effect_type {
            PostProcessingType::Grayscale => "Grayscale",
            PostProcessingType::Sepia => "Sepia",
            PostProcessingType::Invert => "Invert",
            PostProcessingType::Brightness => "Brightness",
            PostProcessingType::Contrast => "Contrast",
            PostProcessingType::Saturation => "Saturation",
            PostProcessingType::Custom => "Custom",
        };
        
        write!(f, "PostProcessing({}, intensity: {})", effect_name, self.intensity)
    }
} 