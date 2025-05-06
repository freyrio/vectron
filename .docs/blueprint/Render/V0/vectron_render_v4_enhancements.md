# Vectron Render Crate 4.0 Blueprint

## Overview

This blueprint outlines a comprehensive redesign of the Vectron Render crate, incorporating an advanced coordinate system, flexible unit system, organized elements module, tessellation capabilities, and operation renaming. The goal is to create a modern, flexible rendering system that supports both 2D and 3D graphics with a clean, intuitive API.

## 1. Core Architecture

### 1.1 Core Module Structure

```
vectron_render/
├── src/
│   ├── core/
│   │   ├── renderable.rs       # Renderable trait definitions
│   │   ├── style.rs            # Style traits and implementations
│   │   ├── effect.rs           # Visual effects
│   │   ├── render_operation.rs # Rendering operations (renamed from Operation)
│   │   ├── transform.rs        # Transform utilities
│   │   ├── color.rs            # Color system
│   │   ├── units.rs            # Unit system
│   │   └── tessellation.rs     # Tessellation traits and utilities
│   │
│   ├── elements/               # Core renderable elements
│   │   ├── mod.rs              # Re-exports and common functionality
│   │   ├── d2/                 # 2D elements
│   │   │   ├── mod.rs
│   │   │   ├── camera.rs
│   │   │   ├── layer.rs
│   │   │   ├── text.rs
│   │   │   └── geometry/
│   │   │       ├── mod.rs
│   │   │       ├── rectangle.rs
│   │   │       ├── circle.rs
│   │   │       ├── path.rs
│   │   │       └── ...
│   │   └── d3/                 # 3D elements
│   │       ├── mod.rs
│   │       ├── camera.rs
│   │       ├── light.rs
│   │       ├── scene.rs
│   │       └── geometry/
│   │           ├── mod.rs
│   │           ├── box.rs
│   │           ├── sphere.rs
│   │           ├── mesh.rs
│   │           └── ...
│   │
│   ├── tessellation/           # Tessellation module
│   │   ├── mod.rs
│   │   ├── options.rs
│   │   ├── result.rs
│   │   ├── d2/                 # 2D tessellation algorithms
│   │   └── d3/                 # 3D tessellation algorithms
│   │
│   ├── renderer/               # Renderer implementations
│   ├── pipeline/               # Rendering pipelines
│   ├── backend/                # Backend abstractions
│   ├── utils/                  # Utility modules
│   └── lib.rs                  # Public API exports
```

### 1.2 Feature Flags

```toml
[features]
default = ["d2", "d3"]

# Dimension features
d2 = []                 # Enable 2D rendering
d3 = []                 # Enable 3D rendering

# Element type features
d2-geometry = ["d2"]     # 2D geometric elements
d2-text = ["d2"]         # 2D text elements
d3-geometry = ["d3"]     # 3D geometric elements
d3-scene = ["d3"]        # 3D scene elements

# Optimization features
tessellation = []        # Enable tessellation support
gpu-acceleration = []    # Enable GPU accelerated paths
batching = []            # Enable draw call batching
```

## 2. Coordinate and Unit Systems

### 2.1 Coordinate System

```rust
// In core/renderable.rs
pub trait Renderable {
    // Get the origin point of this renderable in its local space
    fn origin(&self) -> Vec3;
    
    // Set the origin point
    fn set_origin(&mut self, origin: Vec3);
    
    // Transform relative to the renderable's origin
    fn transform_local(&mut self, transform: &Transform);
    
    // The bounds in local space
    fn local_bounds(&self) -> BoundingVolume;
    
    // The bounds in world space (after transformation)
    fn world_bounds(&self) -> BoundingVolume;
    
    // Other standard methods...
}

// Specializations for 2D
pub trait Renderable2D: Renderable {
    // Get 2D origin
    fn origin_2d(&self) -> Vec2 {
        let origin = self.origin();
        Vec2::new(origin.x, origin.y)
    }
    
    // Set 2D origin
    fn set_origin_2d(&mut self, origin: Vec2) {
        self.set_origin(Vec3::new(origin.x, origin.y, 0.0));
    }
    
    // 2D bounding rectangle in local space
    fn local_rect(&self) -> Rect;
    
    // 2D bounding rectangle in world space
    fn world_rect(&self) -> Rect;
}

// Specializations for 3D
pub trait Renderable3D: Renderable {
    // 3D bounding box in local space
    fn local_box(&self) -> BoundingBox;
    
    // 3D bounding box in world space
    fn world_box(&self) -> BoundingBox;
}
```

