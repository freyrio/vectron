mod error;
mod types;

pub use error::*;
pub use types::*;

/// Generate a new unique ID for resources
pub fn generate_id() -> u64 {
    uuid::Uuid::new_v4().as_u128() as u64
} 