# Vectron GPU Architecture Blueprint

## 1. Overview

Vectron GPU is the graphics abstraction layer for the Vectron engine, designed to provide a consistent API across multiple graphics backends while maintaining small binary size and excellent developer experience.

### Goals

- **Small Binary Size**: Compile only what's needed, strip optional components
- **Developer Experience**: Intuitive API, helpful error messages, debuggability
- **Performance**: Predictable performance, minimal runtime overhead
- **Portability**: Support multiple backends (DirectX, Vulkan, Metal, etc.)
- **Modularity**: Clean separation of concerns, compile-time features

### Architecture Layers

1. **API Layer**: Public-facing interface with tiered abstraction levels
2. **Registry Layer**: Resource management and tracking
3. **Backend Layer**: Backend-specific implementations
4. **Utility Layer**: Cross-cutting concerns like debugging, profiling, and error handling

## 2. API Layer

The API layer defines the public interface of Vectron GPU, with three tiers of abstraction.

### 2.1 Tiered API Design

```rust
// Low-level API (thin abstraction)
pub trait GpuBackend {
    fn create_buffer_raw(&self, desc: &BufferDesc) -> Result<BufferHandle, GpuError>;
    // Other low-level methods...
}

// Standard API (what most applications will use)
pub struct GpuDevice {
    // Internal fields...
}

impl GpuDevice {
    pub fn create_buffer(&self, desc: BufferDesc) -> Result<Buffer, GpuError> {
        // Implementation using GpuBackend...
    }
    // Other standard methods...
}

// High-level API (convenience methods)
impl GpuDevice {
    pub fn create_vertex_buffer<T: VertexData>(&self, data: &[T]) -> Result<Buffer, GpuError> {
        // Implementation using standard API...
    }
    // Other high-level methods...
}
```

### 2.2 Resource Types

Resources are represented by handle wrappers and descriptor structs:

```rust
// Handle types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BufferHandle(u32);

// Descriptor structs (using builder pattern)
pub struct BufferDesc {
    size: usize,
    usage: BufferUsage,
    memory_flags: MemoryFlags,
    debug_name: Option<String>,
}

impl BufferDesc {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            usage: BufferUsage::DEFAULT,
            memory_flags: MemoryFlags::DEFAULT,
            debug_name: None,
        }
    }
    
    pub fn usage(mut self, usage: BufferUsage) -> Self {
        self.usage = usage;
        self
    }
    
    // Other builder methods...
}
```

### 2.3 Command Generation

Commands follow a builder pattern for ease of use and chaining:

```rust
// Command buffer with builder pattern
pub struct CommandBuffer {
    // Internal fields...
}

impl CommandBuffer {
    pub fn set_pipeline(&mut self, pipeline: &Pipeline) -> &mut Self {
        // Implementation...
        self
    }
    
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) -> &mut Self {
        // Implementation...
        self
    }
    
    pub fn draw(&mut self, vertex_count: u32, instance_count: u32) -> &mut Self {
        // Implementation...
        self
    }
    
    // Other methods...
}
```

### 2.4 Shader Interface

Shaders are pre-compiled and reflected during build:

```rust
// Shader module with reflection data
pub struct ShaderModule {
    handle: ShaderHandle,
    reflection: ShaderReflection,
}

// Generated from build-time reflection
#[derive(Debug)]
pub struct ShaderReflection {
    pub entry_point: &'static str,
    pub bindings: &'static [BindingInfo],
    pub push_constants: Option<PushConstantInfo>,
}

// Usage example
let pipeline = device.create_pipeline(PipelineDesc::new()
    .vertex_shader(shaders::BASIC_VERTEX)
    .fragment_shader(shaders::BASIC_FRAGMENT)
    .vertex_layout(layouts::BASIC_VERTEX)
    .render_target_format(TextureFormat::RGBA8_UNORM));
```

## 3. Registry Layer

The registry layer manages resource creation, tracking, and lifetime.

### 3.1 Resource Registry

