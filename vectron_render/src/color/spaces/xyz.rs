// XYZ color space implementation
// This is a device-independent color space that serves as an intermediate
// for conversion between other color spaces.
use crate::color::Color;
use super::ColorSpace;

/// XYZ color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ {
    /// X component
    pub x: f32,
    /// Y component (luminance)
    pub y: f32,
    /// Z component
    pub z: f32,
}

impl XYZ {
    /// Creates a new XYZ color.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { 
            x: x.max(0.0), 
            y: y.max(0.0), 
            z: z.max(0.0),
        }
    }
    
    /// Linear interpolation between two XYZ colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }
    
    /// Reference white point D65 in XYZ space
    pub const D65: Self = Self {
        x: 0.95047,
        y: 1.0,
        z: 1.08883,
    };
    
    /// Reference white point D50 in XYZ space
    pub const D50: Self = Self {
        x: 0.96422,
        y: 1.0,
        z: 0.82521,
    };
    
    /// Reference white point E (equal energy) in XYZ space
    pub const E: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl ColorSpace for XYZ {
    fn to_rgba(&self) -> Color {
        // XYZ to sRGB conversion
        // Using D65 reference white and sRGB matrix
        
        // XYZ to linear RGB transformation matrix
        // Matrix based on the sRGB standard with D65 reference white
        let x = self.x;
        let y = self.y;
        let z = self.z;
        
        // XYZ to linear RGB
        let r_linear = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g_linear = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b_linear = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
        
        // Apply sRGB gamma correction (linear to sRGB)
        // This is an approximation of the standard sRGB transfer function
        let gamma_correct = |v: f32| -> f32 {
            if v <= 0.0031308 {
                v * 12.92
            } else {
                1.055 * v.powf(1.0 / 2.4) - 0.055
            }
        };
        
        let r = gamma_correct(r_linear);
        let g = gamma_correct(g_linear);
        let b = gamma_correct(b_linear);
        
        // Clamp values to [0, 1] range
        Color::new(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            1.0,
        )
    }
    
    fn from_rgba(color: &Color) -> Self {
        // sRGB to XYZ conversion
        let r = color.r;
        let g = color.g;
        let b = color.b;
        
        // sRGB to linear RGB (inverse gamma correction)
        let inverse_gamma = |v: f32| -> f32 {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        
        let r_linear = inverse_gamma(r);
        let g_linear = inverse_gamma(g);
        let b_linear = inverse_gamma(b);
        
        // Linear RGB to XYZ transformation matrix
        // This is the inverse of the XYZ to RGB matrix
        let x = 0.4124564 * r_linear + 0.3575761 * g_linear + 0.1804375 * b_linear;
        let y = 0.2126729 * r_linear + 0.7151522 * g_linear + 0.0721750 * b_linear;
        let z = 0.0193339 * r_linear + 0.1191920 * g_linear + 0.9503041 * b_linear;
        
        Self::new(x, y, z)
    }
}

/// XYZA color space representation (XYZ with Alpha).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZA {
    /// X component
    pub x: f32,
    /// Y component (luminance)
    pub y: f32,
    /// Z component
    pub z: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl XYZA {
    /// Creates a new XYZA color.
    pub fn new(x: f32, y: f32, z: f32, a: f32) -> Self {
        Self { 
            x: x.max(0.0), 
            y: y.max(0.0), 
            z: z.max(0.0),
            a: a.clamp(0.0, 1.0),
        }
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.x, self.y, self.z, alpha)
    }
    
    /// Linear interpolation between two XYZA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl ColorSpace for XYZA {
    fn to_rgba(&self) -> Color {
        let rgb = XYZ::new(self.x, self.y, self.z).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.a)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let xyz = XYZ::from_rgba(color);
        Self::new(xyz.x, xyz.y, xyz.z, color.a)
    }
}

// From implementations for convenience
impl From<XYZ> for XYZA {
    fn from(xyz: XYZ) -> Self {
        Self::new(xyz.x, xyz.y, xyz.z, 1.0)
    }
}

impl From<XYZA> for XYZ {
    fn from(xyza: XYZA) -> Self {
        Self::new(xyza.x, xyza.y, xyza.z)
    }
}

impl From<Color> for XYZ {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for XYZA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<XYZ> for Color {
    fn from(xyz: XYZ) -> Self {
        xyz.to_rgba()
    }
}

impl From<XYZA> for Color {
    fn from(xyza: XYZA) -> Self {
        xyza.to_rgba()
    }
} 