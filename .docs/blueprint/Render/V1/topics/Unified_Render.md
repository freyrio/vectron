# Unified Render Engine Architecture

This document outlines a modern, flexible rendering engine architecture combining the best aspects of both Graphite and Vectron Render architectures.

## 1. Core Principles & Goals

**Goals:**
- Provide an intuitive, composable, and efficient rendering API in idiomatic Rust
- Support both immediate and retained mode rendering
- Enable high-quality 2D vector graphics, text, and 3D rendering
- Achieve optimal performance via batching, tessellation, and GPU acceleration
- Maintain platform independence through backend abstraction
- Enable flexible styling, effects, and transformations
- Support responsive layouts through a flexible unit system
- Minimize binary size through optional feature flags

**Design Philosophy:**
- **Modularity**: Clear separation of concerns with well-defined interfaces
- **Composition**: Combine simple primitives to create complex visuals
- **Performance**: Optimize common paths while supporting advanced features
- **Flexibility**: Allow for various rendering approaches and techniques
- **Rust Idioms**: Leverage ownership, traits, enums, and type safety

## 2. Project Structure

```
render_engine/
├── Cargo.toml          # Feature flags: d2, d3, text, advanced_effects, etc.
├── src/
│   ├── lib.rs          # Public API exports and module definitions
│   │
│   ├── core/           # Core concepts and traits
│   │   ├── mod.rs      # Core module exports
│   │   ├── error.rs    # Error types and handling
│   │   ├── math.rs     # Vector/matrix math utilities
│   │   ├── types.rs    # Fundamental type definitions
│   │   ├── units.rs    # Unit system (px, pct, em, vw, vh)
│   │   ├── color.rs    # Color types and operations
│   │   ├── style.rs    # Style traits and implementations
│   │   ├── effect.rs   # Visual effects (shadows, blur, etc.)
│   │   ├── transform.rs # Transform utilities and operations 
│   │   ├── renderable.rs # Renderable trait definition
│   │   ├── tessellation.rs # Tessellation traits and utilities
│   │   └── operation.rs # Operation definition (element + style + transform)
│   │
│   ├── backend/        # Abstract backend interface
│   │   ├── mod.rs      # Backend module exports
│   │   ├── device.rs   # BackendDevice trait and abstractions
│   │   ├── resource.rs # Resource handles and descriptors
│   │   ├── state.rs    # Pipeline state definitions
│   │   └── commands.rs # Command buffer abstraction
│   │
│   ├── elements/       # Renderable elements
│   │   ├── mod.rs      # Element module exports
│   │   ├── d2/         # 2D elements
│   │   │   ├── mod.rs  
│   │   │   ├── path/   # Path element
│   │   │   │   ├── mod.rs # Path definition
│   │   │   │   └── tessellation.rs # Path tessellation
│   │   │   ├── triangle/  # Triangle elements
│   │   │   │   ├── mod.rs # Triangle definition  
│   │   │   │   └── tessellation.rs # Triangle tessellation
│   │   │   ├── rect/   # Rectangle elements
│   │   │   │   ├── mod.rs # Rectangle definition  
│   │   │   │   └── tessellation.rs # Rectangle tessellation
│   │   │   ├── circle/ # Circle element
│   │   │   │   ├── mod.rs # Circle definition
│   │   │   │   └── tessellation.rs # Circle tessellation
│   │   │   ├── text/   # Text elements
│   │   │   │   ├── mod.rs # Text definitions
│   │   │   │   ├── font.rs # Font loading/management
│   │   │   │   ├── layout.rs # Text layout algorithms
│   │   │   │   └── cache.rs # Glyph caching
│   │   │   └── camera.rs # 2D camera
│   │   │
│   │   └── d3/         # 3D elements
│   │       ├── mod.rs
│   │       ├── mesh/   # Mesh element
│   │       │   ├── mod.rs # Mesh definition
│   │       │   └── tessellation.rs # Mesh tessellation
│   │       ├── primitives/ # 3D primitive shapes
│   │       │   ├── mod.rs # Primitive exports
│   │       │   ├── box.rs # 3D box 
│   │       │   ├── sphere.rs # Sphere primitive
│   │       │   └── plane.rs # Plane primitive
│   │       ├── material.rs # Material definitions
│   │       ├── light.rs # Light types
│   │       ├── camera.rs # 3D camera definitions
│   │       └── scene.rs # 3D scene representation
│   │
│   ├── pipeline/       # Rendering pipelines
│   │   ├── mod.rs      # Pipeline exports
│   │   ├── cache.rs    # Pipeline caching
│   │   ├── vector.rs   # 2D vector pipeline
│   │   ├── text.rs     # Text rendering pipeline
│   │   ├── mesh.rs     # 3D mesh pipeline
│   │   └── effects.rs  # Post-processing effects
│   │
│   ├── renderer/       # Main rendering orchestration
│   │   ├── mod.rs      # Renderer exports
│   │   ├── context.rs  # User-facing rendering context
│   │   ├── batcher.rs  # Operation batching
│   │   ├── translator.rs # Command translation
│   │   └── resource_cache.rs # Resource management
│   │
│   └── utils/          # Common utilities
│       ├── mod.rs
│       ├── math_helpers.rs # Common math helpers
│       ├── debug.rs    # Debug helpers and overlays
│       └── profiling.rs # Performance profiling
```

