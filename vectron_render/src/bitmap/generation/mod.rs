// Bitmap generation module
//
// Provides procedural generation of bitmap textures

pub mod noise;
pub mod patterns;
pub mod fractals;
pub mod compositor;

// Re-export commonly used generators for convenience
pub use noise::{NoiseType, NoiseGenerator};
pub use patterns::{gradient, checkerboard, grid, dots};
pub use fractals::{mandelbrot, julia};
pub use compositor::Compositor; 