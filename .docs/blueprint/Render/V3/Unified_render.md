# Unified Render Engine Architecture

This document outlines a modern, flexible rendering engine architecture.

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
render_core/
├── Cargo.toml          # Feature flags and dependencies
├── src/
│   ├── lib.rs          # Public API exports and module definitions
│   │
│   ├── core/           # Core concepts and traits
│   │   ├── mod.rs      # Core module exports
│   │   ├── error.rs    # Error types and handling
│   │   │
│   │   ├── geometry/   # Core geometric types
│   │   │   ├── mod.rs
│   │   │   ├── point.rs      # Point<D> implementation
│   │   │   ├── bounds.rs     # Bounding volume implementations
│   │   │   └── transform.rs  # High-level transform interface
│   │   │
│   │   ├── traits/    # Core traits
│   │   │   ├── mod.rs
│   │   │   ├── renderer.rs      # Renderer trait definition
│   │   │   ├── renderable.rs    # Renderable trait definition
│   │   │   ├── tessellable.rs   # Tessellable trait definition
│   │   │   ├── rasterizeable.rs # Rasterizeable trait definition
│   │   │   ├── effect.rs        # Effect trait definition
│   │   │   └── style.rs         # Style trait definition
│   │   │
│   │   ├── operation/ # Operation system
│   │   │   ├── mod.rs
│   │   │   ├── render.rs     # RenderOperation definition
│   │   │   ├── batch.rs      # BatchableOperation and batching logic
│   │   │   └── builder.rs    # Operation builder pattern
│   │   │
│   │   └── types/     # Common type definitions
│   │       ├── mod.rs
│   │       └── common.rs      # Shared type definitions
│   │
│   ├── d2/            # 2D-specific implementations
│   │   ├── mod.rs
│   │   ├── elements/  # 2D elements
│   │   │   ├── mod.rs
│   │   │   ├── rect.rs       # Rectangle implementation
│   │   │   ├── circle.rs     # Circle implementation
│   │   │   ├── path.rs       # Path implementation
│   │   │   └── text.rs       # Text element implementation
│   │   │
│   │   ├── pipeline/  # 2D pipeline
│   │   │   ├── mod.rs
│   │   │   ├── fill.rs       # Fill pipeline
│   │   │   ├── stroke.rs     # Stroke pipeline
│   │   │   └── text.rs       # Text rendering pipeline
│   │   │
│   │   └── tessellation/     # 2D tessellation
│   │       ├── mod.rs
│   │       ├── path.rs       # Path tessellation
│   │       └── text.rs       # Text tessellation
│   │
│   ├── d3/            # 3D-specific implementations
│   │   ├── mod.rs
│   │   ├── elements/  # 3D elements
│   │   │   ├── mod.rs
│   │   │   ├── mesh.rs       # Mesh implementation
│   │   │   └── camera.rs     # Camera implementation
│   │   │
│   │   ├── pipeline/  # 3D pipeline
│   │   │   ├── mod.rs
│   │   │   ├── mesh.rs       # Mesh rendering pipeline
│   │   │   └── material.rs   # Material handling
│   │   │
│   │   └── tessellation/     # 3D tessellation
│   │       ├── mod.rs
│   │       └── mesh.rs       # Mesh tessellation
│   │
│   ├── resource/      # Resource management
│   │   ├── mod.rs
│   │   ├── cache/     # Resource caching
│   │   │   ├── mod.rs
│   │   │   ├── geometry.rs   # Geometry cache
│   │   │   ├── texture.rs    # Texture cache
│   │   │   └── pipeline.rs   # Pipeline cache
│   │   │
│   │   ├── buffer/    # Buffer management
│   │   │   ├── mod.rs
│   │   │   ├── vertex.rs     # Vertex buffer
│   │   │   └── index.rs      # Index buffer
│   │   │
│   │   └── upload/    # Resource upload management
│   │       ├── mod.rs
│   │       └── queue.rs      # Upload queue
│   │
│   ├── style/         # Style system
│   │   ├── mod.rs
│   │   ├── fill.rs    # Fill style implementation
│   │   ├── stroke.rs  # Stroke style implementation
│   │   ├── paint.rs   # Paint implementations
│   │   └── material.rs # Material system (for 3D)
│   │
│   ├── effect/        # Effect system
│   │   ├── mod.rs
│   │   ├── shadow.rs  # Shadow effect
│   │   ├── blur.rs    # Blur effect
│   │   └── post.rs    # Post-processing effects
│   │
│   ├── color/         # Color system
│   │   ├── mod.rs
│   │   ├── types.rs   # Base color types
│   │   ├── spaces/    # Color spaces
│   │   │   ├── mod.rs
│   │   │   ├── rgb.rs
│   │   │   ├── cmyk.rs
│   │   │   ├── hsv.rs
│   │   │   ├── hsl.rs
│   │   │   ├── lab.rs
│   │   │   ├── xyz.rs
│   │   │   └── lch.rs
│   │   │
│   │   ├── gradient.rs # Gradient definitions
│   │   ├── blend.rs    # Blending modes
│   │   └── named.rs    # Named colors
│   │
│   ├── units/         # Unit system
│   │   ├── mod.rs
│   │   ├── types.rs   # Unit type definitions
│   │   ├── logical.rs # Logical units (em, rem, vh, vw, %)
│   │   ├── physical.rs # Physical units (px, pt, in, cm, mm)
│   │   └── context.rs # Unit resolution context
│   │
│   ├── backend/       # Backend abstraction
│   │   ├── mod.rs
│   │   ├── device.rs  # BackendDevice trait
│   │   ├── resource.rs # Resource handles
│   │   ├── state.rs   # Pipeline state
│   │   └── commands.rs # Command buffer
│   │
│   ├── renderer/      # Main rendering orchestration
│   │   ├── mod.rs
│   │   ├── context.rs # User-facing context
│   │   ├── batcher.rs # Operation batching
│   │   └── translator.rs # Command translation
│   │
│   ├── math/          # Mathematical operations and implementations
│   │   ├── mod.rs
│   │   │
│   │   ├── vector/    # Vector operations
│   │   │   ├── mod.rs
│   │   │   ├── vec2.rs       # 2D vector operations
│   │   │   ├── vec3.rs       # 3D vector operations
│   │   │   └── vec4.rs       # 4D vector operations
│   │   │
│   │   ├── matrix/    # Matrix operations
│   │   │   ├── mod.rs
│   │   │   ├── mat2.rs       # 2x2 matrix operations
│   │   │   ├── mat3.rs       # 3x3 matrix operations
│   │   │   └── mat4.rs       # 4x4 matrix operations
│   │   │
│   │   ├── quaternion/ # Quaternion operations
│   │   │   ├── mod.rs
│   │   │   ├── quat.rs       # Quaternion implementation
│   │   │   └── rotation.rs   # Rotation utilities
│   │   │
│   │   ├── transform/ # Transform implementations
│   │   │   ├── mod.rs
│   │   │   ├── transform2.rs # 2D transform implementation
│   │   │   └── transform3.rs # 3D transform implementation
│   │   │
│   │   ├── interpolation/ # Interpolation utilities
│   │   │   ├── mod.rs
│   │   │   ├── lerp.rs      # Linear interpolation
│   │   │   └── bezier.rs    # Bezier curve interpolation
│   │   │
│   │   └── utils/     # Mathematical utilities
│   │       ├── mod.rs
│   │       ├── constants.rs  # Mathematical constants
│   │       └── functions.rs  # Common math functions
│   │
│   └── utils/         # Common utilities
│       ├── mod.rs
│       ├── debug.rs   # Debug utilities
│       └── profiling.rs # Performance profiling
```

### Feature Flags

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

# Optional components
color_spaces = []   # Additional color space support
icc_profiles = []   # ICC profile support
debug = []          # Debug features and validation
```

