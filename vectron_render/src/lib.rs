/*!
# Vectron Render

A flexible rendering system that builds on top of the Vectron GPU abstraction.
It offers a tiered API approach for different levels of control and abstraction,
supporting 2D vector graphics, text rendering, and 3D rendering through a consistent interface.
*/

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Core modules
pub mod error;
pub mod core;
pub mod math;
// Re-export key components
pub use error::RenderError;
