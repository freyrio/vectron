/*!
 * Raw resource handling for the bare API
 */

/// Type alias for resource IDs
pub type ResourceId = u64;

/// Type-safe handle for a geometry resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GeometryHandle(pub(crate) ResourceId);

/// Type-safe handle for a texture resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureHandle(pub(crate) ResourceId);

/// Type-safe handle for a buffer resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BufferHandle(pub(crate) ResourceId);

/// Type-safe handle for a pipeline resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PipelineHandle(pub(crate) ResourceId);

/// Type-safe handle for a shader resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShaderHandle(pub(crate) ResourceId);

/// Type-safe handle for a sampler resource
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SamplerHandle(pub(crate) ResourceId); 