### Key Organizational Principles

1. **Clear Separation of Concerns**:
   - Core geometric types and traits in `core/`
   - 2D and 3D implementations separated
   - Resource management centralized
   - Pipeline implementations organized by dimension

2. **Modular Design**:
   - Each component is self-contained
   - Clear dependencies between modules
   - Feature flags control optional components

3. **Performance Focus**:
   - Resource caching and management
   - Efficient buffer handling
   - Pipeline state management

4. **Extensibility**:
   - Backend abstraction for multiple graphics APIs
   - Plugin system for effects
   - Customizable pipeline stages

5. **Developer Experience**:
   - Clear file organization
   - Consistent naming conventions
   - Comprehensive documentation
   - Debug and profiling tools

6. **Interface vs Implementation Separation**:
   - Core module defines WHAT can be done (interfaces, traits, types)
   - Math module defines HOW it's done (concrete implementations)
   - This separation ensures:
     - Clear boundaries between interface and implementation
     - No redundancy in mathematical operations
     - Easy to swap or optimize implementations
     - Better maintainability and testing

## 3. Core Concepts

### Matrix Structure and Coordinate System

To ensure consistency across all rendering operations, the engine defines specific conventions for matrix representation and coordinate systems.

#### Matrix Representation

```rust
// Core geometric types use generics for efficient implementation
pub struct Point<const D: usize> {
    coordinates: [f32; D],
}

pub type Point2D = Point<2>;
pub type Point3D = Point<3>;

// Transform operations use generics for efficient matrix math
pub struct Transform<const D: usize> {
    // Stored as a flat array in column-major order
    // For D=2: [c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2]
    // For D=3: [c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, ...]
    matrix: [f32; (D + 1) * (D + 1)],
}

pub type Transform2D = Transform<2>;
pub type Transform3D = Transform<3>;

// Public API uses enums for runtime polymorphism
pub enum PointInSpace {
    D2(Point2D),
    D3(Point3D),
}

pub enum TransformInSpace {
    D2(Transform2D),
    D3(Transform3D),
}
```

