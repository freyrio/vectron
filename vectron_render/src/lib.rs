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
pub mod utils;
pub mod backend;
pub mod resources;
pub mod pipelines;
pub mod api;
pub mod core;
pub mod commands;
pub mod shaders;

// Re-export key components
pub use error::RenderError;
pub use api::{Vectron, VectronBuilder, RenderOptions};
pub use resources::ResourceManager;

// Re-export core components for the new operation-centric approach
pub use core::drawable::{Drawable, RenderContext, RenderHints};
pub use core::style::{Style, Fill, Stroke, Paint};
pub use core::effect::{Effect, Shadow, Blur, Glow};
pub use core::operation::Operation;
pub use core::color::{Color, constants as color_constants};
pub use commands::{CommandQueue, BatchQueue, OptimizingQueue};

// Public API for convenient imports using feature flags
#[cfg(feature = "vector")]
pub use api::vector;

#[cfg(feature = "text")]
pub use api::text;

#[cfg(feature = "three_d")]
pub use api::three_d;
