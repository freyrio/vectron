// Transform module exports
pub mod transform2;
pub mod transform3;

// Re-exports for convenience
pub use transform2::*;
pub use transform3::*;

use crate::core::types::common::Dimensionality;

/// A transform either in 2D or 3D space.
/// This is a runtime abstraction that can hold either type of transform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformInSpace {
    D2(Transform2D),
    D3(Transform3D),
}

impl TransformInSpace {
    /// Creates a new identity transform in the specified dimensionality.
    pub fn identity(dim: Dimensionality) -> Self {
        match dim {
            Dimensionality::D2 => Self::D2(Transform2D::identity()),
            Dimensionality::D3 => Self::D3(Transform3D::identity()),
        }
    }
    
    /// Returns the dimensionality of this transform.
    pub fn dimensionality(&self) -> Dimensionality {
        match self {
            Self::D2(_) => Dimensionality::D2,
            Self::D3(_) => Dimensionality::D3,
        }
    }
} 