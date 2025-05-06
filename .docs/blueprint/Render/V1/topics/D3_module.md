# Refining the D3 API for Three.js Compatibility with Rust Idioms

This is an excellent direction for the D3 API within this rendering engine. Let me explore how we could revise the D3 elements and API layer to align with Three.js while maintaining Rust idioms and supporting broader use cases like 3D CAD.

## Three.js Alignment with Rust Idioms

Three.js provides a well-established, intuitive API for 3D graphics. Adapting its paradigms to Rust would involve:

### Scene Graph Structure

```rust
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub background: Option<Background>,
    pub environment: Option<CubeTexture>,
    pub fog: Option<Fog>,
}

pub struct Object3D {
    pub name: String,
    pub position: Vector3,
    pub rotation: Euler,
    pub quaternion: Quaternion,
    pub scale: Vector3,
    pub matrix: Matrix4,
    pub visible: bool,
    pub children: Vec<Object3D>,
    pub parent: Option<Weak<RefCell<Object3D>>>,
    pub user_data: HashMap<String, Value>,
    // Rust-specific: Using traits for polymorphism
    pub renderable: Option<Box<dyn Renderable3D>>,
}
```

### Camera System

```rust
pub trait Camera: Renderable3D {
    fn get_projection_matrix(&self) -> Matrix4;
    fn get_view_matrix(&self) -> Matrix4;
    fn update_matrix_world(&mut self, force: bool);
}

pub struct PerspectiveCamera {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub zoom: f32,
    pub view: Object3D,
    // Cached matrices
    projection_matrix: Matrix4,
    view_matrix: Matrix4,
}

pub struct OrthographicCamera {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
    pub zoom: f32,
    pub view: Object3D,
    // Cached matrices
    projection_matrix: Matrix4,
    view_matrix: Matrix4,
}
```

### Materials System

```rust
pub trait Material: Style {
    fn clone_material(&self) -> Box<dyn Material>;
    fn is_transparent(&self) -> bool;
    fn needs_update(&self) -> bool;
    fn set_needs_update(&mut self, value: bool);
}

pub struct MeshStandardMaterial {
    pub color: Color,
    pub roughness: f32,
    pub metalness: f32,
    pub map: Option<TextureHandle>,
    pub normal_map: Option<TextureHandle>,
    pub roughness_map: Option<TextureHandle>,
    pub metalness_map: Option<TextureHandle>,
    pub ao_map: Option<TextureHandle>,
    pub emissive: Color,
    pub emissive_map: Option<TextureHandle>,
    pub alpha_test: f32,
    pub transparent: bool,
    pub side: RenderSide,
    pub flat_shading: bool,
    pub wireframe: bool,
    pub needs_update: bool,
}

// More material types: MeshBasicMaterial, MeshPhongMaterial, etc.
```

### Geometry System

```rust
pub trait BufferGeometry: Tessellable {
    fn get_attribute(&self, name: &str) -> Option<&BufferAttribute>;
    fn get_attribute_mut(&mut self, name: &str) -> Option<&mut BufferAttribute>;
    fn set_attribute(&mut self, name: &str, attribute: BufferAttribute);
    fn get_index(&self) -> Option<&BufferAttribute>;
    fn set_index(&mut self, index: BufferAttribute);
    fn compute_vertex_normals(&mut self);
    fn compute_tangents(&mut self);
}

pub struct BufferAttribute {
    pub array: Vec<f32>,  // Could be generic or enum for different types
    pub item_size: usize, // How many values per vertex
    pub normalized: bool,
    pub usage: BufferUsage,
    pub version: u32,
}

// Primitive geometry factories
pub fn create_box_geometry(width: f32, height: f32, depth: f32) -> impl BufferGeometry {
    // Implementation
}

pub fn create_sphere_geometry(radius: f32, width_segments: u32, height_segments: u32) -> impl BufferGeometry {
    // Implementation
}
```

## Bevy-Inspired Enhancements

While aligning with Three.js, we can adopt some of Bevy's strengths:

### Builder Pattern for Configuration

