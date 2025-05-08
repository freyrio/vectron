// Effect trait implementation for Vectron Render
//
// This module defines the core Effect trait and common effect implementations
// for visual effects that can be applied to rendered elements.

use std::any::Any;
use crate::core::error::RenderError;
use crate::core::traits::renderable::Renderable;

/// Trait for effects that can be applied to rendered elements
pub trait Effect: Any + Send + Sync {
    /// Apply the effect before rendering the element
    /// This is called before an element is processed for rendering
    fn apply_pre(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        Ok(())
    }
    
    /// Apply the effect after rendering
    /// This is called after an element has been rendered
    fn apply_post(&self, renderable: &dyn Renderable) -> Result<(), RenderError> {
        Ok(())
    }
    
    /// Returns self as Any for downcast operations
    fn as_any(&self) -> &dyn Any;
    
    /// Check if this effect can be batched with others
    fn is_batchable(&self) -> bool {
        false
    }
    
    /// Clone this effect
    fn clone_effect(&self) -> Box<dyn Effect>;
} 