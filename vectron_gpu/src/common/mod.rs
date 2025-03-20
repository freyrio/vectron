mod error;
mod types;
mod error_context;

pub use error::*;
pub use types::*;
pub use error_context::*;

/// Generate a new unique ID for resources
pub fn generate_id() -> u64 {
    uuid::Uuid::new_v4().as_u128() as u64
} 