```rust
pub struct MeshBuilder {
    geometry: Option<Box<dyn BufferGeometry>>,
    material: Option<Box<dyn Material>>,
    name: Option<String>,
    cast_shadow: bool,
    receive_shadow: bool,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            geometry: None,
            material: None,
            name: None,
            cast_shadow: true,
            receive_shadow: true,
        }
    }
    
    pub fn with_geometry<G: BufferGeometry + 'static>(mut self, geometry: G) -> Self {
        self.geometry = Some(Box::new(geometry));
        self
    }
    
    pub fn with_material<M: Material + 'static>(mut self, material: M) -> Self {
        self.material = Some(Box::new(material));
        self
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn build(self) -> Result<Mesh, RenderError> {
        // Build and validate mesh
    }
}
```

### CAD-Specific Extensions

```rust
// For precise CAD operations
pub struct CADGeometry {
    // Base geometry
    pub base_geometry: BufferGeometryImpl,
    // CAD-specific metadata
    pub topology: CADTopology,
    // For boolean operations
    pub csg_tree: Option<CSGNode>,
}

// Topological entities for CAD
pub struct CADTopology {
    pub vertices: Vec<TopologicalVertex>,
    pub edges: Vec<TopologicalEdge>,
    pub faces: Vec<TopologicalFace>,
    pub solids: Vec<TopologicalSolid>,
}

// Boolean operations (CSG)
pub enum CSGOperation {
    Union,
    Subtract,
    Intersect,
}

pub struct CSGNode {
    pub operation: CSGOperation,
    pub left: Box<CSGNode>,
    pub right: Box<CSGNode>,
    pub geometry: Option<CADGeometry>,
}
```

## Revising the D3 Module Structure

Based on these considerations, here's a refined directory structure for the D3 module:

```
elements/
├── d3/
│   ├── mod.rs                   # Module exports
│   ├── scene.rs                 # Scene definition
│   ├── object3d.rs              # Base 3D object
│   │
│   ├── camera/                  # Camera implementations
│   │   ├── mod.rs              
│   │   ├── perspective.rs       # Perspective camera
│   │   ├── orthographic.rs      # Orthographic camera
│   │   └── controls.rs          # Camera controls
│   │
│   ├── geometry/                # Geometry implementations
│   │   ├── mod.rs              
│   │   ├── buffer_geometry.rs   # Core buffer geometry
│   │   ├── attribute.rs         # Buffer attributes
│   │   ├── primitives/
│   │   │   ├── mod.rs           # Primitive exports
│   │   │   ├── box.rs           # Box geometry
│   │   │   ├── sphere.rs        # Sphere geometry
│   │   │   ├── cylinder.rs      # Cylinder geometry
│   │   │   └── plane.rs         # Plane geometry
│   │   └── cad/                 # CAD-specific geometries
│   │       ├── mod.rs
│   │       ├── brep.rs          # Boundary representation
│   │       ├── csg.rs           # Constructive solid geometry
│   │       └── nurbs.rs         # NURBS surfaces
│   │
│   ├── material/                # Material implementations
│   │   ├── mod.rs
│   │   ├── basic.rs             # Basic material
│   │   ├── standard.rs          # PBR material
│   │   ├── phong.rs             # Phong material
│   │   └── shader_material.rs   # Custom shader material
│   │
│   ├── light/                   # Light implementations
│   │   ├── mod.rs
│   │   ├── ambient.rs           # Ambient light
│   │   ├── directional.rs       # Directional light
│   │   ├── point.rs             # Point light
│   │   └── spot.rs              # Spot light
│   │
│   ├── mesh.rs                  # Mesh implementation
│   ├── group.rs                 # Group node
│   ├── helpers.rs               # Visual helpers (axes, grid, etc.)
│   └── loaders/                 # Model loaders
│       ├── mod.rs
│       ├── gltf.rs              # glTF loader
│       ├── obj.rs               # OBJ loader
│       └── step.rs              # STEP loader for CAD
```

## API Layer Implementation

For the API layer within the crate (rather than as a separate crate), we would:

