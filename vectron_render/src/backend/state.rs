/*!
 * Render state management
 */

use crate::api::bare::state::{DrawState, Transform, Rect, BlendMode};
use crate::error::RenderError;

/// Render state for the backend
pub struct RenderState {
    /// Stack of transforms
    transform_stack: Vec<Transform>,
    
    /// Current transform
    current_transform: Transform,
    
    /// Stack of scissor rectangles
    scissor_stack: Vec<Option<Rect>>,
    
    /// Current scissor rectangle
    current_scissor: Option<Rect>,
    
    /// Current stencil level
    stencil_level: u8,
    
    /// Current blend mode
    blend_mode: BlendMode,
}

impl RenderState {
    /// Create a new render state
    pub fn new() -> Self {
        Self {
            transform_stack: Vec::new(),
            current_transform: identity_transform(),
            scissor_stack: Vec::new(),
            current_scissor: None,
            stencil_level: 0,
            blend_mode: BlendMode::Alpha,
        }
    }
    
    /// Push the current transform onto the stack
    pub fn push_transform(&mut self) {
        self.transform_stack.push(self.current_transform);
    }
    
    /// Pop a transform from the stack
    pub fn pop_transform(&mut self) -> Result<(), RenderError> {
        if let Some(transform) = self.transform_stack.pop() {
            self.current_transform = transform;
            Ok(())
        } else {
            Err(RenderError::StackUnderflow("Transform stack empty".into()))
        }
    }
    
    /// Push the current scissor rectangle onto the stack
    pub fn push_scissor(&mut self) {
        self.scissor_stack.push(self.current_scissor);
    }
    
    /// Pop a scissor rectangle from the stack
    pub fn pop_scissor(&mut self) -> Result<(), RenderError> {
        if let Some(scissor) = self.scissor_stack.pop() {
            self.current_scissor = scissor;
            Ok(())
        } else {
            Err(RenderError::StackUnderflow("Scissor stack empty".into()))
        }
    }
    
    /// Apply a draw state to this render state
    pub fn apply_from(&mut self, state: &DrawState) {
        self.current_transform = state.transform;
        self.current_scissor = state.scissor;
        self.stencil_level = state.stencil;
        self.blend_mode = state.blend_mode;
    }
    
    /// Get the current transform
    pub fn transform(&self) -> Transform {
        self.current_transform
    }
    
    /// Set the current transform
    pub fn set_transform(&mut self, transform: Transform) {
        self.current_transform = transform;
    }
    
    /// Get the current scissor rectangle
    pub fn scissor(&self) -> Option<Rect> {
        self.current_scissor
    }
    
    /// Set the current scissor rectangle
    pub fn set_scissor(&mut self, scissor: Option<Rect>) {
        self.current_scissor = scissor;
    }
    
    /// Get the current stencil level
    pub fn stencil_level(&self) -> u8 {
        self.stencil_level
    }
    
    /// Set the current stencil level
    pub fn set_stencil_level(&mut self, level: u8) {
        self.stencil_level = level;
    }
    
    /// Get the current blend mode
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    
    /// Set the current blend mode
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }
}

/// Create an identity transformation matrix
fn identity_transform() -> Transform {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
} 