### 2.2 Unit System

```rust
// In core/units.rs
pub enum Unit<T: UnitValue = f32> {
    // Absolute units
    Pixels(T),
    Points(T),           // 1/72 inch
    Inches(T),
    Millimeters(T),
    
    // Relative units
    Percent(T),          // Relative to parent
    ViewportWidth(T),    // Proportion of viewport width
    ViewportHeight(T),   // Proportion of viewport height
    ViewportMin(T),      // Proportion of min(width, height)
    ViewportMax(T),      // Proportion of max(width, height)
    
    // Computed units
    Computed(T),         // Already computed value
    
    // Expression (for computed values with dependencies)
    Expression(Box<dyn Fn(&UnitContext) -> T>),
}

pub struct UnitContext {
    pub dpi: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub parent_width: Option<f32>,
    pub parent_height: Option<f32>,
    pub parent_depth: Option<f32>,
}

// Core functionality
impl<T: UnitValue> Unit<T> {
    pub fn compute(&self, context: &UnitContext) -> T {
        match self {
            Unit::Pixels(val) => *val,
            Unit::Points(val) => *val * (context.dpi / 72.0),
            Unit::Percent(val) => {
                if let (Some(parent_w), Some(parent_h)) = (context.parent_width, context.parent_height) {
                    *val * T::from_f32(0.01 * (parent_w + parent_h) * 0.5)
                } else {
                    *val // Fallback
                }
            },
            // Other unit calculations...
            Unit::Computed(val) => *val,
            Unit::Expression(expr) => expr(context),
        }
    }
    
    // Convenience methods
    pub fn px(value: T) -> Self { Unit::Pixels(value) }
    pub fn pt(value: T) -> Self { Unit::Points(value) }
    pub fn pct(value: T) -> Self { Unit::Percent(value) }
    pub fn vw(value: T) -> Self { Unit::ViewportWidth(value) }
    pub fn vh(value: T) -> Self { Unit::ViewportHeight(value) }
}

// Trait for types that can be used with units
pub trait UnitValue: Copy + std::ops::Mul<f32, Output = Self> {
    fn as_f32(&self) -> f32;
    fn from_f32(val: f32) -> Self;
}

// Implementations for common types
impl UnitValue for f32 {
    fn as_f32(&self) -> f32 { *self }
    fn from_f32(val: f32) -> Self { val }
}

impl UnitValue for i32 {
    fn as_f32(&self) -> f32 { *self as f32 }
    fn from_f32(val: f32) -> Self { val.round() as i32 }
}

// Implement From for automatic conversions
impl From<f32> for Unit<f32> {
    fn from(val: f32) -> Self {
        Unit::Pixels(val) // Default is pixels
    }
}

impl From<i32> for Unit<f32> {
    fn from(val: i32) -> Self {
        Unit::Pixels(val as f32)
    }
}
```

## 3. Elements Module

### 3.1 2D Elements

