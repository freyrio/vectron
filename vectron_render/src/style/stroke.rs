// Stroke style implementation for Vectron Render
//
// This module provides the Stroke style which can be applied to shapes.

use std::any::Any;
use crate::core::{Style, RenderError, GeometryHandle, Renderer};
use crate::style::Paint;
use crate::units::Unit;

/// Defines how the endpoints of a line are drawn
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineCap {
    /// Flat end (no extension)
    Butt,
    /// Rounded semicircle end
    Round,
    /// Square end (extends by half line width)
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        Self::Butt
    }
}

/// Defines how line segments join together
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineJoin {
    /// Sharp corner
    Miter,
    /// Rounded corner
    Round,
    /// Beveled corner
    Bevel,
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::Miter
    }
}

/// Defines a dash pattern for stroked lines
#[derive(Debug, Clone, PartialEq)]
pub struct DashPattern {
    /// Array of dash and gap lengths
    pub segments: Vec<f32>,
    /// Offset to start the pattern
    pub offset: f32,
}

impl DashPattern {
    /// Create a new dash pattern with the specified segments
    pub fn new(segments: Vec<f32>, offset: f32) -> Self {
        Self { segments, offset }
    }
    
    /// Create a simple dashed line pattern
    pub fn dashed(dash_length: f32, gap_length: f32) -> Self {
        Self::new(vec![dash_length, gap_length], 0.0)
    }
    
    /// Create a dotted line pattern
    pub fn dotted(gap: f32) -> Self {
        Self::new(vec![0.1, gap], 0.0)
    }
}

/// Style for stroking the outlines of shapes
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    /// The paint to use for the stroke
    pub paint: Paint,
    /// The width of the stroke
    pub width: Unit<f32>,
    /// How to render the endpoints
    pub line_cap: LineCap,
    /// How to render the corners
    pub line_join: LineJoin,
    /// Optional dash pattern
    pub dash_pattern: Option<DashPattern>,
    /// Miter limit (only used with LineJoin::Miter)
    pub miter_limit: f32,
    /// The z-index (rendering order)
    pub z_index: i32,
}

impl Stroke {
    /// Create a new stroke style with the specified paint and width
    pub fn new(paint: impl Into<Paint>, width: impl Into<Unit<f32>>) -> Self {
        Self {
            paint: paint.into(),
            width: width.into(),
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            dash_pattern: None,
            miter_limit: 4.0,
            z_index: 0,
        }
    }
    
    /// Set the line cap for this stroke
    pub fn with_line_cap(mut self, line_cap: LineCap) -> Self {
        self.line_cap = line_cap;
        self
    }
    
    /// Set the line join for this stroke
    pub fn with_line_join(mut self, line_join: LineJoin) -> Self {
        self.line_join = line_join;
        self
    }
    
    /// Set the dash pattern for this stroke
    pub fn with_dash_pattern(mut self, dash_pattern: DashPattern) -> Self {
        self.dash_pattern = Some(dash_pattern);
        self
    }
    
    /// Set the miter limit for this stroke
    pub fn with_miter_limit(mut self, miter_limit: f32) -> Self {
        self.miter_limit = miter_limit;
        self
    }
    
    /// Set the z-index for this style
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}

impl Style for Stroke {
    fn apply(&self, renderer: &mut dyn Renderer, geometry_handle: GeometryHandle) -> Result<(), RenderError> {
        renderer.apply_style(self, geometry_handle)
    }
    
    fn is_batchable(&self) -> bool {
        self.paint.is_batchable() && self.dash_pattern.is_none()
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