**Key Conventions:**

1. **Storage Format:** All matrices are stored in **column-major** format for compatibility with common graphics APIs and SIMD optimization.

2. **Transformation Order:** Transformations are applied in right-to-left order when written in mathematical notation:
   ```
   result = matrix3 * matrix2 * matrix1 * vector
   ```
   This means matrix1 is applied first, then matrix2, then matrix3.

3. **Matrix Multiplication:** When multiplying matrices A and B:
   ```
   C = A * B
   ```
   This means "apply transform B first, then apply transform A".

4. **Affine Transforms:** 2D transforms use 3×3 matrices and 3D transforms use 4×4 matrices to represent affine transformations (translation, rotation, scale, shear).

#### Coordinate Systems

**3D Coordinate System:**
- **Right-handed** coordinate system
- X axis points right
- Y axis points up
- Z axis points out of the screen (toward the viewer)
- Rotation follows the right-hand rule

**2D Coordinate System:**
- X axis points right
- Y axis points down (screen space convention)
- Positive rotation is clockwise

**Coordinate Space Definitions:**
- **Local Space:** Object's own coordinate system
- **World Space:** Global coordinate system where objects are positioned
- **View Space:** Camera's coordinate system (camera at origin, looking down negative Z)
- **Clip Space:** Normalized coordinates after projection (-1 to 1 in each dimension)
- **Screen Space:** Pixel coordinates (origin at top-left, +X right, +Y down)

**Viewport Transformation:**
- Maps from normalized device coordinates (-1 to 1) to screen coordinates (pixels)
- Origin is top-left for 2D rendering
- Y-coordinate is flipped between clip space and screen space

#### Utility Functions

```rust
impl<const D: usize> Transform<D> {
    pub fn identity() -> Self {
        let mut matrix = [0.0; (D + 1) * (D + 1)];
        for i in 0..=D {
            matrix[i * (D + 1) + i] = 1.0;
        }
        Self { matrix }
    }
    
    pub fn translation(offset: Point<D>) -> Self {
        let mut matrix = Self::identity().matrix;
        for i in 0..D {
            matrix[i * (D + 1) + D] = offset.coordinates[i];
        }
        Self { matrix }
    }
    
    pub fn scaling(scale: Point<D>) -> Self {
        let mut matrix = [0.0; (D + 1) * (D + 1)];
        for i in 0..D {
            matrix[i * (D + 1) + i] = scale.coordinates[i];
        }
        matrix[D * (D + 1) + D] = 1.0;
        Self { matrix }
    }
}

// 3D-specific transformations
impl Transform3D {
    pub fn rotation_x(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        let mut matrix = Self::identity().matrix;
        matrix[5] = cos;
        matrix[6] = sin;
        matrix[9] = -sin;
        matrix[10] = cos;
        Self { matrix }
    }
    
    pub fn rotation_y(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        let mut matrix = Self::identity().matrix;
        matrix[0] = cos;
        matrix[2] = -sin;
        matrix[8] = sin;
        matrix[10] = cos;
        Self { matrix }
    }
    
    pub fn rotation_z(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        let mut matrix = Self::identity().matrix;
        matrix[0] = cos;
        matrix[1] = sin;
        matrix[4] = -sin;
        matrix[5] = cos;
        Self { matrix }
    }
    
    pub fn look_at(eye: Point3D, target: Point3D, up: Point3D) -> Self {
        // Implementation of look-at matrix
    }
    
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        // Implementation of perspective projection matrix
    }
    
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        // Implementation of orthographic projection matrix
    }
}

// 2D-specific transformations
impl Transform2D {
    pub fn rotation(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        let mut matrix = Self::identity().matrix;
        matrix[0] = cos;
        matrix[1] = sin;
        matrix[3] = -sin;
        matrix[4] = cos;
        Self { matrix }
    }
    
    pub fn skew(x: f32, y: f32) -> Self {
        let mut matrix = Self::identity().matrix;
        matrix[1] = y;
        matrix[3] = x;
        Self { matrix }
    }
}
```

This clear definition of matrix structure and coordinate systems ensures consistent calculations throughout the rendering pipeline, whether using the GPU tessellation path or CPU rasterization.