## 3. API Layer Architecture

While the core components provide a flexible foundation, direct use of the `RenderContext` and `RenderOperation` for high-frequency drawing (like in immediate mode UIs) can lead to performance bottlenecks due to repeated allocations and potential data cloning. To address this and provide more ergonomic interfaces for specific use cases, a dedicated **API Layer** sits on top of the core engine.

```
+-------------------------------------------------+
|               Public Facing APIs                |
| (SVG API, D2 API, D3 API, Graphic API, etc.)    |
+-------------------------------------------------+
    |                 |                 |
    | (Immediate Mode)| (Retained Mode) |  <-- Mode handled within each API
    |  - Arena Alloc  |  - Handles      |
    |  - Ref Passing  |  - Data Stores  |
    |                 |                 |
+-------------------------------------------------+
|            Core Render Engine                   |
| (RenderContext, Renderer, RenderOperation,     |
|  ResourceCache, Tessellation, Elements, Backend)|
+-------------------------------------------------+
```

**Key Aspects:**

-   **Specialized APIs:** Different crates or modules can provide APIs tailored to specific needs (e.g., `d2_api`, `d3_api`, `svg_api`).
-   **Mode Handling:** Each API can implement strategies suitable for immediate or retained mode rendering paradigms.
    -   **Immediate Mode:** Utilizes techniques like per-frame **arena allocation** (e.g., using `bumpalo`) to allocate temporary `Renderable` objects and styles, passing references down to the core `Renderer`. This significantly reduces heap allocation overhead per draw call.
    -   **Retained Mode:** Manages application-level objects (Scene graphs, UI trees, SVG DOMs) internally. These objects store **handles** (e.g., `GeometryHandle`, `StyleHandle`) to resources managed by the core engine's `ResourceCache` (or the API layer's own stores). Rendering involves traversing the structure and submitting commands to the core `Renderer` using these lightweight handles, avoiding per-frame data cloning.
-   **Performance Optimization:** This layer is the primary place for implementing paradigm-specific optimizations, translating the user-friendly API calls into efficient sequences of core `Renderer` operations.
-   **Ergonomics:** Provides users with more idiomatic and convenient interfaces compared to directly manipulating `RenderOperation` objects for common tasks.

The `RenderContext` described later serves as the foundational context, often wrapped or utilized internally by these higher-level APIs.

## 4. Key Components & Concepts

### Core Concepts

#### Renderable Trait
The foundation for any drawable element:

```rust
pub trait Renderable {
    /// Get the local origin point
    fn origin(&self) -> Point;
    
    /// Set the local origin point
    fn set_origin(&mut self, origin: Point);
    
    /// Get local bounds without any transforms
    fn local_bounds(&self) -> BoundingVolume;
    
    /// Get bounds with current transform applied
    fn world_bounds(&self, transform: &Transform) -> BoundingVolume;
    
    /// Perform a transform relative to the origin
    fn transform_local(&mut self, transform: &Transform);
    
    /// Check if this renderable is a 2D element
    fn is_2d(&self) -> bool;
    
    /// Check if this renderable is a 3D element
    fn is_3d(&self) -> bool;
    
    /// Create a clone of this renderable
    fn clone_renderable(&self) -> Box<dyn Renderable>;
}

pub trait Renderable2D: Renderable {
    /// Get the 2D bounding rectangle in local space
    fn local_rect(&self) -> Rect;
    
    /// Get the 2D bounding rectangle in world space
    fn world_rect(&self, transform: &Transform) -> Rect;
}

pub trait Renderable3D: Renderable {
    /// Get the 3D bounding box in local space
    fn local_box(&self) -> BoundingBox;
    
    /// Get the 3D bounding box in world space
    fn world_box(&self, transform: &Transform) -> BoundingBox;
}
```

