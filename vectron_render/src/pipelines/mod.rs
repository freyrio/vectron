/*!
 * Specialized rendering pipelines
 */

// Vector rendering pipeline
#[cfg(feature = "vector")]
pub mod vector;

// Text rendering pipeline
#[cfg(feature = "text")]
pub mod text;

// 3D rendering pipeline
#[cfg(feature = "three_d")]
pub mod three_d;

// Effect pipeline
#[cfg(feature = "effect")]
pub mod effects;

use crate::api::bare::resources::PipelineHandle;
use crate::backend::context::GpuDevice;
use crate::error::RenderError; 