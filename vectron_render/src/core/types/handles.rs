// Resource handle system using phantom types for Vectron Render
//
// This module implements a type-safe handle system for various resources using
// the "phantom type" pattern. This approach provides compile-time type safety
// while maintaining a consistent underlying representation.
//
// ## Phantom Types Pattern
//
// Phantom types are type parameters that don't appear in the data structure's fields
// but are used to create distinct types for the type system. This allows us to create
// different handle types (TextureHandle, MaterialHandle, etc.) that are structurally
// identical but treated as completely different types by the compiler.
//
// Benefits of this approach:
//
// 1. **Type Safety**: Functions can require specific handle types, preventing accidental
//    use of a BufferHandle where a TextureHandle is expected.
//
// 2. **Unified Implementation**: All handle types share the same underlying code,
//    reducing duplication while maintaining type distinctions.
//
// 3. **Extensibility**: New handle types can be added by simply defining a new marker
//    type and type alias, without implementing the same methods repeatedly.
//
// 4. **Size Efficiency**: All handles remain the same size as a single UUID,
//    with the type information existing only at compile time.
//
// ## Usage Example
//
// ```
// // Function that specifically requires a texture handle
// fn bind_texture(texture: TextureHandle) { /* ... */ }
//
// // This compiles:
// let texture = TextureHandle::new();
// bind_texture(texture);
//
// // This will not compile:
// let buffer = BufferHandle::new();
// bind_texture(buffer); // Type error!
// ```

use super::uuid::Uuid;
use std::marker::PhantomData;

/// Generic resource handle with phantom type for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceHandle<T> {
    id: Uuid,
    _marker: PhantomData<T>,
}

impl<T> ResourceHandle<T> {
    /// Create a new handle with the specified ID
    pub fn with_id(id: Uuid) -> Self {
        Self { 
            id, 
            _marker: PhantomData 
        }
    }
    
    /// Create a new handle with a sequential ID
    pub fn sequential() -> Self {
        Self::with_id(Uuid::sequential())
    }
    
    /// Create a new handle from a u64 value
    pub fn from_u64(value: u64) -> Self {
        Self::with_id(Uuid::from_u64(value))
    }
    
    /// Get the UUID value of this handle
    pub fn uuid(&self) -> Uuid {
        self.id
    }
    
    /// Get the ID of this handle (shorthand for uuid)
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    /// Get the low 64 bits of this handle's UUID (for backward compatibility)
    pub fn value(&self) -> u64 {
        self.id.low()
    }
    
    /// Convert this handle to a raw UUID
    pub fn to_uuid(&self) -> Uuid {
        self.id
    }
    
    /// Create a handle from a raw UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self::with_id(uuid)
    }
}

impl<T> Default for ResourceHandle<T> 
where
    ResourceHandle<T>: Clone
{
    fn default() -> Self {
        Self::sequential()
    }
}

// Define marker types for different resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometryMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipelineMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroupMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexBufferMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexBufferMarker;

// Type aliases for each resource handle
pub type TextureHandle = ResourceHandle<TextureMarker>;
pub type PatternHandle = ResourceHandle<PatternMarker>;
pub type MaterialHandle = ResourceHandle<MaterialMarker>;
pub type GeometryHandle = ResourceHandle<GeometryMarker>;
pub type BufferHandle = ResourceHandle<BufferMarker>;
pub type SamplerHandle = ResourceHandle<SamplerMarker>;
pub type PipelineHandle = ResourceHandle<PipelineMarker>;
pub type BindGroupHandle = ResourceHandle<BindGroupMarker>;
pub type ShaderHandle = ResourceHandle<ShaderMarker>;
pub type VertexBufferHandle = ResourceHandle<VertexBufferMarker>;
pub type IndexBufferHandle = ResourceHandle<IndexBufferMarker>;

// Implement any resource-specific methods if needed
impl TextureHandle {
    /// Create a new texture handle with no arguments (creates sequential ID)
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl PatternHandle {
    // Add pattern-specific methods here if needed
}

impl MaterialHandle {
    // Add material-specific methods here if needed
}

impl GeometryHandle {
    /// Create a new geometry handle
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl BufferHandle {
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl SamplerHandle {
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl PipelineHandle {
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl BindGroupHandle {
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl ShaderHandle {
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl VertexBufferHandle {
    /// Create a new vertex buffer handle
    pub fn new() -> Self {
        Self::sequential()
    }
}

impl IndexBufferHandle {
    /// Create a new index buffer handle
    pub fn new() -> Self {
        Self::sequential()
    }
}