#### Unit System
A flexible system for defining positions and sizes:

```rust
pub enum UnitKind {
    Pixel,       // Absolute pixels
    Point,       // Points (1/72 inch)
    Percent,     // Percent of parent
    Em,          // Relative to font size
    ViewportWidth, // % of viewport width
    ViewportHeight, // % of viewport height
    // ... other units
}

pub struct Unit<T = f32> {
    kind: UnitKind,
    value: T,
}

pub struct UnitContext {
    dpi: f32,
    viewport_size: Size,
    parent_size: Size,
    font_size: f32,
}

impl<T: Float> Unit<T> {
    // Convert unit to pixel value using context
    pub fn resolve(&self, context: &UnitContext) -> T {
        match self.kind {
            UnitKind::Pixel => self.value,
            UnitKind::Point => self.value * context.dpi / 72.0,
            UnitKind::Percent => self.value * context.parent_size.width.min(context.parent_size.height) / 100.0,
            UnitKind::Em => self.value * context.font_size,
            UnitKind::ViewportWidth => self.value * context.viewport_size.width / 100.0,
            UnitKind::ViewportHeight => self.value * context.viewport_size.height / 100.0,
            // ... other conversions
        }
    }
    
    // Convenience constructors
    pub fn px(value: T) -> Self { Self { kind: UnitKind::Pixel, value } }
    pub fn pt(value: T) -> Self { Self { kind: UnitKind::Point, value } }
    pub fn pct(value: T) -> Self { Self { kind: UnitKind::Percent, value } }
    pub fn em(value: T) -> Self { Self { kind: UnitKind::Em, value } }
    pub fn vw(value: T) -> Self { Self { kind: UnitKind::ViewportWidth, value } }
    pub fn vh(value: T) -> Self { Self { kind: UnitKind::ViewportHeight, value } }
}
```

#### Tessellation System
For converting high-level shapes into GPU-ready geometry:

```rust
// Core tessellation trait
pub trait Tessellable: Renderable {
    /// Convert the element to GPU-ready geometry
    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> TessellationResult;
}

// Options controlling the tessellation process
pub struct TessellationOptions {
    pub quality: TessellationQuality, // Controls detail level
    pub tolerance: f32,               // Max error tolerance
    pub generate_normals: bool,       // Whether to generate normals
    pub generate_uvs: bool,           // Whether to generate UV coordinates
    pub cull_mode: CullMode,          // Face culling mode
}

// Result of tessellation - can be either complete or incremental
pub enum TessellationResult {
    Complete(TessellationData),
    Incremental(VertexStream), // For large meshes processed incrementally
}

pub struct TessellationData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub bounds: BoundingVolume,
    pub stats: TessellationStats,
}
```

#### Operation System
Combining elements with styles and transforms:

```rust
// The core rendering operation
pub struct RenderOperation {
    pub element: Box<dyn Renderable>,
    pub transform: Transform,
    pub styles: Vec<Box<dyn Style>>,
    pub effects: Vec<Box<dyn Effect>>,
    pub clip: Option<ClipRegion>,
}

impl RenderOperation {
    /// Create a new operation
    pub fn new<R: Renderable + 'static>(element: R) -> Self { ... }
    
    /// Add a style to the operation
    pub fn with_style<S: Style + 'static>(mut self, style: S) -> Self { ... }
    
    /// Add an effect to the operation
    pub fn with_effect<E: Effect + 'static>(mut self, effect: E) -> Self { ... }
    
    /// Set the transform for the operation
    pub fn with_transform(mut self, transform: Transform) -> Self { ... }
    
    /// Execute the operation using the provided renderer
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Implementation...
    }
}
```

#### Style System
Defining the visual appearance of elements:

