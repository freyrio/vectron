// Module exports for core traits

// Core trait definitions
pub mod renderable;
pub mod tessellable;
pub mod style;
pub mod effect;
pub mod renderer;

// Re-exports for convenient imports
pub use renderable::*;
pub use tessellable::*;
pub use style::*; 
pub use effect::*;
pub use renderer::*;