```rust
// Internal registry structure
struct ResourceRegistry {
    buffers: ResourcePool<BufferEntry>,
    textures: ResourcePool<TextureEntry>,
    shaders: ResourcePool<ShaderEntry>,
    pipelines: ResourcePool<PipelineEntry>,
    samplers: ResourcePool<SamplerEntry>,
    render_targets: ResourcePool<RenderTargetEntry>,
}

// Resource pool with optional validation
struct ResourcePool<T> {
    resources: Vec<Option<T>>,
    free_list: Vec<u32>,
    
    #[cfg(feature = "validation")]
    allocation_sites: HashMap<u32, SourceLocation>,
}
```

### 3.2 Resource Lifetime

```rust
// Optional reference counting
#[cfg(feature = "ref_counting")]
struct ResourceRef<H: Handle> {
    handle: H,
    registry: Weak<ResourceRegistry>,
}

#[cfg(feature = "ref_counting")]
impl<H: Handle> Drop for ResourceRef<H> {
    fn drop(&mut self) {
        if let Some(registry) = self.registry.upgrade() {
            registry.release(self.handle);
        }
    }
}
```

### 3.3 Resource Validation

Compile-time optional validation layer:

```rust
// Validation layer that can be compiled out
#[cfg(feature = "validation")]
impl ResourceRegistry {
    fn validate_buffer_usage(&self, handle: BufferHandle, usage: BufferUsage) -> Result<(), GpuError> {
        let buffer = self.get_buffer(handle)?;
        if !buffer.usage.contains(usage) {
            return Err(GpuError::InvalidUsage {
                resource: format!("Buffer({})", handle.0),
                requested: format!("{:?}", usage),
                allowed: format!("{:?}", buffer.usage),
            });
        }
        Ok(())
    }
    
    // Other validation methods...
}

// No-op in release builds
#[cfg(not(feature = "validation"))]
impl ResourceRegistry {
    #[inline(always)]
    fn validate_buffer_usage(&self, _: BufferHandle, _: BufferUsage) -> Result<(), GpuError> {
        Ok(())
    }
}
```

## 4. Backend Layer

The backend layer implements the graphics API abstractions for each supported platform.

### 4.1 Backend Trait

```rust
// Core backend trait
pub trait GpuBackend: Send + Sync {
    fn name(&self) -> &str;
    fn features(&self) -> BackendFeatures;
    
    // Device operations
    fn create_device(&self, desc: &DeviceDesc) -> Result<Box<dyn GpuDevice>, GpuError>;
    
    // Other backend methods...
}
```

### 4.2 Backend Factory

```rust
// Backend factory function
pub fn create_backend() -> Box<dyn GpuBackend> {
    #[cfg(all(feature = "dx12", target_os = "windows"))]
    {
        return Box::new(dx12::DirectX12Backend::new());
    }
    
    #[cfg(all(feature = "vulkan", any(target_os = "windows", target_os = "linux", target_os = "android")))]
    {
        return Box::new(vulkan::VulkanBackend::new());
    }
    
    #[cfg(all(feature = "metal", any(target_os = "macos", target_os = "ios")))]
    {
        return Box::new(metal::MetalBackend::new());
    }
    
    // Fallback to software rendering or panic
    #[cfg(feature = "software")]
    {
        return Box::new(software::SoftwareBackend::new());
    }
    
    panic!("No compatible GPU backend available");
}
```

### 4.3 Backend Implementation Structure

Each backend implementation is split into functional modules:

```
vectron_gpu/src/backends/directx/
├── mod.rs                 # Public exports
├── dx12.rs                # Main backend struct and trait implementation
├── device.rs              # Device creation and management
├── debug.rs               # DirectX-specific debug features
├── error.rs               # Backend-specific error extensions
├── resources/             # Resource management
│   ├── mod.rs
│   ├── buffer.rs          # Buffer implementation
│   ├── texture.rs         # Texture implementation
│   ├── shader.rs          # Shader implementation
│   └── pipeline.rs        # Pipeline implementation
├── commands.rs            # Command recording and execution
├── surface.rs             # Surface/swapchain operations
└── types.rs               # DirectX12-specific types
```

### 4.4 Backend Extension Traits

Extension traits for modular implementation:

