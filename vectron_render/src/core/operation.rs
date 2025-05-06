use std::any::Any;

use crate::core::renderable::Renderable;
use crate::core::transform::Transform;
use crate::core::error::RenderError;

/// Represents a visual style that can be applied to a renderable element
pub trait Style: Any + Send + Sync {
    /// Apply this style to the provided geometry through the renderer
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
    
    /// Cast to Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Determine if this style can be batched with others
    fn is_batchable(&self) -> bool {
        false
    }
}

/// Represents a visual effect that can be applied pre or post-rendering
pub trait Effect: Any + Send + Sync {
    /// Apply the effect before rendering (e.g., preprocessing the renderable)
    fn apply_pre(&self, renderer: &mut dyn Renderer, element: &dyn Renderable) -> Result<(), RenderError> {
        Ok(())
    }
    
    /// Apply the effect after rendering (e.g., postprocessing the geometry)
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        Ok(())
    }
    
    /// Cast to Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Defines a clipping region for rendering operations
#[derive(Debug, Clone)]
pub enum ClipRegion {
    /// Rectangular clipping region
    Rect { x: f32, y: f32, width: f32, height: f32 },
    
    /// Path-based clipping region
    Path(Box<dyn Renderable>),
    
    /// Intersection of multiple clip regions
    Intersection(Vec<ClipRegion>),
}

/// Placeholder for geometry data produced during rendering
#[derive(Debug)]
pub struct GeometryData {
    // Implementation details would depend on the specific renderer
    // This is a placeholder for the concept
}

/// Trait defining renderer capabilities
pub trait Renderer {
    /// Push the current state onto the state stack
    fn push_state(&mut self);
    
    /// Pop the state stack, restoring the previous state
    fn pop_state(&mut self) -> Result<(), RenderError>;
    
    /// Set the current transform
    fn set_transform(&mut self, transform: &Transform);
    
    /// Set the current clip region
    fn set_clip(&mut self, clip: &ClipRegion);
    
    /// Tessellate an element into geometry
    fn tessellate(&mut self, element: &dyn Tessellable, options: &TessellationOptions) -> Result<GeometryData, RenderError>;
    
    /// Fill geometry with the specified paint and rule
    fn fill_geometry(&mut self, geometry: &GeometryData, paint: &Paint, rule: FillRule) -> Result<(), RenderError>;
    
    /// Stroke geometry with the specified paint and stroke options
    fn stroke_geometry(&mut self, geometry: &GeometryData, paint: &Paint, width: f32, join: LineJoin, cap: LineCap) -> Result<(), RenderError>;
    
    /// Draw text
    //fn draw_text(&mut self, text: &TextRun) -> Result<(), RenderError>;
    
    /// Draw a 3D mesh
    //fn draw_mesh(&mut self, mesh: &Mesh, material: &Material) -> Result<(), RenderError>;
    
    /// Apply a blur effect to geometry
    fn apply_blur(&mut self, geometry: &GeometryData, radius: f32) -> Result<(), RenderError>;
}

/// The core rendering operation combining an element with styles and transforms
#[derive(Debug)]
pub struct RenderOperation {
    /// The element to render
    pub element: Box<dyn Renderable>,
    
    /// The transform to apply
    pub transform: Transform,
    
    /// The styles to apply
    pub styles: Vec<Box<dyn Style>>,
    
    /// The effects to apply
    pub effects: Vec<Box<dyn Effect>>,
    
    /// Optional clipping region
    pub clip: Option<ClipRegion>,
}

impl RenderOperation {
    /// Create a new operation with the specified renderable element
    pub fn new<R: Renderable + 'static>(element: R) -> Self {
        Self {
            element: Box::new(element),
            transform: Transform::identity(),
            styles: Vec::new(),
            effects: Vec::new(),
            clip: None,
        }
    }
    
    /// Add a style to the operation
    pub fn with_style<S: Style + 'static>(mut self, style: S) -> Self {
        self.styles.push(Box::new(style));
        self
    }
    
    /// Add an effect to the operation
    pub fn with_effect<E: Effect + 'static>(mut self, effect: E) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
    
    /// Set the transform for the operation
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    
    /// Set the clip region for the operation
    pub fn with_clip(mut self, clip: ClipRegion) -> Self {
        self.clip = Some(clip);
        self
    }
    
    /// Execute the operation using the provided renderer
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Save the current state
        renderer.push_state();
        
        // Apply the transform
        renderer.set_transform(&self.transform);
        
        // Apply the clip if present
        if let Some(clip) = &self.clip {
            renderer.set_clip(clip);
        }
        
        // Apply pre-rendering effects
        for effect in &self.effects {
            effect.apply_pre(renderer, self.element.as_ref())?;
        }
        
        // Tessellate the element if it's tessellable
        if let Some(tessellable) = self.element.as_any().downcast_ref::<dyn Tessellable>() {
            let options = TessellationOptions {
                quality: TessellationQuality::Medium,
                tolerance: 0.1,
                generate_normals: false,
                generate_uvs: false,
                cull_mode: CullMode::None,
            };
            
            let geometry = renderer.tessellate(tessellable, &options)?;
            
            // Apply styles
            for style in &self.styles {
                style.apply(renderer, &geometry)?;
            }
            
            // Apply post-rendering effects
            for effect in &self.effects {
                effect.apply_post(renderer, &geometry)?;
            }
        } else {
            // Handle non-tessellable elements (e.g., text, images)
            // Implementation would depend on the specific element types
        }
        
        // Restore the previous state
        renderer.pop_state()?;
        
        Ok(())
    }
}

