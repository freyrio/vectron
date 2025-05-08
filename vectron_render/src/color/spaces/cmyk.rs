// CMYK (Cyan, Magenta, Yellow, Key) color space implementation
use crate::color::Color;
use super::ColorSpace;

/// CMYK color space representation for print-oriented applications.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CMYK {
    /// Cyan component [0.0, 1.0]
    pub c: f32,
    /// Magenta component [0.0, 1.0]
    pub m: f32,
    /// Yellow component [0.0, 1.0]
    pub y: f32,
    /// Key (Black) component [0.0, 1.0]
    pub k: f32,
}

impl CMYK {
    /// Creates a new CMYK color.
    ///
    /// * `c` - Cyan [0.0, 1.0]
    /// * `m` - Magenta [0.0, 1.0]
    /// * `y` - Yellow [0.0, 1.0]
    /// * `k` - Key (Black) [0.0, 1.0]
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self {
            c: c.clamp(0.0, 1.0),
            m: m.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            k: k.clamp(0.0, 1.0),
        }
    }
    
    /// Creates a CMYK color from percentages (0-100 range).
    pub fn from_percentages(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self::new(c / 100.0, m / 100.0, y / 100.0, k / 100.0)
    }
    
    /// Convert CMYK values to percentages (0-100 range).
    pub fn to_percentages(&self) -> (f32, f32, f32, f32) {
        (
            self.c * 100.0,
            self.m * 100.0,
            self.y * 100.0,
            self.k * 100.0,
        )
    }
    
    /// Linear interpolation between two CMYK colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            c: self.c + (other.c - self.c) * t,
            m: self.m + (other.m - self.m) * t,
            y: self.y + (other.y - self.y) * t,
            k: self.k + (other.k - self.k) * t,
        }
    }
    
    /// Adjusts the black (Key) level while maintaining the overall color appearance.
    pub fn with_black_level(&self, k: f32) -> Self {
        let k = k.clamp(0.0, 1.0);
        let old_k = self.k;
        
        // If we're already at black (k = 1.0), return a color with the new k value
        if old_k >= 1.0 - f32::EPSILON {
            return Self::new(self.c, self.m, self.y, k);
        }
        
        // Adjust CMY values to maintain the same color appearance with a new K
        let scale = (1.0 - k) / (1.0 - old_k);
        Self::new(
            (self.c * scale).min(1.0),
            (self.m * scale).min(1.0),
            (self.y * scale).min(1.0),
            k,
        )
    }
}

impl ColorSpace for CMYK {
    fn to_rgba(&self) -> Color {
        // CMYK to RGB conversion
        // Formula: RGB = (1 - CMY) * (1 - K)
        let r = (1.0 - self.c) * (1.0 - self.k);
        let g = (1.0 - self.m) * (1.0 - self.k);
        let b = (1.0 - self.y) * (1.0 - self.k);
        
        Color::new(r, g, b, 1.0)
    }
    
    fn from_rgba(color: &Color) -> Self {
        // RGB to CMYK conversion
        let r = color.r;
        let g = color.g;
        let b = color.b;
        
        // Handle black separately
        if r < f32::EPSILON && g < f32::EPSILON && b < f32::EPSILON {
            return Self::new(0.0, 0.0, 0.0, 1.0);
        }
        
        // Calculate CMY
        let c = 1.0 - r;
        let m = 1.0 - g;
        let y = 1.0 - b;
        
        // Find K (minimum of C, M, Y)
        let k = c.min(m).min(y);
        
        // If K is 1, then CMY are all 0
        if (k - 1.0).abs() < f32::EPSILON {
            return Self::new(0.0, 0.0, 0.0, 1.0);
        }
        
        // Adjust CMY by removing K component
        let c_final = (c - k) / (1.0 - k);
        let m_final = (m - k) / (1.0 - k);
        let y_final = (y - k) / (1.0 - k);
        
        Self::new(c_final, m_final, y_final, k)
    }
}

/// CMYKA (CMYK with Alpha) color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CMYKA {
    /// Cyan component [0.0, 1.0]
    pub c: f32,
    /// Magenta component [0.0, 1.0]
    pub m: f32,
    /// Yellow component [0.0, 1.0]
    pub y: f32,
    /// Key (Black) component [0.0, 1.0]
    pub k: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl CMYKA {
    /// Creates a new CMYKA color.
    pub fn new(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        Self {
            c: c.clamp(0.0, 1.0),
            m: m.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            k: k.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }
    
    /// Creates a new CMYKA color from a CMYK color and alpha value.
    pub fn from_cmyk(cmyk: CMYK, a: f32) -> Self {
        Self::new(cmyk.c, cmyk.m, cmyk.y, cmyk.k, a)
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.c, self.m, self.y, self.k, alpha)
    }
    
    /// Linear interpolation between two CMYKA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            c: self.c + (other.c - self.c) * t,
            m: self.m + (other.m - self.m) * t,
            y: self.y + (other.y - self.y) * t,
            k: self.k + (other.k - self.k) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl ColorSpace for CMYKA {
    fn to_rgba(&self) -> Color {
        let rgb = CMYK::new(self.c, self.m, self.y, self.k).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let cmyk = CMYK::from_rgba(color);
        Self::new(cmyk.c, cmyk.m, cmyk.y, cmyk.k, color.a)
    }
}

// From implementations for convenience
impl From<CMYK> for CMYKA {
    fn from(cmyk: CMYK) -> Self {
        Self::new(cmyk.c, cmyk.m, cmyk.y, cmyk.k, 1.0)
    }
}

impl From<CMYKA> for CMYK {
    fn from(cmyka: CMYKA) -> Self {
        Self::new(cmyka.c, cmyka.m, cmyka.y, cmyka.k)
    }
}

impl From<Color> for CMYK {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for CMYKA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<CMYK> for Color {
    fn from(cmyk: CMYK) -> Self {
        cmyk.to_rgba()
    }
}

impl From<CMYKA> for Color {
    fn from(cmyka: CMYKA) -> Self {
        cmyka.to_rgba()
    }
} 