// HSL (Hue, Saturation, Lightness) color space implementation
use crate::color::Color;
use super::ColorSpace;
use super::rgb::RGB;

/// HSL color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSL {
    /// Hue component [0.0, 360.0)
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Lightness component [0.0, 1.0]
    pub l: f32,
}

impl HSL {
    /// Creates a new HSL color.
    ///
    /// * `h` - Hue in degrees [0.0, 360.0)
    /// * `s` - Saturation [0.0, 1.0]
    /// * `l` - Lightness [0.0, 1.0]
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            // Normalize hue to [0, 360) range
            h: h.rem_euclid(360.0),
            // Clamp saturation and lightness to [0, 1]
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }
    
    /// Linear interpolation between two HSL colors.
    ///
    /// This interpolates h, s, and l components individually.
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
        let s = self.s + (other.s - self.s) * t;
        let l = self.l + (other.l - self.l) * t;
        
        Self::new(h, s, l)
    }
    
    /// Adjusts the hue by adding the specified degrees.
    pub fn adjust_hue(&self, degrees: f32) -> Self {
        Self::new(self.h + degrees, self.s, self.l)
    }
    
    /// Adjusts the saturation by the specified amount.
    pub fn adjust_saturation(&self, amount: f32) -> Self {
        Self::new(self.h, (self.s + amount).clamp(0.0, 1.0), self.l)
    }
    
    /// Adjusts the lightness by the specified amount.
    pub fn adjust_lightness(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.l + amount).clamp(0.0, 1.0))
    }
}

impl ColorSpace for HSL {
    fn to_rgba(&self) -> Color {
        // HSL to RGB conversion algorithm
        // Based on the formula from https://en.wikipedia.org/wiki/HSL_and_HSV
        
        let h = self.h;
        let s = self.s;
        let l = self.l;
        
        if s <= 0.0 {
            // If saturation is 0, it's a shade of gray
            return Color::new(l, l, l, 1.0);
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
        
        Color::new(r, g, b, 1.0)
    }
    
    fn from_rgba(color: &Color) -> Self {
        // RGB to HSL conversion algorithm
        // Based on the formula from https://en.wikipedia.org/wiki/HSL_and_HSV
        
        let r = color.r;
        let g = color.g;
        let b = color.b;
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        // Calculate lightness
        let l = (max + min) / 2.0;
        
        // Calculate saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        
        // Calculate hue
        let h = if delta == 0.0 {
            0.0 // Achromatic (gray)
        } else if max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        
        Self::new(h, s, l)
    }
}

/// HSLA (Hue, Saturation, Lightness, Alpha) color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSLA {
    /// Hue component [0.0, 360.0)
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Lightness component [0.0, 1.0]
    pub l: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl HSLA {
    /// Creates a new HSLA color.
    ///
    /// * `h` - Hue in degrees [0.0, 360.0)
    /// * `s` - Saturation [0.0, 1.0]
    /// * `l` - Lightness [0.0, 1.0]
    /// * `a` - Alpha [0.0, 1.0]
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self {
            // Normalize hue to [0, 360) range
            h: h.rem_euclid(360.0),
            // Clamp saturation, lightness, and alpha to [0, 1]
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.h, self.s, self.l, alpha)
    }
    
    /// Linear interpolation between two HSLA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // Use HSL lerp for h, s, l components
        let hsl = HSL::new(self.h, self.s, self.l).lerp(&HSL::new(other.h, other.s, other.l), t);
        
        // Interpolate alpha separately
        let a = self.a + (other.a - self.a) * t;
        
        Self::new(hsl.h, hsl.s, hsl.l, a)
    }
}

impl ColorSpace for HSLA {
    fn to_rgba(&self) -> Color {
        let rgb = HSL::new(self.h, self.s, self.l).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let hsl = HSL::from_rgba(color);
        Self::new(hsl.h, hsl.s, hsl.l, color.a)
    }
}

// From implementations for convenience
impl From<HSL> for HSLA {
    fn from(hsl: HSL) -> Self {
        Self::new(hsl.h, hsl.s, hsl.l, 1.0)
    }
}

impl From<HSLA> for HSL {
    fn from(hsla: HSLA) -> Self {
        Self::new(hsla.h, hsla.s, hsla.l)
    }
}

impl From<Color> for HSL {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for HSLA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<HSL> for Color {
    fn from(hsl: HSL) -> Self {
        hsl.to_rgba()
    }
}

impl From<HSLA> for Color {
    fn from(hsla: HSLA) -> Self {
        hsla.to_rgba()
    }
}

// Add From<RGB> implementation for HSL
impl From<RGB> for HSL {
    fn from(rgb: RGB) -> Self {
        let r = rgb.r;
        let g = rgb.g;
        let b = rgb.b;
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        // Calculate lightness
        let l = (max + min) / 2.0;
        
        // Calculate saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        
        // Calculate hue
        let h = if delta == 0.0 {
            0.0 // Achromatic (gray)
        } else if max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        
        Self::new(h, s, l)
    }
} 