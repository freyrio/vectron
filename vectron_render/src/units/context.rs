// Unit resolution context for Vectron Render
//
// This module provides the context needed to resolve units to pixel values.

use super::types::{Unit, UnitKind, Size};
use super::physical::constants;

/// Context for resolving units to their absolute pixel values.
///
/// This context contains all the information needed to convert relative
/// units (like em, %, vw) to absolute pixel values.
#[derive(Debug, Clone)]
pub struct UnitContext {
    /// Device pixel ratio (dppx) - logical pixels to physical pixels
    pub device_pixel_ratio: f32,
    
    /// Pixels per inch for the current device
    pub ppi: f32,
    
    /// Current viewport size in pixels
    pub viewport_size: (f32, f32),
    
    /// Parent container size in pixels
    pub parent_size: (f32, f32),
    
    /// Current font size in pixels
    pub font_size: f32,
    
    /// Root font size in pixels
    pub root_font_size: f32,
}

impl Default for UnitContext {
    fn default() -> Self {
        Self {
            device_pixel_ratio: 1.0,
            ppi: constants::DEFAULT_PPI,
            viewport_size: (800.0, 600.0),
            parent_size: (100.0, 100.0),
            font_size: 16.0,
            root_font_size: 16.0,
        }
    }
}

impl UnitContext {
    /// Creates a new unit context with the specified properties.
    pub fn new(
        device_pixel_ratio: f32,
        ppi: f32,
        viewport_size: (f32, f32),
        parent_size: (f32, f32),
        font_size: f32,
        root_font_size: f32,
    ) -> Self {
        Self {
            device_pixel_ratio,
            ppi,
            viewport_size,
            parent_size,
            font_size,
            root_font_size,
        }
    }
    
    /// Creates a new unit context with the specified viewport size.
    pub fn with_viewport_size(viewport_width: f32, viewport_height: f32) -> Self {
        let mut ctx = Self::default();
        ctx.viewport_size = (viewport_width, viewport_height);
        ctx
    }
    
    /// Creates a new context for a child element with the given parent size and font size.
    pub fn create_child_context(&self, parent_size: (f32, f32), font_size: Option<f32>) -> Self {
        Self {
            device_pixel_ratio: self.device_pixel_ratio,
            ppi: self.ppi,
            viewport_size: self.viewport_size,
            parent_size,
            font_size: font_size.unwrap_or(self.font_size),
            root_font_size: self.root_font_size,
        }
    }
    
    /// Resolves a unit to its absolute pixel value.
    pub fn resolve<T: Into<f32> + From<f32> + Copy>(&self, unit: Unit<T>) -> T {
        let value: f32 = unit.value.into();
        
        let pixels = match unit.kind {
            // Absolute units
            UnitKind::Pixel => value,
            UnitKind::Point => value * self.ppi / constants::POINTS_PER_INCH,
            UnitKind::Inch => value * self.ppi,
            UnitKind::Centimeter => value * self.ppi / constants::CM_PER_INCH,
            UnitKind::Millimeter => value * self.ppi / constants::MM_PER_INCH,
            
            // Font-relative units
            UnitKind::Em => value * self.font_size,
            UnitKind::Rem => value * self.root_font_size,
            
            // Parent-relative units
            UnitKind::Percent => value * 0.01 * ((self.parent_size.0 + self.parent_size.1) / 2.0),
            
            // Viewport-relative units
            UnitKind::ViewportWidth => value * 0.01 * self.viewport_size.0,
            UnitKind::ViewportHeight => value * 0.01 * self.viewport_size.1,
            UnitKind::ViewportMin => value * 0.01 * self.viewport_size.0.min(self.viewport_size.1),
            UnitKind::ViewportMax => value * 0.01 * self.viewport_size.0.max(self.viewport_size.1),
        };
        
        T::from(pixels)
    }
    
    /// Resolves a size to its absolute pixel dimensions.
    pub fn resolve_size<T: Into<f32> + From<f32> + Copy>(&self, size: Size<T>) -> (T, T) {
        let width = self.resolve(size.width);
        let height = self.resolve(size.height);
        (width, height)
    }
    
    /// Converts device-independent pixels (DIPs) to physical pixels.
    pub fn dips_to_physical_pixels(&self, dips: f32) -> f32 {
        dips * self.device_pixel_ratio
    }
    
    /// Converts physical pixels to device-independent pixels (DIPs).
    pub fn physical_pixels_to_dips(&self, pixels: f32) -> f32 {
        pixels / self.device_pixel_ratio
    }
}

/// Extension trait to provide unit resolution methods directly on Unit struct
pub trait UnitResolution<T> {
    /// Resolve this unit to pixels using the provided context
    fn resolve(&self, context: &UnitContext) -> T;
    
    /// Convert this unit to pixels assuming default context values
    fn to_pixels(&self) -> T;
}

impl<T: Into<f32> + From<f32> + Copy> UnitResolution<T> for Unit<T> {
    fn resolve(&self, context: &UnitContext) -> T {
        context.resolve(*self)
    }
    
    fn to_pixels(&self) -> T {
        UnitContext::default().resolve(*self)
    }
} 