```rust
// Extension trait for device operations
mod device {
    use super::DirectX12Backend;
    
    pub(super) trait DeviceExt {
        fn create_device_impl(&self, desc: &DeviceDesc) -> Result<DirectX12Device, GpuError>;
        // Other device-related methods...
    }
    
    impl DeviceExt for DirectX12Backend {
        fn create_device_impl(&self, desc: &DeviceDesc) -> Result<DirectX12Device, GpuError> {
            // Implementation...
        }
        // Other method implementations...
    }
}

// Main implementation delegates to extension traits
impl GpuBackend for DirectX12Backend {
    fn create_device(&self, desc: &DeviceDesc) -> Result<Box<dyn GpuDevice>, GpuError> {
        let device = self.create_device_impl(desc)?;
        Ok(Box::new(device))
    }
    // Other delegating methods...
}
```

## 5. Error System

The error system provides robust error handling with cross-cutting utilities and backend-specific extensions.

### 5.1 Core Error Types

```rust
// In src/common/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpuError {
    #[error("Backend initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Invalid handle: {0}")]
    InvalidHandle(String),
    
    #[error("Invalid resource: {0}")]
    InvalidResource(String),
    
    #[error("Invalid buffer: {0}")]
    InvalidBuffer(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Resource creation failed: {0}")]
    ResourceCreationFailed(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Surface error: {0}")]
    SurfaceError(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("Unimplemented feature: {0}")]
    Unimplemented(String),
    
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Device lost: {0}")]
    DeviceLost(String),
    
    #[error("Buffer update failed (size: {size}, offset: {offset}): {reason}")]
    BufferUpdateFailed {
        size: usize,
        offset: usize,
        reason: String,
    },
    
    #[error("Pipeline creation failed: {reason}")]
    PipelineCreationFailed {
        reason: String,
        shader_errors: Option<Vec<String>>,
    },
    
    // Backend error with code formatting
    #[error("Backend error [{backend}]: {message}{}", format_code(.code))]
    BackendError {
        backend: String,
        message: String,
        code: Option<i32>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl GpuError {
    // Helper method to create a backend error with formatted code
    pub fn backend_error<S: Into<String>>(
        backend: S, 
        message: S, 
        code: Option<i32>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>
    ) -> Self {
        GpuError::BackendError {
            backend: backend.into(),
            message: message.into(),
            code,
            source,
        }
    }
}

// Helper function for thiserror to format the code
fn format_code(code: &Option<i32>) -> String {
    match code {
        Some(code) => format!(" (code: 0x{:X})", code),
        None => String::new(),
    }
}
```

### 5.2 Error Context

```rust
// In src/common/error_context.rs
use super::GpuError;

pub struct ErrorContext {
    pub function: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl ErrorContext {
    pub fn current() -> Self {
        Self {
            function: "",
            file: file!(),
            line: line!(),
        }
    }
    
    pub fn log(&self, error: &GpuError) {
        log::error!("{} at {}:{}: {}", 
                  self.function, self.file, self.line, error);
    }
}

// Utility macro for error propagation with context
#[macro_export]
macro_rules! gpu_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let ctx = $crate::common::ErrorContext::current();
                ctx.log(&e);
                return Err(e);
            }
        }
    };
    ($expr:expr, $context:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                log::error!("{} at {}:{}: {}", 
                          $context, file!(), line!(), e);
                return Err(e);
            }
        }
    };
}
```

### 5.3 Backend-Specific Error Extensions

```rust
// In src/backends/directx/error.rs
use crate::common::GpuError;
use windows::core::Error as WindowsError;

pub trait DirectXErrorExt {
    fn to_gpu_error(self, context: &str) -> GpuError;
}

impl DirectXErrorExt for WindowsError {
    fn to_gpu_error(self, context: &str) -> GpuError {
        let code = self.code().0;
        
        // Map specific DirectX error codes to appropriate GpuError variants
        match code {
            // Device removed/reset (0x887A0005)
            0x887A0005 => GpuError::DeviceLost(format!("{}: {}", context, self)),
            
            // Out of memory (0x8007000E)
            0x8007000E => GpuError::OutOfMemory,
            
            // Invalid arguments (0x80070057)
            0x80070057 => GpuError::InvalidArgument(format!("{}: {}", context, self)),
            
            // General backend error for other codes
            _ => GpuError::backend_error(
                "DirectX12", 
                format!("{}: {}", context, self),
                Some(code as i32),
                Some(Box::new(self))
            ),
        }
    }
}

// Usage example
fn create_device_impl(&self, desc: &DeviceDesc) -> Result<DirectX12Device, GpuError> {
    let device: ID3D12Device = unsafe {
        D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)
            .map_err(|e| e.to_gpu_error("Failed to create D3D12 device"))?
    };
    
    // Rest of implementation...
    Ok(device)
}
```

