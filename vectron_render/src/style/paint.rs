// Paint implementations for Vectron Render
//
// This module provides structures for defining paints (colors, gradients, textures)
// that can be applied to shapes.

use std::fmt;
use crate::color::{Color, LinearGradient, RadialGradient, ConicGradient};
use crate::core::types::{TextureHandle, PatternHandle};

/// Different methods of painting elements (fill or stroke)
#[derive(Debug, Clone, PartialEq)]
pub enum Paint {
    /// Solid color paint
    Solid(Color),
    /// Linear gradient paint
    LinearGradient(LinearGradient),
    /// Radial gradient paint
    RadialGradient(RadialGradient),
    /// Conic gradient paint
    ConicGradient(ConicGradient),
    /// Texture paint (from an image)
    Texture(TextureHandle),
    /// Pattern paint (repeating elements)
    Pattern(PatternHandle),
}

impl Paint {
    /// Create a new solid color paint
    pub fn solid(color: impl Into<Color>) -> Self {
        Self::Solid(color.into())
    }
    
    /// Create a new linear gradient paint
    pub fn linear_gradient(gradient: LinearGradient) -> Self {
        Self::LinearGradient(gradient)
    }
    
    /// Create a new radial gradient paint
    pub fn radial_gradient(gradient: RadialGradient) -> Self {
        Self::RadialGradient(gradient)
    }
    
    /// Create a new conic gradient paint
    pub fn conic_gradient(gradient: ConicGradient) -> Self {
        Self::ConicGradient(gradient)
    }
    
    /// Create a new texture paint from a texture handle
    pub fn texture(handle: TextureHandle) -> Self {
        Self::Texture(handle)
    }
    
    /// Create a new pattern paint from a pattern handle
    pub fn pattern(handle: PatternHandle) -> Self {
        Self::Pattern(handle)
    }
    
    /// Returns whether this paint can be used in batched rendering
    pub fn is_batchable(&self) -> bool {
        match self {
            Self::Solid(_) => true,
            Self::Texture(_) => true,
            // Gradients and patterns typically require more state changes
            _ => false,
        }
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self::Solid(Color::BLACK)
    }
}

impl From<Color> for Paint {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<LinearGradient> for Paint {
    fn from(gradient: LinearGradient) -> Self {
        Self::LinearGradient(gradient)
    }
}

impl From<RadialGradient> for Paint {
    fn from(gradient: RadialGradient) -> Self {
        Self::RadialGradient(gradient)
    }
}

impl From<ConicGradient> for Paint {
    fn from(gradient: ConicGradient) -> Self {
        Self::ConicGradient(gradient)
    }
}

impl fmt::Display for Paint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solid(color) => write!(f, "Solid({})", color),
            Self::LinearGradient(gradient) => write!(f, "LinearGradient({})", gradient),
            Self::RadialGradient(gradient) => write!(f, "RadialGradient({})", gradient),
            Self::ConicGradient(gradient) => write!(f, "ConicGradient({})", gradient),
            Self::Texture(handle) => write!(f, "Texture({:?})", handle),
            Self::Pattern(handle) => write!(f, "Pattern({:?})", handle),
        }
    }
} 