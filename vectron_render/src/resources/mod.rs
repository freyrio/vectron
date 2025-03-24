/*!
 * Resource management for the rendering system
 */

mod cache;
mod geometry;
mod texture;
mod font;

// Re-export public types from submodules
pub use cache::ResourceCache;
pub use geometry::{GeometryResource, GeometryType, GeometryDesc};
pub use texture::{TextureResource, PixelFormat};
pub use font::{FontResource, FontMetrics};

use std::collections::HashMap;
use crate::api::bare::resources::{GeometryHandle, TextureHandle, BufferHandle, ResourceId}; 