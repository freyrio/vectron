// Transform stack implementation for Vectron Render
//
// This module manages the stack of transforms for rendering operations.

use crate::core::geometry::transform::Transform;

/// TransformStack manages a stack of transforms for rendering operations.
/// It handles pushing, popping, and composing transforms to maintain the
/// current transformation state.
pub struct TransformStack {
    /// The stack of transforms
    stack: Vec<Transform>,
}

impl TransformStack {
    /// Create a new empty transform stack
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    /// Push a transform onto the stack
    pub fn push(&mut self, transform: Transform) {
        self.stack.push(transform);
    }
    
    /// Pop a transform from the stack
    pub fn pop(&mut self) -> Option<Transform> {
        if self.stack.len() <= 1 {
            // Always keep at least one transform on the stack (identity)
            return None;
        }
        
        self.stack.pop()
    }
    
    /// Get the current transform (top of stack)
    pub fn current(&self) -> &Transform {
        self.stack.last().unwrap_or_else(|| {
            // This should never happen as we always maintain at least one transform
            panic!("Transform stack is empty - this is a bug")
        })
    }
    
    /// Set the current transform (replace top of stack)
    pub fn set_current(&mut self, transform: Transform) {
        if self.stack.is_empty() {
            self.stack.push(transform);
        } else {
            let idx = self.stack.len() - 1;
            self.stack[idx] = transform;
        }
    }
    
    /// Clear the stack and reset to identity
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    /// Get the depth of the stack
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transform_stack_operations() {
        let mut stack = TransformStack::new();
        
        // Stack starts empty
        assert_eq!(stack.depth(), 0);
        
        // Push identity transform
        stack.push(Transform::identity());
        assert_eq!(stack.depth(), 1);
        
        // Add translation
        let translate = Transform::translation(10.0, 20.0, 0.0);
        stack.push(translate.clone());
        assert_eq!(stack.depth(), 2);
        
        // Check current
        assert_eq!(*stack.current(), translate);
        
        // Pop should work
        let popped = stack.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), translate);
        assert_eq!(stack.depth(), 1);
        
        // Clear resets
        stack.clear();
        assert_eq!(stack.depth(), 0);
    }
} 