// RGB color space implementation
use crate::color::Color;
use super::ColorSpace;
use super::hsl::HSL;

/// RGB color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    /// Red component [0.0, 1.0]
    pub r: f32,
    /// Green component [0.0, 1.0]
    pub g: f32,
    /// Blue component [0.0, 1.0]
    pub b: f32,
}

impl RGB {
    /// Creates a new RGB color.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    
    /// Creates a new RGB color from 8-bit component values [0-255].
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
    
    /// Converts to 8-bit RGB components.
    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        (
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
    
    /// Linear interpolation between two RGB colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

impl ColorSpace for RGB {
    fn to_rgba(&self) -> Color {
        Color::new(self.r, self.g, self.b, 1.0)
    }
    
    fn from_rgba(color: &Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

/// RGBA color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA {
    /// Red component [0.0, 1.0]
    pub r: f32,
    /// Green component [0.0, 1.0]
    pub g: f32,
    /// Blue component [0.0, 1.0]
    pub b: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl RGBA {
    /// Creates a new RGBA color.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Creates a new RGBA color from 8-bit component values [0-255].
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    /// Converts to 8-bit RGBA components.
    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.r, self.g, self.b, alpha)
    }
    
    /// Linear interpolation between two RGBA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl ColorSpace for RGBA {
    fn to_rgba(&self) -> Color {
        Color::new(self.r, self.g, self.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

// From implementations for convenience
impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, 1.0)
    }
}

impl From<RGBA> for RGB {
    fn from(rgba: RGBA) -> Self {
        Self::new(rgba.r, rgba.g, rgba.b)
    }
}

impl From<Color> for RGB {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for RGBA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        rgb.to_rgba()
    }
}

impl From<RGBA> for Color {
    fn from(rgba: RGBA) -> Self {
        rgba.to_rgba()
    }
}

// Add From<HSL> implementation for RGB
impl From<HSL> for RGB {
    fn from(hsl: HSL) -> Self {
        let h = hsl.h;
        let s = hsl.s;
        let l = hsl.l;
        
        if s <= 0.0 {
            // If saturation is 0, it's a shade of gray
            return Self::new(l, l, l);
        }
        
        // Helper function to convert hue to RGB
        let hue_to_rgb = |p: f32, q: f32, mut t: f32| {
            if t < 0.0 { t += 1.0; }
            if t > 1.0 { t -= 1.0; }
            
            if t < 1.0/6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0/2.0 {
                return q;
            }
            if t < 2.0/3.0 {
                return p + (q - p) * (2.0/3.0 - t) * 6.0;
            }
            p
        };
        
        let q = if l < 0.5 { 
            l * (1.0 + s) 
        } else { 
            l + s - l * s 
        };
        
        let p = 2.0 * l - q;
        let hk = h / 360.0;  // Convert hue to [0, 1] range
        
        let r = hue_to_rgb(p, q, hk + 1.0/3.0);
        let g = hue_to_rgb(p, q, hk);
        let b = hue_to_rgb(p, q, hk - 1.0/3.0);
        
        Self::new(r, g, b)
    }
} 