### Dimension-Aware Types

The engine uses a balanced approach for handling 2D and 3D operations, using generics for core geometric types and concrete types for higher-level components:

```rust
// Core geometric types use generics for efficient implementation
pub struct Point<const D: usize> {
    coordinates: [f32; D],
}

pub type Point2D = Point<2>;
pub type Point3D = Point<3>;

// Public API uses enums for runtime polymorphism
pub enum PointInSpace {
    D2(Point2D),
    D3(Point3D),
}

// Transform operations use generics for efficient matrix math
pub struct Transform<const D: usize> {
    matrix: [[f32; D + 1]; D + 1], // D+1 for homogeneous coordinates
}

pub type Transform2D = Transform<2>;
pub type Transform3D = Transform<3>;

pub enum TransformInSpace {
    D2(Transform2D),
    D3(Transform3D),
}

// Vertex data uses generics for type-safe geometry
pub struct Vertex<const D: usize> {
    position: Point<D>,
    normal: Option<Point<D>>,
    tex_coords: Option<[f32; 2]>,
}

pub type Vertex2D = Vertex<2>;
pub type Vertex3D = Vertex<3>;
```

### Renderable Trait System

The foundation for any drawable element:

```rust
pub trait Renderable {
    /// Get the dimensionality of this renderable
    fn dimensionality(&self) -> Dimensionality;
    
    /// Get the local origin point
    fn origin(&self) -> PointInSpace;
    
    /// Set the local origin point
    fn set_origin(&mut self, origin: PointInSpace);
    
    /// Get local bounds without any transforms
    fn local_bounds(&self) -> BoundingVolume;
    
    /// Get bounds with current transform applied
    fn world_bounds(&self, transform: &TransformInSpace) -> BoundingVolume;
    
    /// Perform a transform relative to the origin
    fn transform_local(&mut self, transform: &TransformInSpace);
    
    /// Create a clone of this renderable
    fn clone_renderable(&self) -> Box<dyn Renderable>;
}

pub trait Object2D: Renderable {
    /// Get the 2D bounding rectangle in local space
    fn local_rect(&self) -> Rect;
    
    /// Get the 2D bounding rectangle in world space
    fn world_rect(&self, transform: &Transform2D) -> Rect;
    
    /// Get the 2D origin point
    fn origin_2d(&self) -> Point2D {
        match self.origin() {
            PointInSpace::D2(p) => p,
            _ => panic!("Expected 2D point for 2D renderable"),
        }
    }
}

pub trait Object3D: Renderable {
    /// Get the 3D bounding box in local space
    fn local_box(&self) -> BoundingBox;
    
    /// Get the 3D bounding box in world space
    fn world_box(&self, transform: &Transform3D) -> BoundingBox;
    
    /// Get the 3D origin point
    fn origin_3d(&self) -> Point3D {
        match self.origin() {
            PointInSpace::D3(p) => p,
            _ => panic!("Expected 3D point for 3D renderable"),
        }
    }
}
```

### Bounding Volume System

```rust
// Concrete implementations for 2D and 3D
pub struct AABB2D {
    min: Point2D,
    max: Point2D,
}

pub struct AABB3D {
    min: Point3D,
    max: Point3D,
}

// Common interface for bounding volumes
pub trait BoundingVolume {
    fn contains_point(&self, point: &PointInSpace) -> bool;
    fn intersects(&self, other: &dyn BoundingVolume) -> bool;
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume>;
}

impl BoundingVolume for AABB2D {
    fn contains_point(&self, point: &PointInSpace) -> bool {
        match point {
            PointInSpace::D2(p) => {
                p.coordinates[0] >= self.min.coordinates[0] &&
                p.coordinates[0] <= self.max.coordinates[0] &&
                p.coordinates[1] >= self.min.coordinates[1] &&
                p.coordinates[1] <= self.max.coordinates[1]
            }
            _ => false,
        }
    }
    // ... other implementations
}

impl BoundingVolume for AABB3D {
    // ... similar implementation for 3D
}
```

### Tessellation System

```rust
pub trait Tessellable: Renderable {
    /// Convert the element to GPU-ready geometry
    fn tessellate(&self, options: &TessellationOptions) -> TessellationResult;
}

pub struct TessellationOptions {
    pub quality: TessellationQuality,
    pub tolerance: f32,
    pub generate_normals: bool,
    pub generate_uvs: bool,
    pub cull_mode: CullMode,
}

pub struct TessellationResult {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    bounds: Box<dyn BoundingVolume>,
    stats: TessellationStats,
}

// Vertex type uses generics for type-safe geometry
pub struct Vertex {
    position: PointInSpace,
    normal: Option<PointInSpace>,
    tex_coords: Option<[f32; 2]>,
}
```

