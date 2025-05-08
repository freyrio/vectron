// Resource management for Vectron Render
//
// This module provides structures for managing rendering resources
// such as buffers, textures, and pipelines.

pub mod cache;
pub mod buffer;
pub mod upload;

// Re-exports
pub use buffer::GeometryBuffer;
pub use cache::ResourceCache;
pub use upload::{PendingUpload, UploadQueue}; 