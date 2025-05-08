// HSV (Hue, Saturation, Value) color space implementation
use crate::color::Color;
use super::ColorSpace;

/// HSV color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    /// Hue component [0.0, 360.0)
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Value component [0.0, 1.0]
    pub v: f32,
}

impl HSV {
    /// Creates a new HSV color.
    ///
    /// * `h` - Hue in degrees [0.0, 360.0)
    /// * `s` - Saturation [0.0, 1.0]
    /// * `v` - Value [0.0, 1.0]
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self {
            // Normalize hue to [0, 360) range
            h: h.rem_euclid(360.0),
            // Clamp saturation and value to [0, 1]
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
        }
    }
    
    /// Linear interpolation between two HSV colors.
    ///
    /// This interpolates h, s, and v components individually.
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
        let v = self.v + (other.v - self.v) * t;
        
        Self::new(h, s, v)
    }
    
    /// Adjusts the hue by adding the specified degrees.
    pub fn adjust_hue(&self, degrees: f32) -> Self {
        Self::new(self.h + degrees, self.s, self.v)
    }
    
    /// Adjusts the saturation by the specified amount.
    pub fn adjust_saturation(&self, amount: f32) -> Self {
        Self::new(self.h, (self.s + amount).clamp(0.0, 1.0), self.v)
    }
    
    /// Adjusts the value by the specified amount.
    pub fn adjust_value(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.v + amount).clamp(0.0, 1.0))
    }
}

impl ColorSpace for HSV {
    fn to_rgba(&self) -> Color {
        // HSV to RGB conversion algorithm
        // Based on the formula from https://en.wikipedia.org/wiki/HSL_and_HSV
        
        let h = self.h;
        let s = self.s;
        let v = self.v;
        
        if s <= 0.0 {
            // If saturation is 0, it's a shade of gray
            return Color::new(v, v, v, 1.0);
        }
        
        let hh = (h % 360.0) / 60.0;  // Sector 0 to 5
        let i = hh.floor() as i32;    // Integer part of hue
        let ff = hh - i as f32;       // Fractional part of hue
        
        let p = v * (1.0 - s);
        let q = v * (1.0 - (s * ff));
        let t = v * (1.0 - (s * (1.0 - ff)));
        
        // Determine RGB based on the sector
        let (r, g, b) = match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q), // This covers i = 5
        };
        
        Color::new(r, g, b, 1.0)
    }
    
    fn from_rgba(color: &Color) -> Self {
        // RGB to HSV conversion algorithm
        // Based on the formula from https://en.wikipedia.org/wiki/HSL_and_HSV
        
        let r = color.r;
        let g = color.g;
        let b = color.b;
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        // Calculate value (brightness)
        let v = max;
        
        // Calculate saturation
        let s = if max != 0.0 { delta / max } else { 0.0 };
        
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
        
        Self::new(h, s, v)
    }
}

/// HSVA (Hue, Saturation, Value, Alpha) color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSVA {
    /// Hue component [0.0, 360.0)
    pub h: f32,
    /// Saturation component [0.0, 1.0]
    pub s: f32,
    /// Value component [0.0, 1.0]
    pub v: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl HSVA {
    /// Creates a new HSVA color.
    ///
    /// * `h` - Hue in degrees [0.0, 360.0)
    /// * `s` - Saturation [0.0, 1.0]
    /// * `v` - Value [0.0, 1.0]
    /// * `a` - Alpha [0.0, 1.0]
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            // Normalize hue to [0, 360) range
            h: h.rem_euclid(360.0),
            // Clamp saturation, value, and alpha to [0, 1]
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.h, self.s, self.v, alpha)
    }
    
    /// Linear interpolation between two HSVA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // Use HSV lerp for h, s, v components
        let hsv = HSV::new(self.h, self.s, self.v).lerp(&HSV::new(other.h, other.s, other.v), t);
        
        // Interpolate alpha separately
        let a = self.a + (other.a - self.a) * t;
        
        Self::new(hsv.h, hsv.s, hsv.v, a)
    }
}

impl ColorSpace for HSVA {
    fn to_rgba(&self) -> Color {
        let rgb = HSV::new(self.h, self.s, self.v).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let hsv = HSV::from_rgba(color);
        Self::new(hsv.h, hsv.s, hsv.v, color.a)
    }
}

// From implementations for convenience
impl From<HSV> for HSVA {
    fn from(hsv: HSV) -> Self {
        Self::new(hsv.h, hsv.s, hsv.v, 1.0)
    }
}

impl From<HSVA> for HSV {
    fn from(hsva: HSVA) -> Self {
        Self::new(hsva.h, hsva.s, hsva.v)
    }
}

impl From<Color> for HSV {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for HSVA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<HSV> for Color {
    fn from(hsv: HSV) -> Self {
        hsv.to_rgba()
    }
}

impl From<HSVA> for Color {
    fn from(hsva: HSVA) -> Self {
        hsva.to_rgba()
    }
} 