### Pipeline System

```rust
// Concrete pipeline types for 2D and 3D
pub struct Pipeline2D {
    vertex_shader: ShaderHandle,
    fragment_shader: ShaderHandle,
    vertex_layout: VertexLayout,
}

pub struct Pipeline3D {
    vertex_shader: ShaderHandle,
    fragment_shader: ShaderHandle,
    vertex_layout: VertexLayout,
}

pub struct PipelineCache {
    pipelines_2d: HashMap<PipelineKey, Pipeline2D>,
    pipelines_3d: HashMap<PipelineKey, Pipeline3D>,
}

impl PipelineCache {
    pub fn get_or_create_pipeline_2d(&mut self, key: PipelineKey, device: &mut dyn BackendDevice) -> Pipeline2D {
        // Get existing pipeline or create a new one
    }
    
    pub fn get_or_create_pipeline_3d(&mut self, key: PipelineKey, device: &mut dyn BackendDevice) -> Pipeline3D {
        // Get existing pipeline or create a new one
    }
}
```

### Resource Management

```rust
pub struct GeometryBuffer {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

pub struct ResourceCache {
    geometries: HashMap<GeometryHandle, GeometryBuffer>,
    textures: HashMap<TextureHandle, TextureDesc>,
    buffers: HashMap<BufferHandle, BufferDesc>,
    
    // Maps from our handles to backend GPU resources
    gpu_geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>, // Vertex, Index
    gpu_textures: HashMap<TextureHandle, gpu::TextureHandle>,
    
    pending_uploads: Vec<PendingUpload>,
}

impl ResourceCache {
    pub fn register_geometry(&mut self, geometry: GeometryBuffer) -> GeometryHandle {
        // Assign a new handle, store the geometry
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

## 4. API Layer Architecture

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
| (RenderContext, Renderer, RenderOperation...)   |
+-------------------------------------------------+
    |                                 |
    v                                 v
+----------------+               +----------------+
| GPU Pipeline   |               | CPU Pipeline   |
| (Tessellation) |               | (Rasterization)|
+----------------+               +----------------+
```

**Key Aspects:**

-   **Specialized APIs:** Different crates or modules can provide APIs tailored to specific needs (e.g., `d2_api`, `d3_api`, `svg_api`).
-   **Mode Handling:** Each API can implement strategies suitable for immediate or retained mode rendering paradigms.
    -   **Immediate Mode:** Utilizes techniques like per-frame **arena allocation** (e.g., using `bumpalo`) to allocate temporary `Renderable` objects and styles, passing references down to the core `Renderer`. This significantly reduces heap allocation overhead per draw call.
    -   **Retained Mode:** Manages application-level objects (Scene graphs, UI trees, SVG DOMs) internally. These objects store **handles** (e.g., `GeometryHandle`, `StyleHandle`) to resources managed by the core engine's `ResourceCache` (or the API layer's own stores). Rendering involves traversing the structure and submitting commands to the core `Renderer` using these lightweight handles, avoiding per-frame data cloning.
-   **Performance Optimization:** This layer is the primary place for implementing paradigm-specific optimizations, translating the user-friendly API calls into efficient sequences of core `Renderer` operations.
-   **Ergonomics:** Provides users with more idiomatic and convenient interfaces compared to directly manipulating `RenderOperation` objects for common tasks.

The `RenderContext` described later serves as the foundational context, often wrapped or utilized internally by these higher-level APIs.

## 5. Key Components & Concepts

### Core Concepts

#### Dimension-Aware Types

The engine uses a hybrid approach for handling 2D and 3D operations:

```rust
// Core generic types for internal operations
pub struct Point<const D: usize> {
    coordinates: [f32; D],
}

pub type Point2D = Point<2>;
pub type Point3D = Point<3>;

// Public API uses enums for runtime polymorphism
pub enum PointInSpace {
    D2(Point2D),
    D3(Point3D),
}

// Dimension-specific transforms
pub struct Transform<const D: usize> {
    matrix: [[f32; D + 1]; D + 1], // D+1 for homogeneous coordinates
}

pub type Transform2D = Transform<2>;
pub type Transform3D = Transform<3>;

pub enum TransformInSpace {
    D2(Transform2D),
    D3(Transform3D),
}
```

#### Renderable Trait System

The foundation for any drawable element:

```rust
pub trait Renderable {
    /// Get the dimensionality of this renderable
    fn dimensionality(&self) -> Dimensionality;
    
    /// Get the local origin point
    fn origin(&self) -> PointInSpace;
    
    /// Set the local origin point
    fn set_origin(&mut self, origin: PointInSpace);
    
    /// Get local bounds without any transforms
    fn local_bounds(&self) -> BoundingVolume;
    
    /// Get bounds with current transform applied
    fn world_bounds(&self, transform: &TransformInSpace) -> BoundingVolume;
    
    /// Perform a transform relative to the origin
    fn transform_local(&mut self, transform: &TransformInSpace);
    
    /// Create a clone of this renderable
    fn clone_renderable(&self) -> Box<dyn Renderable>;
}

pub trait Object2D: Renderable {
    /// Get the 2D bounding rectangle in local space
    fn local_rect(&self) -> Rect;
    
    /// Get the 2D bounding rectangle in world space
    fn world_rect(&self, transform: &Transform2D) -> Rect;
    
    /// Get the 2D origin point
    fn origin_2d(&self) -> Point2D {
        match self.origin() {
            PointInSpace::D2(p) => p,
            _ => panic!("Expected 2D point for 2D renderable"),
        }
    }
}

pub trait Object3D: Renderable {
    /// Get the 3D bounding box in local space
    fn local_box(&self) -> BoundingBox;
    
    /// Get the 3D bounding box in world space
    fn world_box(&self, transform: &Transform3D) -> BoundingBox;
    
    /// Get the 3D origin point
    fn origin_3d(&self) -> Point3D {
        match self.origin() {
            PointInSpace::D3(p) => p,
            _ => panic!("Expected 3D point for 3D renderable"),
        }
    }
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
    fn tessellate(&self, options: &TessellationOptions) -> TessellationResultInSpace;
}

// Options controlling the tessellation process
pub struct TessellationOptions {
    pub quality: TessellationQuality, // Controls detail level
    pub tolerance: f32,               // Max error tolerance
    pub generate_normals: bool,       // Whether to generate normals
    pub generate_uvs: bool,           // Whether to generate UV coordinates
    pub cull_mode: CullMode,          // Face culling mode
}

// Generic vertex type
pub struct Vertex<const D: usize> {
    position: Point<D>,
    normal: Option<Point<D>>,
    tex_coords: Option<[f32; 2]>,
}

pub type Vertex2D = Vertex<2>;
pub type Vertex3D = Vertex<3>;

// Result of tessellation
pub struct TessellationResult<const D: usize> {
    vertices: Vec<Vertex<D>>,
    indices: Vec<u32>,
    bounds: BoundingVolume<D>,
    stats: TessellationStats,
}

// For runtime handling
pub enum TessellationResultInSpace {
    D2(TessellationResult<2>),
    D3(TessellationResult<3>),
}
```

#### Bounding Volume System

```rust
pub trait BoundingVolume<const D: usize> {
    fn contains_point(&self, point: &Point<D>) -> bool;
    fn intersects(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Self;
}

pub struct AABB<const D: usize> {
    min: Point<D>,
    max: Point<D>,
}

pub type AABB2D = AABB<2>;
pub type AABB3D = AABB<3>;

// For runtime handling
pub enum BoundingVolumeInSpace {
    D2(AABB2D),
    D3(AABB3D),
}

impl<const D: usize> AABB<D> {
    pub fn from_points(points: &[Point<D>]) -> Self {
        let mut min = [f32::MAX; D];
        let mut max = [f32::MIN; D];
        
        for point in points {
            for i in 0..D {
                min[i] = min[i].min(point.coordinates[i]);
                max[i] = max[i].max(point.coordinates[i]);
            }
        }
        
        Self {
            min: Point { coordinates: min },
            max: Point { coordinates: max },
        }
    }
}
```

#### Operation System
Combining elements with styles and transforms:

```rust
pub struct RenderOperation {
    element: Box<dyn Renderable>,
    transform: TransformInSpace,
    styles: Vec<Box<dyn Style>>,
    effects: Vec<Box<dyn Effect>>,
    clip: Option<ClipRegion>,
}

impl RenderOperation {
    /// Create a new operation
    pub fn new<R: Renderable + 'static>(element: R) -> Self {
        Self {
            element: Box::new(element),
            transform: match element.dimensionality() {
                Dimensionality::D2 => TransformInSpace::D2(Transform2D::identity()),
                Dimensionality::D3 => TransformInSpace::D3(Transform3D::identity()),
            },
            styles: Vec::new(),
            effects: Vec::new(),
            clip: None,
        }
    }
    
    /// Add a style to the operation
    pub fn with_style<S: Style + 'static>(mut self, style: S) -> Self {
        self.styles.push(Box::new(style));
        self
    }
    
    /// Add an effect to the operation
    pub fn with_effect<E: Effect + 'static>(mut self, effect: E) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
    
    /// Set the transform for the operation
    pub fn with_transform(mut self, transform: TransformInSpace) -> Self {
        self.transform = transform;
        self
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

impl Object2D for Rectangle {
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

impl Object2D for TextRun {
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

impl Object3D for Mesh {
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

#### Pipeline System

```rust
pub struct Pipeline<const D: usize> {
    vertex_shader: ShaderHandle,
    fragment_shader: ShaderHandle,
    vertex_layout: VertexLayout<D>,
}