## 6. Shader System

The shader system handles compilation, reflection, and variant generation.

### 6.1 Build-time Shader Compilation

```rust
// build.rs
fn main() {
    let shader_compiler = ShaderCompiler::new();
    
    // Compile shader for all backends
    shader_compiler.compile("shaders/basic.glsl", "shaders/compiled/basic")
        .with_define("MAX_LIGHTS", "4")
        .with_backends(&["dx12", "vulkan", "metal"])
        .generate_reflection(true)
        .run()
        .expect("Failed to compile shader");
}
```

### 6.2 Generated Shader Modules

```rust
// Generated from build process
pub mod shaders {
    pub struct BasicShader {
        #[cfg(feature = "dx12")]
        pub dx12: ShaderBytecode,
        #[cfg(feature = "vulkan")]
        pub vulkan: ShaderBytecode,
        #[cfg(feature = "metal")]
        pub metal: ShaderBytecode,
        
        pub reflection: ShaderReflection,
    }
    
    pub const BASIC_VERTEX: BasicShader = BasicShader {
        #[cfg(feature = "dx12")]
        dx12: ShaderBytecode {
            data: include_bytes!("../shaders/compiled/basic.vertex.dx12.bin"),
            entry_point: "main",
        },
        // Other backend bytecode...
        
        reflection: ShaderReflection {
            bindings: &[
                BindingInfo { set: 0, binding: 0, name: "u_Matrices", ty: BindingType::UniformBuffer },
                // Other bindings...
            ],
            // Other reflection data...
        },
    };
}
```

### 6.3 Shader Variants

```rust
// Variant generation
pub struct ShaderVariantBuilder<'a> {
    base: &'a ShaderModule,
    defines: HashMap<String, String>,
}

impl<'a> ShaderVariantBuilder<'a> {
    pub fn define(mut self, name: &str, value: &str) -> Self {
        self.defines.insert(name.to_string(), value.to_string());
        self
    }
    
    pub fn build(&self, device: &GpuDevice) -> Result<ShaderModule, GpuError> {
        // Implementation...
    }
}

// Usage
let shadow_variant = shaders::BASIC_VERTEX.variant()
    .define("SHADOW_PASS", "1")
    .build(&device)?;
```

## 7. Debug System

The debug system provides both backend-specific debugging and cross-cutting utilities.

### 7.1 Cross-Cutting Debug Utilities

```rust
// In src/debug/mod.rs
mod labels;
mod markers;
mod capture;
mod validation;
mod resource_tracker;

pub use labels::ResourceLabels;
pub use markers::PerformanceMarkers;
pub use capture::FrameCapture;
pub use resource_tracker::ResourceTracker;

// Public API consolidation
pub struct GpuProfiler {
    #[cfg(feature = "gpu_timers")]
    markers: PerformanceMarkers,
}

impl GpuProfiler {
    pub fn begin_scope(&mut self, cmd: &mut CommandBuffer, name: &str) {
        #[cfg(feature = "gpu_timers")]
        self.markers.begin_scope(cmd, name);
    }
    
    pub fn end_scope(&mut self, cmd: &mut CommandBuffer) {
        #[cfg(feature = "gpu_timers")]
        self.markers.end_scope(cmd);
    }
    
    // Other methods...
}

pub struct GpuLogger;

impl GpuLogger {
    pub fn set_resource_name<T: Resource>(resource: &T, name: &str) {
        #[cfg(feature = "debug_labels")]
        ResourceLabels::set_label(resource, name);
    }
    
    // Other methods...
}
```

### 7.2 Backend-Specific Debug Implementation

