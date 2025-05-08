// LAB (L*a*b*) color space implementation
// This is a perceptually uniform color space designed to approximate human vision.
use crate::color::Color;
use super::ColorSpace;
use super::xyz::XYZ;

/// LAB color space representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LAB {
    /// L* component (lightness) [0.0, 100.0]
    pub l: f32,
    /// a* component (green-red) [-128.0, 127.0]
    pub a: f32,
    /// b* component (blue-yellow) [-128.0, 127.0]
    pub b: f32,
}

impl LAB {
    /// Creates a new LAB color.
    ///
    /// * `l` - Lightness component [0.0, 100.0]
    /// * `a` - Green-red component [-128.0, 127.0]
    /// * `b` - Blue-yellow component [-128.0, 127.0]
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self {
            l: l.clamp(0.0, 100.0),
            a: a.clamp(-128.0, 127.0),
            b: b.clamp(-128.0, 127.0),
        }
    }
    
    /// Linear interpolation between two LAB colors.
    ///
    /// LAB is designed to be perceptually uniform, making it ideal for interpolation.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            l: self.l + (other.l - self.l) * t,
            a: self.a + (other.a - self.a) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
    
    /// Converts this LAB color to XYZ color space.
    /// 
    /// Uses D65 white point by default.
    pub fn to_xyz(&self) -> XYZ {
        self.to_xyz_with_white_point(XYZ::D65)
    }
    
    /// Converts this LAB color to XYZ color space with a specific white point.
    pub fn to_xyz_with_white_point(&self, white_point: XYZ) -> XYZ {
        // L*a*b* to XYZ conversion
        let l = self.l;
        let a = self.a;
        let b = self.b;
        
        // Helper function for the inverse lab transformation
        let finv = |t: f32| -> f32 {
            if t > 0.206893 {
                t.powi(3)
            } else {
                (t - 16.0 / 116.0) / 7.787
            }
        };
        
        // Calculate intermediate values
        let l_adj = (l + 16.0) / 116.0;
        let x = white_point.x * finv(l_adj + a / 500.0);
        let y = white_point.y * finv(l_adj);
        let z = white_point.z * finv(l_adj - b / 200.0);
        
        XYZ::new(x, y, z)
    }
    
    /// Converts an XYZ color to LAB color space.
    /// 
    /// Uses D65 white point by default.
    pub fn from_xyz(xyz: &XYZ) -> Self {
        Self::from_xyz_with_white_point(xyz, XYZ::D65)
    }
    
    /// Converts an XYZ color to LAB color space with a specific white point.
    pub fn from_xyz_with_white_point(xyz: &XYZ, white_point: XYZ) -> Self {
        // XYZ to L*a*b* conversion
        
        // Calculate XYZ ratios relative to white point
        let x_ratio = xyz.x / white_point.x;
        let y_ratio = xyz.y / white_point.y;
        let z_ratio = xyz.z / white_point.z;
        
        // Helper function for the lab transformation
        let f = |t: f32| -> f32 {
            if t > 0.008856 {
                t.cbrt()
            } else {
                (7.787 * t) + (16.0 / 116.0)
            }
        };
        
        // Calculate LAB components
        let fx = f(x_ratio);
        let fy = f(y_ratio);
        let fz = f(z_ratio);
        
        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);
        
        Self::new(l, a, b)
    }
    
    /// Calculates the perceptual difference (deltaE) between two LAB colors.
    ///
    /// Uses the CIE94 formula which is more accurate than the simpler Euclidean distance.
    pub fn delta_e(&self, other: &LAB) -> f32 {
        // CIE94 formula for color difference
        // This is a simpler formula than CIEDE2000 but still good for most purposes
        
        let l1 = self.l;
        let a1 = self.a;
        let b1 = self.b;
        
        let l2 = other.l;
        let a2 = other.a;
        let b2 = other.b;
        
        let delta_l = l1 - l2;
        let delta_a = a1 - a2;
        let delta_b = b1 - b2;
        
        let c1 = (a1 * a1 + b1 * b1).sqrt();
        let c2 = (a2 * a2 + b2 * b2).sqrt();
        let delta_c = c1 - c2;
        
        let delta_h_squared = delta_a * delta_a + delta_b * delta_b - delta_c * delta_c;
        let delta_h = if delta_h_squared > 0.0 {
            delta_h_squared.sqrt()
        } else {
            0.0
        };
        
        // Weighting factors
        let sl = 1.0;
        let sc = 1.0 + 0.045 * c1;
        let sh = 1.0 + 0.015 * c1;
        
        let delta_l_term = delta_l / sl;
        let delta_c_term = delta_c / sc;
        let delta_h_term = delta_h / sh;
        
        (delta_l_term * delta_l_term + delta_c_term * delta_c_term + delta_h_term * delta_h_term).sqrt()
    }
}

impl ColorSpace for LAB {
    fn to_rgba(&self) -> Color {
        // LAB -> XYZ -> RGB conversion
        let xyz = self.to_xyz();
        xyz.to_rgba()
    }
    
    fn from_rgba(color: &Color) -> Self {
        // RGB -> XYZ -> LAB conversion
        let xyz = XYZ::from_rgba(color);
        Self::from_xyz(&xyz)
    }
}

/// LABA color space representation (LAB with Alpha).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LABA {
    /// L* component (lightness) [0.0, 100.0]
    pub l: f32,
    /// a* component (green-red) [-128.0, 127.0]
    pub a: f32,
    /// b* component (blue-yellow) [-128.0, 127.0]
    pub b: f32,
    /// Alpha component [0.0, 1.0]
    pub alpha: f32,
}

impl LABA {
    /// Creates a new LABA color.
    pub fn new(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self {
            l: l.clamp(0.0, 100.0),
            a: a.clamp(-128.0, 127.0),
            b: b.clamp(-128.0, 127.0),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
    
    /// Returns a new color with the given opacity.
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::new(self.l, self.a, self.b, alpha)
    }
    
    /// Linear interpolation between two LABA colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            l: self.l + (other.l - self.l) * t,
            a: self.a + (other.a - self.a) * t,
            b: self.b + (other.b - self.b) * t,
            alpha: self.alpha + (other.alpha - self.alpha) * t,
        }
    }
}

impl ColorSpace for LABA {
    fn to_rgba(&self) -> Color {
        let rgb = LAB::new(self.l, self.a, self.b).to_rgba();
        Color::new(rgb.r, rgb.g, rgb.b, self.alpha)
    }
    
    fn from_rgba(color: &Color) -> Self {
        let lab = LAB::from_rgba(color);
        Self::new(lab.l, lab.a, lab.b, color.a)
    }
}

// From implementations for convenience
impl From<LAB> for LABA {
    fn from(lab: LAB) -> Self {
        Self::new(lab.l, lab.a, lab.b, 1.0)
    }
}

impl From<LABA> for LAB {
    fn from(laba: LABA) -> Self {
        Self::new(laba.l, laba.a, laba.b)
    }
}

impl From<Color> for LAB {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<Color> for LABA {
    fn from(color: Color) -> Self {
        Self::from_rgba(&color)
    }
}

impl From<LAB> for Color {
    fn from(lab: LAB) -> Self {
        lab.to_rgba()
    }
}

impl From<LABA> for Color {
    fn from(laba: LABA) -> Self {
        laba.to_rgba()
    }
}

impl From<XYZ> for LAB {
    fn from(xyz: XYZ) -> Self {
        Self::from_xyz(&xyz)
    }
}

impl From<LAB> for XYZ {
    fn from(lab: LAB) -> Self {
        lab.to_xyz()
    }
} 