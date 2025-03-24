/*!
 * API layer with multiple tiers of abstraction
 */

// Low-level drawing API
pub mod bare;

// Standard rendering API
pub mod standard;

// Conditionally include feature-specific APIs
#[cfg(feature = "vector")]
pub mod vector;

#[cfg(feature = "text")]
pub mod text;

#[cfg(feature = "three_d")]
pub mod three_d;

// Re-export common API types at the module level
pub use bare::DrawCommand;
pub use standard::Renderer; 