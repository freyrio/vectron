/*!
 * Color utilities for the rendering system
 */

/// Core color types and operations for the rendering system
/// This module centralizes color handling for the entire renderer

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 - 1.0)
    pub r: f32,
    /// Green component (0.0 - 1.0)
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    pub b: f32,
    /// Alpha component (0.0 - 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color with RGBA components
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a new color with RGB components and alpha=1.0
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    /// Create a new color from 8-bit RGBA values
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    /// Create a new color from 8-bit RGB values (with alpha=1.0)
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
    
    /// Create a new color from a hexadecimal value
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        let a = if hex > 0xFFFFFF { ((hex >> 24) & 0xFF) as u8 } else { 255 };
        
        Self::from_rgba8(r, g, b, a)
    }
    
    /// Create a new transparent color (RGBA 0,0,0,0)
    pub fn transparent() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
    
    /// Create a new black color (RGB 0,0,0)
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    
    /// Create a new white color (RGB 1,1,1)
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    
    /// Convert to sRGB vector [r, g, b, a]
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    /// Multiply this color by another
    pub fn multiply(&self, other: &Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
    
    /// Linear interpolation between this color and another
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
    
    /// Apply a premultiplied alpha to this color
    pub fn premultiply(&self) -> Self {
        Self {
            r: self.r * self.a,
            g: self.g * self.a,
            b: self.b * self.a,
            a: self.a,
        }
    }
    
    /// Set the alpha value while keeping the RGB components
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }
}

/// Common color constants
pub mod constants {
    use super::Color;
    
    // Basic colors
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    
    // Gray tones
    pub const GRAY_10: Color = Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 };
    pub const GRAY_20: Color = Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
    pub const GRAY_30: Color = Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 };
    pub const GRAY_40: Color = Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 };
    pub const GRAY_50: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const GRAY_60: Color = Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 };
    pub const GRAY_70: Color = Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 };
    pub const GRAY_80: Color = Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 };
    pub const GRAY_90: Color = Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
}

/// Linear gradient between two colors
#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub start_point: [f32; 2],
    pub end_point: [f32; 2],
    pub stops: Vec<GradientStop>,
}

/// A color stop in a gradient
#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

impl LinearGradient {
    /// Create a new linear gradient from start to end with two colors
    pub fn new(start: [f32; 2], end: [f32; 2], start_color: Color, end_color: Color) -> Self {
        Self {
            start_point: start,
            end_point: end,
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: start_color,
                },
                GradientStop {
                    position: 1.0,
                    color: end_color,
                },
            ],
        }
    }
    
    /// Add a color stop to the gradient
    pub fn add_stop(&mut self, position: f32, color: Color) -> &mut Self {
        // Insert sorted by position
        let pos = position.clamp(0.0, 1.0);
        let idx = self.stops.binary_search_by(|s| {
            s.position.partial_cmp(&pos).unwrap()
        }).unwrap_or_else(|e| e);
        
        self.stops.insert(idx, GradientStop {
            position: pos,
            color,
        });
        
        self
    }
} 