```rust
// In src/backends/directx/debug.rs
use crate::{debug::resource_tracker::ResourceTracker, GpuError};
use windows::Win32::Graphics::Direct3D12::*;
use windows::core::Interface;

pub struct DirectX12Debug {
    info_queue: Option<ID3D12InfoQueue>,
    debug_device: Option<ID3D12DebugDevice>,
    resource_tracker: ResourceTracker<D3D12_RESOURCE_STATES>,
}

impl DirectX12Debug {
    pub fn new(device: &ID3D12Device) -> Result<Self, GpuError> {
        let info_queue = match device.cast::<ID3D12InfoQueue>() {
            Ok(q) => {
                // Configure queue
                unsafe {
                    // Don't break on warnings in release mode
                    let break_severity = if cfg!(debug_assertions) {
                        D3D12_MESSAGE_SEVERITY_WARNING
                    } else {
                        D3D12_MESSAGE_SEVERITY_ERROR
                    };
                    
                    let _ = q.SetBreakOnSeverity(break_severity, true);
                    let _ = q.SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_CORRUPTION, true);
                }
                Some(q)
            },
            Err(_) => None
        };
        
        let debug_device = match device.cast::<ID3D12DebugDevice>() {
            Ok(d) => Some(d),
            Err(_) => None,
        };
        
        Ok(Self {
            info_queue,
            debug_device,
            resource_tracker: ResourceTracker::new(),
        })
    }
    
    pub fn track_resource(&mut self, id: u64, name: &str, initial_state: D3D12_RESOURCE_STATES) {
        self.resource_tracker.track(id, name.to_string(), initial_state);
    }
    
    pub fn update_resource_state(&mut self, id: u64, new_state: D3D12_RESOURCE_STATES) -> Result<(), GpuError> {
        self.resource_tracker.update_state(id, new_state)
    }
    
    pub fn validate_resource_state(&self, id: u64, expected_state: D3D12_RESOURCE_STATES) -> Result<(), GpuError> {
        // Validate that the resource is in the expected state
        // Implementation depends on how resource_tracker is structured
        Ok(())
    }
    
    pub fn check_and_log_messages(&self) {
        if let Some(info_queue) = &self.info_queue {
            unsafe {
                let count = info_queue.GetNumStoredMessages();
                for i in 0..count {
                    // Get the size needed for the message
                    let mut size = 0;
                    let _ = info_queue.GetMessage(i, None, &mut size);
                    
                    // Only process if we got a valid size
                    if size > 0 {
                        // Allocate buffer and get the message
                        let mut buffer = vec![0u8; size as usize];
                        let message_ptr = buffer.as_mut_ptr() as *mut _;
                        
                        if let Ok(_) = info_queue.GetMessage(i, Some(message_ptr), &mut size) {
                            // Process message here
                            log::debug!("DirectX12 Debug: Message {}", i);
                        }
                        
                    }
                    
                    // Parse message and log according to severity
                    // Simplified for brevity
                }
                
                // Clear messages after logging
                info_queue.ClearStoredMessages();
            }
        }
    }
    
    pub fn report_live_objects(&self) {
        if let Some(debug_device) = &self.debug_device {
            log::info!("Reporting live DirectX 12 objects...");
            unsafe {
                let _ = debug_device.ReportLiveDeviceObjects(D3D12_RLDO_DETAIL);
            }
        }
    }
    
    // More DirectX-specific debugging utilities
}
```

## 8. Pipeline State Objects

Pipeline state objects define the complete graphics or compute pipeline configuration.

### 8.1 Pipeline State Descriptors

```rust
// Pipeline descriptor
pub struct PipelineDesc {
    shaders: PipelineShaders,
    vertex_layout: Option<VertexLayout>,
    primitive_type: PrimitiveType,
    rasterizer_state: RasterizerState,
    blend_state: BlendState,
    depth_stencil_state: DepthStencilState,
    render_target_formats: SmallVec<[TextureFormat; 4]>,
    depth_format: Option<TextureFormat>,
    sample_count: u32,
    debug_name: Option<String>,
}

// Builder pattern
impl PipelineDesc {
    pub fn new() -> Self {
        // Default implementation...
    }
    
    pub fn vertex_shader(mut self, shader: &ShaderModule) -> Self {
        self.shaders.vertex = Some(shader.clone());
        self
    }
    
    // Other builder methods...
}
```

### 8.2 Pipeline Caching

