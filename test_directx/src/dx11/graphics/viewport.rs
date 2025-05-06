//! Viewport module for managing DirectX viewport state.

use windows::Win32::Graphics::Direct3D11::D3D11_VIEWPORT;

/// Represents a viewport area on the render target.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// X coordinate of the top-left corner.
    pub x: f32,
    /// Y coordinate of the top-left corner.
    pub y: f32,
    /// Width of the viewport.
    pub width: f32,
    /// Height of the viewport.
    pub height: f32,
    /// Minimum depth value.
    pub min_depth: f32,
    /// Maximum depth value.
    pub max_depth: f32,
}

impl Viewport {
    /// Creates a new Viewport.
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        }
    }

    /// Converts the Viewport struct to the DirectX D3D11_VIEWPORT struct.
    pub fn to_d3d11(&self) -> D3D11_VIEWPORT {
        D3D11_VIEWPORT {
            TopLeftX: self.x,
            TopLeftY: self.y,
            Width: self.width,
            Height: self.height,
            MinDepth: self.min_depth,
            MaxDepth: self.max_depth,
        }
    }
}

impl Default for Viewport {
    /// Creates a default viewport (usually covering the whole target).
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0, // Typically set during surface creation/resize
            height: 0.0, // Typically set during surface creation/resize
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }
}
