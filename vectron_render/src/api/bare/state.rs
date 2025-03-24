/*!
 * Render state management for the bare API
 */

/// Transformation matrix (row-major 4x4)
pub type Transform = [[f32; 4]; 4];

/// Rectangle with position and size
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

/// The rendering state associated with drawing commands
#[derive(Debug, Clone)]
pub struct DrawState {
    /// Current transformation matrix
    pub transform: Transform,
    
    /// Current scissor rectangle (if active)
    pub scissor: Option<Rect>,
    
    /// Current stencil value
    pub stencil: u8,
    
    /// Current blend mode
    pub blend_mode: BlendMode,
}

impl Default for DrawState {
    fn default() -> Self {
        Self {
            transform: identity_transform(),
            scissor: None,
            stencil: 0,
            blend_mode: BlendMode::Alpha,
        }
    }
}

/// Blend modes for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// No blending (opaque)
    Opaque,
    
    /// Standard alpha blending
    Alpha,
    
    /// Additive blending
    Additive,
    
    /// Multiplicative blending
    Multiply,
    
    /// Screen blending
    Screen,
}

/// Create an identity transformation matrix
pub fn identity_transform() -> Transform {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
} 