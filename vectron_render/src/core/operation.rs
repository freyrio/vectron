use std::marker::PhantomData;
use crate::error::RenderError;
use crate::utils::math::{Transform, Rect};

/// Operation that combines a drawable with styling and effects
pub struct Operation<D: Drawable> {
    /// The drawable object
    pub drawable: D,
    /// List of styles to apply
    pub styles: Vec<Box<dyn Style>>,
    /// List of effects to apply
    pub effects: Vec<Box<dyn Effect>>,
    /// Transform to apply
    pub transform: Transform,
    /// Optional clipping rectangle
    pub clip: Option<Rect>,
}

impl<D: Drawable> Operation<D> {
    /// Create a new operation with a drawable
    pub fn new(drawable: D) -> Self {
        Self {
            drawable,
            styles: Vec::new(),
            effects: Vec::new(),
            transform: Transform::identity(),
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
    
    /// Set the clipping rectangle for the operation
    pub fn with_clip(mut self, clip: Rect) -> Self {
        self.clip = Some(clip);
        self
    }
    
    /// Execute this operation on a renderer
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Save current state
        renderer.push_state();
        
        // Apply transform and clip
        if let Some(clip) = self.clip {
            renderer.set_clip(clip);
        }
        renderer.set_transform(self.transform);
        
        // Apply pre-effects
        for effect in &self.effects {
            effect.apply_pre(renderer, &self.drawable)?;
        }
        
        // Convert drawable to geometry
        let geometry = self.drawable.to_geometry()?;
        
        // Apply styles
        for style in &self.styles {
            style.apply(renderer, &geometry)?;
        }
        
        // Apply post-effects
        for effect in &self.effects {
            effect.apply_post(renderer, &geometry)?;
        }
        
        // Restore state
        renderer.pop_state()?;
        
        Ok(())
    }
    
    /// Check if this operation can be batched
    pub fn is_batchable(&self) -> bool {
        // Operations can be batched if:
        // 1. They have exactly one style
        // 2. The style is a common type (Fill, Stroke, etc.)
        // 3. They have no effects
        // 4. The drawable is batchable
        
        if self.styles.len() != 1 || !self.effects.is_empty() {
            return false;
        }
        
        // Check if both style and drawable support batching
        if let Some(drawable) = self.drawable_as_batchable() {
            self.styles[0].is_batchable() && drawable.is_batchable()
        } else {
            false
        }
    }
    
    /// Try to get the drawable as a BatchableDrawable
    fn drawable_as_batchable(&self) -> Option<&dyn BatchableDrawable> {
        None // This would need to be implemented based on concrete drawables
    }
}

/// Trait for operations that can be executed as commands
pub trait CommandOperation {
    /// Execute the operation using a renderer
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError>;
    
    /// Try to get this operation as a batchable operation
    fn as_batchable(&self) -> Option<&dyn Batchable> {
        None // Default is not batchable
    }
}

/// Implement CommandOperation for our Operation<D> type
impl<D: Drawable + 'static> CommandOperation for Operation<D> {
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        self.execute(renderer)
    }
}

/// Command list for executing multiple operations
pub struct CommandList {
    pub operations: Vec<Box<dyn CommandOperation>>,
}

impl CommandList {
    /// Create a new empty command list
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    
    /// Add an operation to the command list
    pub fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        self.operations.push(Box::new(operation));
        self
    }
    
    /// Clear all operations
    pub fn clear(&mut self) {
        self.operations.clear();
    }
    
    /// Execute all operations in the list
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        for operation in &self.operations {
            operation.execute(renderer)?;
        }
        Ok(())
    }
}

// Import dependencies
use crate::core::drawable::{Drawable, BatchableDrawable};
use crate::core::style::{Style, Renderer};
use crate::core::effect::Effect;
use crate::batching::Batchable; 