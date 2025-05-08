# Migration from ResourceId to Uuid

This document outlines the necessary changes for migrating from `ResourceId` to `Uuid` in the Vectron Render codebase.

## Background

The codebase has transitioned from using simple `ResourceId` (which wrapped a u64) to a more robust `Uuid` type (which contains 128-bit IDs with high and low parts). This migration improves resource identification and allows for more advanced use cases including distributed resource generation.

## Status Update

The migration is now complete. The `ResourceId` type has been completely removed from the codebase in favor of `Uuid`. All components now use `Uuid` for resource identification.

## Migrated Components

The following components now use `Uuid`:

1. Resource handles in `backend/resource.rs`:
   - `BufferHandle`
   - `TextureHandle`
   - `SamplerHandle`
   - `PipelineHandle`
   - `BindGroupHandle`
   - `ShaderHandle`

2. `GeometryHandle` in `core/traits/style.rs`

3. `PendingUpload::handle` in `backend/resource.rs`

4. `ClipRegion` path and volume handles in `core/types/common.rs`

5. `BindGroupLayoutHandle` in `backend/device.rs`

## Additional Projects Updated

### vectron_wgpu

- Updated `WgpuResourceId` to wrap `Uuid` instead of `ResourceId`
- Modified imports to use `vectron_render::core::types::common::Uuid` instead of `ResourceId`

### vectron_gpu 

- Removed `ResourceId` type alias
- Added `vectron_render` as a dependency to access the `Uuid` type

## Migration Pattern Used

```rust
// Old code (removed)
struct MyHandle(ResourceId);

impl MyHandle {
    pub fn new() -> Self {
        Self(ResourceId::new())
    }
}

// New code
struct MyHandle(Uuid);

impl MyHandle {
    pub fn new() -> Self {
        Self(Uuid::sequential())
    }
}
```

## Raw ID Access

- Use `uuid.low()` to access the low 64 bits
- Use `uuid.high()` for the high 64 bits when needed
- For a full 128-bit UUID value, use both `uuid.high()` and `uuid.low()`

## Benefits of Using Uuid

1. **Better uniqueness guarantees**: 128-bit UUIDs provide significantly better guarantees against collisions than 64-bit IDs.

2. **Future distributed resource generation**: The high 64 bits can be used to identify the source of resource generation in distributed systems.

3. **Compatibility with standard UUID formats**: The structure makes it easy to potentially interface with standard UUID formats like UUID v4.

4. **More robust resource tracking**: Enhanced debugging and resource management with better identifier uniqueness. 