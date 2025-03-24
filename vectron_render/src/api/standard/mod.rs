/*!
 * Standard rendering API with common rendering operations
 */

mod renderer;
mod batch;
mod frame;

// Re-export public types
pub use renderer::Renderer;
pub use batch::BatchRenderer;
pub use frame::Frame;

// Common type definitions
pub use crate::resources::ResourceCache;
pub use crate::backend::RenderContext; 