// Module exports
pub mod error;
pub mod types;
pub mod geometry;
pub mod traits;
pub mod operation;

// Re-exports
pub use error::{RenderError, BackendError};
pub use geometry::{Point, Point2D, Point3D, Transform, transform_point };
pub use traits::{Renderable, Tessellable, Style, Effect, Renderer};
pub use operation::{ BatchKey, BatchableOperation, Batcher, RenderOperation, RenderOperationBuilder };
pub use types::{Uuid, TextureHandle, MaterialHandle, GeometryHandle, BufferHandle, PipelineHandle, BindGroupHandle, ShaderHandle, SamplerHandle, VertexBufferHandle, IndexBufferHandle,
    ClipRegion, FilterMode, CompareFunction, IndexFormat, Face, FrontFace, PrimitiveTopology, PolygonMode, CullMode, TessellationQuality, ResourceType, TextureFormat, Dimensionality};