```rust
// Pipeline cache mechanism
struct PipelineCache {
    cache_key_to_handle: HashMap<PipelineCacheKey, PipelineHandle>,
    persistent_cache: Option<PersistentCache>,
    async_queue: Option<AsyncPipelineQueue>,
}

impl PipelineCache {
    pub fn get_or_create(
        &mut self, 
        desc: &PipelineDesc,
        backend: &dyn GpuBackend
    ) -> Result<PipelineHandle, GpuError> {
        let key = self.compute_cache_key(desc);
        
        if let Some(handle) = self.cache_key_to_handle.get(&key) {
            return Ok(*handle);
        }
        
        // Try to load from persistent cache
        if let Some(handle) = self.try_load_from_persistent_cache(&key)? {
            return Ok(handle);
        }
        
        // Create new pipeline
        let handle = backend.create_pipeline_raw(desc)?;
        self.cache_key_to_handle.insert(key, handle);
        
        Ok(handle)
    }
    
    // Other methods...
}
```

### 8.3 Asynchronous Pipeline Creation

```rust
// Async pipeline creation
struct AsyncPipelineQueue {
    queue: crossbeam_channel::Sender<AsyncPipelineJob>,
    results: Mutex<HashMap<u64, PipelineCreationResult>>,
}

impl AsyncPipelineQueue {
    pub fn schedule(&self, desc: PipelineDesc, priority: u32) -> AsyncPipelineHandle {
        let job_id = generate_unique_id();
        let job = AsyncPipelineJob { id: job_id, desc, priority };
        
        self.queue.send(job).expect("Pipeline worker thread died");
        
        AsyncPipelineHandle { id: job_id }
    }
    
    pub fn is_ready(&self, handle: AsyncPipelineHandle) -> bool {
        self.results.lock().unwrap().contains_key(&handle.id)
    }
    
    pub fn get(&self, handle: AsyncPipelineHandle) -> Option<Result<PipelineHandle, GpuError>> {
        self.results.lock().unwrap().remove(&handle.id)
    }
}
```

## 9. Memory Management

Memory management strategies for efficient resource allocation.

### 9.1 Memory Allocation

```rust
// Memory allocation strategy
pub enum MemoryFlags {
    Default,
    GpuOnly,
    CpuVisible,
    CpuCached,
    Transient,
    Custom(u32),
}

// Heap allocator
struct HeapAllocator {
    heaps: Vec<Heap>,
    small_block_allocator: SmallBlockAllocator,
    large_block_allocator: LargeBlockAllocator,
}

impl HeapAllocator {
    fn allocate(&mut self, size: usize, alignment: usize, flags: MemoryFlags) -> Result<Allocation, GpuError> {
        if size <= SMALL_ALLOCATION_THRESHOLD {
            self.small_block_allocator.allocate(size, alignment, flags)
        } else if size <= LARGE_ALLOCATION_THRESHOLD {
            self.large_block_allocator.allocate(size, alignment, flags)
        } else {
            self.allocate_dedicated_heap(size, alignment, flags)
        }
    }
    
    // Other methods...
}
```

### 9.2 Resource Pooling

```rust
// Resource pool for frequently created/destroyed resources
pub struct ResourcePool<T> {
    available: Vec<T>,
    in_use: HashMap<ResourceHandle, T>,
    allocator: Allocator,
}

impl<T: PoolableResource> ResourcePool<T> {
    pub fn acquire(&mut self, desc: &T::Descriptor) -> Result<ResourceHandle, GpuError> {
        // Try to find a matching resource in the available pool
        if let Some(index) = self.find_compatible_resource(desc) {
            let resource = self.available.swap_remove(index);
            let handle = ResourceHandle(self.next_handle);
            self.next_handle += 1;
            self.in_use.insert(handle, resource);
            return Ok(handle);
        }
        
        // Otherwise allocate a new resource
        let resource = T::create(&self.allocator, desc)?;
        let handle = ResourceHandle(self.next_handle);
        self.next_handle += 1;
        self.in_use.insert(handle, resource);
        
        Ok(handle)
    }
    
    pub fn release(&mut self, handle: ResourceHandle) {
        if let Some(resource) = self.in_use.remove(&handle) {
            // Return to available pool if under capacity, otherwise destroy
            if self.available.len() < self.max_pool_size {
                self.available.push(resource);
            } else {
                resource.destroy(&self.allocator);
            }
        }
    }
}
```

### 9.3 Deferred Destruction

