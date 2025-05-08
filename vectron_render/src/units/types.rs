// Unit type definitions for Vectron Render

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// Defines the different kinds of units supported by Vectron Render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitKind {
    /// Absolute pixels - device dependent
    Pixel,
    
    /// Points (1/72 inch)
    Point,
    
    /// Percent of parent container (0-100)
    Percent,
    
    /// Relative to font size
    Em,
    
    /// Relative to root font size
    Rem,
    
    /// Percentage of viewport width (0-100)
    ViewportWidth,
    
    /// Percentage of viewport height (0-100)
    ViewportHeight,
    
    /// Percentage of the smaller of viewport width/height
    ViewportMin,
    
    /// Percentage of the larger of viewport width/height
    ViewportMax,
    
    /// Inches - physical unit
    Inch,
    
    /// Centimeters - physical unit
    Centimeter,
    
    /// Millimeters - physical unit
    Millimeter,
}

impl fmt::Display for UnitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitKind::Pixel => write!(f, "px"),
            UnitKind::Point => write!(f, "pt"),
            UnitKind::Percent => write!(f, "%"),
            UnitKind::Em => write!(f, "em"),
            UnitKind::Rem => write!(f, "rem"),
            UnitKind::ViewportWidth => write!(f, "vw"),
            UnitKind::ViewportHeight => write!(f, "vh"),
            UnitKind::ViewportMin => write!(f, "vmin"),
            UnitKind::ViewportMax => write!(f, "vmax"),
            UnitKind::Inch => write!(f, "in"),
            UnitKind::Centimeter => write!(f, "cm"),
            UnitKind::Millimeter => write!(f, "mm"),
        }
    }
}

/// A value with an associated unit kind.
/// 
/// This struct represents a value in a specific unit, like "10px" or "2.5em".
/// Generic over the value type to support different numeric types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit<T = f32> {
    /// The kind of unit (px, pt, em, etc.)
    pub kind: UnitKind,
    
    /// The value in the given unit
    pub value: T,
}

impl<T> Unit<T> {
    /// Creates a new unit value.
    pub fn new(value: T, kind: UnitKind) -> Self {
        Self { kind, value }
    }
}

impl<T: fmt::Display> fmt::Display for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.kind)
    }
}

// Implement math operations for Unit when the inner type supports them

impl<T: Add<Output = T>> Add for Unit<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.kind, other.kind, "Cannot add units of different kinds");
        Self {
            kind: self.kind,
            value: self.value + other.value,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Unit<T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.kind, other.kind, "Cannot subtract units of different kinds");
        Self {
            kind: self.kind,
            value: self.value - other.value,
        }
    }
}

impl<T: Mul<f32, Output = T>> Mul<f32> for Unit<T> {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            kind: self.kind,
            value: self.value * scalar,
        }
    }
}

impl<T: Div<f32, Output = T>> Div<f32> for Unit<T> {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            kind: self.kind,
            value: self.value / scalar,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Unit<T> {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self {
            kind: self.kind,
            value: -self.value,
        }
    }
}

/// Represents a size with width and height units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<T = f32> {
    /// Width component
    pub width: Unit<T>,
    
    /// Height component
    pub height: Unit<T>,
}

impl<T> Size<T> {
    /// Creates a new size with the given width and height units.
    pub fn new(width: Unit<T>, height: Unit<T>) -> Self {
        Self { width, height }
    }
}

impl<T: fmt::Display> fmt::Display for Size<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
} 