// Basic color types for the Vectron Render engine
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents a color with red, green, blue, and alpha components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component [0.0, 1.0]
    pub r: f32,
    /// Green component [0.0, 1.0]
    pub g: f32,
    /// Blue component [0.0, 1.0]
    pub b: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl Color {
    /// Creates a new color with the specified components.
    ///
    /// All components should be in the range [0.0, 1.0].
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new opaque color (alpha = 1.0).
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Creates a new color from 8-bit component values.
    ///
    /// Each component is in the range [0, 255] and will be converted to float [0.0, 1.0].
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Creates a new opaque color from 8-bit component values.
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

    /// Creates a new color from a hexadecimal RGB value.
    ///
    /// The format is 0xRRGGBB with an alpha of 1.0.
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self::from_rgb8(r, g, b)
    }

    /// Creates a new color from a hexadecimal RGBA value.
    ///
    /// The format is 0xRRGGBBAA.
    pub fn from_hex_with_alpha(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as u8;
        Self::from_rgba8(r, g, b, a)
    }

    /// Converts this color to 8-bit component values.
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

    /// Returns the luminance of this color.
    pub fn luminance(&self) -> f32 {
        // Relative luminance in colorimetric spaces
        // https://en.wikipedia.org/wiki/Relative_luminance
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// Returns whether this color is considered "light" (for determining contrast).
    pub fn is_light(&self) -> bool {
        self.luminance() > 0.5
    }

    /// Returns the contrast color (black or white) based on this color's luminance.
    pub fn contrast_color(&self) -> Self {
        if self.is_light() {
            Self::BLACK
        } else {
            Self::WHITE
        }
    }

    /// Linear interpolation between two colors.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Pre-defined transparent color (all components = 0.0)
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    /// Pre-defined black color
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Pre-defined white color
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Pre-defined red color
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Pre-defined green color
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    /// Pre-defined blue color
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

// Display implementation for debugging
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b, a) = self.to_rgba8();
        write!(f, "rgba({}, {}, {}, {})", r, g, b, a)
    }
}

// Operator implementations for colors

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        };
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a * scalar,
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a * scalar,
        };
    }
}

// Implement Mul between colors (component-wise multiplication)
impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

// From implementations for convenience
impl From<(f32, f32, f32)> for Color {
    fn from(rgb: (f32, f32, f32)) -> Self {
        Self::rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self::new(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self::from_rgb8(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        Self::from_rgba8(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Self::from_hex(hex)
    }
}

/// Represents an alpha (transparency) value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alpha(pub f32);

impl Alpha {
    /// Creates a new alpha value clamped to the range [0.0, 1.0].
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Returns the alpha value.
    pub fn value(&self) -> f32 {
        self.0
    }

    /// Creates a fully opaque alpha value (1.0).
    pub const OPAQUE: Self = Self(1.0);

    /// Creates a fully transparent alpha value (0.0).
    pub const TRANSPARENT: Self = Self(0.0);
}

impl From<f32> for Alpha {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<Alpha> for f32 {
    fn from(alpha: Alpha) -> Self {
        alpha.0
    }
}

// This can be expanded with additional color types as needed 