```rust
// Deferred resource destruction
struct DeferredDestructionQueue {
    queues: Vec<Vec<PendingDestruction>>,
    current_frame: usize,
}

impl DeferredDestructionQueue {
    pub fn enqueue(&mut self, resource: ResourceHandle, destructor: Box<dyn FnOnce() + Send + Sync>) {
        let pending = PendingDestruction { resource, destructor };
        self.queues[self.current_frame].push(pending);
    }
    
    pub fn advance_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.queues.len();
        
        // Process all destructions from the new current frame
        for pending in self.queues[self.current_frame].drain(..) {
            (pending.destructor)();
        }
    }
}
```

## 10. Concurrency Model

Approach to multi-threaded command generation and execution.

### 10.1 Thread-Safe Resource Creation

```rust
// Thread-safe device with interior mutability
pub struct GpuDevice {
    registry: Arc<RwLock<ResourceRegistry>>,
    backend: Arc<dyn GpuBackend>,
    // Other fields...
}

impl GpuDevice {
    pub fn create_buffer(&self, desc: BufferDesc) -> Result<Buffer, GpuError> {
        let handle = {
            let mut registry = self.registry.write().unwrap();
            let backend_buffer = self.backend.create_buffer_raw(&desc)?;
            registry.register_buffer(backend_buffer, &desc)
        };
        
        Ok(Buffer { handle, device: Arc::downgrade(&self.device) })
    }
    
    // Other methods...
}
```

### 10.2 Parallel Command Recording

```rust
// Thread-local command buffer recording
pub struct CommandRecorder {
    thread_id: ThreadId,
    command_pool: ThreadLocal<CommandPool>,
}

impl CommandRecorder {
    pub fn begin_recording(&self) -> CommandBuffer {
        let pool = self.command_pool.get_or_init(|| CommandPool::new(self.thread_id));
        pool.create_command_buffer()
    }
    
    pub fn submit(&self, cmd: CommandBuffer) -> SubmitToken {
        let pool = self.command_pool.get().expect("Command buffer from wrong thread");
        pool.submit_command_buffer(cmd)
    }
}

// Parallel recording usage example
let cmd1 = scope.spawn(|| {
    let cmd = device.begin_recording();
    // Record scene objects...
    device.submit(cmd)
});

let cmd2 = scope.spawn(|| {
    let cmd = device.begin_recording();
    // Record UI elements...
    device.submit(cmd)
});

// Wait for both recording threads
let token1 = cmd1.join().unwrap();
let token2 = cmd2.join().unwrap();

// Execute the command buffers
device.execute(&[token1, token2]);
```

### 10.3 Work Stealing

```rust
// Work stealing for command generation
struct WorkStealingCommandRecorder {
    global_queue: WorkStealingQueue<RenderJob>,
    thread_pool: ThreadPool,
}

impl WorkStealingCommandRecorder {
    pub fn schedule(&self, job: RenderJob) {
        self.global_queue.push(job);
    }
    
    pub fn execute_all(&self) -> Vec<CommandBuffer> {
        self.thread_pool.execute(|| {
            while let Some(job) = self.global_queue.pop() {
                let cmd = self.record_job(job);
                self.completed_commands.lock().unwrap().push(cmd);
            }
        });
        
        // Wait for all jobs to complete
        self.thread_pool.wait();
        
        // Collect all generated command buffers
        std::mem::take(&mut *self.completed_commands.lock().unwrap())
    }
}
```

## 11. Feature Flags and Compilation

Control over which components are compiled into the binary.

### 11.1 Cargo Features

```toml
# In Cargo.toml
[features]
default = ["dx12", "validation"]

# Backends
dx12 = []
vulkan = []
metal = []
software = []

# Debug features
validation = []
debug_labels = []
gpu_timers = ["validation"]
frame_capture = ["validation"]
profiling = ["gpu_timers"]

# Optimizations
small_binary = []
ref_counting = []
persistent_cache = []
```

### 11.2 Conditional Compilation