```rust
// In elements/d2/geometry/rectangle.rs
pub struct Rectangle {
    pub x: Unit<f32>,
    pub y: Unit<f32>,
    pub width: Unit<f32>,
    pub height: Unit<f32>,
    pub color: Color,
    pub origin: Vec2,
}

impl Rectangle {
    pub fn new(
        x: impl Into<Unit<f32>>,
        y: impl Into<Unit<f32>>,
        width: impl Into<Unit<f32>>,
        height: impl Into<Unit<f32>>,
        color: Color
    ) -> Self {
        let width_val = width.into();
        let height_val = height.into();
        
        Self {
            x: x.into(),
            y: y.into(),
            width: width_val,
            height: height_val,
            color,
            origin: Vec2::new(0.0, 0.0), // Default to top-left
        }
    }
    
    pub fn center(
        x: impl Into<Unit<f32>>,
        y: impl Into<Unit<f32>>,
        width: impl Into<Unit<f32>>,
        height: impl Into<Unit<f32>>,
        color: Color
    ) -> Self {
        let mut rect = Self::new(x, y, width, height, color);
        rect.set_origin_2d(Vec2::new(0.5, 0.5)); // Center origin
        rect
    }
    
    // Compute actual pixel values
    pub fn resolved_bounds(&self, context: &UnitContext) -> Rect {
        Rect::new(
            self.x.compute(context),
            self.y.compute(context),
            self.width.compute(context),
            self.height.compute(context)
        )
    }
}

impl Renderable for Rectangle {
    fn origin(&self) -> Vec3 {
        // Map origin from normalized [0,1] space to actual coordinates
        let bounds = self.local_bounds();
        Vec3::new(
            bounds.min.x + (bounds.max.x - bounds.min.x) * self.origin.x,
            bounds.min.y + (bounds.max.y - bounds.min.y) * self.origin.y,
            0.0
        )
    }
    
    fn set_origin(&mut self, origin: Vec3) {
        self.origin = Vec2::new(origin.x, origin.y);
    }
    
    fn transform_local(&mut self, transform: &Transform) {
        // Implementation that respects the origin
    }
    
    fn local_bounds(&self) -> BoundingVolume {
        // Get bounds using the unit values directly (without computing)
        // Used for hierarchical transformations
        BoundingVolume::Rect(self.local_rect())
    }
    
    fn world_bounds(&self) -> BoundingVolume {
        // Apply transformations to local_bounds
        BoundingVolume::Rect(self.world_rect())
    }
}

impl Renderable2D for Rectangle {
    fn local_rect(&self) -> Rect {
        // Return rectangle based on the current unit values
        // (would be computed during rendering)
        Rect::new(0.0, 0.0, 0.0, 0.0) // Placeholder
    }
    
    fn world_rect(&self) -> Rect {
        // Return transformed rectangle
        Rect::new(0.0, 0.0, 0.0, 0.0) // Placeholder
    }
}
```

### 3.2 3D Elements

```rust
// In elements/d3/geometry/box.rs
pub struct Box3D {
    pub center: Vec3Unit,
    pub size: Vec3Unit,
    pub color: Color,
    pub origin: Vec3,
}

// Vec3 with units
pub struct Vec3Unit {
    pub x: Unit<f32>,
    pub y: Unit<f32>,
    pub z: Unit<f32>,
}

impl Box3D {
    pub fn new(
        center: Vec3Unit,
        size: Vec3Unit,
        color: Color
    ) -> Self {
        Self {
            center,
            size,
            color,
            origin: Vec3::new(0.5, 0.5, 0.5), // Default to center
        }
    }
    
    pub fn cube(
        center: Vec3Unit,
        size: impl Into<Unit<f32>>,
        color: Color
    ) -> Self {
        let size_unit = size.into();
        Self::new(
            center,
            Vec3Unit {
                x: size_unit.clone(),
                y: size_unit.clone(),
                z: size_unit,
            },
            color
        )
    }
    
    // Compute actual values
    pub fn resolved_bounds(&self, context: &UnitContext) -> BoundingBox {
        // Compute real dimensions from unit values
        let center = Vec3::new(
            self.center.x.compute(context),
            self.center.y.compute(context),
            self.center.z.compute(context)
        );
        
        let size = Vec3::new(
            self.size.x.compute(context),
            self.size.y.compute(context),
            self.size.z.compute(context)
        );
        
        BoundingBox::from_center(center, size * 0.5)
    }
}

impl Renderable for Box3D {
    // Implementation similar to Rectangle but for 3D
}

impl Renderable3D for Box3D {
    fn local_box(&self) -> BoundingBox {
        // Return box based on the current unit values
        BoundingBox::default() // Placeholder
    }
    
    fn world_box(&self) -> BoundingBox {
        // Return transformed box
        BoundingBox::default() // Placeholder
    }
}
```

## 4. Tessellable and Tessellation Module

### 4.1 Tessellable Trait

```rust
// In core/tessellation.rs
pub trait Tessellable {
    // The vertex type produced by tessellation
    type VertexType;
    
    // The index type (usually u16 or u32)
    type IndexType;
    
    // Tessellate the renderable into geometry
    fn tessellate(&self, options: &TessellationOptions) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError>;
}
```

### 4.2 Tessellation Module