pub type Pipeline2D = Pipeline<2>;
pub type Pipeline3D = Pipeline<3>;

// For runtime handling
pub enum PipelineInSpace {
    D2(Pipeline2D),
    D3(Pipeline3D),
}

impl<const D: usize> Pipeline<D> {
    pub fn bind(&self, cmd: &mut dyn CommandBuffer) {
        // Binding logic specific to dimension
    }
}

pub struct PipelineCache {
    pipelines: HashMap<PipelineKey, PipelineInSpace>,
}

impl PipelineCache {
    pub fn get_or_create_pipeline(&mut self, key: PipelineKey, device: &mut dyn BackendDevice) -> PipelineInSpace {
        // Get existing pipeline or create a new one
    }
}
```

#### Resource Management

```rust
pub struct GeometryBuffer<const D: usize> {
    vertices: Vec<Vertex<D>>,
    indices: Vec<u32>,
}

pub type GeometryBuffer2D = GeometryBuffer<2>;
pub type GeometryBuffer3D = GeometryBuffer<3>;

// For runtime handling
pub enum GeometryBufferInSpace {
    D2(GeometryBuffer2D),
    D3(GeometryBuffer3D),
}

pub struct ResourceCache {
    geometries: HashMap<GeometryHandle, GeometryBufferInSpace>,
    textures: HashMap<TextureHandle, TextureDesc>,
    buffers: HashMap<BufferHandle, BufferDesc>,
    
    // Maps from our handles to backend GPU resources
    gpu_geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>, // Vertex, Index
    gpu_textures: HashMap<TextureHandle, gpu::TextureHandle>,
    
    pending_uploads: Vec<PendingUpload>,
}

