// Fill style implementation for Vectron Render
//
// This module provides the Fill style which can be applied to shapes.

use std::any::Any;
use crate::core::{Style, GeometryHandle, Renderer,RenderError};
use crate::style::paint::Paint;

/// The rule for determining which parts of a shape to fill
/// when the shape has self-intersections or holes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRule {
    /// Fill shapes using the non-zero winding rule
    NonZero,
    /// Fill shapes using the even-odd rule
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::NonZero
    }
}

/// Style for filling shapes with a paint
#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    /// The paint to use for filling
    pub paint: Paint,
    /// The fill rule to apply
    pub rule: FillRule,
    /// The z-index (rendering order)
    pub z_index: i32,
}

impl Fill {
    /// Create a new fill style with the specified paint
    pub fn new(paint: impl Into<Paint>) -> Self {
        Self {
            paint: paint.into(),
            rule: FillRule::default(),
            z_index: 0,
        }
    }
    
    /// Set the fill rule for this style
    pub fn with_rule(mut self, rule: FillRule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set the z-index for this style
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}

impl Style for Fill {
    fn apply(&self, renderer: &mut dyn Renderer, geometry_handle: GeometryHandle) -> Result<(), RenderError> {
        renderer.apply_style(self, geometry_handle)
    }
    
    fn is_batchable(&self) -> bool {
        self.paint.is_batchable()
    }
    
    fn z_index(&self) -> i32 {
        self.z_index
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn clone_style(&self) -> Box<dyn Style> {
        Box::new(self.clone())
    }
} 