```rust
// In tessellation/options.rs
pub struct TessellationOptions {
    pub quality: TessellationQuality,
    pub tolerance: f32,
    pub cull_mode: CullMode,
    pub flags: TessellationFlags,
}

pub enum TessellationQuality {
    Low,
    Medium,
    High,
    Custom(u32), // Custom detail level (e.g., subdivision count)
}

pub enum CullMode {
    None,
    Back,
    Front,
}

bitflags! {
    pub struct TessellationFlags: u32 {
        const GENERATE_NORMALS = 0x01;
        const GENERATE_UVS = 0x02;
        const GENERATE_TANGENTS = 0x04;
        const OPTIMIZE_VERTICES = 0x08;
        const OPTIMIZE_INDICES = 0x10;
    }
}

// In tessellation/result.rs
pub struct TessellationResult<V, I> {
    pub vertices: Vec<V>,
    pub indices: Vec<I>,
    pub bounds: BoundingVolume,
    pub statistics: TessellationStatistics,
}

pub struct TessellationStatistics {
    pub vertex_count: usize,
    pub index_count: usize,
    pub triangle_count: usize,
    pub tessellation_time_ms: f32,
}

// Common vertex types
pub mod vertex {
    pub type Vertex2D = [f32; 2];
    pub type Vertex3D = [f32; 3];
    pub type VertexWithNormal = ([f32; 3], [f32; 3]);
    pub type VertexWithUV = ([f32; 3], [f32; 2]);
    pub type VertexFull = ([f32; 3], [f32; 3], [f32; 2], [f32; 4]);
}
```

### 4.3 Implementation Example

```rust
// In elements/d2/geometry/rectangle.rs
impl Tessellable for Rectangle {
    type VertexType = vertex::VertexWithUV;
    type IndexType = u16;
    
    fn tessellate(&self, options: &TessellationOptions) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError> {
        // Get resolved bounds if context is available
        let rect = match options.context {
            Some(ctx) => self.resolved_bounds(ctx),
            None => Rect::new(0.0, 0.0, 1.0, 1.0), // Default to unit square if no context
        };
        
        // Create vertices (position, uv)
        let vertices = vec![
            ([rect.x, rect.y, 0.0], [0.0, 0.0]),
            ([rect.x + rect.width, rect.y, 0.0], [1.0, 0.0]),
            ([rect.x + rect.width, rect.y + rect.height, 0.0], [1.0, 1.0]),
            ([rect.x, rect.y + rect.height, 0.0], [0.0, 1.0]),
        ];
        
        // Create indices for two triangles
        let indices = vec![0, 1, 2, 0, 2, 3];
        
        // Return the result
        Ok(TessellationResult {
            vertices,
            indices,
            bounds: BoundingVolume::Rect(rect),
            statistics: TessellationStatistics {
                vertex_count: 4,
                index_count: 6,
                triangle_count: 2,
                tessellation_time_ms: 0.0, // Not measured in this simple case
            }
        })
    }
}

// In elements/d3/geometry/box.rs
impl Tessellable for Box3D {
    type VertexType = vertex::VertexWithNormal;
    type IndexType = u16;
    
    fn tessellate(&self, options: &TessellationOptions) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError> {
        // Similar implementation for 3D box
        // ...
    }
}
```

## 5. RenderOperation (Renamed from Operation)

```rust
// In core/render_operation.rs
pub struct RenderOperation<R: Renderable> {
    pub renderable: R,
    pub styles: Vec<Box<dyn Style>>,
    pub effects: Vec<Box<dyn Effect>>,
    pub transform: Transform,
    pub clip: Option<Rect>,
}

impl<R: Renderable> RenderOperation<R> {
    pub fn new(renderable: R) -> Self {
        Self {
            renderable,
            styles: Vec::new(),
            effects: Vec::new(),
            transform: Transform::identity(),
            clip: None,
        }
    }
    
    pub fn with_style<S: Style + 'static>(mut self, style: S) -> Self {
        self.styles.push(Box::new(style));
        self
    }
    
    pub fn with_effect<E: Effect + 'static>(mut self, effect: E) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
    
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    
    pub fn with_clip(mut self, clip: Rect) -> Self {
        self.clip = Some(clip);
        self
    }
    
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Save current state
        renderer.push_state();
        
        // Apply transform and clip
        renderer.set_transform(self.transform);
        if let Some(clip) = self.clip {
            renderer.set_clip(clip);
        }
        
        // Apply pre-effects
        for effect in &self.effects {
            effect.apply_pre(renderer, &self.renderable)?;
        }
        
        // Generate geometry from renderable
        let geometry = if let Some(tessellable) = self.as_tessellable() {
            // Generate via tessellation if implemented
            let options = renderer.tessellation_options();
            tessellable.tessellate(&options)
                .map_err(|e| RenderError::TessellationFailed(e.to_string()))?
        } else {
            // Fall back to manual geometry generation
            self.generate_geometry(renderer)?
        };
        
        // Apply styles in order
        for style in &self.styles {
            style.apply(renderer, &geometry)?;
        }
        
        // Apply post-effects
        for effect in &self.effects {
            effect.apply_post(renderer, &geometry)?;
        }
        
        // Restore state
        renderer.pop_state()?;
        
        Ok(())
    }
    
    // Try to get the renderable as Tessellable
    fn as_tessellable(&self) -> Option<&dyn Tessellable> {
        // Attempt to downcast to Tessellable
        None // Default implementation
    }
    
    // Generate geometry manually if tessellation not available
    fn generate_geometry(&self, renderer: &mut dyn Renderer) -> Result<GeometryData, RenderError> {
        // Fallback implementation for non-tessellable renderables
        Err(RenderError::NotImplemented("Renderable does not implement Tessellable and no fallback exists".into()))
    }
}
```

