// LCH (Lightness, Chroma, Hue) color space implementation
// This is a cylindrical representation of LAB, similar to how HSL relates to RGB.
use crate::color::Color;
use super::ColorSpace;
use super::lab::LAB;
use std::f32::consts::PI;

/// LCH color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LCH {
    /// L* component (lightness) [0.0, 100.0]
    pub l: f32,
    /// C* component (chroma/saturation) [0.0, ~150.0]
    pub c: f32,
    /// h component (hue angle in degrees) [0.0, 360.0)
    pub h: f32,
}

impl LCH {
    /// Creates a new LCH color.
    ///
    /// * `l` - Lightness [0.0, 100.0]
    /// * `c` - Chroma [0.0, ~150.0] (although theoretical max is higher)
    /// * `h` - Hue in degrees [0.0, 360.0)
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        Self {
            l: l.clamp(0.0, 100.0),
            c: c.max(0.0), // Chroma can technically exceed 150 but is usually lower
            h: h.rem_euclid(360.0),
        }
    }
    
    /// Linear interpolation between two LCH colors.
    ///
    /// LCH is perceptually uniform, making it excellent for interpolation.
    /// For hue, it uses the shortest path around the color wheel.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // For hue, we need to handle the circular nature of the color wheel
        let mut h_diff = other.h - self.h;
        
        // Ensure we take the shorter path around the color wheel
        if h_diff > 180.0 {
            h_diff -= 360.0;
        } else if h_diff < -180.0 {
            h_diff += 360.0;
        }
        
        let h = (self.h + h_diff * t).rem_euclid(360.0);
        let l = self.l + (other.l - self.l) * t;
        let c = self.c + (other.c - self.c) * t;
        
        Self::new(l, c, h)
    }
    
    /// Adjusts the lightness by the specified amount.
    pub fn adjust_lightness(&self, amount: f32) -> Self {
        Self::new(self.l + amount, self.c, self.h)
    }
    
    /// Adjusts the chroma (saturation) by the specified amount.
    pub fn adjust_chroma(&self, amount: f32) -> Self {
        Self::new(self.l, self.c + amount, self.h)
    }
    
    /// Adjusts the hue by adding the specified degrees.
    pub fn adjust_hue(&self, degrees: f32) -> Self {
        Self::new(self.l, self.c, self.h + degrees)
    }
    
    /// Converts this LCH color to LAB color space.
    pub fn to_lab(&self) -> LAB {
        // LCH to LAB conversion
        // h is in degrees, convert to radians for trigonometric functions
        let h_rad = self.h * PI / 180.0;
        
        // Convert polar coordinates (C, h) to cartesian (a, b)
        let a = self.c * h_rad.cos();
        let b = self.c * h_rad.sin();
        
        LAB::new(self.l, a, b)
    }
    
    /// Converts a LAB color to LCH color space.
    pub fn from_lab(lab: &LAB) -> Self {
        // LAB to LCH conversion
        let l = lab.l;
        let a = lab.a;
        let b = lab.b;
        
        // Calculate chroma (distance from origin in a-b plane)
        let c = (a * a + b * b).sqrt();
        
        // Calculate hue angle
        let h = if c < f32::EPSILON {
            0.0 // Achromatic (gray) case, hue is undefined
        } else {
            let mut h = b.atan2(a) * 180.0 / PI;
            
            // Normalize to [0, 360) range
            if h < 0.0 {
                h += 360.0;
            }
            
            h
        };
        
        Self::new(l, c, h)
    }
}

impl ColorSpace for LCH {
    fn to_rgba(&self) -> Color {
        // LCH -> LAB -> RGB conversion
        let lab = self.to_lab();
        lab.to_rgba()
    }
    
    fn from_rgba(color: &Color) -> Self {
        // RGB -> LAB -> LCH conversion
        let lab = LAB::from_rgba(color);
        Self::from_lab(&lab)
    }
}

/// LCHA color space representation (LCH with Alpha).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LCHA {
    /// L* component (lightness) [0.0, 100.0]
    pub l: f32,
    /// C* component (chroma/saturation) [0.0, ~150.0]
    pub c: f32,
    /// h component (hue angle in degrees) [0.0, 360.0)
    pub h: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl LCHA {
    /// Creates a new LCHA color.
    pub fn new(l: f32, c: f32, h: f32, a: f32) -> Self {
        Self {
            l: l.clamp(0.0, 100.0),
            c: c.max(0.0),
            h: h.rem_euclid(360.0),
            a: a.clamp(0.0, 1.0),
        }
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.l, self.c, self.h, alpha)
    }
    
    /// Linear interpolation between two LCHA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // Use LCH lerp for l, c, h components
        let lch = LCH::new(self.l, self.c, self.h).lerp(&LCH::new(other.l, other.c, other.h), t);
        
        // Interpolate alpha separately
        let a = self.a + (other.a - self.a) * t;
        
        Self::new(lch.l, lch.c, lch.h, a)
    }
}

impl ColorSpace for LCHA {
    fn to_rgba(&self) -> Color {
        let rgb = LCH::new(self.l, self.c, self.h).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let lch = LCH::from_rgba(color);
        Self::new(lch.l, lch.c, lch.h, color.a)
    }
}

// From implementations for convenience
impl From<LCH> for LCHA {
    fn from(lch: LCH) -> Self {
        Self::new(lch.l, lch.c, lch.h, 1.0)
    }
}

impl From<LCHA> for LCH {
    fn from(lcha: LCHA) -> Self {
        Self::new(lcha.l, lcha.c, lcha.h)
    }
}

impl From<Color> for LCH {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for LCHA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<LCH> for Color {
    fn from(lch: LCH) -> Self {
        lch.to_rgba()
    }
}

impl From<LCHA> for Color {
    fn from(lcha: LCHA) -> Self {
        lcha.to_rgba()
    }
}

impl From<LAB> for LCH {
    fn from(lab: LAB) -> Self {
        Self::from_lab(&lab)
    }
}

impl From<LCH> for LAB {
    fn from(lch: LCH) -> Self {
        lch.to_lab()
    }
} 