```rust
// In src/api/mod.rs
pub mod d3; // D3 API module

// In src/api/d3/mod.rs
//! Three.js-inspired API for 3D rendering in Rust

use crate::core::{self, Transform, Units};
use crate::elements::d3 as elements;
use crate::renderer::RenderContext;

// Scene management
pub struct Scene {
    inner: elements::Scene,
    // Additional API-level state
}

impl Scene {
    pub fn new() -> Self {
        Self {
            inner: elements::Scene::new(),
        }
    }
    
    pub fn add(&mut self, object: &mut Object3D) {
        self.inner.objects.push(object.inner.clone());
    }
    
    pub fn remove(&mut self, object: &Object3D) {
        // Implementation
    }
    
    // More scene methods
}

// Object3D wrapper
pub struct Object3D {
    inner: elements::Object3D,
}

impl Object3D {
    pub fn new() -> Self {
        Self {
            inner: elements::Object3D::new(),
        }
    }
    
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.inner.position.set(x, y, z);
    }
    
    pub fn add_child(&mut self, child: &mut Object3D) {
        // Clone the child's inner object and add it
        let mut child_inner = child.inner.clone();
        // Set parent reference appropriately
        self.inner.add_child(child_inner);
    }
    
    // More methods
}

// Mesh creation
pub struct Mesh {
    inner: elements::Mesh,
    object: Object3D,
}

impl Mesh {
    pub fn new(geometry: impl Into<BoxedBufferGeometry>, material: impl Into<BoxedMaterial>) -> Self {
        let geo = geometry.into();
        let mat = material.into();
        Self {
            inner: elements::Mesh::new(geo.inner, mat.inner),
            object: Object3D::new(),
        }
    }
    
    // Delegate Object3D methods
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.object.set_position(x, y, z);
    }
    
    // Mesh-specific methods
}

// Type-erased types for API convenience
pub struct BoxedBufferGeometry {
    inner: Box<dyn elements::BufferGeometry>,
}

pub struct BoxedMaterial {
    inner: Box<dyn elements::Material>,
}

// Geometry factories
pub fn box_geometry(width: f32, height: f32, depth: f32) -> BoxedBufferGeometry {
    BoxedBufferGeometry {
        inner: Box::new(elements::geometry::primitives::create_box_geometry(width, height, depth)),
    }
}

// Material factories
pub fn standard_material() -> BoxedMaterial {
    BoxedMaterial {
        inner: Box::new(elements::material::MeshStandardMaterial::new()),
    }
}

// Renderer wrapper
pub struct Renderer {
    context: RenderContext,
}

impl Renderer {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }
    
    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        // Implementation
    }
}
```

## Example Usage

This API would enable Three.js-like usage patterns while remaining idiomatic Rust:

```rust
use render_engine::api::d3::{self, Scene, Mesh, PerspectiveCamera};

// Create a scene
let mut scene = Scene::new();

// Create a camera
let mut camera = PerspectiveCamera::new(75.0, width as f32 / height as f32, 0.1, 1000.0);
camera.set_position(0.0, 0.0, 5.0);

// Create a mesh
let geometry = d3::box_geometry(1.0, 1.0, 1.0);
let material = d3::standard_material().with_color(d3::Color::rgb(0.8, 0.2, 0.2));
let mut cube = Mesh::new(geometry, material);

// Add to scene
scene.add(&mut cube);

// Create renderer
let mut renderer = d3::Renderer::new(render_context);

// In render loop
renderer.render(&scene, &camera);

// Animate
cube.rotate_x(0.01);
cube.rotate_y(0.01);
```

## CAD-Specific Extensions

For CAD applications, we'd extend the API:

```rust
// CAD-specific extensions
pub mod cad {
    use super::*;
    
    pub struct BrepGeometry {
        inner: elements::geometry::cad::BrepGeometry,
    }
    
    // CSG operations
    pub fn union(a: &Mesh, b: &Mesh) -> Result<Mesh, RenderError> {
        // Implementation
    }
    
    pub fn subtract(a: &Mesh, b: &Mesh) -> Result<Mesh, RenderError> {
        // Implementation
    }
    
    pub fn intersect(a: &Mesh, b: &Mesh) -> Result<Mesh, RenderError> {
        // Implementation
    }
    
    // STEP file handling
    pub fn load_step(path: &Path) -> Result<Mesh, RenderError> {
        // Implementation
    }
}
```

## Conclusion

This approach:

1. Maintains Three.js's intuitive API structure while adapting it to Rust idioms
2. Incorporates Bevy-inspired builder patterns and ergonomics
3. Extends functionality for specialized use cases like CAD
4. Keeps the implementation within the existing module structure
5. Provides both low-level element definitions and a high-level API layer

The result is a D3 system that feels familiar to Three.js users while leveraging Rust's type system and ownership model, and extending functionality for specialized domains like CAD.