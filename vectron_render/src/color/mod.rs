// Color system for Vectron Render
//
// This module provides a comprehensive color system with various color spaces,
// gradient definitions, blending modes, and named colors.

mod types;
mod blend;
mod gradient;
mod named;
pub mod spaces;

// Public re-exports
pub use types::*;
pub use blend::*;
pub use gradient::{LinearGradient, RadialGradient, ConicGradient};
pub use named::*;
pub use spaces::*;

// Common color types and utility functions may be defined here 