// Clip stack implementation for Vectron Render
//
// This module manages the stack of clipping regions for rendering operations.

use crate::core::types::ClipRegion;

/// ClipStack manages a stack of clipping regions for rendering operations.
/// It handles pushing and popping clip regions to maintain the current clipping state.
pub struct ClipStack {
    /// The stack of clip regions
    stack: Vec<ClipRegion>,
}

impl ClipStack {
    /// Create a new empty clip stack
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    /// Push a clip region onto the stack
    pub fn push(&mut self, clip: ClipRegion) {
        self.stack.push(clip);
    }
    
    /// Pop a clip region from the stack
    pub fn pop(&mut self) -> Option<ClipRegion> {
        self.stack.pop()
    }
    
    /// Get the current clip region (top of stack)
    pub fn current(&self) -> Option<&ClipRegion> {
        self.stack.last()
    }
    
    /// Clear the stack
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    /// Get the depth of the stack
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clip_stack_operations() {
        let mut stack = ClipStack::new();
        
        // Stack starts empty
        assert_eq!(stack.depth(), 0);
        assert!(stack.is_empty());
        
        // Create a clip region
        let clip = ClipRegion::Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        
        // Push clip
        stack.push(clip.clone());
        assert_eq!(stack.depth(), 1);
        assert!(!stack.is_empty());
        
        // Check current
        let current = stack.current();
        assert!(current.is_some());
        assert_eq!(*current.unwrap(), clip);
        
        // Pop should work
        let popped = stack.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), clip);
        assert_eq!(stack.depth(), 0);
        assert!(stack.is_empty());
        
        // Current should be None
        assert!(stack.current().is_none());
    }
} 