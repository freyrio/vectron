# wgpu v 25.0.0 API Documentation

wgpu is a cross-platform graphics and compute library based on [WebGPU](https://gpuweb.github.io/gpuweb/). This document provides a comprehensive reference of its API.

## Core Concepts

- **Instance**: Entry point to wgpu
- **Adapter**: Handle to a physical graphics/compute device 
- **Device**: Logical device for creating resources
- **Queue**: Submits command buffers for execution
- **Surface**: Target for rendering (typically a window)
- **Buffer**: Memory for storing data
- **Texture**: Storage for image data
- **Shader**: Program that runs on the GPU

## Types and Descriptors

wgpu uses descriptor structs to define how resources should be created. This section covers key type definitions and descriptors.

### Instance Types

```rust
// Describes WGSL language extensions
pub struct WgslLanguageFeatures: u32 {
    const ReadOnlyAndReadWriteStorageTextures = 1 << 0;
    const Packed4x8IntegerDotProduct = 1 << 1;
    const UnrestrictedPointerParameters = 1 << 2;
    const PointerCompositeAccess = 1 << 3;
}

// Describes backend options for initializing wgpu
pub struct InstanceDescriptor {
    pub backends: Backends,  // Which graphics backends to use
    pub flags: InstanceFlags,  // Additional options
    pub dx12_shader_compiler: Dx12Compiler,  // DX12-specific compiler option
    pub gles_minor_version: Option<Gles3MinorVersion>,  // For OpenGL ES 3.0+
    pub backend_options: BackendOptions,  // Backend-specific options
}

// Available backend flags
pub struct Backends: u32 {
    const VULKAN = 1 << 0;
    const GL = 1 << 1;
    const METAL = 1 << 2;
    const DX12 = 1 << 3;
    const DX11 = 1 << 4;
    const BROWSER_WEBGPU = 1 << 5;
    const PRIMARY = Self::VULKAN.bits() | Self::METAL.bits() | Self::DX12.bits() | Self::BROWSER_WEBGPU.bits();
    const SECONDARY = Self::GL.bits() | Self::DX11.bits();
    const ALL = Self::PRIMARY.bits() | Self::SECONDARY.bits();
}
```

### Adapter Types

```rust
// Options for requesting an adapter
pub struct RequestAdapterOptions<'a, 'b> {
    pub power_preference: PowerPreference,
    pub compatible_surface: Option<&'a Surface<'b>>,
    pub force_fallback_adapter: bool, 
}

// Power preference for adapter selection
pub enum PowerPreference {
    LowPower,
    HighPerformance,
}

// Information about an adapter
pub struct AdapterInfo {
    pub name: String,
    pub vendor: usize,
    pub device: usize,
    pub device_type: DeviceType,
    pub driver: String,
    pub driver_info: String,
    pub backend: Backend,
}

// The type of device
pub enum DeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
}
```

### Device Types

```rust
// Describes a device to be created
pub struct DeviceDescriptor<'a> {
    pub label: Label<'a>,
    pub required_features: Features,
    pub required_limits: Limits,
    pub device_read_memory_hints: MemoryHints,
    pub device_write_memory_hints: MemoryHints,
}

// Features that can be enabled on a device
pub struct Features: u64 {
    // Core features supported in WebGPU
    const DEPTH_CLIP_CONTROL = WGPUFeatures::DEPTH_CLIP_CONTROL;
    const DEPTH32FLOAT_STENCIL8 = WGPUFeatures::DEPTH32FLOAT_STENCIL8;
    const TEXTURE_COMPRESSION_BC = WGPUFeatures::TEXTURE_COMPRESSION_BC;
    const TEXTURE_COMPRESSION_ETC2 = WGPUFeatures::TEXTURE_COMPRESSION_ETC2;
    const TEXTURE_COMPRESSION_ASTC = WGPUFeatures::TEXTURE_COMPRESSION_ASTC;
    const TIMESTAMP_QUERY = WGPUFeatures::TIMESTAMP_QUERY;
    const INDIRECT_FIRST_INSTANCE = WGPUFeatures::INDIRECT_FIRST_INSTANCE;
    // ... many more features
}

// Limits that constrain resource creation and pipeline execution
pub struct Limits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_texture_array_layers: u32,
    pub max_bind_groups: u32,
    pub max_bind_groups_plus_vertex_buffers: u32,
    pub max_bindings_per_bind_group: u32,
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    pub max_sampled_textures_per_shader_stage: u32,
    pub max_samplers_per_shader_stage: u32,
    pub max_storage_buffers_per_shader_stage: u32,
    pub max_storage_textures_per_shader_stage: u32,
    pub max_uniform_buffers_per_shader_stage: u32,
    pub max_uniform_buffer_binding_size: u32,
    pub max_storage_buffer_binding_size: u32,
    pub max_vertex_buffers: u32,
    pub max_vertex_attributes: u32,
    pub max_vertex_buffer_array_stride: u32,
    // ... many more limits
}
```

### Buffer Types

```rust
// Descriptor for creating a buffer
pub struct BufferDescriptor<'a> {
    pub label: Label<'a>,
    pub size: BufferAddress,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

// Possible uses for a buffer
pub struct BufferUsages: u32 {
    const MAP_READ = 0x0001;       // Buffer can be mapped for reading
    const MAP_WRITE = 0x0002;      // Buffer can be mapped for writing
    const COPY_SRC = 0x0004;       // Buffer can be used as copy source
    const COPY_DST = 0x0008;       // Buffer can be used as copy destination
    const INDEX = 0x0010;          // Buffer can be used as index buffer
    const VERTEX = 0x0020;         // Buffer can be used as vertex buffer
    const UNIFORM = 0x0040;        // Buffer can be used as uniform buffer
    const STORAGE = 0x0080;        // Buffer can be used as storage buffer
    const INDIRECT = 0x0100;       // Buffer can be used for indirect commands
    const QUERY_RESOLVE = 0x0200;  // Buffer can be used for query resolves
}

// Mode for mapping buffers
pub enum MapMode {
    Read,  // Map for reading
    Write, // Map for writing
}
```

### Texture Types

```rust
// Descriptor for creating a texture
pub struct TextureDescriptor<'a> {
    pub label: Label<'a>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: &'a [TextureFormat],
}

// Size of a texture
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

// Texture dimension
pub enum TextureDimension {
    D1,      // 1D texture
    D2,      // 2D texture
    D3,      // 3D texture
}

// Possible uses for a texture
pub struct TextureUsages: u32 {
    const COPY_SRC = 0x01;           // Texture can be used as copy source
    const COPY_DST = 0x02;           // Texture can be used as copy destination
    const TEXTURE_BINDING = 0x04;    // Texture can be bound for sampling
    const STORAGE_BINDING = 0x08;    // Texture can be bound for storage access
    const RENDER_ATTACHMENT = 0x10;  // Texture can be used as render attachment
}

// Descriptor for creating a texture view
pub struct TextureViewDescriptor<'a> {
    pub label: Label<'a>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}

// Dimension for a texture view
pub enum TextureViewDimension {
    D1,          // 1D view
    D2,          // 2D view
    D2Array,     // 2D array view
    Cube,        // Cube view
    CubeArray,   // Cube array view
    D3,          // 3D view
}

// Aspect of a texture to view
pub enum TextureAspect {
    All,         // All aspects
    StencilOnly, // Stencil only
    DepthOnly,   // Depth only
    Plane0,      // Plane 0 for planar formats
    Plane1,      // Plane 1 for planar formats
    Plane2,      // Plane 2 for planar formats
}
```

### Shader Types

```rust
// Descriptor for creating a shader module
pub struct ShaderModuleDescriptor<'a> {
    pub label: Label<'a>,
    pub source: ShaderSource<'a>,
}

// Source for a shader module
pub enum ShaderSource<'a> {
    Wgsl(&'a str),                           // WGSL source code
    #[cfg(any(wgpu_core, naga))]
    Naga(naga::Module),                      // Naga module
    #[cfg(feature = "spirv")]
    SpirV(Cow<'a, [u32]>),                   // SPIR-V binary
    #[cfg(feature = "glsl")]
    Glsl {                                   // GLSL source code
        source: Cow<'a, str>,               
        stage: naga::ShaderStage,
        defines: naga::FastHashMap<String, String>,
    },
}

// Shader stages
pub struct ShaderStages: u32 {
    const NONE = 0;
    const VERTEX = 1 << 0;      // Vertex shader stage
    const FRAGMENT = 1 << 1;    // Fragment shader stage
    const COMPUTE = 1 << 2;     // Compute shader stage
    const VERTEX_FRAGMENT = Self::VERTEX.bits() | Self::FRAGMENT.bits();
    const TASK = 1 << 3;        // Task shader stage
    const MESH = 1 << 4;        // Mesh shader stage
}
```

### Binding Types

```rust
// Descriptor for creating a bind group layout
pub struct BindGroupLayoutDescriptor<'a> {
    pub label: Label<'a>,
    pub entries: &'a [BindGroupLayoutEntry],
}

// Entry in a bind group layout
pub struct BindGroupLayoutEntry {
    pub binding: u32,              // Binding index
    pub visibility: ShaderStages,  // Shader stages that can access this binding
    pub ty: BindingType,           // Type of binding
    pub count: Option<NonZeroU32>, // Array count for arrays of bindings
}

// Type of binding
pub enum BindingType {
    Buffer {
        ty: BufferBindingType,
        has_dynamic_offset: bool, 
        min_binding_size: Option<BufferSize>,
    },
    Sampler(SamplerBindingType),
    Texture {
        sample_type: TextureSampleType,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: TextureFormat,
        view_dimension: TextureViewDimension,
    },
}

// Type of buffer binding
pub enum BufferBindingType {
    Uniform,                       // Uniform buffer
    Storage { read_only: bool },   // Storage buffer
}

// Type of sampler binding
pub enum SamplerBindingType {
    Filtering,                     // Filtering sampler 
    NonFiltering,                  // Non-filtering sampler
    Comparison,                    // Comparison sampler
}

// Access mode for storage textures
pub enum StorageTextureAccess {
    WriteOnly,                     // Write-only access
    ReadOnly,                      // Read-only access (requires feature)
    ReadWrite,                     // Read-write access (requires feature)
}
```

### Pipeline Types

```rust
// Descriptor for creating a render pipeline
pub struct RenderPipelineDescriptor<'a> {
    pub label: Label<'a>,
    pub layout: Option<&'a PipelineLayout>,
    pub vertex: VertexState<'a>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState<'a>>,
    pub multiview: Option<NonZeroU32>,
    pub cache: Option<&'a PipelineCache>,
}

// Vertex stage of a render pipeline
pub struct VertexState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
    pub buffers: &'a [VertexBufferLayout<'a>],
}

// Fragment stage of a render pipeline
pub struct FragmentState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
    pub targets: &'a [Option<ColorTargetState>],
}

// Layout of vertex buffers and attributes
pub struct VertexBufferLayout<'a> {
    pub array_stride: BufferAddress,
    pub step_mode: VertexStepMode,
    pub attributes: &'a [VertexAttribute],
}

// Step mode for vertex buffers
pub enum VertexStepMode {
    Vertex,                        // Advance per vertex
    Instance,                      // Advance per instance
    VertexBufferNotUsed,           // Buffer not used in this pipeline
}

// Vertex attribute
pub struct VertexAttribute {
    pub format: VertexFormat,      // Format of the attribute
    pub offset: BufferAddress,     // Offset in bytes from start of vertex
    pub shader_location: u32,      // Location in the shader
}

// Primitive assembly state
pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: Option<Face>,
    pub unclipped_depth: bool,
    pub polygon_mode: PolygonMode,
    pub conservative: bool,
}

// Primitive topology
pub enum PrimitiveTopology {
    PointList,                     // Points
    LineList,                      // Lines (separate)
    LineStrip,                     // Lines (connected)
    TriangleList,                  // Triangles (separate)
    TriangleStrip,                 // Triangles (connected)
}

// Descriptor for creating a compute pipeline
pub struct ComputePipelineDescriptor<'a> {
    pub label: Label<'a>,
    pub layout: Option<&'a PipelineLayout>,
    pub module: &'a ShaderModule,
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
    pub cache: Option<&'a PipelineCache>,
}
```

## Instance

The starting point for wgpu. Creates adapters and surfaces.

```rust
pub struct Instance {
    inner: dispatch::DispatchInstance,
}

impl Instance {
    // Create a new instance with the specified backends
    pub fn new(instance_desc: &InstanceDescriptor) -> Self
    
    // Returns which backends can be picked for the current build configuration
    pub const fn enabled_backend_features() -> Backends
    
    // Retrieves all available Adapters that match the given Backends
    pub fn enumerate_adapters(&self, backends: Backends) -> Vec<Adapter>
    
    // Retrieves an Adapter which matches the given RequestAdapterOptions
    pub fn request_adapter(
        &self,
        options: &RequestAdapterOptions<'_, '_>,
    ) -> impl Future<Output = Result<Adapter, RequestAdapterError>> + WasmNotSend
    
    // Creates a surface from a window
    pub fn create_surface<'window>(
        &self,
        target: impl Into<SurfaceTarget<'window>>,
    ) -> Result<Surface<'window>, CreateSurfaceError>
    
    // Poll all devices
    pub fn poll_all(&self, force_wait: bool) -> bool
    
    // Returns the supported WGSL language features
    pub fn wgsl_language_features(&self) -> WgslLanguageFeatures
}
```

## Adapter

Handle to a physical graphics and/or compute device.

```rust
pub struct Adapter {
    inner: dispatch::DispatchAdapter,
}

impl Adapter {
    // Request a connection to a physical device, creating a logical device
    pub fn request_device(
        &self,
        desc: &DeviceDescriptor<'_>,
    ) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> + WasmNotSend
    
    // Check if adapter can present to a surface
    pub fn is_surface_supported(&self, surface: &Surface<'_>) -> bool
    
    // The features supported by this adapter
    pub fn features(&self) -> Features
    
    // The limits supported by this adapter
    pub fn limits(&self) -> Limits
    
    // Get info about the adapter itself
    pub fn get_info(&self) -> AdapterInfo
    
    // Get downlevel capabilities
    pub fn get_downlevel_capabilities(&self) -> DownlevelCapabilities
    
    // Returns the features supported for a given texture format
    pub fn get_texture_format_features(&self, format: TextureFormat) -> TextureFormatFeatures
    
    // Generate a timestamp using the presentation engine clock
    pub fn get_presentation_timestamp(&self) -> PresentationTimestamp
}
```

## Surface

Handle to a presentable surface (e.g., a window).

```rust
pub struct Surface<'window> {
    inner: dispatch::DispatchSurface,
    config: Mutex<Option<SurfaceConfiguration>>,
    _handle_source: Option<Box<dyn WindowHandle + 'window>>,
}

impl Surface<'_> {
    // Returns the capabilities of the surface when used with the given adapter
    pub fn get_capabilities(&self, adapter: &Adapter) -> SurfaceCapabilities
    
    // Return a default SurfaceConfiguration for this surface with the adapter
    pub fn get_default_config(
        &self,
        adapter: &Adapter,
        width: u32,
        height: u32,
    ) -> Option<SurfaceConfiguration>
    
    // Configure the surface for presentation
    pub fn configure(&self, device: &Device, config: &SurfaceConfiguration)
    
    // Returns the next texture to be presented by the swapchain
    pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError>
}
```

## SurfaceTarget

Describes the target for a surface.

```rust
pub enum SurfaceTarget<'window> {
    // Window handle
    Window(Box<dyn WindowHandle + 'window>),
    
    // Canvas element (web only)
    #[cfg(any(webgpu, webgl))]
    Canvas(web_sys::HtmlCanvasElement),
    
    // OffscreenCanvas (web only)
    #[cfg(any(webgpu, webgl))]
    OffscreenCanvas(web_sys::OffscreenCanvas),
}
```

## Device

Open connection to a graphics and/or compute device.

```rust
pub struct Device {
    inner: dispatch::DispatchDevice,
}

impl Device {
    // Check for resource cleanups and mapping callbacks
    pub fn poll(&self, poll_type: PollType) -> Result<PollStatus, PollError>
    
    // The features which can be used on this device
    pub fn features(&self) -> Features
    
    // The limits which can be used on this device
    pub fn limits(&self) -> Limits
    
    // Creates a shader module
    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor<'_>) -> ShaderModule
    
    // Creates an empty CommandEncoder
    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor<'_>) -> CommandEncoder
    
    // Creates a RenderBundleEncoder
    pub fn create_render_bundle_encoder<'a>(
        &self,
        desc: &RenderBundleEncoderDescriptor<'_>,
    ) -> RenderBundleEncoder<'a>
    
    // Creates a BindGroup
    pub fn create_bind_group(&self, desc: &BindGroupDescriptor<'_>) -> BindGroup
    
    // Creates a BindGroupLayout
    pub fn create_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor<'_>,
    ) -> BindGroupLayout
    
    // Creates a PipelineLayout
    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor<'_>) -> PipelineLayout
    
    // Creates a RenderPipeline
    pub fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor<'_>) -> RenderPipeline
    
    // Creates a ComputePipeline
    pub fn create_compute_pipeline(&self, desc: &ComputePipelineDescriptor<'_>) -> ComputePipeline
    
    // Creates a Buffer
    pub fn create_buffer(&self, desc: &BufferDescriptor<'_>) -> Buffer
    
    // Creates a Texture
    pub fn create_texture(&self, desc: &TextureDescriptor<'_>) -> Texture
    
    // Creates a Sampler
    pub fn create_sampler(&self, desc: &SamplerDescriptor<'_>) -> Sampler
    
    // Creates a QuerySet
    pub fn create_query_set(&self, desc: &QuerySetDescriptor<'_>) -> QuerySet
    
    // Set error handler for uncaptured errors
    pub fn on_uncaptured_error(&self, handler: Box<dyn UncapturedErrorHandler>)
    
    // Push an error scope
    pub fn push_error_scope(&self, filter: ErrorFilter)
    
    // Pop an error scope
    pub fn pop_error_scope(&self) -> impl Future<Output = Option<Error>> + WasmNotSend
    
    // Creates a PipelineCache
    pub unsafe fn create_pipeline_cache(
        &self,
        desc: &PipelineCacheDescriptor<'_>,
    ) -> PipelineCache
    
    // Create a Bottom-Level Acceleration Structure
    pub fn create_blas(
        &self,
        desc: &CreateBlasDescriptor<'_>,
        sizes: BlasGeometrySizeDescriptors,
    ) -> Blas
    
    // Create a Top-Level Acceleration Structure
    pub fn create_tlas(&self, desc: &CreateTlasDescriptor<'_>) -> Tlas
    
    // Destroy the device
    pub fn destroy(&self)
}
```

## Queue

Handle to a command queue on a device.

```rust
pub struct Queue {
    inner: dispatch::DispatchQueue,
}

impl Queue {
    // Submit command buffers for execution
    pub fn submit<I: IntoIterator<Item = CommandBuffer>>(&self, command_buffers: I)
    
    // Writes data to a buffer
    pub fn write_buffer(
        &self,
        buffer: &Buffer,
        offset: BufferAddress,
        data: &[u8],
    )
    
    // Writes data to a texture
    pub fn write_texture(
        &self,
        texture: TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: TextureDataLayout,
        size: Extent3d,
    )
    
    // Submit commands and generate a timestamp when they finish
    pub fn submit_timestamp_only(&self) -> impl Future<Output = Option<PresentationTimestamp>> + WasmNotSend
    
    // Submit and wait for completion
    pub fn submit_and_wait<I: IntoIterator<Item = CommandBuffer>>(
        &self,
        command_buffers: I,
    ) -> Result<(), SubmitError>
}
```

## Buffer

Handle to a GPU-accessible buffer.

```rust
pub struct Buffer {
    inner: dispatch::DispatchBuffer,
    map_context: Arc<Mutex<MapContext>>,
    size: BufferAddress,
    usage: BufferUsages,
}

impl Buffer {
    // Return the binding view of the entire buffer
    pub fn as_entire_binding(&self) -> BindingResource<'_>
    
    // Return the binding view of the entire buffer
    pub fn as_entire_buffer_binding(&self) -> BufferBinding<'_>
    
    // Get a slice of the buffer
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_>
    
    // Unmap the buffer
    pub fn unmap(&self)
    
    // Destroy the buffer
    pub fn destroy(&self)
    
    // Get the size of the buffer
    pub fn size(&self) -> BufferAddress
    
    // Get the usage of the buffer
    pub fn usage(&self) -> BufferUsages
    
    // Map a slice of the buffer asynchronously
    pub fn map_async<S: RangeBounds<BufferAddress>>(
        &self,
        mode: MapMode,
        bounds: S,
        callback: impl FnOnce(Result<(), BufferAsyncError>) + WasmNotSend + 'static,
    )
    
    // Get a read-only view of a mapped buffer slice
    pub fn get_mapped_range<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferView<'_>
    
    // Get a mutable view of a mapped buffer slice
    pub fn get_mapped_range_mut<S: RangeBounds<BufferAddress>>(
        &self,
        bounds: S,
    ) -> BufferViewMut<'_>
}
```

## BufferSlice

A slice of a buffer.

```rust
pub struct BufferSlice<'a> {
    buffer: &'a Buffer,
    offset: BufferAddress,
    size: BufferSize,
}

impl<'a> BufferSlice<'a> {
    // Create a subslice
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'a>
    
    // Map a slice of the buffer asynchronously
    pub fn map_async(
        &self,
        mode: MapMode,
        callback: impl FnOnce(Result<(), BufferAsyncError>) + WasmNotSend + 'static,
    )
    
    // Get a read-only view of a mapped buffer slice
    pub fn get_mapped_range(&self) -> BufferView<'a>
    
    // Get a mutable view of a mapped buffer slice
    pub fn get_mapped_range_mut(&self) -> BufferViewMut<'a>
    
    // Get the parent buffer
    pub fn buffer(&self) -> &'a Buffer
    
    // Get the offset from the start of the buffer
    pub fn offset(&self) -> BufferAddress
    
    // Get the size of the slice
    pub fn size(&self) -> BufferSize
}
```

## Texture

Handle to a texture on the GPU.

```rust
pub struct Texture {
    inner: dispatch::DispatchTexture,
    descriptor: TextureDescriptor<'static>,
}

impl Texture {
    // Creates a view of this texture
    pub fn create_view(&self, desc: &TextureViewDescriptor<'_>) -> TextureView
    
    // Destroy the texture
    pub fn destroy(&self)
    
    // Make an TexelCopyTextureInfo representing the whole texture
    pub fn as_image_copy(&self) -> TexelCopyTextureInfo<'_>
    
    // Get the size of the texture
    pub fn size(&self) -> Extent3d
    
    // Get the width of the texture
    pub fn width(&self) -> u32
    
    // Get the height of the texture
    pub fn height(&self) -> u32
    
    // Get the depth or array layers of the texture
    pub fn depth_or_array_layers(&self) -> u32
    
    // Get the mip level count of the texture
    pub fn mip_level_count(&self) -> u32
    
    // Get the sample count of the texture
    pub fn sample_count(&self) -> u32
    
    // Get the dimension of the texture
    pub fn dimension(&self) -> TextureDimension
    
    // Get the format of the texture
    pub fn format(&self) -> TextureFormat
    
    // Get the allowed usages of the texture
    pub fn usage(&self) -> TextureUsages
}
```

## TextureView

A view into a texture.

```rust
pub struct TextureView {
    inner: dispatch::DispatchTextureView,
}
```

## ShaderModule

A shader module.

```rust
pub struct ShaderModule {
    inner: dispatch::DispatchShaderModule,
}
```

## RenderPipeline

Handle to a rendering (graphics) pipeline.

```rust
pub struct RenderPipeline {
    inner: dispatch::DispatchRenderPipeline,
}

impl RenderPipeline {
    // Get the bind group layout at the given index
    pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout
}
```

## ComputePipeline

Handle to a compute pipeline.

```rust
pub struct ComputePipeline {
    inner: dispatch::DispatchComputePipeline,
}

impl ComputePipeline {
    // Get the bind group layout at the given index
    pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout
}
```

## CommandEncoder

Encodes commands for submission to a queue.

```rust
pub struct CommandEncoder {
    inner: dispatch::DispatchCommandEncoder,
}

impl CommandEncoder {
    // Begins recording of a render pass
    pub fn begin_render_pass<'a>(&'a mut self, desc: &RenderPassDescriptor<'a, '_>) -> RenderPass<'a>
    
    // Begins recording of a compute pass
    pub fn begin_compute_pass<'a>(&'a mut self, desc: &ComputePassDescriptor<'a>) -> ComputePass<'a>
    
    // Copy data from one buffer to another
    pub fn copy_buffer_to_buffer(
        &mut self,
        source: &Buffer,
        source_offset: BufferAddress,
        destination: &Buffer,
        destination_offset: BufferAddress,
        copy_size: BufferAddress,
    )
    
    // Copy data from a buffer to a texture
    pub fn copy_buffer_to_texture(
        &mut self,
        source: BufferCopyView<'_>,
        destination: TexelCopyTextureInfo<'_>,
        copy_size: Extent3d,
    )
    
    // Copy data from a texture to a buffer
    pub fn copy_texture_to_buffer(
        &mut self,
        source: TexelCopyTextureInfo<'_>,
        destination: BufferCopyView<'_>,
        copy_size: Extent3d,
    )
    
    // Copy data from one texture to another
    pub fn copy_texture_to_texture(
        &mut self,
        source: TexelCopyTextureInfo<'_>,
        destination: TexelCopyTextureInfo<'_>,
        copy_size: Extent3d,
    )
    
    // Finish recording and produce a command buffer
    pub fn finish(self) -> CommandBuffer
}
```

## RenderPass

Encodes render pass commands.

```rust
pub struct RenderPass<'a> {
    parent: &'a mut CommandEncoder,
    inner: dispatch::DispatchRenderPass,
}

impl<'a> RenderPass<'a> {
    // Set the active render pipeline
    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline)
    
    // Set the active bind group
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &'a BindGroup,
        offsets: &[DynamicOffset],
    )
    
    // Set a vertex buffer
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &'a Buffer, offset: BufferAddress)
    
    // Set the index buffer
    pub fn set_index_buffer(&mut self, buffer: &'a Buffer, index_format: IndexFormat, offset: BufferAddress)
    
    // Draw vertices
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>)
    
    // Draw vertices using indices
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>)
}
```

## ComputePass

Encodes compute pass commands.

```rust
pub struct ComputePass<'a> {
    parent: &'a mut CommandEncoder,
    inner: dispatch::DispatchComputePass,
}

impl<'a> ComputePass<'a> {
    // Set the active compute pipeline
    pub fn set_pipeline(&mut self, pipeline: &'a ComputePipeline)
    
    // Set the active bind group
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &'a BindGroup,
        offsets: &[DynamicOffset],
    )
    
    // Dispatch compute work
    pub fn dispatch(&mut self, x: u32, y: u32, z: u32)
    
    // Dispatch compute work using an indirect buffer
    pub fn dispatch_indirect(
        &mut self,
        indirect_buffer: &'a Buffer,
        indirect_offset: BufferAddress,
    )
}
```

## BindGroup

Collection of resources bound together and referenced by pipelines.

```rust
pub struct BindGroup {
    inner: dispatch::DispatchBindGroup,
}
```

## BindGroupLayout

Defines the interface between a set of resources bound in a bind group and their accessibility in shader stages.

```rust
pub struct BindGroupLayout {
    inner: dispatch::DispatchBindGroupLayout,
}
```

## PipelineLayout

Defines the layout of a pipeline.

```rust
pub struct PipelineLayout {
    inner: dispatch::DispatchPipelineLayout,
}
```

## Sampler

Handle to a texture sampler.

```rust
pub struct Sampler {
    inner: dispatch::DispatchSampler,
}
```

## QuerySet

Handle to a query set.

```rust
pub struct QuerySet {
    inner: dispatch::DispatchQuerySet,
}
```

## Common Enums and Types

### BufferUsages

Describes how a buffer can be used.

```rust
pub enum BufferUsages {
    MAP_READ,
    MAP_WRITE,
    COPY_SRC,
    COPY_DST,
    INDEX,
    VERTEX,
    UNIFORM,
    STORAGE,
    INDIRECT,
    QUERY_RESOLVE,
}
```

### TextureUsages

Describes how a texture can be used.

```rust
pub enum TextureUsages {
    COPY_SRC,
    COPY_DST,
    TEXTURE_BINDING,
    STORAGE_BINDING,
    RENDER_ATTACHMENT,
}
```

### TextureFormat

Describes the texel format of a texture.

```rust
pub enum TextureFormat {
    // Various formats including:
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    // ... many more formats
}
```

### Features

Features supported by an adapter or enabled on a device.

```rust
pub enum Features {
    // Various features including:
    DEPTH_CLIP_CONTROL,
    TIMESTAMP_QUERY,
    PIPELINE_STATISTICS_QUERY,
    // ... many more features
}
```

### Limits

Limits supported by an adapter or enforced by a device.

```rust
pub struct Limits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    // ... many more limits
}
```

## Error Types

- `RequestAdapterError`: Error when requesting an adapter
- `RequestDeviceError`: Error when requesting a device
- `CreateSurfaceError`: Error when creating a surface
- `SurfaceError`: Error when acquiring the next swapchain texture
- `BufferAsyncError`: Error when mapping a buffer asynchronously
- `Error`: General wgpu error

## Memory and Resource Management

- `Label`: Debug label for objects
- `MapMode`: Mode for mapping buffers (Read/Write)
- `PollType`: Controls buffer mapping and device polling behavior

## Surface and Window Integration

The Surface API in wgpu provides functionality for creating, configuring, and presenting to surfaces, which are usually windows or canvases where rendered content is displayed.

### Surface Types

```rust
// Describes a Surface configuration
pub struct SurfaceConfiguration {
    pub usage: TextureUsages,          // How the surface textures will be used
    pub format: TextureFormat,         // Format of the surface textures
    pub width: u32,                    // Width of the surface
    pub height: u32,                   // Height of the surface
    pub present_mode: PresentMode,     // How to synchronize with the display
    pub alpha_mode: CompositeAlphaMode, // How alpha values are handled
    pub view_formats: Vec<TextureFormat>, // Additional formats for texture views
    pub desired_maximum_frame_latency: u32, // Number of frames that can be in flight
}

// Presentation modes for surface
pub enum PresentMode {
    Immediate,         // No VSync - display frames as soon as they're ready
    Fifo,              // Traditional VSync - wait for the next VBL 
    FifoRelaxed,       // Relaxed VSync - use VSync if fps > refresh rate, otherwise immediate
    Mailbox,           // Replace-previous mode - replace pending frames with newer ones
}

// Alpha compositing mode for the surface
pub enum CompositeAlphaMode {
    Auto,              // Let WGPU choose the appropriate mode
    Opaque,            // Surface has no transparency
    PreMultiplied,     // Surface uses pre-multiplied alpha
    PostMultiplied,    // Surface uses post-multiplied alpha
    Inherit,           // Use the alpha mode of the window/compositor
}

// Surface capabilities returned by adapter
pub struct SurfaceCapabilities {
    pub formats: Vec<TextureFormat>,  // Supported texture formats
    pub present_modes: Vec<PresentMode>, // Supported presentation modes
    pub alpha_modes: Vec<CompositeAlphaMode>, // Supported alpha modes
    pub usages: TextureUsages,        // Supported texture usages
}

// Possible errors when acquiring the next surface texture
pub enum SurfaceError {
    Timeout,           // Timeout while waiting for next frame
    Outdated,          // Surface needs to be reconfigured
    Lost,              // Surface was lost and is no longer valid
    OutOfMemory,       // System ran out of memory
    Other,             // Other error occurred
}
```

### Surface Texture Management

```rust
// Surface texture returned by get_current_texture
pub struct SurfaceTexture {
    pub texture: Texture,           // The texture to render to
    pub suboptimal: bool,           // Whether the configuration is suboptimal
}

impl SurfaceTexture {
    // Present the texture to the surface
    pub fn present(self)
}
```

### Window System Integration

wgpu provides several ways to create surfaces from window handles across different platforms:

```rust
// Safe targets for creating surfaces
pub enum SurfaceTarget<'window> {
    // Window handle from raw-window-handle crate
    Window(Box<dyn WindowHandle + 'window>),
    
    // Canvas element (web only)
    #[cfg(any(webgpu, webgl))]
    Canvas(web_sys::HtmlCanvasElement),
    
    // OffscreenCanvas (web only)
    #[cfg(any(webgpu, webgl))]
    OffscreenCanvas(web_sys::OffscreenCanvas),
}

// Unsafe targets for advanced use cases
pub enum SurfaceTargetUnsafe {
    // Raw window and display handles
    RawHandle {
        raw_display_handle: raw_window_handle::RawDisplayHandle,
        raw_window_handle: raw_window_handle::RawWindowHandle,
    },
    
    // DRM-based surface (Linux only)
    #[cfg(all(unix, not(target_vendor = "apple"), not(target_family = "wasm")))]
    Drm {
        fd: i32,              // File descriptor of DRM device
        plane: u32,           // DRM plane index
        connector_id: u32,    // DRM connector ID
        width: u32,           // Display width
        height: u32,          // Display height
        refresh_rate: u32,    // Display refresh rate * 1000
    },
    
    // Metal-specific surfaces
    #[cfg(metal)]
    CoreAnimationLayer(*mut core::ffi::c_void),
    
    // DirectX-specific surfaces
    #[cfg(dx12)]
    CompositionVisual(*mut core::ffi::c_void),
    
    #[cfg(dx12)]
    SurfaceHandle(*mut core::ffi::c_void),
    
    #[cfg(dx12)]
    SwapChainPanel(*mut core::ffi::c_void),
}

impl SurfaceTargetUnsafe {
    // Safely creates a raw handle target from a window
    pub unsafe fn from_window<T>(window: &T) 
    -> Result<Self, raw_window_handle::HandleError>
    where T: HasDisplayHandle + HasWindowHandle
}

// Error when creating a surface
pub struct CreateSurfaceError {
    // Different underlying error types depending on platform
}
```

### Surface API Usage

The following is an overview of how surfaces are created, configured, and used:

```rust
// Creating a surface
let surface = instance.create_surface(window)?;

// Getting surface capabilities
let capabilities = surface.get_capabilities(&adapter);

// Getting a default configuration
let config = surface.get_default_config(&adapter, width, height)?;

// Configuring a surface
surface.configure(&device, &config);

// Getting the next surface texture
let surface_texture = surface.get_current_texture()?;

// Rendering to the surface texture
let view = surface_texture.texture.create_view(&Default::default());
// ... render to view ...

// Presenting the surface texture
surface_texture.present();
```

The lifecycle typically involves:

1. Creating a surface from a window or canvas
2. Querying adapter capabilities for the surface
3. Configuring the surface with desired format, size, and presentation mode
4. In the render loop:
   - Getting the next texture to render to
   - Creating a view of the texture
   - Recording and submitting render commands
   - Presenting the texture

The Surface API is designed to work across platforms with the same API, while abstracting away the platform-specific details of window system integration.