## 6. Public API Exports

```rust
// In lib.rs
pub mod core;
pub mod elements;
pub mod tessellation;
pub mod renderer;
pub mod pipeline;
pub mod backend;
pub mod utils;

// Re-export core traits
pub use core::renderable::{Renderable, Renderable2D, Renderable3D};
pub use core::style::{Style, Fill, Stroke, Paint};
pub use core::effect::{Effect, Shadow, Blur, Glow};
pub use core::render_operation::RenderOperation;
pub use core::color::{Color, constants as color_constants};
pub use core::units::{Unit, UnitContext, UnitValue};
pub use core::tessellation::Tessellable;

// Re-export tessellation utilities
pub use tessellation::{TessellationOptions, TessellationResult, TessellationQuality};

// Re-export elements based on features
#[cfg(feature = "d2-geometry")]
pub use elements::d2::geometry::{Rectangle, Circle, Ellipse, Path};

#[cfg(feature = "d2-text")]
pub use elements::d2::{Text, TextBlock};

#[cfg(feature = "d2")]
pub use elements::d2::{Camera as Camera2D, Layer as Layer2D};

#[cfg(feature = "d3-geometry")]
pub use elements::d3::geometry::{Box3D, Sphere, Cylinder, Mesh};

#[cfg(feature = "d3-scene")]
pub use elements::d3::{Camera as Camera3D, Light, Scene};
```

## 7. Implementation Priorities and Timeline

1. **Phase 1: Core Framework (Month 1)**
   - Implement core traits (Renderable, Style, Effect)
   - Implement unit system
   - Implement coordinate system
   - Implement RenderOperation

2. **Phase 2: Element Implementation (Month 2)**
   - Implement 2D elements (geometric + non-geometric)
   - Implement 3D elements (geometric + non-geometric)
   - Integrate coordinate and unit systems

3. **Phase 3: Tessellation System (Month 3)**
   - Implement Tessellable trait
   - Implement tessellation algorithms for 2D elements
   - Implement tessellation algorithms for 3D elements
   - Optimize tessellation performance

4. **Phase 4: Integration and Optimization (Month 4)**
   - Complete remaining implementations
   - Performance optimization
   - Documentation and examples
   - Final testing and bug fixes

## 8. Conclusion

The Vectron Render 4.0 redesign introduces significant improvements:

1. **Coordinate System**: Each renderable now has its own internal coordinate system with an origin, making transformations more intuitive and enabling hierarchical relationships.

2. **Unit System**: A flexible, zero-cost unit system supports absolute units (pixels, points), relative units (percentages, viewport-relative), and dynamic expressions, enabling responsive layouts.

3. **Elements Module**: Clear organization of renderable elements into dimensional categories (2D/3D) and type categories (geometric/non-geometric), with proper feature flags for selective inclusion.

4. **Tessellation System**: A comprehensive tessellation system with configurable quality options, enabling efficient geometry generation tailored to the rendering context.

5. **Renamed Operations**: Clearer naming with RenderOperation makes the API more intuitive and aligns with the focus on rendering.

This blueprint focuses on creating a modern, flexible rendering system that's both powerful and intuitive. By addressing coordinate systems, units, organization, and tessellation, the redesign enables a wide range of rendering applications while maintaining a clean and consistent API.