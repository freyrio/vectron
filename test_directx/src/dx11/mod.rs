//! DirectX 11 backend implementation.

pub mod backend;
pub mod graphics;
pub mod resources;
pub mod types;
pub mod debug;

pub use backend::Dx11Backend;
pub use types::{RenderCommand, SurfaceDescriptor, BufferId, PipelineId, IndexFormat};
pub use resources::{BufferDesc, BufferUsage, PipelineDesc, CullMode, ShaderType};