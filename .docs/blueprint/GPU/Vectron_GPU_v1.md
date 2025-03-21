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
4. **Utility Layer**: Cross-cutting concerns like debugging, profiling

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

## 5. Shader System

The shader system handles compilation, reflection, and variant generation.

### 5.1 Build-time Shader Compilation

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

### 5.2 Generated Shader Modules

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

### 5.3 Shader Variants

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

## 6. Pipeline State Objects

Pipeline state objects define the complete graphics or compute pipeline configuration.

### 6.1 Pipeline State Descriptors

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

### 6.2 Pipeline Caching

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

### 6.3 Asynchronous Pipeline Creation

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

## 7. Debugging and Profiling

Debugging and profiling tools that can be optionally compiled.

### 7.1 Resource Labeling

```rust
// Debug names for resources
impl BufferDesc {
    pub fn label(mut self, name: impl Into<String>) -> Self {
        #[cfg(feature = "debug_labels")]
        {
            self.debug_name = Some(name.into());
        }
        self
    }
}

// Implementation
#[cfg(feature = "debug_labels")]
fn set_object_name(device: &ID3D12Device, obj: impl ID3D12Object, name: &str) {
    let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    obj.SetName(PWSTR(wide_name.as_ptr() as *mut _)).ok();
}

#[cfg(not(feature = "debug_labels"))]
fn set_object_name(_: &ID3D12Device, _: impl ID3D12Object, _: &str) {}
```

### 7.2 GPU Timers

```rust
// GPU timing system
pub struct GpuTimer {
    #[cfg(feature = "gpu_timers")]
    internal: GpuTimerImpl,
}

impl GpuTimer {
    pub fn begin_scope(&mut self, cmd: &mut CommandBuffer, name: &str) {
        #[cfg(feature = "gpu_timers")]
        self.internal.begin_scope(cmd, name);
    }
    
    pub fn end_scope(&mut self, cmd: &mut CommandBuffer) {
        #[cfg(feature = "gpu_timers")]
        self.internal.end_scope(cmd);
    }
    
    pub fn get_timing_results(&self) -> Vec<TimingResult> {
        #[cfg(feature = "gpu_timers")]
        return self.internal.get_timing_results();
        
        #[cfg(not(feature = "gpu_timers"))]
        return Vec::new();
    }
}
```

### 7.3 Debug Capture System

```rust
// Frame capture system
pub struct FrameCapture {
    #[cfg(feature = "frame_capture")]
    internal: FrameCaptureImpl,
}

impl FrameCapture {
    pub fn begin(&mut self, device: &GpuDevice) {
        #[cfg(feature = "frame_capture")]
        self.internal.begin(device);
    }
    
    pub fn end(&mut self) -> Option<FrameCaptureSummary> {
        #[cfg(feature = "frame_capture")]
        return Some(self.internal.end());
        
        #[cfg(not(feature = "frame_capture"))]
        return None;
    }
    
    pub fn save(&self, path: &Path) -> Result<(), GpuError> {
        #[cfg(feature = "frame_capture")]
        return self.internal.save(path);
        
        #[cfg(not(feature = "frame_capture"))]
        return Ok(());
    }
}
```

### 7.4 Validation Layer

```rust
// Resource state validation
#[cfg(feature = "validation")]
impl CommandBuffer {
    fn validate_draw(&self) -> Result<(), GpuError> {
        if self.current_pipeline.is_none() {
            return Err(GpuError::MissingPipeline);
        }
        
        let pipeline = self.registry.get_pipeline(self.current_pipeline.unwrap())?;
        
        // Validate bound resources match pipeline expectations
        for binding in &pipeline.reflection.bindings {
            if !self.bound_resources.contains(binding.slot) {
                return Err(GpuError::MissingBinding {
                    name: binding.name.to_string(),
                    slot: binding.slot,
                });
            }
        }
        
        Ok(())
    }
}
```

## 8. Memory Management

Memory management strategies for efficient resource allocation.

### 8.1 Memory Allocation

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

### 8.2 Resource Pooling

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

### 8.3 Deferred Destruction

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

## 9. Concurrency Model

Approach to multi-threaded command generation and execution.

### 9.1 Thread-Safe Resource Creation

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

### 9.2 Parallel Command Recording

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

### 9.3 Work Stealing

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

## 10. Feature Flags and Compilation

Control over which components are compiled into the binary.

### 10.1 Cargo Features

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

### 10.2 Conditional Compilation

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

### 10.3 Binary Size Optimizations

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

## 11. Summary

The Vectron GPU architecture provides:

1. **Tiered API**: Low-level, standard, and high-level abstractions
2. **Modular Backend**: Split by functionality with extension traits
3. **Offline Work**: Build-time shader compilation and reflection
4. **Resource Management**: Efficient tracking and validation
5. **Pipeline Optimization**: Caching and async creation
6. **Debugging**: Optional instrumentation and validation
7. **Concurrency**: Thread-safe design for multi-core utilization
8. **Small Binary**: Feature flags for minimal size

This architecture balances developer experience, performance, and binary size with compile-time feature selection, allowing applications to include only what they need while maintaining a clean, consistent API.