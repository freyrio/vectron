# Vectron Render Crate Blueprint v4.0

**Version:** 4.0  
**Date:** March 23, 2025 (Updated)

## Table of Contents

1.  [Overview](#1-overview)
2.  [Core Architecture](#2-core-architecture)
    *   [Core Module Structure](#21-core-module-structure)
    *   [Feature Flags](#22-feature-flags)
3.  [Coordinate and Unit Systems](#3-coordinate-and-unit-systems)
    *   [Coordinate System](#31-coordinate-system)
    *   [Unit System](#32-unit-system)
4.  [Elements Module](#4-elements-module)
    *   [2D Elements](#41-2d-elements)
    *   [3D Elements](#42-3d-elements)
5.  [Tessellation System](#5-tessellation-system)
    *   [Tessellable Trait](#51-tessellable-trait)
    *   [Tessellation Module](#52-tessellation-module)
    *   [Implementation Example](#53-implementation-example)
6.  [Render Operation System](#6-render-operation-system)
    *   [RenderOperation Structure](#61-renderoperation-structure)
    *   [Core Color System](#62-core-color-system)
7.  [Resource Layer](#7-resource-layer)
    *   [Resource Types](#71-resource-types)
    *   [Resource Cache](#72-resource-cache)
    *   [Geometry Management](#73-geometry-management)
    *   [Path Generation](#74-path-generation)
8.  [Pipeline Layer](#8-pipeline-layer)
    *   [Vector Rendering Pipeline](#81-vector-rendering-pipeline)
    *   [Text Rendering Pipeline](#82-text-rendering-pipeline)
    *   [3D Rendering Pipeline](#83-3d-rendering-pipeline)
    *   [Effect Pipeline](#84-effect-pipeline)
9.  [Backend Abstraction](#9-backend-abstraction)
    *   [GPU Interface](#91-gpu-interface)
    *   [Command Translation](#92-command-translation)
    *   [State Management](#93-state-management)
    *   [Resource Binding](#94-resource-binding)
10. [Performance Optimizations](#10-performance-optimizations)
    *   [Batching](#101-batching)
    *   [Culling](#102-culling)
    *   [Caching](#103-caching)
    *   [Parallelism](#104-parallelism)
11. [Error Handling](#11-error-handling)
    *   [Error Types](#111-error-types)
    *   [Recovery Strategies](#112-recovery-strategies)
12. [Public API Exports](#12-public-api-exports)
13. [Implementation Priorities and Timeline](#13-implementation-priorities-and-timeline)
14. [Conclusion](#14-conclusion)

## 1. Overview

The Vectron Render crate provides a flexible rendering system that builds on top of the Vectron GPU abstraction. This v4.0 redesign focuses on an advanced coordinate system, flexible unit system, organized elements module, tessellation capabilities, and operation renaming to create a modern, flexible rendering system supporting both 2D and 3D graphics with a clean, intuitive API.

### Goals

- **Modern API**: Intuitive and consistent API for 2D and 3D.
- **Flexibility**: Support for various rendering techniques, backends, and units.
- **Performance**: Efficient rendering through tessellation, batching, and GPU acceleration.
- **Composition**: Combine renderable elements, styles, and effects easily.
- **Extensibility**: Modular design allowing new elements, styles, and effects.
- **Platform Independence**: Works across platforms through the GPU abstraction.

### Architecture Layers (Conceptual)

1.  **Core**: Fundamental traits (`Renderable`, `Style`, `Effect`, `Tessellable`), coordinate/unit systems, color, transforms.
2.  **Elements**: Concrete renderable items (shapes, text, meshes, scenes) organized by dimension (2D/3D).
3.  **Tessellation**: Geometry generation logic.
4.  **Operations**: Combining elements with styles, effects, and transforms (`RenderOperation`).
5.  **Resource Layer**: Management of GPU resources (textures, buffers, fonts).
6.  **Pipeline Layer**: Specialized rendering pipelines (vector, text, 3D, effects).
7.  **Backend Abstraction**: Interface with the GPU crate (command translation, state management).

## 2. Core Architecture

### 2.1 Core Module Structure

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

### 2.2 Feature Flags

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

## 3. Coordinate and Unit Systems

### 3.1 Coordinate System

The new coordinate system emphasizes local origins and transformations for `Renderable` objects.

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

### 3.2 Unit System

A flexible unit system allows defining sizes and positions in various absolute and relative units.

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
                // Note: Percentage calculation needs context (e.g., relative to width or height?)
                // This example averages parent dimensions, likely needs refinement based on axis.
                if let (Some(parent_w), Some(parent_h)) = (context.parent_width, context.parent_height) {
                    *val * T::from_f32(0.01 * (parent_w + parent_h) * 0.5) 
                } else {
                    *val // Fallback or error?
                }
            },
            Unit::ViewportWidth(val) => *val * T::from_f32(0.01 * context.viewport_width),
            Unit::ViewportHeight(val) => *val * T::from_f32(0.01 * context.viewport_height),
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
pub trait UnitValue: Copy + std::ops::Mul<f32, Output = Self> + std::ops::Add<Self, Output = Self> + Clone {
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

// Implement From for automatic conversions (e.g., 100.0 becomes Unit::Pixels(100.0))
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

## 4. Elements Module

Renderable items are organized into a dedicated `elements` module, categorized by dimension.

### 4.1 2D Elements

```rust
// In elements/d2/geometry/rectangle.rs
pub struct Rectangle {
    pub x: Unit<f32>,
    pub y: Unit<f32>,
    pub width: Unit<f32>,
    pub height: Unit<f32>,
    pub color: Color, // Basic styling, likely replaced by Style trait
    pub origin: Vec2, // Local origin [0, 1] relative to bounds
}

impl Rectangle {
    pub fn new(
        x: impl Into<Unit<f32>>,
        y: impl Into<Unit<f32>>,
        width: impl Into<Unit<f32>>,
        height: impl Into<Unit<f32>>,
        color: Color
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
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
    
    // Compute actual pixel values based on context
    pub fn resolved_bounds(&self, context: &UnitContext) -> Rect {
        let w = self.width.compute(context);
        let h = self.height.compute(context);
        let computed_x = self.x.compute(context);
        let computed_y = self.y.compute(context);
        
        // Adjust position based on origin
        let origin_offset_x = w * self.origin.x;
        let origin_offset_y = h * self.origin.y;
        
        Rect::new(
            computed_x - origin_offset_x,
            computed_y - origin_offset_y,
            w,
            h
        )
    }
}

impl Renderable for Rectangle {
    fn origin(&self) -> Vec3 {
        Vec3::new(self.origin.x, self.origin.y, 0.0)
    }
    
    fn set_origin(&mut self, origin: Vec3) {
        self.origin = Vec2::new(origin.x, origin.y).clamp(Vec2::ZERO, Vec2::ONE);
    }
    
    fn transform_local(&mut self, _transform: &Transform) {
        // Transforms are typically applied via RenderOperation, 
        // but this could apply internal transforms if needed.
    }
    
    fn local_bounds(&self) -> BoundingVolume {
        // Bounds before unit computation and origin adjustment (relative to 0,0)
        // Requires a way to represent unit-based bounds or use a default context.
        // Placeholder: Assumes Pixel units for simplicity here.
        let width = if let Unit::Pixels(w) = self.width { w } else { 0.0 };
        let height = if let Unit::Pixels(h) = self.height { h } else { 0.0 };
        BoundingVolume::Rect(Rect::new(0.0, 0.0, width, height)) 
    }
    
    fn world_bounds(&self) -> BoundingVolume {
        // This would typically be calculated by the rendering system using
        // resolved_bounds() and the RenderOperation's transform.
        // Placeholder:
        self.local_bounds() 
    }
}

impl Renderable2D for Rectangle {
    fn local_rect(&self) -> Rect {
        // See local_bounds comment. Placeholder:
        if let BoundingVolume::Rect(rect) = self.local_bounds() {
             rect
        } else {
            Rect::default()
        }
    }
    
    fn world_rect(&self) -> Rect {
        // See world_bounds comment. Placeholder:
        if let BoundingVolume::Rect(rect) = self.world_bounds() {
             rect
        } else {
            Rect::default()
        }
    }
}
```

### 4.2 3D Elements

```rust
// In elements/d3/geometry/box.rs
pub struct Box3D {
    pub center: Vec3Unit, // Position defined by center
    pub size: Vec3Unit,   // Dimensions
    pub color: Color,     // Basic styling
    pub origin: Vec3,     // Local origin [0, 1] relative to bounds
}

// Vec3 with units per component
#[derive(Clone)]
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
    
    // Compute actual values based on context
    pub fn resolved_bounds(&self, context: &UnitContext) -> BoundingBox {
        let size = Vec3::new(
            self.size.x.compute(context),
            self.size.y.compute(context),
            self.size.z.compute(context)
        );
        let center = Vec3::new(
            self.center.x.compute(context),
            self.center.y.compute(context),
            self.center.z.compute(context)
        );
        
        // Adjust center based on origin
        let origin_offset = size * (self.origin - Vec3::splat(0.5));
        let adjusted_center = center - origin_offset;
        
        BoundingBox::from_center_size(adjusted_center, size)
    }
}

impl Renderable for Box3D {
    fn origin(&self) -> Vec3 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin.clamp(Vec3::ZERO, Vec3::ONE);
    }

    fn transform_local(&mut self, _transform: &Transform) {
        // Applied via RenderOperation
    }

    fn local_bounds(&self) -> BoundingVolume {
        // Bounds before unit computation, relative to (0,0,0), size based on units.
        // Placeholder: Assumes Pixel units.
        let size_x = if let Unit::Pixels(sx) = self.size.x { sx } else { 0.0 };
        let size_y = if let Unit::Pixels(sy) = self.size.y { sy } else { 0.0 };
        let size_z = if let Unit::Pixels(sz) = self.size.z { sz } else { 0.0 };
        BoundingVolume::Box(BoundingBox::from_center_size(Vec3::ZERO, Vec3::new(size_x, size_y, size_z)))
    }

    fn world_bounds(&self) -> BoundingVolume {
        // Calculated by renderer. Placeholder:
        self.local_bounds()
    }
}

impl Renderable3D for Box3D {
    fn local_box(&self) -> BoundingBox {
        if let BoundingVolume::Box(b) = self.local_bounds() { b } else { BoundingBox::default() }
    }

    fn world_box(&self) -> BoundingBox {
       if let BoundingVolume::Box(b) = self.world_bounds() { b } else { BoundingBox::default() }
    }
}
```

## 5. Tessellation System

A dedicated system handles the conversion of `Renderable` elements into GPU-ready geometry (vertices and indices).

### 5.1 Tessellable Trait

```rust
// In core/tessellation.rs
use crate::core::units::UnitContext; // Assuming BoundingVolume is also in core

// Forward declare or define BoundingVolume, TessellationError
pub enum BoundingVolume { Rect(Rect), Box(BoundingBox) } // Example
#[derive(Debug)] pub struct TessellationError(String);
impl std::fmt::Display for TessellationError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }

pub trait Tessellable {
    // The vertex type produced by tessellation
    type VertexType;
    
    // The index type (usually u16 or u32)
    type IndexType;
    
    // Tessellate the renderable into geometry
    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError>;
}
```

### 5.2 Tessellation Module

```rust
// In tessellation/mod.rs or core/tessellation.rs
use bitflags::bitflags;
// Assume Rect, BoundingBox are defined elsewhere (e.g., core::math::geometry)
#[derive(Default, Debug, Clone, Copy)] pub struct Rect { pub x: f32, pub y: f32, pub width: f32, pub height: f32 }
impl Rect { pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self { Self { x, y, width, height } } }
#[derive(Default, Debug, Clone, Copy)] pub struct BoundingBox { pub min: Vec3, pub max: Vec3 } 
impl BoundingBox { pub fn from_center_size(center: Vec3, size: Vec3) -> Self { let half_size = size * 0.5; Self { min: center - half_size, max: center + half_size } } }

// In tessellation/options.rs
#[derive(Default)]
pub struct TessellationOptions {
    pub quality: TessellationQuality,
    pub tolerance: f32, // e.g., for path flattening
    pub cull_mode: CullMode,
    pub flags: TessellationFlags,
}

#[derive(Default)]
pub enum TessellationQuality {
    Low,
    #[default] Medium,
    High,
    Custom(u32), // Custom detail level (e.g., subdivision count)
}

#[derive(Default)]
pub enum CullMode {
    #[default] None,
    Back,
    Front,
}

bitflags! {
    #[derive(Default)]
    pub struct TessellationFlags: u32 {
        const GENERATE_NORMALS    = 0x01;
        const GENERATE_UVS        = 0x02;
        const GENERATE_TANGENTS   = 0x04; // Requires normals and UVs
        const OPTIMIZE_VERTICES = 0x08; // e.g., weld vertices
        const OPTIMIZE_INDICES  = 0x10; // e.g., optimize for vertex cache
    }
}

// In tessellation/result.rs
pub struct TessellationResult<V, I> {
    pub vertices: Vec<V>,
    pub indices: Vec<I>,
    pub bounds: BoundingVolume, // Bounds of the generated geometry
    pub statistics: TessellationStatistics,
}

#[derive(Default)]
pub struct TessellationStatistics {
    pub vertex_count: usize,
    pub index_count: usize,
    pub triangle_count: usize,
    pub tessellation_time_ms: f32,
}

// Common vertex types (can be defined here or in a dedicated vertex module)
pub mod vertex {
    // Basic 2D vertex (Position only)
    pub type Vertex2D = [f32; 2]; 
    // Basic 3D vertex (Position only)
    pub type Vertex3D = [f32; 3];
    // 3D vertex with position and normal
    pub type VertexWithNormal = ([f32; 3], [f32; 3]);
    // 3D vertex with position and UV coordinates
    pub type VertexWithUV = ([f32; 3], [f32; 2]);
    // Full 3D vertex (Pos, Normal, UV, Tangent) - Tangent might be [f32; 3] or [f32; 4]
    pub type VertexFull = ([f32; 3], [f32; 3], [f32; 2], [f32; 4]); 
}
```

### 5.3 Implementation Example

```rust
// In elements/d2/geometry/rectangle.rs
use crate::core::tessellation::{Tessellable, TessellationOptions, TessellationResult, TessellationError, vertex, BoundingVolume, TessellationStatistics};
use crate::core::units::UnitContext;

impl Tessellable for Rectangle {
    type VertexType = vertex::VertexWithUV; // Position (3D for consistency) + UV
    type IndexType = u16;
    
    fn tessellate(&self, _options: &TessellationOptions, context: &UnitContext) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError> {
        // Get resolved bounds using the unit context
        let rect = self.resolved_bounds(context);
        
        let start_time = std::time::Instant::now(); // Basic timing

        // Create vertices (position [x,y,z], uv [u,v])
        // Assuming Z=0 for 2D elements in a 3D world
        let vertices = vec![
            // Top-left
            ([rect.x, rect.y, 0.0], [0.0, 0.0]), 
            // Top-right
            ([rect.x + rect.width, rect.y, 0.0], [1.0, 0.0]),
            // Bottom-right
            ([rect.x + rect.width, rect.y + rect.height, 0.0], [1.0, 1.0]), 
            // Bottom-left
            ([rect.x, rect.y + rect.height, 0.0], [0.0, 1.0]), 
        ];
        
        // Create indices for two triangles (quad)
        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
        
        let statistics = TessellationStatistics {
            vertex_count: vertices.len(),
            index_count: indices.len(),
            triangle_count: indices.len() / 3,
            tessellation_time_ms: start_time.elapsed().as_secs_f32() * 1000.0,
        };

        // Return the result
        Ok(TessellationResult {
            vertices,
            indices,
            bounds: BoundingVolume::Rect(rect), // Use the computed bounds
            statistics,
        })
    }
}

// In elements/d3/geometry/box.rs
use crate::core::tessellation::{Tessellable, TessellationOptions, TessellationResult, TessellationError, vertex, BoundingVolume, TessellationStatistics, TessellationFlags};
use crate::core::units::UnitContext;

impl Tessellable for Box3D {
    type VertexType = vertex::VertexWithNormal; // Position + Normal
    type IndexType = u16;
    
    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError> {
        let bounds = self.resolved_bounds(context);
        let min = bounds.min;
        let max = bounds.max;
        
        let start_time = std::time::Instant::now();

        // Define the 8 vertices of the box
        let v = [
            [min.x, min.y, min.z], [max.x, min.y, min.z], [max.x, max.y, min.z], [min.x, max.y, min.z], // Bottom face
            [min.x, min.y, max.z], [max.x, min.y, max.z], [max.x, max.y, max.z], [min.x, max.y, max.z], // Top face
        ];

        // Define normals for each face
        let n = [
            [ 0.0,  0.0, -1.0], // Front
            [ 0.0,  0.0,  1.0], // Back
            [-1.0,  0.0,  0.0], // Left
            [ 1.0,  0.0,  0.0], // Right
            [ 0.0, -1.0,  0.0], // Bottom
            [ 0.0,  1.0,  0.0], // Top
        ];

        let mut vertices: Vec<Self::VertexType> = Vec::with_capacity(24); // 6 faces * 4 vertices
        let mut indices: Vec<Self::IndexType> = Vec::with_capacity(36); // 6 faces * 2 triangles * 3 indices

        // Helper to add a quad
        let mut add_quad = |i0: usize, i1: usize, i2: usize, i3: usize, normal_idx: usize| {
            let base_index = vertices.len() as u16;
            let normal = if options.flags.contains(TessellationFlags::GENERATE_NORMALS) { n[normal_idx] } else { [0.0, 0.0, 0.0] };
            vertices.push((v[i0], normal));
            vertices.push((v[i1], normal));
            vertices.push((v[i2], normal));
            vertices.push((v[i3], normal));
            indices.extend_from_slice(&[base_index, base_index + 1, base_index + 2, base_index, base_index + 2, base_index + 3]);
        };

        // Generate faces
        add_quad(0, 3, 2, 1, 0); // Front face (Z-)
        add_quad(4, 5, 6, 7, 1); // Back face (Z+)
        add_quad(0, 4, 7, 3, 2); // Left face (X-)
        add_quad(1, 2, 6, 5, 3); // Right face (X+)
        add_quad(0, 1, 5, 4, 4); // Bottom face (Y-)
        add_quad(3, 7, 6, 2, 5); // Top face (Y+)
        
        // TODO: Add UVs if GENERATE_UVS flag is set
        // TODO: Add Tangents if GENERATE_TANGENTS flag is set
        // TODO: Apply vertex/index optimizations if flags are set

        let statistics = TessellationStatistics {
            vertex_count: vertices.len(),
            index_count: indices.len(),
            triangle_count: indices.len() / 3,
            tessellation_time_ms: start_time.elapsed().as_secs_f32() * 1000.0,
        };

        Ok(TessellationResult {
            vertices,
            indices,
            bounds: BoundingVolume::Box(bounds),
            statistics,
        })
    }
}
```

## 6. Render Operation System

The core mechanism for defining a render job, combining a `Renderable` with styles, effects, transforms, and clipping.

### 6.1 RenderOperation Structure

```rust
// In core/render_operation.rs
use crate::core::renderable::Renderable;
use crate::core::style::Style; // Assuming Style trait exists
use crate::core::effect::Effect; // Assuming Effect trait exists
use crate::core::transform::Transform; // Assuming Transform exists
use crate::core::tessellation::{Tessellable, TessellationOptions, TessellationResult, TessellationError}; // Import Tessellable traits
// Assuming Rect, Renderer, RenderError, GeometryData are defined
// pub trait Renderer { ... fn tessellation_options(&self) -> TessellationOptions; ... }
// pub struct GeometryData { ... }

pub struct RenderOperation<R: Renderable> {
    pub renderable: R,
    pub styles: Vec<Box<dyn Style>>,
    pub effects: Vec<Box<dyn Effect>>,
    pub transform: Transform, // World transform for this operation
    pub clip: Option<Rect>,   // Clipping rectangle
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
    
    // Builder methods
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
    
    // Executes the rendering steps for this operation
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // 1. Setup State (Transform, Clip)
        renderer.push_state(); // Save current renderer state
        renderer.set_transform(&self.transform);
        if let Some(clip) = self.clip {
            renderer.set_clip(clip);
        }
        
        // 2. Apply Pre-Effects
        for effect in &self.effects {
            // Effects might need geometry or bounds, pass renderable
            effect.apply_pre(renderer, &self.renderable)?; 
        }
        
        // 3. Generate Geometry (Tessellation or Fallback)
        // Attempt to get the renderable as a Tessellable trait object
        // This requires R to potentially implement Tessellable.
        // A runtime check or compile-time bound is needed.
        // Using a helper method `try_tessellate` for demonstration.
        
        let geometry_result = self.try_tessellate(renderer);
            
        let geometry = match geometry_result {
            Ok(tess_result) => {
                // Convert TessellationResult into GeometryData the renderer understands
                renderer.upload_geometry(tess_result)? 
            }
            Err(RenderError::UnsupportedOperation(_)) => {
                // Fallback if not Tessellable or tessellation fails gracefully
                self.generate_geometry_fallback(renderer)?
            }
            Err(e) => {
                // Handle non-recoverable tessellation errors
                renderer.pop_state()?; // Ensure state is popped on error
                return Err(e);
            }
        };

        // 4. Apply Styles
        for style in &self.styles {
            // Styles operate on the generated geometry
            style.apply(renderer, &geometry)?; 
        }
        
        // 5. Apply Post-Effects
        for effect in &self.effects {
            // Post-effects operate on the styled geometry
            effect.apply_post(renderer, &geometry)?; 
        }
        
        // 6. Restore State
        renderer.pop_state()?;
        
        Ok(())
    }

    // Helper to attempt tessellation if R implements Tessellable
    // This requires careful handling of trait objects and potentially Any.
    // A cleaner approach might involve dedicated operation types or trait bounds.
    fn try_tessellate<V, I>(
        &self, 
        renderer: &dyn Renderer
    ) -> Result<TessellationResult<V, I>, RenderError> 
    where 
        R: Tessellable<VertexType = V, IndexType = I> + 'static // Bound R to Tessellable
    {
        let options = renderer.tessellation_options();
        let context = renderer.unit_context(); // Renderer needs to provide UnitContext
        
        // Directly call tessellate since R is bound
        self.renderable.tessellate(&options, &context)
            .map_err(|e| RenderError::TessellationFailed(e.to_string()))
    }

    // Fallback geometry generation (if R is not Tessellable)
    fn generate_geometry_fallback(
        &self, 
        _renderer: &dyn Renderer
    ) -> Result<GeometryData, RenderError> {
        // If R doesn't implement Tessellable, how is geometry generated?
        // This might involve older methods or specific logic per Renderable type.
        // For this blueprint, we assume Tessellable is the primary path.
        Err(RenderError::UnsupportedOperation(
            "Renderable does not implement Tessellable and no fallback exists".into()
        ))
    }
}

// --- Dummy trait/struct definitions for context ---
pub trait Style { 
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
    fn as_any(&self) -> &dyn std::any::Any; // Needed for batching example later
    fn is_batchable(&self) -> bool; // Needed for batching example later
} 
pub trait Effect { 
    fn apply_pre(&self, renderer: &mut dyn Renderer, renderable: &dyn std::any::Any) -> Result<(), RenderError>; // Pass Any for flexibility
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
}
pub mod transform { #[derive(Default, Clone, Copy)] pub struct Transform; impl Transform { pub fn identity() -> Self { Self{} } pub fn to_mat4_array(&self) -> [f32; 16] { [0f32; 16] } } }
use transform::Transform;
// Redefine RenderError to include TessellationFailed
#[derive(Debug)] pub enum RenderError { TessellationFailed(String), GpuError(String), InvalidState(String), StackUnderflow(String), UnsupportedOperation(String), ResourceCreationFailed(String), ResourceUpdateFailed(String), InvalidResourceHandle(String), ShaderCompilationFailed(String), TextLayoutError(String), PathTessellationError(String), Other(String)}
pub struct GeometryData { pub vertices: Vec<u8>, pub indices: Vec<u8>, pub handle: GeometryHandle }
pub trait Renderer {
    fn push_state(&mut self);
    fn pop_state(&mut self) -> Result<(), RenderError>;
    fn set_transform(&mut self, transform: &Transform);
    fn set_clip(&mut self, rect: Rect);
    fn tessellation_options(&self) -> TessellationOptions;
    fn unit_context(&self) -> UnitContext; // Provide context for unit computation
    fn upload_geometry<V, I>(&mut self, result: TessellationResult<V, I>) -> Result<GeometryData, RenderError>;
    // Other methods needed by Style/Effect...
    fn fill_geometry(&mut self, geometry: &GeometryData, paint: &Paint, rule: FillRule) -> Result<(), RenderError>;
    fn stroke_geometry(&mut self, geometry: &GeometryData, paint: &Paint, width: f32, join: LineJoin, cap: LineCap, dash: Option<&DashPattern>) -> Result<(), RenderError>;
    fn context(&self) -> &RenderContext; // Existing method assumed
    fn set_material(&mut self, mat: &Material) -> Result<(), RenderError>; // Existing method assumed
    fn set_camera(&mut self, cam: CameraHandle) -> Result<(), RenderError>; // Existing method assumed
    fn draw_geometry(&mut self, geo: GeometryData) -> Result<(), RenderError>; // Existing method assumed
    fn translate(&mut self, x: f32, y: f32); // Existing method assumed
    fn set_blur(&mut self, radius: f32); // Existing method assumed
    fn set_instance_data(&mut self, data: &[InstanceData]) -> Result<(), RenderError>; // For batching
    fn fill_geometry_instanced(&mut self, geo: &GeometryData, paint: &Paint, rule: FillRule, count: usize) -> Result<(), RenderError>; // For batching
}
// --- Dummy types assumed by original code ---
#[derive(Default, Clone)] pub struct Paint; impl Paint { pub fn Solid(_:Color) -> Self { Self{} } pub fn LinearGradient(_:LinearGradient) -> Self { Self{} } }
pub enum FillRule { NonZero }
pub enum LineJoin { Miter }
pub enum LineCap { Butt }
pub struct DashPattern;
pub struct Vec2{ pub x: f32, pub y: f32 } impl Vec2 { pub fn new(x:f32, y:f32)->Self{Self{x,y}} pub const ZERO: Vec2 = Vec2{x:0.0, y:0.0}; pub const ONE: Vec2 = Vec2{x:1.0, y:1.0}; pub fn clamp(self, min: Vec2, max: Vec2) -> Self { Self { x: self.x.clamp(min.x, max.x), y: self.y.clamp(min.y, max.y) } } }
pub struct Vec3{ pub x: f32, pub y: f32, pub z: f32 } impl Vec3 { pub fn new(x:f32, y:f32, z:f32)->Self{Self{x,y,z}} pub fn splat(v:f32)->Self{Self{x:v,y:v,z:v}} pub const ZERO: Vec3 = Vec3{x:0.0, y:0.0, z:0.0}; pub const ONE: Vec3 = Vec3{x:1.0, y:1.0, z:1.0}; pub fn clamp(self, min: Vec3, max: Vec3) -> Self { Self { x: self.x.clamp(min.x, max.x), y: self.y.clamp(min.y, max.y), z: self.z.clamp(min.z, max.z) } } }
impl std::ops::Mul<f32> for Vec3 { type Output = Self; fn mul(self, rhs: f32) -> Self { Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs } } }
impl std::ops::Sub<Vec3> for Vec3 { type Output = Self; fn sub(self, rhs: Vec3) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } } }
impl std::ops::Mul<Vec3> for Vec3 { type Output = Self; fn mul(self, rhs: Vec3) -> Self { Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z } } }
#[derive(Default)] pub struct RenderContext;
#[derive(Default)] pub struct Material;
#[derive(Clone, Copy)] pub struct CameraHandle;
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 } impl Color { pub fn to_rgba_array(&self) -> [f32; 4] { [self.r, self.g, self.b, self.a] }}
#[derive(Clone)] pub struct LinearGradient;
```

This `RenderOperation` replaces the previous composable command system (`Draw2D`, `Draw3D`), integrating the `Renderable` trait and leveraging the `Tessellable` trait for geometry generation.

### 6.2 Core Color System

The core color system remains essential for defining colors used in styles and effects.

```rust
/// Core color type for the rendering system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 - 1.0)
    pub r: f32,
    /// Green component (0.0 - 1.0)
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    pub b: f32,
    /// Alpha component (0.0 - 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color with RGBA components
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r: r.clamp(0.0, 1.0), g: g.clamp(0.0, 1.0), b: b.clamp(0.0, 1.0), a: a.clamp(0.0, 1.0) }
    }
    
    /// Create a new color with RGB components (alpha=1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }
    
    /// Create a new color from 8-bit RGBA values
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    /// Create a color from a hexadecimal value (e.g., 0xRRGGBB or 0xAARRGGBB)
    pub fn from_hex(hex: u32) -> Self {
        let (r, g, b, a) = if hex > 0xFFFFFF {
            // Assume AARRGGBB
            let a = ((hex >> 24) & 0xFF) as u8;
            let r = ((hex >> 16) & 0xFF) as u8;
            let g = ((hex >> 8) & 0xFF) as u8;
            let b = (hex & 0xFF) as u8;
            (r, g, b, a)
        } else {
            // Assume RRGGBB, alpha = 255
            let r = ((hex >> 16) & 0xFF) as u8;
            let g = ((hex >> 8) & 0xFF) as u8;
            let b = (hex & 0xFF) as u8;
            (r, g, b, 255)
        };
        
        Self::from_rgba8(r, g, b, a)
    }
    
    /// Multiply this color by another (component-wise)
    pub fn multiply(&self, other: &Color) -> Self {
        Self::rgba(self.r * other.r, self.g * other.g, self.b * other.b, self.a * other.a)
    }
    
    /// Linear interpolation between colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::rgba(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }
    
    /// Apply premultiplied alpha
    pub fn premultiply(&self) -> Self {
        Self::rgba(self.r * self.a, self.g * self.a, self.b * self.a, self.a)
    }
}

/// Common color constants
pub mod color_constants {
    use super::Color;
    
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    // Add more constants as needed (e.g., gray tones)
}

/// Linear gradient definition
#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub start_point: [f32; 2], // Or Vec2
    pub end_point: [f32; 2],   // Or Vec2
    pub stops: Vec<GradientStop>,
}

/// A color stop in a gradient (position 0.0 to 1.0)
#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

impl LinearGradient {
    /// Create a new linear gradient from start to end with two colors
    pub fn new(start: [f32; 2], end: [f32; 2], start_color: Color, end_color: Color) -> Self {
        Self {
            start_point: start,
            end_point: end,
            stops: vec![
                GradientStop { position: 0.0, color: start_color },
                GradientStop { position: 1.0, color: end_color },
            ],
        }
    }
    
    /// Add a color stop to the gradient, maintaining sorted order
    pub fn add_stop(&mut self, position: f32, color: Color) -> &mut Self {
        let pos = position.clamp(0.0, 1.0);
        // Find insertion point to keep stops sorted by position
        let idx = self.stops.binary_search_by(|s| s.position.partial_cmp(&pos).unwrap())
            .unwrap_or_else(|e| e);
        
        self.stops.insert(idx, GradientStop { position: pos, color });
        self
    }
}

// RadialGradient definition could be added similarly...
```

## 7. Resource Layer

The resource layer manages the creation, tracking, and lifecycle of rendering resources. The core concepts remain similar, but geometry management now relies more heavily on the tessellation system.

### 7.1 Resource Types

```rust
// Core resource types with type safety (Handles are typically lightweight IDs)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GeometryHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FontHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MaterialHandle(pub(crate) ResourceId);

// Common resource ID type (e.g., using a generational index or simple counter)
type ResourceId = u64; 
```

### 7.2 Resource Cache

```rust
// Resource cache for efficient resource management
use std::collections::HashMap;
// Assuming ResourcePool, BufferHandle, TextureId are defined by the GPU backend/abstraction
type BufferHandle = u64; // Placeholder
type TextureId = u64; // Placeholder
type ResourcePool<T> = HashMap<ResourceId, T>; // Example implementation

pub struct ResourceCache {
    geometries: ResourcePool<GeometryResource>, // Stores CPU-side geometry info
    textures: ResourcePool<TextureResource>,   // Stores CPU-side texture info
    fonts: ResourcePool<FontResource>,         // Stores CPU-side font info
    materials: ResourcePool<MaterialResource>, // Stores CPU-side material info
    
    // Mapping from our handles to GPU backend handles
    geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>, // Vertex + Index buffer handles
    texture_handles: HashMap<TextureHandle, TextureId>, // Backend texture ID
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            geometries: HashMap::new(),
            textures: HashMap::new(),
            fonts: HashMap::new(),
            materials: HashMap::new(),
            geometry_buffers: HashMap::new(),
            texture_handles: HashMap::new(),
        }
    }
    
    // Creates or updates geometry resource (CPU side)
    pub fn manage_geometry(&mut self, desc: GeometryDesc) -> Result<GeometryHandle, RenderError> {
        // Generate a unique ID or use a pool allocator
        let handle = GeometryHandle(rand::random()); // Example ID generation
        
        let resource = GeometryResource::new(desc);
        self.geometries.insert(handle.0, resource);
        
        // GPU upload happens separately, often managed by the backend/renderer
        // when the geometry is first needed for drawing.
        
        Ok(handle)
    }
    
    // Create a texture resource
    pub fn manage_texture(&mut self, desc: TextureDesc) -> Result<TextureHandle, RenderError> {
        let handle = TextureHandle(rand::random());
        let resource = TextureResource::new(desc); // Assume TextureResource exists
        self.textures.insert(handle.0, resource);
        // GPU upload managed separately
        Ok(handle)
    }
    
    // Get the CPU-side geometry resource description
    pub fn get_geometry_desc(&self, handle: GeometryHandle) -> Option<&GeometryDesc> {
        self.geometries.get(&handle.0).map(|res| &res.desc)
    }
    
    // Store/Retrieve the mapping to GPU buffers for a given geometry handle
    pub fn map_gpu_geometry(&mut self, handle: GeometryHandle, vertex_buffer: BufferHandle, index_buffer: BufferHandle) {
        self.geometry_buffers.insert(handle, (vertex_buffer, index_buffer));
    }
    
    pub fn get_gpu_geometry_buffers(&self, handle: GeometryHandle) -> Option<(BufferHandle, BufferHandle)> {
        self.geometry_buffers.get(&handle).copied()
    }
    
    // Similar methods for textures, fonts, materials...
}

// --- Dummy types assumed ---
#[derive(Clone)] pub struct GeometryResource { desc: GeometryDesc, dirty: bool }
impl GeometryResource { pub fn new(desc: GeometryDesc) -> Self { Self { desc, dirty: true } } }
#[derive(Clone)] pub struct TextureResource { desc: TextureDesc }
impl TextureResource { pub fn new(desc: TextureDesc) -> Self { Self { desc } } }
#[derive(Clone)] pub struct FontResource;
#[derive(Clone)] pub struct MaterialResource;
#[derive(Clone)] pub struct TextureDesc;
```

### 7.3 Geometry Management

Geometry is primarily generated via the `Tessellable` trait and then managed by the `ResourceCache`.

```rust
// Geometry types and management
// Defines how geometry data might change over time, hints for GPU buffer usage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryUsage {
    Static,   // Rarely changes, optimal for static GPU buffers
    Dynamic,  // Changes occasionally, use dynamic GPU buffers with updates
    Stream,   // Changes every frame, use streaming buffers
}

// Description of geometry data on the CPU side.
// Generated by tessellation.
#[derive(Clone)]
pub struct GeometryDesc {
    // Generic vertex data - could be Vec<V> where V is the VertexType from Tessellable
    pub vertices: Vec<u8>, // Using raw bytes for simplicity here
    pub vertex_stride: usize,
    
    // Generic index data - could be Vec<I> where I is IndexType
    pub indices: Vec<u8>, // Using raw bytes
    pub index_format: IndexFormat, // e.g., u16 or u32
    
    pub usage: GeometryUsage,
    pub bounds: BoundingVolume, // Store bounds calculated during tessellation
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

// Represents the geometry resource tracked by the cache.
#[derive(Clone)]
pub struct GeometryResource {
    pub desc: GeometryDesc,
    // GPU buffer handles are stored separately in the cache mapping.
    // Dirty flag indicates if CPU data needs re-uploading to GPU.
    pub dirty: bool, 
}

impl GeometryResource {
    pub fn new(desc: GeometryDesc) -> Self {
        Self { desc, dirty: true } // Mark as dirty initially
    }
    
    // Update vertex data on the CPU side
    pub fn update_vertices(&mut self, vertices: Vec<u8>, vertex_stride: usize) {
        // Basic update, might need more sophisticated logic for dynamic/stream usage
        self.desc.vertices = vertices;
        self.desc.vertex_stride = vertex_stride;
        self.dirty = true; 
    }
    
    // Mark as clean after uploading to GPU
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}
```

### 7.4 Path Generation

Path generation remains important for vector graphics. `Path` itself should implement `Renderable` and `Tessellable`.

```rust
// Path representation and building
// (Assuming Point is defined, e.g., Vec2)

pub struct PathBuilder {
    commands: Vec<PathCommand>,
    current_point: Option<Point>,
}

#[derive(Clone, Debug)]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadraticCurveTo { control: Point, end: Point },
    CubicCurveTo { control1: Point, control2: Point, end: Point }, // Renamed from BezierCurveTo
    ArcTo { center: Point, radius: f32, start_angle: f32, end_angle: f32 }, // Consider using svg arc representation (radii, rotation, flags)
    Close,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self { commands: Vec::new(), current_point: None }
    }
    
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        let point = Point { x, y };
        self.commands.push(PathCommand::MoveTo(point));
        self.current_point = Some(point);
        self
    }
    
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        let point = Point { x, y };
        if self.current_point.is_some() {
            self.commands.push(PathCommand::LineTo(point));
            self.current_point = Some(point);
        } else {
            // Maybe treat as MoveTo if no current point? Or error?
            self.move_to(x, y); 
        }
        self
    }
    
    // Other path building methods (quad_to, cubic_to, arc_to, close)...
    
    pub fn close(&mut self) -> &mut Self {
        if !self.commands.is_empty() {
             // Find the start of the current subpath
             let start_point = self.commands.iter().rev().find_map(|cmd| match cmd {
                 PathCommand::MoveTo(p) => Some(*p),
                 _ => None,
             });
             
             if let Some(start) = start_point {
                 if self.current_point != Some(start) {
                    self.commands.push(PathCommand::LineTo(start)); // Implicit LineTo if closing needed
                 }
                 self.commands.push(PathCommand::Close);
                 self.current_point = Some(start); // Point after Close is the subpath start
             }
        }
        self
    }
    
    pub fn build(&self) -> Path {
        // Calculate bounds during build process
        let bounds = self.calculate_bounds();
        Path {
            commands: self.commands.clone(),
            bounds,
            // Initialize origin if paths have origins
        }
    }
    
    fn calculate_bounds(&self) -> Rect {
        // Iterate through commands and update min/max points
        // Need to handle curves accurately (bounding box of control points is approximation)
        // Placeholder implementation:
        let mut min_x = f32::MAX; let mut min_y = f32::MAX;
        let mut max_x = f32::MIN; let mut max_y = f32::MIN;
        
        for cmd in &self.commands {
             match cmd {
                 PathCommand::MoveTo(p) | PathCommand::LineTo(p) => {
                     min_x = min_x.min(p.x); max_x = max_x.max(p.x);
                     min_y = min_y.min(p.y); max_y = max_y.max(p.y);
                 }
                 // TODO: Handle curve bounds properly
                 _ => {} 
             }
        }
        
        if min_x == f32::MAX { Rect::default() } 
        else { Rect::new(min_x, min_y, max_x - min_x, max_y - min_y) }
    }
}

#[derive(Clone)]
pub struct Path {
    commands: Vec<PathCommand>,
    bounds: Rect, // Pre-calculated bounds
    // origin: Vec2, // If paths need an origin
}

// Path should implement Renderable and Tessellable
impl Renderable for Path {
    fn origin(&self) -> Vec3 { /* Return path origin */ Vec3::ZERO }
    fn set_origin(&mut self, _origin: Vec3) { /* Set path origin */ }
    fn transform_local(&mut self, _transform: &Transform) { /* Apply internal transform */ }
    fn local_bounds(&self) -> BoundingVolume { BoundingVolume::Rect(self.bounds) }
    fn world_bounds(&self) -> BoundingVolume { /* Calculated by renderer */ self.local_bounds() }
    // ... Renderable2D methods ...
}

impl Tessellable for Path {
    type VertexType = vertex::Vertex2D; // Example: Simple 2D vertex for paths
    type IndexType = u32; // Use u32 for potentially large paths

    fn tessellate(&self, options: &TessellationOptions, context: &UnitContext) -> Result<TessellationResult<Self::VertexType, Self::IndexType>, TessellationError> {
        // Use a path tessellation library (e.g., lyon) based on self.commands
        // Use options.tolerance, options.quality etc.
        // Generate vertices and indices for fill or stroke based on how it's used (or options?)
        // This logic is complex and depends on the chosen tessellator.
        
        // Placeholder:
        Err(TessellationError("Path tessellation not implemented".into()))
    }
}

// --- Dummy Point type ---
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Point { pub x: f32, pub y: f32 }
```

## 8. Pipeline Layer

The pipeline layer manages the specialized rendering pipelines. The core concepts remain, but implementations will adapt to use `RenderOperation` execution and geometry from tessellation.

### 8.1 Vector Rendering Pipeline

Handles rendering of 2D geometry (potentially tessellated paths, shapes).

```rust
// Vector rendering pipeline (conceptual)
use std::sync::Arc; // Assuming Arc usage based on original GpuInterface
// Assume PipelineHandle, PipelineDesc, ShaderModule, GpuDevice, CommandBuffer etc. are defined
type PipelineHandle = u64; // Placeholder
struct PipelineDesc; impl PipelineDesc { fn new() -> Self { Self } fn vertex_shader(self, _: ShaderModule) -> Self { self } fn fragment_shader(self, _: ShaderModule) -> Self { self } fn vertex_layout(self, _: VertexLayout) -> Self { self } fn render_target_format(self, _: TextureFormat) -> Self { self } }
struct ShaderModule; mod shaders { pub mod vector { pub const FILL_VERTEX: ShaderModule = ShaderModule; pub const FILL_FRAGMENT: ShaderModule = ShaderModule; pub const STROKE_VERTEX: ShaderModule = ShaderModule; pub const STROKE_FRAGMENT: ShaderModule = ShaderModule; } }
struct VertexLayout; mod layouts { pub const VECTOR_VERTEX: VertexLayout = VertexLayout; }
enum TextureFormat { RGBA8_UNORM }
struct GpuDevice; impl GpuDevice { fn create_pipeline(&self, _: PipelineDesc) -> Result<PipelineHandle, String> { Ok(0) } }
struct CommandBuffer;
struct FillStyle; struct StrokeStyle;

pub struct VectorPipeline {
    fill_pipeline: PipelineHandle,
    stroke_pipeline: PipelineHandle,
    // Potentially pipelines for different fill/stroke rules or anti-aliasing methods
}

impl VectorPipeline {
    pub fn new(device: Arc<GpuDevice>) -> Result<Self, RenderError> {
        // Create fill pipeline using appropriate shaders and vertex layouts
        let fill_pipeline = device.create_pipeline(PipelineDesc::new()
            // .vertex_shader(shaders::vector::FILL_VERTEX)
            // .fragment_shader(shaders::vector::FILL_FRAGMENT)
            // .vertex_layout(layouts::VECTOR_VERTEX)
            // .render_target_format(TextureFormat::RGBA8_UNORM)
            // Configure blend states, stencil operations etc.
            ).map_err(|e| RenderError::ResourceCreationFailed(format!("Fill pipeline: {}", e)))?;
            
        // Create stroke pipeline similarly...
        let stroke_pipeline = device.create_pipeline(PipelineDesc::new()).map_err(|e| RenderError::ResourceCreationFailed(format!("Stroke pipeline: {}", e)))?;
        
        Ok(Self { fill_pipeline, stroke_pipeline })
    }
    
    // Methods to configure and execute draw calls using these pipelines,
    // likely called by the CommandTranslator based on RenderOperation styles.
    // pub fn setup_fill(&self, cmd: &mut CommandBuffer, style: &FillStyle) -> Result<(), RenderError> { ... }
    // pub fn setup_stroke(&self, cmd: &mut CommandBuffer, style: &StrokeStyle) -> Result<(), RenderError> { ... }
}
```

### 8.2 Text Rendering Pipeline

Handles rendering of text, potentially using MSDF or bitmap techniques.

```rust
// Text rendering pipeline (conceptual)
// Assume relevant structs/enums are defined (MSDF_SHADER, TEXT_VERTEX, BlendState, etc.)
mod shaders { pub mod text { pub const MSDF_VERTEX: ShaderModule = ShaderModule; pub const MSDF_FRAGMENT: ShaderModule = ShaderModule; } }
mod layouts { pub const TEXT_VERTEX: VertexLayout = VertexLayout; }
enum BlendState { ALPHA_BLEND }
struct FontResource; struct TextLayout;

pub struct TextPipeline {
    msdf_pipeline: PipelineHandle,  // Multi-channel signed distance field
    bitmap_pipeline: PipelineHandle, // Basic bitmap fonts
}

impl TextPipeline {
    pub fn new(device: Arc<GpuDevice>) -> Result<Self, RenderError> {
        let msdf_pipeline = device.create_pipeline(PipelineDesc::new()
            // .vertex_shader(shaders::text::MSDF_VERTEX)
            // .fragment_shader(shaders::text::MSDF_FRAGMENT)
            // .vertex_layout(layouts::TEXT_VERTEX)
            // .blend_state(BlendState::ALPHA_BLEND)
            // .render_target_format(TextureFormat::RGBA8_UNORM)
            ).map_err(|e| RenderError::ResourceCreationFailed(format!("MSDF pipeline: {}", e)))?;
            
        let bitmap_pipeline = device.create_pipeline(PipelineDesc::new()).map_err(|e| RenderError::ResourceCreationFailed(format!("Bitmap pipeline: {}", e)))?;
        
        Ok(Self { msdf_pipeline, bitmap_pipeline })
    }
    
    // Methods to render text glyphs or layouts using these pipelines.
    // pub fn render_glyphs(&self, cmd: &mut CommandBuffer, font: &FontResource, glyphs: &[GlyphInfo], ...) -> Result<(), RenderError> { ... }
    // pub fn render_layout(&self, cmd: &mut CommandBuffer, layout: &TextLayout, ...) -> Result<(), RenderError> { ... }
}
```

### 8.3 3D Rendering Pipeline

Handles rendering of 3D meshes, materials, lighting.

```rust
// 3D rendering pipeline (conceptual)
// Assume relevant structs/enums are defined (PBR_SHADER, MESH_VERTEX, DepthStencilState, etc.)
mod shaders { pub mod three_d { pub const PBR_VERTEX: ShaderModule = ShaderModule; pub const PBR_FRAGMENT: ShaderModule = ShaderModule; } }
mod layouts { pub const MESH_VERTEX: VertexLayout = VertexLayout; }
enum DepthStencilState { DEPTH_TEST_WRITE }
enum TextureFormat3D { RGBA8_UNORM, Depth24PlusStencil8 }
struct MeshResource; struct MaterialResource3D; struct Camera;

pub struct ThreeDPipeline {
    pbr_pipeline: PipelineHandle,  // Physically-based rendering
    shadow_pipeline: PipelineHandle, // For shadow map generation
    skybox_pipeline: PipelineHandle,
    // Other pipelines (e.g., unlit, wireframe)
}

impl ThreeDPipeline {
    pub fn new(device: Arc<GpuDevice>) -> Result<Self, RenderError> {
        let pbr_pipeline = device.create_pipeline(PipelineDesc::new()
            // .vertex_shader(shaders::three_d::PBR_VERTEX)
            // .fragment_shader(shaders::three_d::PBR_FRAGMENT)
            // .vertex_layout(layouts::MESH_VERTEX)
            // .depth_stencil_state(DepthStencilState::DEPTH_TEST_WRITE)
            // .render_target_format(TextureFormat3D::RGBA8_UNORM)
            // .depth_stencil_format(TextureFormat3D::Depth24PlusStencil8)
            ).map_err(|e| RenderError::ResourceCreationFailed(format!("PBR pipeline: {}", e)))?;
            
        let shadow_pipeline = device.create_pipeline(PipelineDesc::new()).map_err(|e| RenderError::ResourceCreationFailed(format!("Shadow pipeline: {}", e)))?;
        let skybox_pipeline = device.create_pipeline(PipelineDesc::new()).map_err(|e| RenderError::ResourceCreationFailed(format!("Skybox pipeline: {}", e)))?;
        
        Ok(Self { pbr_pipeline, shadow_pipeline, skybox_pipeline })
    }
    
    // Methods to render meshes with specific materials/lighting models.
    // pub fn render_pbr_mesh(&self, cmd: &mut CommandBuffer, mesh: &MeshResource, material: &MaterialResource3D, transform: &Transform, camera: &Camera, lights: &[LightInfo]) -> Result<(), RenderError> { ... }
}
```

### 8.4 Effect Pipeline

Handles post-processing effects like blur, bloom, shadows applied to rendered output.

```rust
// Effect pipeline for post-processing (conceptual)
mod shaders { pub mod effects { pub const BLUR_VERTEX: ShaderModule = ShaderModule; pub const BLUR_FRAGMENT: ShaderModule = ShaderModule; } }
mod layouts { pub const QUAD_VERTEX: VertexLayout = VertexLayout; }

pub struct EffectPipeline {
    blur_pipeline: PipelineHandle,
    // Other effect pipelines (bloom, tone mapping, etc.)
}

impl EffectPipeline {
    pub fn new(device: Arc<GpuDevice>) -> Result<Self, RenderError> {
        let blur_pipeline = device.create_pipeline(PipelineDesc::new()
            // .vertex_shader(shaders::effects::BLUR_VERTEX)
            // .fragment_shader(shaders::effects::BLUR_FRAGMENT)
            // .vertex_layout(layouts::QUAD_VERTEX) // Usually renders a fullscreen quad
            // .render_target_format(TextureFormat::RGBA8_UNORM) // Or format of the input texture
            ).map_err(|e| RenderError::ResourceCreationFailed(format!("Blur pipeline: {}", e)))?;
            
        Ok(Self { blur_pipeline })
    }
    
    // Methods to apply effects, often taking textures as input/output.
    // pub fn apply_blur(&self, cmd: &mut CommandBuffer, source_texture: TextureHandle, target_texture: TextureHandle, radius: f32) -> Result<(), RenderError> { ... }
}
```

## 9. Backend Abstraction

The backend abstraction layer interfaces with the GPU crate. Its role remains crucial but internal details (like command translation) adapt to the new operation system.

### 9.1 GPU Interface

Provides a consistent API over the underlying GPU crate (`vectron_gpu`).

```rust
// GPU interface for abstracting the GPU crate (conceptual)
use std::sync::Arc;
// Assume GpuDevice, CommandBuffer, GpuResourceInfo, PipelineKey, BufferDesc, TextureDesc, PipelineDesc etc. are defined by the GPU abstraction layer
struct GpuDevice { /* ... methods to interact with GPU ... */ }
impl GpuDevice { 
    fn get_surface(&self) -> Option<SurfaceInfo> { None }
    fn configure_surface(&self, _: SurfaceInfo, _: u32, _: u32, _: PresentMode, _: TextureFormat) -> Result<(), String> { Ok(()) }
    fn begin_frame(&self) -> Result<CommandBuffer, String> { Ok(CommandBuffer{}) }
    fn end_frame(&self, _: CommandBuffer) -> Result<(), String> { Ok(()) }
    fn create_buffer(&self, _: BufferDesc) -> Result<BufferHandle, String> { Ok(0) }
    fn write_buffer<T: bytemuck::Pod>(&self, _: BufferHandle, _: u64, _: &[T]) -> Result<(), String> { Ok(()) }
    fn create_texture(&self, _: TextureDesc) -> Result<TextureHandle, String> { Ok(TextureHandle(0)) }
    fn create_render_pipeline(&self, _: PipelineDesc) -> Result<PipelineHandle, String> { Ok(0) }
}
struct GpuResourceInfo; struct PipelineKey; impl PipelineKey { fn to_pipeline_desc(&self) -> PipelineDesc { PipelineDesc{} }}
struct SurfaceInfo; impl SurfaceInfo { fn width(&self) -> u32 {0} fn height(&self) -> u32 {0} }
enum PresentMode { Fifo }
struct BufferDesc; impl BufferDesc { fn clone(&self)->Self{Self{}} fn vertex_buffer(_: usize, _: BufferUsage)->Self{Self{}} fn index_buffer(_: usize, _: BufferUsage)->Self{Self{}} }
enum BufferUsage { VERTEX, INDEX }

pub struct GpuInterface {
    device: Arc<GpuDevice>,
    command_buffer: Option<CommandBuffer>, // Current frame's command buffer
    resource_map: HashMap<ResourceId, GpuResourceInfo>, // Track GPU resources associated with our IDs
    pipeline_cache: HashMap<PipelineKey, PipelineHandle>, // Cache compiled GPU pipelines
}

impl GpuInterface {
    pub fn new(device: Arc<GpuDevice>) -> Self {
        Self {
            device,
            command_buffer: None,
            resource_map: HashMap::new(),
            pipeline_cache: HashMap::new(),
        }
    }
    
    pub fn begin_frame(&mut self, width: u32, height: u32) -> Result<(), RenderError> {
        let surface = self.device.get_surface()
            .ok_or_else(|| RenderError::InvalidState("No surface available".into()))?;
            
        // Simplified configuration check
        if surface.width() != width || surface.height() != height {
            self.device.configure_surface(surface, width, height, PresentMode::Fifo, TextureFormat::RGBA8_UNORM /* Example format */)
                .map_err(|e| RenderError::GpuError(format!("Failed to configure surface: {}", e)))?;
        }
        
        let cmd = self.device.begin_frame()
            .map_err(|e| RenderError::GpuError(format!("Failed to begin frame: {}", e)))?;
        self.command_buffer = Some(cmd);
        Ok(())
    }
    
    pub fn end_frame(&mut self) -> Result<(), RenderError> {
        if let Some(cmd) = self.command_buffer.take() {
            self.device.end_frame(cmd)
                .map_err(|e| RenderError::GpuError(format!("Failed to end frame: {}", e)))?;
        } else {
            // Warning or error if end_frame called without begin_frame?
        }
        Ok(())
    }
    
    pub fn create_buffer(&mut self, desc: BufferDesc) -> Result<BufferHandle, RenderError> {
        self.device.create_buffer(desc)
            .map_err(|e| RenderError::ResourceCreationFailed(format!("Buffer: {}", e)))
    }
    
    pub fn write_buffer<T: bytemuck::Pod>(&self, buffer: BufferHandle, offset: u64, data: &[T]) -> Result<(), RenderError> {
        self.device.write_buffer(buffer, offset, data)
            .map_err(|e| RenderError::ResourceUpdateFailed(format!("Buffer write: {}", e)))
    }
    
    pub fn create_texture(&mut self, desc: TextureDesc) -> Result<TextureHandle, RenderError> {
        self.device.create_texture(desc)
            .map_err(|e| RenderError::ResourceCreationFailed(format!("Texture: {}", e)))
    }
    
    pub fn create_pipeline(&mut self, desc: PipelineDesc) -> Result<PipelineHandle, RenderError> {
        self.device.create_render_pipeline(desc)
             .map_err(|e| RenderError::ResourceCreationFailed(format!("Pipeline: {}", e)))
    }
    
    pub fn get_or_create_pipeline(&mut self, key: PipelineKey) -> Result<PipelineHandle, RenderError> {
        if let Some(pipeline) = self.pipeline_cache.get(&key) {
            return Ok(*pipeline);
        }
        
        let pipeline_desc = key.to_pipeline_desc(); // Convert key to description
        let pipeline = self.create_pipeline(pipeline_desc)?;
        self.pipeline_cache.insert(key, pipeline); // Assume key is hashable/eq
        Ok(pipeline)
    }
    
    pub fn current_command_buffer(&mut self) -> Result<&mut CommandBuffer, RenderError> {
        self.command_buffer.as_mut()
            .ok_or_else(|| RenderError::InvalidState("No active command buffer".into()))
    }
    
    pub fn device(&self) -> &Arc<GpuDevice> {
        &self.device
    }
    
    // Add methods for managing resource mapping (resource_map)
}
```

### 9.2 Command Translation

Translates `RenderOperation` execution calls into low-level GPU commands via the `GpuInterface`. This is where the core rendering logic resides.

```rust
// Command translation logic (conceptual)
// This struct/module would implement the `Renderer` trait used by `RenderOperation::execute`.

struct CommandTranslator<'a> {
    gpu_interface: &'a mut GpuInterface,
    resource_cache: &'a ResourceCache, // Read-only access to resource metadata
    // References to pipeline instances (VectorPipeline, TextPipeline, etc.)
    vector_pipeline: &'a VectorPipeline, 
    text_pipeline: &'a TextPipeline,
    // ... other pipelines
    
    // Current operation state being translated
    operation_state: RenderOperationState, 
}

// State needed during the translation of a single RenderOperation
#[derive(Default)]
struct RenderOperationState {
    transform: Transform,
    clip: Option<Rect>,
    // Other states modified by Style/Effect apply methods...
}

impl<'a> CommandTranslator<'a> {
    pub fn new(
        gpu: &'a mut GpuInterface, 
        cache: &'a ResourceCache,
        vec_pipe: &'a VectorPipeline,
        txt_pipe: &'a TextPipeline,
        /* ... */
    ) -> Self {
        Self { 
            gpu_interface: gpu, 
            resource_cache: cache, 
            vector_pipeline: vec_pipe, 
            text_pipeline: txt_pipe, 
            /* ... */
            operation_state: RenderOperationState::default(),
        }
    }

    // Main entry point: Translate a list of operations
    pub fn translate_operations(&mut self, operations: &[RenderOperation<impl Renderable>]) -> Result<(), RenderError> {
        let cmd = self.gpu_interface.current_command_buffer()?;
        
        for op in operations {
             // Reset state for each operation? Or rely on op.execute to manage push/pop?
             // Let's assume op.execute manages state via the Renderer trait implementation below.
             op.execute(self)?; // `self` implements Renderer
        }
        Ok(())
    }
    
    // Helper to get GPU buffers for geometry, uploading if necessary
    fn ensure_geometry_uploaded(&mut self, handle: GeometryHandle) -> Result<(BufferHandle, BufferHandle), RenderError> {
        if let Some(buffers) = self.resource_cache.get_gpu_geometry_buffers(handle) {
             // TODO: Check if dirty and needs re-upload
             if let Some(res) = self.resource_cache.geometries.get_mut(&handle.0) { // Need mut cache access? Or handle dirty state differently
                 if res.dirty {
                     // Re-upload logic...
                     self.upload_geometry_data(handle, &res.desc)?;
                     res.mark_clean(); // Mark as clean after upload
                     // Get newly updated buffer handles if they changed
                     return self.resource_cache.get_gpu_geometry_buffers(handle)
                        .ok_or_else(|| RenderError::InvalidState("GPU buffers not found after upload".into()));
                 }
             }
             return Ok(buffers);
        }
        
        // Not uploaded yet, upload now
        if let Some(desc) = self.resource_cache.get_geometry_desc(handle) {
             let buffers = self.upload_geometry_data(handle, desc)?;
             // Mark clean (needs mut access or different flow)
             if let Some(res) = self.resource_cache.geometries.get_mut(&handle.0) {
                 res.mark_clean();
             }
             return Ok(buffers);
        } else {
             Err(RenderError::InvalidResourceHandle(format!("Geometry handle {:?} not found", handle)))
        }
    }

    fn upload_geometry_data(&mut self, handle: GeometryHandle, desc: &GeometryDesc) -> Result<(BufferHandle, BufferHandle), RenderError> {
        // Create vertex buffer
        let vb_usage = match desc.usage { /* Map GeometryUsage to BufferUsage */ _ => BufferUsage::VERTEX };
        let vb = self.gpu_interface.create_buffer(BufferDesc::vertex_buffer(desc.vertices.len(), vb_usage))?;
        self.gpu_interface.write_buffer(vb, 0, bytemuck::cast_slice(&desc.vertices))?; // Assuming vertices are Pod

        // Create index buffer
        let ib_usage = match desc.usage { /* Map GeometryUsage to BufferUsage */ _ => BufferUsage::INDEX };
        let ib = self.gpu_interface.create_buffer(BufferDesc::index_buffer(desc.indices.len(), ib_usage))?;
        self.gpu_interface.write_buffer(ib, 0, bytemuck::cast_slice(&desc.indices))?; // Assuming indices are Pod

        // Map the GPU buffers in the cache
        self.resource_cache.map_gpu_geometry(handle, vb, ib); // Needs mut cache access? Design decision needed.

        Ok((vb, ib))
    }
}


// Implement the Renderer trait for CommandTranslator to be used by RenderOperation::execute
impl<'a> Renderer for CommandTranslator<'a> {
    fn push_state(&mut self) {
        // Push current operation_state onto an internal stack if needed
        // For simplicity, assume RenderOperation itself doesn't nest states deeply within one execute call.
    }
    
    fn pop_state(&mut self) -> Result<(), RenderError> {
        // Pop from internal stack if implemented
        Ok(())
    }
    
    fn set_transform(&mut self, transform: &Transform) {
        self.operation_state.transform = *transform;
        // TODO: Update GPU uniform buffer or push constants
        let cmd = self.gpu_interface.current_command_buffer()?;
        // cmd.set_uniforms(..., transform);
    }
    
    fn set_clip(&mut self, rect: Rect) {
        self.operation_state.clip = Some(rect);
        let cmd = self.gpu_interface.current_command_buffer()?;
        // cmd.set_scissor(rect.x as u32, ...);
    }
    
    fn tessellation_options(&self) -> TessellationOptions {
        // Return default or configured options
        TessellationOptions::default() 
    }

    fn unit_context(&self) -> UnitContext {
        // Get context from viewport size, DPI etc.
        UnitContext { dpi: 96.0, viewport_width: 800.0, viewport_height: 600.0, parent_width: None, parent_height: None, parent_depth: None } // Example values
    }

    fn upload_geometry<V, I>(&mut self, result: TessellationResult<V, I>) -> Result<GeometryData, RenderError> 
    where V: bytemuck::Pod, I: bytemuck::Pod // Assume Pod for direct casting
    {
        let vertices_bytes = bytemuck::cast_slice(&result.vertices).to_vec();
        let indices_bytes = bytemuck::cast_slice(&result.indices).to_vec();
        
        let index_format = if std::mem::size_of::<I>() == 2 { IndexFormat::Uint16 } else { IndexFormat::Uint32 };

        let desc = GeometryDesc {
            vertices: vertices_bytes,
            vertex_stride: std::mem::size_of::<V>(),
            indices: indices_bytes,
            index_format,
            usage: GeometryUsage::Static, // Default, could be smarter
            bounds: result.bounds,
        };
        
        // Manage the geometry (gets a handle)
        let handle = self.resource_cache.manage_geometry(desc.clone())?; // Needs mut cache access
        
        // GPU upload happens lazily via ensure_geometry_uploaded when drawing
        
        Ok(GeometryData { vertices: desc.vertices, indices: desc.indices, handle }) // Return info needed by styles/effects
    }
    
    // --- Implement drawing methods needed by Styles/Effects ---
    
    fn fill_geometry(&mut self, geometry: &GeometryData, _paint: &Paint, _rule: FillRule) -> Result<(), RenderError> {
        let (vb, ib) = self.ensure_geometry_uploaded(geometry.handle)?;
        let cmd = self.gpu_interface.current_command_buffer()?;
        
        // Set fill pipeline
        cmd.set_pipeline(self.vector_pipeline.fill_pipeline); 
        
        // Bind geometry
        cmd.set_vertex_buffer(0, vb);
        cmd.set_index_buffer(ib);
        
        // Set uniforms/bindings based on paint, rule, operation_state
        // ...
        
        // Draw
        let index_count = geometry.indices.len() / match self.resource_cache.get_geometry_desc(geometry.handle).unwrap().index_format {
             IndexFormat::Uint16 => 2,
             IndexFormat::Uint32 => 4,
        };
        cmd.draw_indexed(index_count as u32, 1, 0, 0, 0);
        
        Ok(())
    }

    fn stroke_geometry(&mut self, geometry: &GeometryData, _paint: &Paint, _width: f32, _join: LineJoin, _cap: LineCap, _dash: Option<&DashPattern>) -> Result<(), RenderError> {
        // Similar to fill: ensure uploaded, set stroke pipeline, bind, set uniforms, draw
        Ok(())
    }

    // ... other Renderer methods ...
    fn context(&self) -> &RenderContext { unimplemented!() }
    fn set_material(&mut self, _mat: &Material) -> Result<(), RenderError> { Ok(()) }
    fn set_camera(&mut self, _cam: CameraHandle) -> Result<(), RenderError> { Ok(()) }
    fn draw_geometry(&mut self, _geo: GeometryData) -> Result<(), RenderError> { Ok(()) }
    fn translate(&mut self, _x: f32, _y: f32) { }
    fn set_blur(&mut self, _radius: f32) { }
    fn set_instance_data(&mut self, _data: &[InstanceData]) -> Result<(), RenderError> { Ok(()) }
    fn fill_geometry_instanced(&mut self, _geo: &GeometryData, _paint: &Paint, _rule: FillRule, _count: usize) -> Result<(), RenderError> { Ok(()) }
}

// --- Dummy CommandBuffer methods ---
struct CommandBuffer { }
impl CommandBuffer { 
    fn set_pipeline(&mut self, _: PipelineHandle) {} 
    fn set_vertex_buffer(&mut self, _: u32, _: BufferHandle) {}
    fn set_index_buffer(&mut self, _: BufferHandle) {}
    fn draw_indexed(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn set_scissor(&mut self, _: u32, _: u32, _: u32, _: u32) {}
}

// --- Dummy bytemuck Pod trait ---
mod bytemuck { pub trait Pod {} impl Pod for u8 {} impl Pod for f32 {} impl<T: Pod, const N: usize> Pod for [T; N] {} impl<T: Pod, U: Pod> Pod for (T, U) {} pub fn cast_slice<T: Pod, U: Pod>(_: &[T]) -> &[U] { &[] } }
```

### 9.3 State Management

Manages graphics state (transforms, clipping, stencils) during rendering.

```rust
// State management for the rendering context (within CommandTranslator or GpuInterface)

pub struct GlobalRenderState { // Renamed from RenderState to avoid conflict
    transform_stack: Vec<Transform>,
    current_transform: Transform,
    scissor_stack: Vec<Option<Rect>>,
    current_scissor: Option<Rect>,
    // Other states: blend mode, depth/stencil settings, etc.
    // Stencil level might be part of depth/stencil state.
}

impl GlobalRenderState {
    pub fn new() -> Self {
        Self {
            transform_stack: Vec::new(),
            current_transform: Transform::identity(),
            scissor_stack: Vec::new(),
            current_scissor: None,
            // Initialize other states
        }
    }
    
    // Push/Pop methods manage the stacks
    pub fn push_transform(&mut self) {
        self.transform_stack.push(self.current_transform);
    }
    
    pub fn pop_transform(&mut self) -> Result<(), RenderError> {
        self.current_transform = self.transform_stack.pop()
            .ok_or_else(|| RenderError::StackUnderflow("Transform stack empty".into()))?;
        Ok(())
    }
    
    // Apply methods update the current state
    pub fn apply_transform(&mut self, transform: Transform) {
        // Typically multiplies with the current transform
        // self.current_transform = self.current_transform * transform; 
        self.current_transform = transform; // Or sets absolute as in RenderOperation
    }

    pub fn apply_scissor(&mut self, scissor: Option<Rect>) {
        self.current_scissor = scissor;
    }

    // apply_to_command_buffer would be called by the CommandTranslator
    // to set the actual GPU state based on current_transform, current_scissor etc.
    pub fn apply_to_gpu(&self, cmd: &mut CommandBuffer) {
        // Set GPU transform (uniforms/push constants) from self.current_transform
        
        if let Some(scissor) = self.current_scissor {
            cmd.set_scissor(
                scissor.x as u32, 
                scissor.y as u32, 
                scissor.width as u32, 
                scissor.height as u32
            );
        } else {
            // Disable scissor or set to full viewport
        }
        
        // Apply other states (blend, depth, stencil)
    }
}
```

### 9.4 Resource Binding

Handles binding GPU resources (buffers, textures, samplers) required by shaders.

```rust
// Resource binding logic (likely part of CommandTranslator or GpuInterface)

// Describes a binding required by a shader/pipeline
pub struct ResourceBindingInfo {
    pub resource_type: ResourceBindingType,
    pub handle: ResourceId, // Handle of the resource (e.g., TextureHandle.0)
    pub binding: u32,
    pub set: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceBindingType {
    UniformBuffer,
    StorageBuffer,
    SampledTexture,
    StorageTexture,
    Sampler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BindingPoint {
    pub set: u32, // Descriptor set / bind group index
    pub binding: u32, // Binding index within the set
}

// Function within CommandTranslator or GpuInterface to apply bindings
fn bind_resources_for_pipeline(
    cmd: &mut CommandBuffer, 
    pipeline: PipelineHandle, 
    bindings: &[ResourceBindingInfo],
    resource_cache: &ResourceCache, // Access to GPU handle mappings
    gpu_interface: &GpuInterface, // Access to underlying GPU commands
) -> Result<(), RenderError> 
{
    // 1. Set the pipeline state object
    cmd.set_pipeline(pipeline);
    
    // 2. Create/update descriptor sets (bind groups) based on bindings.
    // This is highly backend-specific (Vulkan, wgpu, etc.).
    // It might involve pooling descriptor sets or updating them dynamically.
    
    // Placeholder logic:
    let mut descriptor_sets = HashMap::<u32, Vec<GpuBinding>>::new();
    
    for binding_info in bindings {
        let gpu_binding = match binding_info.resource_type {
            ResourceBindingType::UniformBuffer | ResourceBindingType::StorageBuffer => {
                 // Get the GPU BufferHandle for binding_info.handle
                 let buffer_handle = resource_cache.get_gpu_buffer(binding_info.handle)?; // Need get_gpu_buffer method
                 GpuBinding::Buffer(buffer_handle)
            }
            ResourceBindingType::SampledTexture => {
                // Get the GPU TextureView for binding_info.handle
                let texture_view = resource_cache.get_gpu_texture_view(binding_info.handle)?; // Need get_gpu_texture_view
                 GpuBinding::Texture(texture_view)
            }
             ResourceBindingType::Sampler => {
                 // Get the GPU Sampler for binding_info.handle
                 let sampler = resource_cache.get_gpu_sampler(binding_info.handle)?; // Need get_gpu_sampler
                 GpuBinding::Sampler(sampler)
             }
            // Handle StorageTexture...
            _ => todo!(),
        };
        
        descriptor_sets.entry(binding_info.binding_point.set)
            .or_default()
            .push(gpu_binding); // Store binding per set
    }
    
    // 3. Bind the descriptor sets to the command buffer
    for (set_index, set_bindings) in descriptor_sets {
        // gpu_interface.bind_descriptor_set(cmd, set_index, &set_bindings)?;
    }

    Ok(())
}

// --- Dummy types for binding ---
type GpuBinding = u64; // Placeholder for actual backend binding resource
impl GpuBinding { const fn Buffer(_: BufferHandle) -> Self { 0 } const fn Texture(_: TextureHandle) -> Self { 0 } const fn Sampler(_: SamplerHandle) -> Self { 0 }}
type SamplerHandle = u64;
// Extend ResourceCache with methods to get backend handles
impl ResourceCache {
    fn get_gpu_buffer(&self, _id: ResourceId) -> Result<BufferHandle, RenderError> { Ok(0) }
    fn get_gpu_texture_view(&self, _id: ResourceId) -> Result<TextureHandle, RenderError> { Ok(TextureHandle(0)) }
    fn get_gpu_sampler(&self, _id: ResourceId) -> Result<SamplerHandle, RenderError> { Ok(0) }
}
```

## 10. Performance Optimizations

Strategies to improve rendering speed.

### 10.1 Batching

Combine multiple `RenderOperation`s into fewer draw calls if they share compatible state (pipeline, textures, etc.).

```rust
// Operation batching system (conceptual)
use std::any::Any;

pub struct BatchingSystem {
    batches: HashMap<BatchKey, RenderOperationBatch>,
    // Need access to renderer/translator context during batch execution
}

// Key to group compatible operations
#[derive(PartialEq, Eq, Hash, Clone)]
struct BatchKey {
    // Based on pipeline, textures, blend modes, depth state etc. required by the style/material
    pipeline_handle: PipelineHandle, // Essential for compatibility
    texture_bindings: Vec<Option<TextureHandle>>, // Textures used (order matters)
    blend_mode: BlendMode, // Example state
    // Other relevant states...
}

// Represents a batch of operations ready for a single (instanced) draw call
struct RenderOperationBatch {
    // Combined geometry (vertices/indices)
    vertices: Vec<u8>, // Use bytes for generic storage
    indices: Vec<u8>,
    vertex_stride: usize,
    index_format: IndexFormat,
    
    // Per-instance data (transforms, colors, custom attributes)
    instance_data: Vec<InstanceData>, // Struct defined below
    
    // Information needed to set up the draw call (uniforms, etc.)
    // This might reference the Style or Material data common to the batch.
    shared_style_data: Option<Box<dyn Any>>, 
    
    operation_count: usize, // For tracking/debugging
}

impl RenderOperationBatch {
    fn new(vertex_stride: usize, index_format: IndexFormat) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            instance_data: Vec::new(),
            shared_style_data: None,
            operation_count: 0,
            vertex_stride, index_format,
        }
    }

     // Add geometry and instance data from an operation to the batch
     fn add(&mut self, geometry: &GeometryData, instance: InstanceData) {
         let index_offset = (self.vertices.len() / self.vertex_stride) as u32; // Vertex offset
         
         self.vertices.extend_from_slice(&geometry.vertices);
         
         // Adjust and add indices
         match self.index_format {
             IndexFormat::Uint16 => {
                 let indices_u16 = bytemuck::cast_slice::<_, u16>(&geometry.indices);
                 for index in indices_u16 {
                     self.indices.extend_from_slice(&bytemuck::bytes_of(&(index + index_offset as u16)));
                 }
             }
             IndexFormat::Uint32 => {
                 let indices_u32 = bytemuck::cast_slice::<_, u32>(&geometry.indices);
                 for index in indices_u32 {
                     self.indices.extend_from_slice(&bytemuck::bytes_of(&(index + index_offset)));
                 }
             }
         }
         
         self.instance_data.push(instance);
         self.operation_count += 1;
     }
}


// Data sent per instance to the shader
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)] // Ensure Pod for GPU transfer
struct InstanceData {
    transform: [[f32; 4]; 4], // World transform matrix
    color_tint: [f32; 4],     // Per-instance color multiplier
    clip_rect: [f32; 4],      // Per-instance clip (if supported by shader)
    custom_data: [f32; 4],    // For style-specific instance data
}

// Blend mode enum (example)
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum BlendMode { Alpha, Additive, Opaque }

impl BatchingSystem {
    pub fn new() -> Self {
        Self { batches: HashMap::new() }
    }
    
    // Add an operation to the appropriate batch
    pub fn add_operation<R: Renderable + Tessellable + 'static>( // Require Tessellable for geometry
        &mut self, 
        operation: &RenderOperation<R>,
        // Need context to determine pipeline and tessellate
        renderer_context: &mut impl Renderer, // Use the Renderer trait for context
    ) -> Result<(), RenderError> 
    where 
        R::VertexType: bytemuck::Pod, 
        R::IndexType: bytemuck::Pod + Eq + std::hash::Hash + Clone + Copy + TryInto<u32> // Constraints for indexing
    {
        // 1. Determine the BatchKey for this operation
        // This depends heavily on the operation's style(s), effects, and renderable.
        // Simple example: Assume one style, no effects for batching.
        if operation.styles.len() != 1 || !operation.effects.is_empty() {
            // Cannot batch this operation (or needs more complex batching logic)
            // Fallback to non-batched rendering (handled elsewhere)
            return Ok(()); 
        }
        
        let style = &operation.styles[0];
        let key = self.determine_batch_key(style)?; // Extract key from style

        // 2. Tessellate the renderable to get geometry
        let tess_result = operation.try_tessellate(renderer_context)?; 
        
        // Need a temporary GeometryData representation
        let temp_geometry = GeometryData {
             vertices: bytemuck::cast_slice(&tess_result.vertices).to_vec(),
             indices: bytemuck::cast_slice(&tess_result.indices).to_vec(),
             handle: GeometryHandle(0), // Not managed by cache yet
        };
        
        let vertex_stride = std::mem::size_of::<R::VertexType>();
        let index_format = if std::mem::size_of::<R::IndexType>() == 2 { IndexFormat::Uint16 } else { IndexFormat::Uint32 };

        // 3. Get or create the batch
        let batch = self.batches.entry(key.clone()).or_insert_with(|| {
             RenderOperationBatch::new(vertex_stride, index_format)
        });

        // 4. Create instance data
        let instance = InstanceData {
            transform: operation.transform.to_mat4_array().into(), // Convert Transform to [[f32; 4]; 4]
            color_tint: [1.0, 1.0, 1.0, 1.0], // Default tint, style might override
            clip_rect: operation.clip.map(|r| [r.x, r.y, r.width, r.height]).unwrap_or([0.0; 4]),
            custom_data: [0.0; 4],
        };
        // TODO: Allow style to customize instance data (e.g., color_tint)

        // 5. Add geometry and instance data to the batch
        batch.add(&temp_geometry, instance);
        
        // Store shared style data if first operation in batch
        if batch.operation_count == 1 {
             // Clone or store reference to style data needed for execution
             // batch.shared_style_data = Some(style.get_shared_data()); 
        }

        Ok(())
    }
    
    // Determine batch key based on style properties
    fn determine_batch_key(&self, style: &Box<dyn Style>) -> Result<BatchKey, RenderError> {
         // This needs specific knowledge of how styles map to pipelines/states
         // Placeholder implementation:
         Ok(BatchKey {
             pipeline_handle: 0, // Get from style analysis
             texture_bindings: vec![], // Get from style analysis
             blend_mode: BlendMode::Alpha, // Get from style analysis
         })
    }

    // Execute all accumulated batches
    pub fn execute_batches(
        &mut self, 
        translator: &mut CommandTranslator // Need translator to issue GPU commands
    ) -> Result<(), RenderError> {
        for (key, batch) in self.batches.drain() { // Drain to consume batches
            if batch.operation_count == 0 { continue; }

            // 1. Prepare combined geometry for GPU
            // Create GPU buffers for batch.vertices and batch.indices
            let batch_geometry_desc = GeometryDesc {
                 vertices: batch.vertices,
                 vertex_stride: batch.vertex_stride,
                 indices: batch.indices,
                 index_format: batch.index_format,
                 usage: GeometryUsage::Stream, // Batches are often frame-specific
                 bounds: BoundingVolume::Box(Default::default()), // Bounds need calculation
            };
            // Use a temporary handle or manage batch geometry differently
            let temp_handle = GeometryHandle(rand::random()); // Example
            let (vb, ib) = translator.upload_geometry_data(temp_handle, &batch_geometry_desc)?;

            // 2. Prepare instance buffer
            let instance_buffer_desc = BufferDesc::vertex_buffer(
                 batch.instance_data.len() * std::mem::size_of::<InstanceData>(),
                 BufferUsage::VERTEX, // Used as a vertex buffer input
            );
            let instance_buffer = translator.gpu_interface.create_buffer(instance_buffer_desc)?;
            translator.gpu_interface.write_buffer(instance_buffer, 0, &batch.instance_data)?;

            // 3. Set pipeline and bindings
            let cmd = translator.gpu_interface.current_command_buffer()?;
            cmd.set_pipeline(key.pipeline_handle);
            // Bind textures from key.texture_bindings
            // Bind uniforms based on batch.shared_style_data

            // 4. Bind geometry and instance buffer
            cmd.set_vertex_buffer(0, vb); // Slot 0 for vertex data
            cmd.set_vertex_buffer(1, instance_buffer); // Slot 1 for instance data (shader must match)
            cmd.set_index_buffer(ib);
            
            // 5. Issue instanced draw call
            let index_count = batch_geometry_desc.indices.len() / match batch.index_format {
                IndexFormat::Uint16 => 2, IndexFormat::Uint32 => 4
            };
            cmd.draw_indexed_instanced(
                 index_count as u32, 
                 batch.operation_count as u32, 
                 0, // first_index
                 0, // base_vertex
                 0  // first_instance
            );

            // Cleanup temporary GPU buffers if needed
            // translator.gpu_interface.release_buffer(vb);
            // translator.gpu_interface.release_buffer(ib);
            // translator.gpu_interface.release_buffer(instance_buffer);
        }
        
        // Clear happens implicitly via drain, or call self.batches.clear() if not draining
        Ok(())
    }
}


// --- Dummy CommandBuffer method for instancing ---
impl CommandBuffer {
    fn draw_indexed_instanced(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
}
```

### 10.2 Culling

Skipping rendering of objects outside the view frustum (3D) or viewport (2D). Culling can happen:
- **CPU-side**: Before creating `RenderOperation`s or before batching. Uses `Renderable::world_bounds()`.
- **GPU-side**: Using techniques like hardware occlusion queries or compute-shader culling.

### 10.3 Caching

- **Resource Cache**: Already discussed (Section 7.2), avoids re-creating GPU resources.
- **Pipeline Cache**: Part of `GpuInterface` (Section 9.1), avoids recompiling shaders.
- **Tessellation Cache**: Optionally cache `GeometryDesc` for complex `Renderable`s that don't change often.
- **Intermediate Results**: Cache results of expensive operations like text layout or effect passes.

### 10.4 Parallelism

Utilize multiple CPU cores:
- **Parallel Tessellation**: Tessellate multiple independent `Renderable`s in parallel (e.g., using `rayon`).
- **Parallel Batch Creation**: Build batches for different `BatchKey`s concurrently.
- **Parallel Command Encoding**: If the backend supports it, encode commands for different render passes in parallel.

## 11. Error Handling

Robust error handling is crucial for a rendering system.

### 11.1 Error Types

Structured error enum provides details about failures. (Adding `TessellationFailed` based on enhancements).

```rust
/// Render error type with detailed information
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to create resource: {0}")]
    ResourceCreationFailed(String),
    
    #[error("Failed to update resource: {0}")]
    ResourceUpdateFailed(String),
    
    #[error("Invalid resource handle: {0}")]
    InvalidResourceHandle(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Stack underflow: {0}")]
    StackUnderflow(String),
    
    #[error("Unsupported operation or feature: {0}")]
    UnsupportedOperation(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Text layout error: {0}")]
    TextLayoutError(String),

    #[error("Tessellation failed: {0}")] // Added from enhancements
    TessellationFailed(String), 
    
    #[error("Path tessellation error: {0}")] // Can likely merge into TessellationFailed
    PathTessellationError(String), 
    
    #[error("GPU error: {0}")] // Wraps errors from the underlying GPU abstraction
    GpuError(String), // Simplified from GpuError(#[from] GpuError) if GpuError isn't defined here
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unexpected internal error: {0}")]
    Other(String),
}

// Helper macro remains useful
#[macro_export]
macro_rules! render_error {
    ($type:path, $message:expr) => {
        $type(format!("{} (at {}:{})", $message, file!(), line!()))
    };
    ($type:path, $fmt:expr, $($arg:tt)*) => {
        $type(format!("{} (at {}:{})", format!($fmt, $($arg)*), file!(), line!()))
    };
}

// Ensure GpuError is defined or handled appropriately
// type GpuError = String; // Example if GpuError is just a string message
```

### 11.2 Recovery Strategies

Define how the renderer attempts to recover from non-fatal errors.

```rust
/// Defines possible actions when a recoverable error occurs.
pub enum RecoveryAction {
    /// Attempt to use a default or fallback value/resource.
    UseFallback,
    /// Retry the failed operation (use with caution to avoid infinite loops).
    Retry,
    /// Skip the current operation/object and continue rendering others.
    Skip,
    /// Abort the current render pass or frame.
    Abort,
}

// Example configuration struct for renderer behavior
pub struct RendererConfig {
    pub allow_fallbacks: bool, // Allow using fallback fonts, materials, etc.
    pub log_errors: bool,
    // Add other configuration options
}

// Trait or mechanism for handling errors within the renderer/translator
pub trait ErrorHandler {
    fn handle_error<T>(
        &mut self, // May need mutable access to update state or fallbacks
        error: RenderError,
        context_info: &str, // e.g., "Tessellating Rectangle" or "Binding PBR material"
        recovery_strategy: impl FnOnce(RecoveryAction) -> Result<T, RenderError> // Closure defines how to recover
    ) -> Result<T, RenderError>;
}

// Example implementation within CommandTranslator or a dedicated handler
impl<'a> ErrorHandler for CommandTranslator<'a> {
     fn handle_error<T>(
         &mut self,
         error: RenderError,
         context_info: &str,
         recovery_strategy: impl FnOnce(RecoveryAction) -> Result<T, RenderError>
     ) -> Result<T, RenderError> {
         // 1. Log the error (optional, based on config)
         // if self.config.log_errors { log::error!("Render error in [{}]: {}", context_info, error); }
         
         // 2. Determine recovery action based on error type
         let action = match &error {
             RenderError::ResourceCreationFailed(_) | RenderError::ResourceUpdateFailed(_) => {
                 // if self.config.allow_fallbacks { RecoveryAction::UseFallback } else { RecoveryAction::Abort }
                 RecoveryAction::Abort // Often fatal if core resources fail
             }
             RenderError::InvalidResourceHandle(_) => RecoveryAction::Skip, // Skip object using bad handle
             RenderError::TessellationFailed(_) => {
                  // Maybe skip the object or try a very low-quality fallback?
                  RecoveryAction::Skip 
             }
             RenderError::ShaderCompilationFailed(_) => RecoveryAction::Abort, // Usually fatal
             RenderError::GpuError(_) | RenderError::InvalidState(_) | RenderError::StackUnderflow(_) => RecoveryAction::Abort, // Often indicates critical issues
             RenderError::UnsupportedOperation(_) => RecoveryAction::Skip, // Skip the unsupported part
             _ => RecoveryAction::Abort, // Default to abort for other/unknown errors
         };
         
         // 3. Execute the provided recovery strategy
         match recovery_strategy(action) {
             Ok(value) => Ok(value), // Recovery successful
             Err(recovery_err) => {
                 // Recovery failed or chose to propagate. Log original error?
                 Err(error) // Return the original error
             }
         }
     }
}

/*
// Example usage within a style's apply method:
impl Style for Fill {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        let result = renderer.fill_geometry(geometry, &self.paint, self.rule);
        
        if let Err(e) = result {
            // Assuming renderer implements ErrorHandler or has access to one
            return renderer.handle_error(e, "Applying Fill Style", |action| {
                 match action {
                     RecoveryAction::Skip => Ok(()), // Skip filling this geometry
                     RecoveryAction::UseFallback => {
                         // Try filling with a default color?
                         renderer.fill_geometry(geometry, &Paint::Solid(color_constants::MAGENTA), self.rule)
                     }
                     _ => Err(e), // Abort or Retry not handled here, propagate original error
                 }
            });
        }
        Ok(())
    }
    // ... other Style methods
}
*/
```

## 12. Public API Exports

Defines the primary interface exposed to users of the `vectron_render` crate.

```rust
// In lib.rs
pub mod core;
pub mod elements;
pub mod tessellation;
pub mod renderer; // Module containing main Renderer interface and implementations
pub mod pipeline; // Expose pipeline details if needed, maybe keep internal?
pub mod backend;  // Expose backend details if needed, maybe keep internal?
pub mod utils;    // Public utility functions/structs

// Re-export core traits and types
pub use core::renderable::{Renderable, Renderable2D, Renderable3D};
pub use core::style::{Style, Fill, Stroke, Paint}; // Assuming these concrete styles exist
pub use core::effect::{Effect, Shadow, Blur, Glow}; // Assuming concrete effects exist
pub use core::render_operation::RenderOperation;
pub use core::color::{Color, color_constants};
pub use core::units::{Unit, UnitContext, UnitValue, Vec3Unit};
pub use core::tessellation::{Tessellable, TessellationError}; // Expose base trait and error
pub use core::transform::Transform; // Expose transform type

// Re-export tessellation configuration and results
pub use tessellation::{TessellationOptions, TessellationResult, TessellationQuality, TessellationFlags, vertex};

// Re-export elements based on feature flags
#[cfg(feature = "d2-geometry")]
pub use elements::d2::geometry::{Rectangle, Circle, /* Ellipse, */ Path}; // Add other 2D shapes

// #[cfg(feature = "d2-text")]
// pub use elements::d2::{Text, TextBlock}; // Text elements

#[cfg(feature = "d2")]
pub use elements::d2::{Camera2D, Layer2D}; // 2D scene/camera elements

#[cfg(feature = "d3-geometry")]
pub use elements::d3::geometry::{Box3D, Sphere, /* Cylinder, */ Mesh}; // Add other 3D shapes

// #[cfg(feature = "d3-scene")]
// pub use elements::d3::{Camera3D, Light, Scene}; // 3D scene elements

// Expose the main Renderer interface
pub use renderer::Renderer; // Assuming the primary interface is here

// Expose core error type
pub use core::RenderError; // Assuming RenderError is moved to core or re-exported

// Potentially hide internal details like Pipeline, Backend unless advanced usage is intended.
```

## 13. Implementation Priorities and Timeline

Outline of development phases.

1.  **Phase 1: Core Framework (Month 1)**
    *   Implement core traits (`Renderable`, `Style`, `Effect`, `Tessellable`).
    *   Implement unit system (`core::units`).
    *   Implement coordinate system aspects within `Renderable`.
    *   Implement `RenderOperation` structure (`core::render_operation`).
    *   Implement basic `Color` and `Transform` systems.

2.  **Phase 2: Element & Tessellation Basics (Month 2)**
    *   Implement basic 2D elements (`Rectangle`, `Circle`, `Path` in `elements::d2`).
    *   Implement basic 3D elements (`Box3D`, `Sphere` in `elements::d3`).
    *   Implement `Tessellable` for these basic elements.
    *   Integrate unit system into element definitions.
    *   Setup basic tessellation module structure (`tessellation/`).

3.  **Phase 3: Rendering Backend & Pipelines (Month 3)**
    *   Implement backend abstraction (`backend::gpu`) connecting to `vectron_gpu`.
    *   Implement resource cache and management (`resources/`).
    *   Implement command translation logic (`backend::commands` or `renderer` module) for basic operations.
    *   Setup basic rendering pipelines (`pipelines/`) for fill/stroke.

4.  **Phase 4: Integration, Features & Optimization (Month 4)**
    *   Integrate tessellation results into the rendering pipeline.
    *   Implement core styles (`Fill`, `Stroke`) and effects (`Shadow`).
    *   Implement remaining elements (Text, Meshes, Scene graphs).
    *   Implement advanced tessellation features (options, quality).
    *   Performance optimization (batching, caching).
    *   Documentation and examples.
    *   Testing and bug fixing.

## 14. Conclusion

The Vectron Render 4.0 redesign introduces significant improvements, creating a modern, flexible foundation for 2D and 3D rendering:

1.  **Coordinate System**: `Renderable`s now possess local origins, simplifying hierarchical transformations.
2.  **Unit System**: A versatile system supports absolute, relative, and computed units for responsive design.
3.  **Elements Module**: Clear organization of renderables into `elements/d2` and `elements/d3`, controlled by feature flags.
4.  **Tessellation System**: The `Tessellable` trait provides a standard way to generate geometry, with configurable options for quality and performance.
5.  **Render Operation**: `RenderOperation` cleanly combines elements, styles, effects, and transforms, serving as the core rendering command.

This blueprint focuses on creating a powerful yet intuitive rendering system. By addressing core concepts like coordinates, units, element organization, and geometry generation via tessellation, this redesign enables a wide range of applications while maintaining a clean API.

