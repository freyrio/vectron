/*!
 * Backend abstraction for interfacing with the GPU
 */

mod context;
mod commands;
mod state;
mod binding;

// Re-export public types
pub use context::RenderContext;
pub use commands::CommandTranslator;
pub use state::RenderState;
pub use binding::{ResourceBinding, ResourceBindingType}; 