```rust
pub trait Style: Any + Send + Sync {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
    fn as_any(&self) -> &dyn Any;
    fn is_batchable(&self) -> bool { false }
}

// Fill style for shapes
pub struct Fill {
    pub paint: Paint,
    pub rule: FillRule,
}

// Stroke style for outlines
pub struct Stroke {
    pub paint: Paint,
    pub width: Unit<f32>,
    pub line_join: LineJoin,
    pub line_cap: LineCap,
    pub dash_pattern: Option<DashPattern>,
}

// Different paint types
pub enum Paint {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Texture(TextureHandle),
    Pattern(PatternHandle),
}
```

#### Effect System
Adding visual effects to elements:

```rust
pub trait Effect: Any + Send + Sync {
    fn apply_pre(&self, renderer: &mut dyn Renderer, element: &dyn Renderable) -> Result<(), RenderError> { Ok(()) }
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> { Ok(()) }
    fn as_any(&self) -> &dyn Any;
}

// Shadow effect
pub struct Shadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
}

// Blur effect
pub struct Blur {
    pub radius: f32,
    pub direction: BlurDirection,
}
```

### Elements Implementation

#### 2D Elements
Fundamental 2D shapes with tessellation:

```rust
// Rectangle element
pub struct Rectangle {
    pub origin: Point,
    pub width: Unit<f32>,
    pub height: Unit<f32>,
    pub corner_radius: [Unit<f32>; 4], // Optional rounded corners
}

impl Rectangle {
    pub fn new(x: impl Into<Unit<f32>>, y: impl Into<Unit<f32>>, 
               width: impl Into<Unit<f32>>, height: impl Into<Unit<f32>>) -> Self {
        // Implementation...
    }
    
    pub fn with_rounded_corners(mut self, radius: impl Into<Unit<f32>>) -> Self {
        // Implementation...
    }
}

impl Renderable for Rectangle {
    // Implementations...
}

impl Renderable2D for Rectangle {
    // Implementations...
}

impl Tessellable for Rectangle {
    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> TessellationResult {
        // For simple rects without rounded corners, generate a simple quad
        if self.has_zero_corner_radius() {
            return self.tessellate_simple_quad(context);
        }
        
        // For rounded rects, generate proper geometry
        self.tessellate_rounded_rect(options, context)
    }
}
```

#### Text Support
Advanced text handling:

```rust
pub struct TextRun {
    pub text: String,
    pub font: FontHandle,
    pub font_size: Unit<f32>,
    pub color: Color,
    pub origin: Point,
    pub max_width: Option<Unit<f32>>,
    pub alignment: TextAlignment,
}

impl Renderable for TextRun {
    // Implementations...
}

impl Renderable2D for TextRun {
    // Implementations...
}

// Text doesn't implement Tessellable directly
// Instead, the TextPipeline handles the specialized rendering
```

#### 3D Elements
Support for 3D rendering:

```rust
pub struct Mesh {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub normals: Option<Vec<Vec3>>,
    pub uvs: Option<Vec<Vec2>>,
    pub material: Option<MaterialHandle>,
    pub origin: Point3D,
}

impl Renderable for Mesh {
    // Implementations...
}

impl Renderable3D for Mesh {
    // Implementations...
}

impl Tessellable for Mesh {
    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> TessellationResult {
        // For meshes, tessellation is mostly about validating and
        // potentially generating missing attributes (normals, tangents)
        self.prepare_mesh_data(options)
    }
}
```

### Rendering Pipeline

#### Resource Management
Efficient GPU resource handling:

```rust
pub struct ResourceCache {
    geometries: HashMap<GeometryHandle, GeometryDesc>,
    textures: HashMap<TextureHandle, TextureDesc>,
    buffers: HashMap<BufferHandle, BufferDesc>,
    
    // Maps from our handles to backend GPU resources
    gpu_geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>, // Vertex, Index
    gpu_textures: HashMap<TextureHandle, gpu::TextureHandle>,
    
    pending_uploads: Vec<PendingUpload>,
}

impl ResourceCache {
    pub fn register_geometry(&mut self, desc: GeometryDesc) -> GeometryHandle {
        // Assign a new handle, store the description
        // Mark as needing upload to GPU
    }
    
    pub fn upload_pending_resources(&mut self, device: &mut dyn BackendDevice) {
        // Process all pending uploads to the GPU
    }
    
    pub fn get_geometry_buffers(&self, handle: GeometryHandle) -> Option<(BufferHandle, BufferHandle)> {
        // Get GPU vertex and index buffer handles
    }
}
```

#### Pipeline Management
Specialized rendering pipelines:

```rust
pub struct PipelineCache {
    pipelines: HashMap<PipelineKey, PipelineHandle>,
}

impl PipelineCache {
    pub fn get_or_create_pipeline(&mut self, key: PipelineKey, device: &mut dyn BackendDevice) -> PipelineHandle {
        // Get existing pipeline or create a new one
    }
}

pub struct VectorPipeline {
    fill_pipeline: PipelineHandle,
    stroke_pipeline: PipelineHandle,
    
    pub fn render_fill(&self, 
                       cmd: &mut dyn CommandBuffer, 
                       geometry: GeometryHandle,
                       color: Color,
                       transform: &Transform) {
        // Set pipeline, uniforms, bind geometry, draw
    }
    
    pub fn render_stroke(&self, 
                        cmd: &mut dyn CommandBuffer,
                        geometry: GeometryHandle,
                        color: Color,
                        width: f32,
                        transform: &Transform) {
        // Similar to fill but with stroke settings
    }
}
```

#### Renderer & Command Translation
Converts high-level operations to GPU commands:

```rust
pub trait Renderer {
    // State management
    fn push_state(&mut self);
    fn pop_state(&mut self) -> Result<(), RenderError>;
    fn set_transform(&mut self, transform: &Transform);
    fn set_clip(&mut self, clip: &ClipRegion);
    
    // Geometry handling
    fn tessellate(&mut self, element: &dyn Tessellable, options: &TessellationOptions) -> Result<GeometryData, RenderError>;
    
    // Drawing operations
    fn fill_geometry(&mut self, geometry: &GeometryData, paint: &Paint, rule: FillRule) -> Result<(), RenderError>;
    fn stroke_geometry(&mut self, geometry: &GeometryData, paint: &Paint, width: f32, join: LineJoin, cap: LineCap) -> Result<(), RenderError>;
    fn draw_text(&mut self, text: &TextRun) -> Result<(), RenderError>;
    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material) -> Result<(), RenderError>;
    
    // Effect operations
    fn apply_blur(&mut self, geometry: &GeometryData, radius: f32) -> Result<(), RenderError>;
    // ... other methods
}

pub struct CommandTranslator {
    resource_cache: ResourceCache,
    pipeline_cache: PipelineCache,
    vector_pipeline: VectorPipeline,
    text_pipeline: TextPipeline,
    mesh_pipeline: MeshPipeline,
    effects_pipeline: EffectsPipeline,
    
    current_state: RenderState,
    state_stack: Vec<RenderState>,
    
    backend_device: Box<dyn BackendDevice>,
    command_buffer: Option<Box<dyn CommandBuffer>>,
}

impl Renderer for CommandTranslator {
    // Implementations of the Renderer trait
    // These methods translate high-level operations to low-level GPU commands
}
```

#### User-Facing Context
The main API for users, often wrapped by higher-level specialized APIs:

```rust
pub struct RenderContext {
    translator: CommandTranslator,
    unit_context: UnitContext,
}

impl RenderContext {
    // Create a new rendering context
    pub fn new(device: Box<dyn BackendDevice>, viewport_size: (u32, u32)) -> Self {
        // Implementation...
    }
    
    // Begin a new frame
    pub fn begin_frame(&mut self) -> Result<(), RenderError> {
        // Implementation...
    }
    
    // End the current frame and present
    pub fn end_frame(&mut self) -> Result<(), RenderError> {
        // Implementation...
    }
    
    // Mid-level drawing methods (can be used directly or by API layers)
    pub fn fill_rect(&mut self, rect: &Rectangle, fill: Fill) -> Result<(), RenderError> {
        let op = RenderOperation::new(rect.clone())
            .with_style(fill);
        // Directly execute using the internal translator
        op.execute(&mut self.translator)
    }
    
    pub fn stroke_rect(&mut self, rect: &Rectangle, stroke: Stroke) -> Result<(), RenderError> {
        let op = RenderOperation::new(rect.clone())
            .with_style(stroke);
        // Directly execute using the internal translator
        op.execute(&mut self.translator)
    }
    
    pub fn draw_text(&mut self, text: &TextRun) -> Result<(), RenderError> {
        let op = RenderOperation::new(text.clone());
        // Directly execute using the internal translator
        op.execute(&mut self.translator)
    }
    
    // Execute pre-constructed operations (used by API layers or advanced users)
    pub fn execute_operation(&mut self, operation: &RenderOperation) -> Result<(), RenderError> {
        operation.execute(&mut self.translator)
    }
    
    // Update the viewport size
    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.unit_context.viewport_size = Size::new(width as f32, height as f32);
    }
}
```

