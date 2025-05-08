// Render context implementation for Vectron Render
//
// This module provides the main interface for rendering operations.

use std::rc::Rc;
use std::cell::RefCell;
use crate::core::error::RenderError;
use crate::core::types::{GeometryHandle, TextureHandle, ShaderHandle, PipelineHandle, BindGroupHandle, ClipRegion};
use crate::core::traits::renderer::Renderer;
use crate::core::geometry::transform::Transform;
use crate::core::operation::render::RenderOperation;
use crate::core::traits::renderable::Renderable;
use crate::core::traits::style::Style;
use crate::core::traits::effect::Effect;
use super::translator::CommandTranslator;
use super::transform_stack::TransformStack;
use super::clip_stack::ClipStack;
use super::stats::RenderStats;

/// The RenderContext provides a high-level interface for rendering operations.
/// It wraps a renderer implementation and provides convenience methods for 
/// common rendering tasks.
pub struct RenderContext {
    /// The underlying renderer implementation
    renderer: Rc<RefCell<CommandTranslator>>,
    
    /// Stats tracking for performance monitoring
    stats: RenderStats,
    
    /// The transform stack for managing coordinate transformations
    transform_stack: TransformStack,
    
    /// The clip stack for managing clipping regions
    clip_stack: ClipStack,
}

impl RenderContext {
    /// Create a new render context with the provided renderer
    pub fn new(renderer: CommandTranslator) -> Self {
        Self {
            renderer: Rc::new(RefCell::new(renderer)),
            stats: RenderStats::new(),
            transform_stack: TransformStack::new(),
            clip_stack: ClipStack::new(),
        }
    }
    
    /// Begin a new frame
    pub fn begin_frame(&mut self) -> Result<(), RenderError> {
        self.stats.begin_frame();
        self.transform_stack.clear();
        self.clip_stack.clear();
        
        // Initialize with identity transform
        self.transform_stack.push(Transform::identity());
        
        self.renderer.borrow_mut().begin_frame()
    }
    
    /// End the current frame and present
    pub fn end_frame(&mut self) -> Result<(), RenderError> {
        let result = self.renderer.borrow_mut().end_frame();
        self.stats.end_frame();
        result
    }
    
    /// Execute a render operation
    pub fn execute_operation(&mut self, operation: &RenderOperation) -> Result<(), RenderError> {
        self.stats.increment_operations();
        
        // Push operation transform
        let current_transform = self.transform_stack.current().clone();
        let combined_transform = current_transform.compose(&operation.transform);
        self.transform_stack.push(combined_transform);
        
        // Push clip if present
        if let Some(clip) = &operation.clip {
            self.clip_stack.push(clip.clone());
            self.renderer.borrow_mut().push_clip(clip)?;
        }
        
        // Apply pre-effects
        for effect in &operation.effects {
            effect.apply_pre(&*operation.element)?;
        }
        
        // Set the current transform on the renderer
        self.renderer.borrow_mut().set_transform(self.transform_stack.current())?;
        
        // Apply styles and draw
        for style in &operation.styles {
            // Perform style-specific rendering
            self.renderer.borrow_mut().apply_style(&**style, GeometryHandle::default())?;
        }
        
        // Apply post-effects
        for effect in &operation.effects {
            effect.apply_post(&*operation.element)?;
        }
        
        // Pop clip if needed
        if operation.clip.is_some() {
            self.clip_stack.pop();
            self.renderer.borrow_mut().pop_clip()?;
        }
        
        // Pop transform
        self.transform_stack.pop();
        
        Ok(())
    }
    
    /// Set the current transform
    pub fn set_transform(&mut self, transform: Transform) -> Result<(), RenderError> {
        self.transform_stack.set_current(transform.clone());
        self.renderer.borrow_mut().set_transform(&transform)
    }
    
    /// Get the current transform
    pub fn get_transform(&self) -> Transform {
        self.transform_stack.current().clone()
    }
    
    /// Push a transform onto the stack
    pub fn push_transform(&mut self, transform: Transform) -> Result<(), RenderError> {
        let current = self.transform_stack.current().clone();
        let combined = current.compose(&transform);
        self.transform_stack.push(combined);
        self.renderer.borrow_mut().set_transform(&combined)
    }
    
    /// Pop a transform from the stack
    pub fn pop_transform(&mut self) -> Result<(), RenderError> {
        self.transform_stack.pop();
        self.renderer.borrow_mut().set_transform(self.transform_stack.current())
    }
    
    /// Push a clip region
    pub fn push_clip(&mut self, clip: ClipRegion) -> Result<(), RenderError> {
        self.clip_stack.push(clip.clone());
        self.renderer.borrow_mut().push_clip(&clip)
    }
    
    /// Pop a clip region
    pub fn pop_clip(&mut self) -> Result<(), RenderError> {
        self.clip_stack.pop();
        if let Some(clip) = self.clip_stack.current() {
            self.renderer.borrow_mut().push_clip(clip)
        } else {
            self.renderer.borrow_mut().pop_clip()
        }
    }
    
    /// Get rendering statistics
    pub fn stats(&self) -> &RenderStats {
        &self.stats
    }
    
    /// Clear the color attachment
    pub fn clear_color(&mut self, attachment: usize, color: [f32; 4]) -> Result<(), RenderError> {
        self.renderer.borrow_mut().clear_color(attachment, color)
    }
    
    /// Clear the depth attachment
    pub fn clear_depth(&mut self, value: f32) -> Result<(), RenderError> {
        self.renderer.borrow_mut().clear_depth(value)
    }
    
    /// Set the viewport
    pub fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), RenderError> {
        self.renderer.borrow_mut().set_viewport(x, y, width, height, 0.0, 1.0)
    }
    
    /// Get the viewport size
    pub fn get_viewport_size(&self) -> (u32, u32) {
        self.renderer.borrow().get_viewport_size()
    }
    
    /// Begin a debug group
    pub fn push_debug_group(&mut self, label: &str) -> Result<(), RenderError> {
        self.renderer.borrow_mut().push_debug_group(label)
    }
    
    /// End a debug group
    pub fn pop_debug_group(&mut self) -> Result<(), RenderError> {
        self.renderer.borrow_mut().pop_debug_group()
    }
} 