```rust
// Conditional backend instantiation
pub fn create_device(desc: &DeviceDesc) -> Result<Arc<GpuDevice>, GpuError> {
    #[cfg(feature = "dx12")]
    if desc.prefer_backend == BackendType::DirectX12 {
        if let Ok(backend) = dx12::create_backend() {
            return Ok(Arc::new(GpuDevice::new(backend, desc)?));
        }
    }
    
    #[cfg(feature = "vulkan")]
    if desc.prefer_backend == BackendType::Vulkan {
        if let Ok(backend) = vulkan::create_backend() {
            return Ok(Arc::new(GpuDevice::new(backend, desc)?));
        }
    }
    
    // Try any available backend
    #[cfg(feature = "dx12")]
    if let Ok(backend) = dx12::create_backend() {
        return Ok(Arc::new(GpuDevice::new(backend, desc)?));
    }
    
    #[cfg(feature = "vulkan")]
    if let Ok(backend) = vulkan::create_backend() {
        return Ok(Arc::new(GpuDevice::new(backend, desc)?));
    }
    
    // No compatible backend
    Err(GpuError::NoCompatibleBackend)
}
```

### 11.3 Binary Size Optimizations

```rust
// Optimize for size with small_binary feature
#[cfg(feature = "small_binary")]
type ResourceMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[cfg(not(feature = "small_binary"))]
type ResourceMap<K, V> = std::collections::HashMap<K, V>;

// Stripped error messages
#[cfg(feature = "small_binary")]
fn format_error(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::DeviceLost => "Device lost",
        // Other minimal messages...
    }
}

#[cfg(not(feature = "small_binary"))]
fn format_error(code: ErrorCode) -> String {
    match code {
        ErrorCode::DeviceLost => "The graphics device was lost. This may happen due to a TDR event, driver crash, or device removal.",
        // Other detailed messages...
    }
}
```

## 12. Summary and Project Structure

The Vectron GPU architecture provides a comprehensive foundation for graphics programming with a focus on flexibility, performance, and developer experience.

### 12.1 Key Architectural Benefits

1. **Tiered API**: Three abstraction levels (low, standard, high) to accommodate different development needs
2. **Modular Backend**: Clean separation by functionality with extension traits
3. **Error Handling**: Rich error system with cross-cutting utilities and backend-specific extensions
4. **Debug System**: Hybrid approach with both common utilities and backend-specific implementations
5. **Resource Management**: Efficient tracking, validation, and lifetime management
6. **Pipeline Optimization**: Caching and asynchronous creation for better performance
7. **Concurrency**: Thread-safe design for multi-core utilization
8. **Compilation Control**: Feature flags for minimal binary size

### 12.2 Project Structure

```
vectron_gpu/
├── Cargo.toml                  # Crate configuration with features
├── build.rs                    # Build script for shader compilation
│
├── src/
│   ├── lib.rs                  # Main entry point and public API
│   │
│   ├── common/                 # Common types and utilities
│   │   ├── mod.rs
│   │   ├── error.rs            # Core error types
│   │   ├── error_context.rs    # Error context utilities
│   │   └── types.rs            # Common type definitions
│   │
│   ├── debug/                  # Cross-cutting debug utilities
│   │   ├── mod.rs
│   │   ├── labels.rs           # Resource naming utilities 
│   │   ├── markers.rs          # Performance region markers
│   │   ├── capture.rs          # Frame capture system
│   │   └── validation.rs       # Validation utilities
│   │
│   ├── registry/               # Resource registration and tracking
│   │   ├── mod.rs
│   │   ├── pool.rs             # Resource pool implementation
│   │   └── tracking.rs         # Resource state tracking
│   │
│   ├── backends/               # Backend implementations
│   │   ├── mod.rs              # Backend factory
│   │   │
│   │   ├── directx/            # DirectX backend
│   │   │   ├── mod.rs          # Public exports
│   │   │   ├── dx12.rs         # Main implementation
│   │   │   ├── debug.rs        # DirectX-specific debug
│   │   │   └── error.rs        # DirectX-specific errors
│   │   │
│   │   ├── vulkan/             # Vulkan backend
│   │   │   ├── mod.rs
│   │   │   ├── vulkan.rs
│   │   │   ├── debug.rs
│   │   │   └── error.rs
│   │   │
│   │   └── ...                 # Other backends
│   │
│   └── shaders/                # Shader system
│       ├── mod.rs
│       ├── compiler.rs         # Shader compilation utilities
│       └── reflection.rs       # Shader reflection
│
└── examples/                   # Example applications
    ├── triangle.rs
    └── compute.rs
```

This architecture balances developer experience, performance, and binary size with compile-time feature selection, allowing applications to include only what they need while maintaining a clean, consistent API.