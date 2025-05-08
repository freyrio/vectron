// Logical units implementation for Vectron Render
//
// This module provides implementations for relative units like em, rem, vh, vw, etc.

use super::types::{Unit, UnitKind};
use std::num::NonZeroU32;

/// Trait for creating logical units
pub trait LogicalUnitFactory<T = f32> {
    /// Create a percent unit (relative to parent container)
    fn percent(value: T) -> Unit<T>;
    
    /// Create an em unit (relative to current font size)
    fn em(value: T) -> Unit<T>;
    
    /// Create a rem unit (relative to root font size)
    fn rem(value: T) -> Unit<T>;
    
    /// Create a viewport width unit (percentage of viewport width)
    fn vw(value: T) -> Unit<T>;
    
    /// Create a viewport height unit (percentage of viewport height)
    fn vh(value: T) -> Unit<T>;
    
    /// Create a viewport minimum unit (percentage of smaller viewport dimension)
    fn vmin(value: T) -> Unit<T>;
    
    /// Create a viewport maximum unit (percentage of larger viewport dimension)
    fn vmax(value: T) -> Unit<T>;
}

impl<T> LogicalUnitFactory<T> for Unit<T> {
    fn percent(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Percent)
    }
    
    fn em(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Em)
    }
    
    fn rem(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Rem)
    }
    
    fn vw(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::ViewportWidth)
    }
    
    fn vh(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::ViewportHeight)
    }
    
    fn vmin(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::ViewportMin)
    }
    
    fn vmax(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::ViewportMax)
    }
}

/// Creates a percent unit (relative to parent container)
pub fn percent<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Percent)
}

/// Creates an em unit (relative to current font size)
pub fn em<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Em)
}

/// Creates a rem unit (relative to root font size)
pub fn rem<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Rem)
}

/// Creates a viewport width unit (percentage of viewport width)
pub fn vw<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::ViewportWidth)
}

/// Creates a viewport height unit (percentage of viewport height)
pub fn vh<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::ViewportHeight)
}

/// Creates a viewport minimum unit (percentage of smaller viewport dimension)
pub fn vmin<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::ViewportMin)
}

/// Creates a viewport maximum unit (percentage of larger viewport dimension)
pub fn vmax<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::ViewportMax)
}

/// Creates a unit that represents a fraction (numerator/denominator)
pub fn fraction<T: std::ops::Div<Output = T> + std::ops::Mul<Output = T> + From<f32> + Copy>(
    numerator: u32,
    denominator: NonZeroU32,
    kind: UnitKind,
) -> Unit<T> {
    let value = T::from(numerator as f32) / T::from(denominator.get() as f32) * T::from(100.0);
    Unit::new(value, kind)
} 