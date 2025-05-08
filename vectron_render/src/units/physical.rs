// Physical units implementation for Vectron Render
//
// This module provides implementations for physical units like px, pt, in, cm, mm.

use super::types::{Unit, UnitKind};

/// Trait for creating physical units
pub trait PhysicalUnitFactory<T = f32> {
    /// Create a pixel unit
    fn px(value: T) -> Unit<T>;
    
    /// Create a point unit (1/72 inch)
    fn pt(value: T) -> Unit<T>;
    
    /// Create an inch unit
    fn inches(value: T) -> Unit<T>;
    
    /// Create a centimeter unit
    fn cm(value: T) -> Unit<T>;
    
    /// Create a millimeter unit
    fn mm(value: T) -> Unit<T>;
}

impl<T> PhysicalUnitFactory<T> for Unit<T> {
    fn px(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Pixel)
    }
    
    fn pt(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Point)
    }
    
    fn inches(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Inch)
    }
    
    fn cm(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Centimeter)
    }
    
    fn mm(value: T) -> Unit<T> {
        Unit::new(value, UnitKind::Millimeter)
    }
}

/// Creates a pixel unit
pub fn px<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Pixel)
}

/// Creates a point unit (1/72 inch)
pub fn pt<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Point)
}

/// Creates an inch unit
pub fn inches<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Inch)
}

/// Creates a centimeter unit
pub fn cm<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Centimeter)
}

/// Creates a millimeter unit
pub fn mm<T>(value: T) -> Unit<T> {
    Unit::new(value, UnitKind::Millimeter)
}

/// Physical unit conversion constants
pub mod constants {
    /// Points per inch
    pub const POINTS_PER_INCH: f32 = 72.0;
    
    /// Pixels per inch (standard screen density, can be overridden by device)
    pub const DEFAULT_PPI: f32 = 96.0;
    
    /// Millimeters per inch
    pub const MM_PER_INCH: f32 = 25.4;
    
    /// Centimeters per inch
    pub const CM_PER_INCH: f32 = 2.54;
} 