impl ResourceCache {
    pub fn register_geometry(&mut self, geometry: GeometryBufferInSpace) -> GeometryHandle {
        // Assign a new handle, store the geometry
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

## 8. Transform-Aware Containers

The architecture supports hierarchical transformations for zoom, scroll, and pan operations through a specialized container concept that avoids naming conflicts with standard graphics terminology:

```rust
/// A specialized container that manages transformation state for visual content
pub struct TransformZone {
    // A unique identifier for this zone
    id: u64,
    
    // Content to be rendered within this zone
    elements: Vec<Box<dyn Renderable>>,
    
    // Transformation state
    scale: f32,  // For zoom operations
    offset: PointInSpace,  // For pan/scroll operations
    
    // Optional visible bounds for clipping
    visible_bounds: Option<Box<dyn BoundingVolume>>,
    
    // Transform stack for hierarchical transformations
    transform_stack: Vec<TransformInSpace>,
    
    // Parent zone for hierarchical relationships
    parent: Option<WeakHandle<TransformZone>>,
    
    // Child zones
    children: Vec<Handle<TransformZone>>,
}

impl TransformZone {
    /// Create a new transform zone
    pub fn new(id: u64) -> Self {
        Self {
            id,
            elements: Vec::new(),
            scale: 1.0,
            offset: match Dimensionality::D2 {
                Dimensionality::D2 => PointInSpace::D2(Point2D::zero()),
                Dimensionality::D3 => PointInSpace::D3(Point3D::zero()),
            },
            visible_bounds: None,
            transform_stack: vec![TransformInSpace::D2(Transform2D::identity())],
            parent: None,
            children: Vec::new(),
        }
    }
    
    /// Set the zoom factor for this zone
    pub fn set_zoom(&mut self, scale: f32) {
        self.scale = scale.max(0.01); // Prevent negative or zero zoom
    }
    
    /// Set the pan/scroll offset for this zone
    pub fn set_offset(&mut self, offset: PointInSpace) {
        self.offset = offset;
    }
    
    /// Set the visible bounds for clipping
    pub fn set_visible_bounds(&mut self, bounds: Option<Box<dyn BoundingVolume>>) {
        self.visible_bounds = bounds;
    }
    
    /// Add an element to this zone
    pub fn add_element(&mut self, element: Box<dyn Renderable>) {
        self.elements.push(element);
    }
    
    /// Add a child zone (for hierarchical transformations)
    pub fn add_child(&mut self, child: Handle<TransformZone>) {
        // Set parent relationship
        if let Some(child_ref) = child.upgrade() {
            child_ref.borrow_mut().parent = Some(WeakHandle::from(child.clone()));
        }
        
        self.children.push(child);
    }
    
    /// Calculate the effective transform for this zone
    pub fn effective_transform(&self) -> TransformInSpace {
        // Start with the base transform from the transform stack
        let mut transform = self.transform_stack.last()
            .cloned()
            .unwrap_or_else(|| TransformInSpace::D2(Transform2D::identity()));
        
        // Apply scale (zoom)
        let scale_transform = match &transform {
            TransformInSpace::D2(_) => {
                TransformInSpace::D2(Transform2D::scaling(Point2D::new(self.scale, self.scale)))
            },
            TransformInSpace::D3(_) => {
                TransformInSpace::D3(Transform3D::scaling(Point3D::new(self.scale, self.scale, self.scale)))
            },
        };
        transform = combine_transforms(&transform, &scale_transform);
        
        // Apply offset (pan/scroll)
        let offset_transform = match &transform {
            TransformInSpace::D2(_) => {
                if let PointInSpace::D2(point) = &self.offset {
                    TransformInSpace::D2(Transform2D::translation(*point))
                } else {
                    TransformInSpace::D2(Transform2D::identity())
                }
            },
            TransformInSpace::D3(_) => {
                if let PointInSpace::D3(point) = &self.offset {
                    TransformInSpace::D3(Transform3D::translation(*point))
                } else {
                    TransformInSpace::D3(Transform3D::identity())
                }
            },
        };
        transform = combine_transforms(&transform, &offset_transform);
        
        // Apply parent transforms if any
        if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                let parent_transform = parent.borrow().effective_transform();
                transform = combine_transforms(&parent_transform, &transform);
            }
        }
        
        transform
    }
    
    /// Render this zone using the provided render context
    pub fn render(&self, context: &mut RenderContext) -> Result<(), RenderError> {
        // Save current transform
        let current_transform = context.get_transform();
        
        // Apply this zone's transform
        context.set_transform(self.effective_transform());
        
        // Set up clipping if needed
        if let Some(bounds) = &self.visible_bounds {
            context.push_clip(bounds.clone());
        }
        
        // Render all elements in this zone
        for element in &self.elements {
            // Create a render operation
            let operation = RenderOperation::new(element.clone_renderable());
            
            // Execute the operation
            context.execute_operation(&operation)?;
        }
        
        // Render all child zones
        for child in &self.children {
            if let Some(child_ref) = child.upgrade() {
                child_ref.borrow().render(context)?;
            }
        }
        
        // Restore clipping
        if self.visible_bounds.is_some() {
            context.pop_clip();
        }
        
        // Restore transform
        context.set_transform(current_transform);
        
        Ok(())
    }
}
```

### Key Features of Transform-Aware Containers

1. **Hierarchical Transformations**:
   - Support for parent-child relationships enables nested transform zones.
   - Transformations cascade down from parent to child zones.
   - Each zone maintains its own transform stack.

2. **Independent Transformation Controls**:
   - Zoom capabilities via scale factor
   - Pan/scroll via translation offset
   - Content clipping via visible bounds

3. **Integration with Core Architecture**:
   - Works with existing `RenderOperation` and `RenderContext` patterns
   - Leverages the dimension-aware types (`PointInSpace`, `TransformInSpace`)
   - Compatible with both 2D and 3D rendering

4. **Usage Scenarios**:
   - Scrollable/zoomable UI containers
   - Complex document viewers
   - Multi-view panels with independent zoom levels
   - Mini-maps and detail views

5. **Naming Considerations**:
   - `TransformZone` name avoids conflicts with common graphics API terminology
   - Does not clash with Vulkan/DirectX/OpenGL "Viewport" concept
   - Clear separation from backend-specific viewport transformations

### Example Usage

```rust
// Create a transform zone for a scrollable container
let scroll_container = TransformZone::new(1);

// Add elements to the container
scroll_container.add_element(Box::new(Rectangle::new(0.0, 0.0, 100.0, 100.0)));

// Set zoom and pan based on user input
scroll_container.set_zoom(1.5); // Zoom to 150%
scroll_container.set_offset(PointInSpace::D2(Point2D::new(-50.0, -30.0))); // Pan

// Set visible bounds for clipping
let visible_bounds = AABB2D::new(
    Point2D::new(0.0, 0.0),
    Point2D::new(200.0, 200.0)
);
scroll_container.set_visible_bounds(Some(Box::new(visible_bounds)));

// Render the container and all its contents with transformations applied
scroll_container.render(&mut context);
```

## 9. Feature Flags

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
transform_zones = [] # Transform-aware container support

# Backend implementations
wgpu_backend = ["wgpu"]  # WGPU backend
vulkan_backend = ["ash"] # Direct Vulkan backend
```

## 10. Conclusion

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