## 5. Rendering Flow

The rendering process, potentially orchestrated by a higher-level API layer, generally follows these steps:

1.  **API Interaction**:
    -   User interacts with a specific API (e.g., `D2Api`, `D3Api`).
    -   For immediate mode, the API might manage an arena and prepare for drawing calls.
    -   For retained mode, the user builds or modifies a scene/document managed by the API.

2.  **Context Setup**:
    -   Internally, the API layer (or the user directly) ensures a `RenderContext` exists for the backend device.
    -   `begin_frame()` is called on the `RenderContext` to start a new frame.

3.  **Element/Command Generation**:
    -   **Immediate Mode:** User calls drawing functions (e.g., `d2_ctx.fill_rect(...)`). The API layer allocates temporary data (potentially in an arena) and prepares calls to the core `Renderer`.
    -   **Retained Mode:** The API layer traverses its internal structure (scene graph, etc.), determines visible/dirty elements, and generates `RenderOperation`s or prepares direct calls to the core `Renderer` using handles.
    -   **Direct Mode:** User manually creates renderable elements and `RenderOperation` objects.

4.  **Operation Execution**:
    -   The API layer calls methods on the core `Renderer` (often via `RenderContext::execute_operation` or similar internal pathways).
    -   Direct users call `execute()` on their `RenderOperation`s or use `RenderContext` convenience methods.

5.  **Tessellation**:
    -   The `Renderer` implementation (e.g., `CommandTranslator`) triggers tessellation via the `Tessellable` trait for elements requiring geometry generation.

6.  **Resource Management**:
   - Tessellated geometry is registered with the `ResourceCache`
   - Resources are uploaded to the GPU as needed
   - Handles are returned for GPU resources

7. **Style/Effect Application**:
   - Styles and effects apply their visual properties
   - They call methods on the `Renderer` trait to perform drawing

8. **Command Translation**:
   - The `CommandTranslator` translates high-level operations to GPU commands
   - It selects appropriate pipelines, binds resources, and configures GPU state
   - It records draw commands into the command buffer

9. **Frame Completion**:
   - User calls `end_frame()` to finalize the frame
   - The command buffer is submitted to the GPU
   - The frame is presented to the screen

## 6. Optimization Features

### Batching

Operations are batched by the core engine (potentially guided by the API layer) to minimize GPU state changes:

```rust
pub struct Batcher {
    batches: HashMap<BatchKey, Vec<BatchableOperation>>,
}

#[derive(Hash, Eq, PartialEq)]
struct BatchKey {
    pipeline_type: PipelineType,
    texture_id: Option<TextureHandle>,
    blend_mode: BlendMode,
    // Other key properties for batching
}

impl Batcher {
    pub fn add(&mut self, operation: BatchableOperation) {
        let key = operation.batch_key();
        self.batches.entry(key).or_default().push(operation);
    }
    
    pub fn execute_batches(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Execute batches in optimal order
        // Sort by pipeline, then texture, etc.
    }
}
```

### Caching

Various caches improve performance:

-   **Resource Cache:** Manages GPU buffers, textures, etc. (Core Engine)
-   **Tessellation Cache:** Avoids re-tessellating unchanged geometry (Core Engine or API Layer)
-   **Pipeline Cache:** Reuses pipeline state objects (Core Engine)
-   **Glyph Cache:** For efficient text rendering (Core Engine / Text Module)
-   **API Layer Caches:** Retained-mode APIs often implement their own caching for scene/document data.

### Arena Allocation (Immediate Mode)

API layers designed for immediate mode rendering typically use **arena allocation** (e.g., `bumpalo`) for per-frame data. This allocates `Renderable` elements, styles, and potentially temporary `RenderOperation` variants within a fast, linear allocator that is reset each frame, drastically reducing the overhead of frequent heap allocations and deallocations common in immediate-mode GUI loops.

### Handle-Based System (Retained Mode)

API layers for retained mode avoid cloning large element data each frame by using **handles**. Application objects (like scene nodes) store lightweight handles (`GeometryHandle`, `StyleHandle`, `TextureHandle`) referring to the actual data managed centrally (e.g., in the `ResourceCache` or API-specific stores). Rendering involves passing these handles to the core engine, which then accesses the necessary GPU resources or data. This is highly efficient for complex, relatively static scenes.

