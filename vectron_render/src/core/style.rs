use std::any::Any;
use crate::error::RenderError;
use crate::resources::GeometryData;
use crate::core::color::Color;

/// Style trait for visual appearance customization
pub trait Style: Send + Sync {
    /// Apply this style to render geometry
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
    
    /// For runtime type identification and downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Whether this style can be batched with similar styles
    fn is_batchable(&self) -> bool {
        false // Default implementation is conservative
    }
    
    /// Set uniforms specific to this style
    fn set_uniforms(&self, cmd: &mut CommandBuffer) -> Result<(), RenderError> {
        Ok(()) // Default is no-op
    }
}

/// Fill style for filling shapes
pub struct Fill {
    pub paint: Paint,
    pub rule: FillRule,
}

impl Fill {
    /// Create a solid color fill
    pub fn solid(color: Color) -> Self {
        Self {
            paint: Paint::Solid(color),
            rule: FillRule::NonZero,
        }
    }
    
    /// Create a linear gradient fill
    pub fn linear_gradient(gradient: LinearGradient) -> Self {
        Self {
            paint: Paint::LinearGradient(gradient),
            rule: FillRule::NonZero,
        }
    }
    
    /// Set the fill rule
    pub fn with_rule(mut self, rule: FillRule) -> Self {
        self.rule = rule;
        self
    }
}

impl Style for Fill {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        renderer.fill_geometry(geometry, &self.paint, self.rule)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn is_batchable(&self) -> bool {
        // Solid colors and simple gradients are batchable
        match &self.paint {
            Paint::Solid(_) => true,
            Paint::LinearGradient(_) => true,
            _ => false,
        }
    }
}

/// Stroke style for outlines
pub struct Stroke {
    pub paint: Paint,
    pub width: f32,
    pub line_join: LineJoin,
    pub line_cap: LineCap,
    pub dash_pattern: Option<DashPattern>,
}

impl Stroke {
    /// Create a new stroke with specified paint and width
    pub fn new(paint: Paint, width: f32) -> Self {
        Self {
            paint,
            width,
            line_join: LineJoin::Miter,
            line_cap: LineCap::Butt,
            dash_pattern: None,
        }
    }
    
    /// Set the line join type
    pub fn with_line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }
    
    /// Set the line cap type
    pub fn with_line_cap(mut self, cap: LineCap) -> Self {
        self.line_cap = cap;
        self
    }
    
    /// Set a dash pattern
    pub fn with_dash_pattern(mut self, pattern: DashPattern) -> Self {
        self.dash_pattern = Some(pattern);
        self
    }
}

impl Style for Stroke {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        renderer.stroke_geometry(
            geometry,
            &self.paint,
            self.width,
            self.line_join,
            self.line_cap,
            self.dash_pattern.as_ref()
        )
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn is_batchable(&self) -> bool {
        // Strokes without dash patterns are generally batchable
        self.dash_pattern.is_none() && 
        match &self.paint {
            Paint::Solid(_) => true,
            _ => false,
        }
    }
}

/// Paint types for fills and strokes
#[derive(Clone)]
pub enum Paint {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Texture(TextureHandle),
    Pattern(PatternHandle),
}

/// Fill rules for determining inside/outside
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

/// Line join types for stroke corners
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Bevel,
    Round,
}

/// Line cap types for stroke endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Dash pattern for strokes
#[derive(Debug, Clone)]
pub struct DashPattern {
    pub segments: Vec<f32>,
    pub offset: f32,
}

impl DashPattern {
    pub fn new(segments: Vec<f32>) -> Self {
        Self {
            segments,
            offset: 0.0,
        }
    }
    
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
}

/// Linear gradient
#[derive(Clone)]
pub struct LinearGradient {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub stops: Vec<GradientStop>,
}

/// Radial gradient
#[derive(Clone)]
pub struct RadialGradient {
    pub center: (f32, f32),
    pub radius: f32,
    pub stops: Vec<GradientStop>,
}

/// Color stop for gradients
#[derive(Clone)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

// Imports needed for the renderer implementation
use crate::backend::CommandBuffer;
use crate::resources::{TextureHandle, PatternHandle};

/// Renderer trait for drawing operations
pub trait Renderer {
    // State management
    fn push_state(&mut self);
    fn pop_state(&mut self) -> Result<(), RenderError>;
    fn set_transform(&mut self, transform: Transform);
    fn set_clip(&mut self, clip: Rect);
    fn translate(&mut self, x: f32, y: f32);
    fn set_blur(&mut self, radius: f32);
    
    // Drawing methods
    fn fill_geometry(
        &mut self,
        geometry: &GeometryData,
        paint: &Paint,
        rule: FillRule
    ) -> Result<(), RenderError>;
    
    fn stroke_geometry(
        &mut self,
        geometry: &GeometryData,
        paint: &Paint,
        width: f32,
        line_join: LineJoin,
        line_cap: LineCap,
        dash_pattern: Option<&DashPattern>
    ) -> Result<(), RenderError>;
    
    // Instanced drawing for batching
    fn set_instance_data(&mut self, instances: &[InstanceData]) -> Result<(), RenderError> {
        // Default implementation for renderers that don't support instancing
        Err(RenderError::UnsupportedOperation("Instanced rendering not supported".into()))
    }
    
    fn fill_geometry_instanced(
        &mut self,
        geometry: &GeometryData,
        paint: &Paint,
        rule: FillRule,
        instance_count: usize
    ) -> Result<(), RenderError> {
        // Default implementation for renderers that don't support instancing
        Err(RenderError::UnsupportedOperation("Instanced rendering not supported".into()))
    }
    
    // Context access
    fn context(&self) -> &RenderContext;
}

// Additional imports needed
use crate::utils::math::{Transform, Rect};
use crate::core::drawable::RenderContext;
use crate::batching::InstanceData; 