// Style system for Vectron Render
//
// This module provides implementations for various styles that can be applied
// to renderable elements including fills, strokes, and patterns.

// Module exports
pub mod fill;
pub mod stroke;
pub mod paint;
pub mod material;

// Re-exports
pub use fill::*;
pub use stroke::*;
pub use paint::Paint;
pub use material::*; 