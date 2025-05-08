// Color spaces module for Vectron Render
//
// This module provides implementations for various color spaces and conversions
// between them, following industry-standard formulas and conventions.

mod rgb;
mod cmyk;
mod hsv;
mod hsl;
mod lab;
mod xyz;
mod lch;

// Public re-exports
pub use rgb::*;
pub use cmyk::*;
pub use hsv::*;
pub use hsl::*;
pub use lab::*;
pub use xyz::*;
pub use lch::*;

/// Trait for color space conversions
pub trait ColorSpace: Sized {
    /// Convert to RGBA color
    fn to_rgba(&self) -> crate::color::Color;
    
    /// Create from RGBA color
    fn from_rgba(color: &crate::color::Color) -> Self;
} 