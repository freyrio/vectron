use std::fmt;
use crate::core::{ClipRegion, Uuid, Effect, Dimensionality,Transform,Style,Renderable};

/// A render operation binds together a renderable element with styles, transforms,
/// effects, and clipping.
///
/// This is a key abstraction in the rendering system that combines "what" to draw
/// with "how" to draw it.
pub struct RenderOperation {
    /// The element to render
    pub element: Box<dyn Renderable>,
    
    /// The transform to apply when rendering
    pub transform: Transform,
    
    /// Styles to apply to the element
    pub styles: Vec<Box<dyn Style>>,
    
    /// Effects to apply to the rendered result
    pub effects: Vec<Box<dyn Effect>>,
    
    /// Clipping region (if any)
    pub clip: Option<ClipRegion>,
    
    /// Optional ID for this operation
    pub id: Option<Uuid>,
    
    /// Optional layer for controlling render order
    pub layer: i32,
}

// Manual Debug implementation since dynamic trait objects don't implement Debug
impl fmt::Debug for RenderOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderOperation")
            .field("dimensionality", &self.element.dimensionality())
            .field("transform", &self.transform)
            .field("styles_count", &self.styles.len())
            .field("effects_count", &self.effects.len())
            .field("clip", &self.clip)
            .field("id", &self.id)
            .field("layer", &self.layer)
            .finish()
    }
}

// Manual Clone implementation for RenderOperation
impl Clone for RenderOperation {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone_renderable(),
            transform: self.transform.clone(),
            styles: self.styles.iter().map(|style| style.clone_style()).collect(),
            effects: self.effects.iter().map(|effect| effect.clone_effect()).collect(),
            clip: self.clip.clone(),
            id: self.id,
            layer: self.layer,
        }
    }
}

impl RenderOperation {
    /// Create a new render operation for the given element
    pub fn new<R: Renderable + 'static>(element: R) -> Self {
        let dim = element.dimensionality();
        Self {
            element: Box::new(element),
            transform: match dim {
                Dimensionality::D2 => Transform::D2(crate::math::transform::Transform2D::identity()),
                Dimensionality::D3 => Transform::D3(crate::math::transform::Transform3D::identity()),
            },
            styles: Vec::new(),
            effects: Vec::new(),
            clip: None,
            id: None,
            layer: 0,
        }
    }
    
    /// Add a style to this operation
    pub fn with_style<S: Style + 'static>(mut self, style: S) -> Self {
        self.styles.push(Box::new(style));
        self
    }
    
    /// Add an effect to this operation
    pub fn with_effect<E: Effect + 'static>(mut self, effect: E) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
    
    /// Set the transform for this operation
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    
    /// Set a clipping region for this operation
    pub fn with_clip(mut self, clip: ClipRegion) -> Self {
        self.clip = Some(clip);
        self
    }
    
    /// Set an ID for this operation
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
    
    /// Set the layer for this operation
    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }
    
    /// Get the dimensionality of this operation
    pub fn dimensionality(&self) -> Dimensionality {
        self.element.dimensionality()
    }
} 