## 7. Backend Abstraction

The rendering engine uses a clean abstraction for the GPU backend:

```rust
pub trait BackendDevice: Send + Sync {
    // Device operations
    fn name(&self) -> &str;
    fn features(&self) -> &DeviceFeatures;
    
    // Frame management
    fn begin_frame(&mut self) -> Result<Box<dyn CommandBuffer>, BackendError>;
    fn end_frame(&mut self, command_buffer: Box<dyn CommandBuffer>) -> Result<(), BackendError>;
    
    // Resource creation
    fn create_buffer(&mut self, desc: &BufferDesc) -> Result<BufferHandle, BackendError>;
    fn create_texture(&mut self, desc: &TextureDesc) -> Result<TextureHandle, BackendError>;
    fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<PipelineHandle, BackendError>;
    
    // Resource updates
    fn update_buffer(&mut self, handle: BufferHandle, data: &[u8], offset: u64) -> Result<(), BackendError>;
    fn update_texture(&mut self, handle: TextureHandle, data: &[u8], offset: [u32; 3], size: [u32; 3]) -> Result<(), BackendError>;
    
    // Resource destruction
    fn destroy_buffer(&mut self, handle: BufferHandle);
    fn destroy_texture(&mut self, handle: TextureHandle);
    fn destroy_pipeline(&mut self, handle: PipelineHandle);
}

pub trait CommandBuffer: Send + Sync {
    // Pipeline and binding operations
    fn set_pipeline(&mut self, pipeline: PipelineHandle) -> Result<(), BackendError>;
    fn set_vertex_buffer(&mut self, slot: u32, buffer: BufferHandle, offset: u64) -> Result<(), BackendError>;
    fn set_index_buffer(&mut self, buffer: BufferHandle, offset: u64, format: IndexFormat) -> Result<(), BackendError>;
    fn set_bind_group(&mut self, index: u32, bind_group: BindGroupHandle) -> Result<(), BackendError>;
    
    // Draw operations
    fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> Result<(), BackendError>;
    fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32) -> Result<(), BackendError>;
    
    // Compute operations
    fn dispatch(&mut self, x: u32, y: u32, z: u32) -> Result<(), BackendError>;
    
    // Render pass operations
    fn begin_render_pass(&mut self, desc: &RenderPassDesc) -> Result<(), BackendError>;
    fn end_render_pass(&mut self) -> Result<(), BackendError>;
    
    // Debug markers
    fn push_debug_group(&mut self, label: &str) -> Result<(), BackendError>;
    fn pop_debug_group(&mut self) -> Result<(), BackendError>;
}
```

## 8. Feature Flags

Feature flags allow users to include only needed functionality:

```toml
[features]
default = ["d2", "text"]

# Core features
d2 = []             # 2D rendering support
d3 = []             # 3D rendering support
text = ["d2"]       # Text rendering (requires d2)

# Advanced features
advanced_effects = [] # Advanced visual effects
svg = ["d2"]        # SVG loading/rendering
animation = []      # Animation support

# Backend implementations
wgpu_backend = ["wgpu"]  # WGPU backend
vulkan_backend = ["ash"] # Direct Vulkan backend
```

## 9. Conclusion

This unified rendering engine architecture combines the strengths of both Graphite and Vectron Render approaches, enhanced by a flexible API layer:

-   **Layered Design**: A core engine provides fundamental capabilities, while specialized API layers offer ergonomic interfaces and paradigm-specific optimizations (immediate/retained modes).
-   **Modular Core**: Clean separation of concerns (elements, styling, tessellation, backend).
-   **Efficient Tessellation**: Integrated with elements for optimal implementation.
-   **Flexible Unit System**: Supports responsive designs with various unit types
-   **Strong Composition**: Operations combine elements, styles, and effects
-   **Resource Management**: Efficient GPU resource handling with caching
-   **Backend Abstraction**: Clean interface for multiple backend implementations
-   **Performance Focus**: Batching, culling, caching, arena allocation (via API layer), and handle-based systems (via API layer) provide multiple optimization avenues.
-   **Feature Control**: Optional features through Cargo feature flags.

The architecture balances flexibility, performance, and usability, creating a modern rendering engine suitable for a wide range of applications, from efficient immediate-mode UIs and high-performance 2D vector graphics to complex 3D visualizations.