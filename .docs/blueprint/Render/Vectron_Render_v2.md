# Vectron Render Crate Blueprint

**Version:** 1.0  
**Date:** March 22, 2025

## Table of Contents

1. [Overview](#1-overview)
2. [API Layer](#2-api-layer)
   - [Tiered API Design](#21-tiered-api-design)
   - [Composable Command System](#22-composable-command-system)
   - [Shader Interface](#23-shader-interface)
   - [Integration Between API Tiers](#24-integration-between-api-tiers)
3. [Resource Layer](#3-resource-layer)
   - [Resource Types](#31-resource-types)
   - [Resource Cache](#32-resource-cache)
   - [Geometry Management](#33-geometry-management)
   - [Path Generation](#34-path-generation)
4. [Pipeline Layer](#4-pipeline-layer)
   - [Vector Rendering Pipeline](#41-vector-rendering-pipeline)
   - [Text Rendering Pipeline](#42-text-rendering-pipeline)
   - [3D Rendering Pipeline](#43-3d-rendering-pipeline)
   - [Effect Pipeline](#44-effect-pipeline)
5. [Backend Abstraction](#5-backend-abstraction)
   - [GPU Interface](#51-gpu-interface)
   - [Command Translation](#52-command-translation)
   - [State Management](#53-state-management)
   - [Resource Binding](#54-resource-binding)
6. [Performance Optimizations](#6-performance-optimizations)
   - [Batching](#61-batching)
   - [Culling](#62-culling)
   - [Caching](#63-caching)
   - [Parallelism](#64-parallelism)
7. [Error Handling](#7-error-handling)
   - [Error Types](#71-error-types)
   - [Recovery Strategies](#72-recovery-strategies)
8. [Feature Flags and Compilation](#8-feature-flags-and-compilation)
   - [Feature Flags](#81-feature-flags)
   - [Conditional Compilation](#82-conditional-compilation)
9. [Project Structure](#9-project-structure)

## 1. Overview

The Vectron Render crate provides a flexible rendering system that builds on top of the Vectron GPU abstraction. It offers a tiered API approach for different levels of control and abstraction, supporting 2D vector graphics, text rendering, and 3D rendering through a consistent interface.

### Goals

- **Modular Design**: Clear separation of concerns with multiple API tiers
- **Flexibility**: Support for various rendering techniques and backends
- **Performance**: Efficient rendering with minimal overhead
- **Composition**: Stackable operations for complex rendering effects
- **Platform Independence**: Works across platforms through the GPU abstraction
- **Unified Approach**: Consistent handling of both 2D and 3D rendering

### Architecture Layers

1. **API Layer**: Multiple tiers of public-facing interfaces
   - Bare API (low-level drawing commands)
   - Standard API (common rendering operations)
   - Vector API (2D vector graphics)
   - Text API (text rendering capabilities)
   - 3D API (3D rendering primitives)

2. **Resource Layer**: Management of rendering resources
   - Geometries (meshes, paths)
   - Materials and styles
   - Textures and fonts
   - Cached rendering data

3. **Pipeline Layer**: Specialized rendering pipelines
   - Vector rendering pipeline
   - Text rendering pipeline
   - 3D rendering pipeline
   - Custom effect pipelines

4. **Backend Abstraction**: Interface with the GPU crate
   - Command translation
   - Resource binding
   - State management

## 2. API Layer

The API layer provides multiple tiers of abstraction, each catering to different needs and use cases.

### 2.1 Tiered API Design

The API is structured into multiple tiers, allowing developers to choose the appropriate level of abstraction for their needs:

```
vectron_render/src/api/
├── mod.rs              # Exports from all API tiers
├── bare/               # Low-level drawing API
│   ├── mod.rs
│   ├── commands.rs     # Basic drawing commands
│   ├── resources.rs    # Raw resource handling
│   └── state.rs        # Render state management
│
├── standard/           # Standard rendering API
│   ├── mod.rs
│   ├── renderer.rs     # Main renderer interface
│   ├── batch.rs        # Batched rendering
│   └── frame.rs        # Frame management
│
├── vector/             # 2D vector graphics API
│   ├── mod.rs
│   ├── path.rs         # Path construction and drawing
│   ├── shapes.rs       # Basic shape primitives
│   ├── stroke.rs       # Stroke styling and rendering
│   └── fill.rs         # Fill styling and rendering
│
├── text/               # Text rendering API
│   ├── mod.rs
│   ├── font.rs         # Font management
│   ├── layout.rs       # Text layout engine
│   └── style.rs        # Text styling
│
└── three_d/            # 3D rendering API
    ├── mod.rs
    ├── mesh.rs         # Mesh creation and management
    ├── material.rs     # Material system
    ├── camera.rs       # Camera handling
    └── scene.rs        # Scene organization
```

Each tier provides an interface suited to different use cases:

#### Bare API

The Bare API provides direct access to low-level drawing commands with minimal abstraction:

```rust
// Bare API for direct draw commands
pub mod bare {
    pub struct DrawList {
        commands: Vec<DrawCommand>,
        state_stack: Vec<DrawState>,
        current_state: DrawState,
    }

    impl DrawList {
        pub fn new() -> Self {
            Self {
                commands: Vec::new(),
                state_stack: Vec::new(),
                current_state: DrawState::default(),
            }
        }
        
        // Push a new state onto the stack
        pub fn push_state(&mut self) -> &mut Self {
            self.state_stack.push(self.current_state.clone());
            self
        }
        
        // Add a raw draw command
        pub fn add_command(&mut self, command: DrawCommand) -> &mut Self {
            self.commands.push(command);
            self
        }
        
        // Other low-level commands...
    }
}
```

#### Standard API

The Standard API provides a more convenient interface for common rendering tasks:

```rust
// Standard API for common rendering operations
pub mod standard {
    pub struct Renderer {
        context: RenderContext,
        resource_cache: ResourceCache,
    }

    impl Renderer {
        pub fn new(gpu_device: &GpuDevice) -> Result<Self, RenderError> {
            // Implementation...
        }
        
        // Begin a new frame
        pub fn begin_frame(&mut self, width: u32, height: u32) -> &mut Self {
            // Implementation...
            self
        }
        
        // Create a new draw list
        pub fn create_draw_list(&self) -> DrawList {
            DrawList::new()
        }
        
        // Submit a draw list for rendering
        pub fn submit(&mut self, draw_list: DrawList) -> &mut Self {
            // Implementation...
            self
        }
    }
}
```

### 2.2 Composable Command System

The rendering system is built around a composable operation pattern that focuses on combining drawable elements with styles and effects.

```rust
// Core trait for anything that can be drawn
pub trait Drawable {
    // Generalized geometry type to support both 2D and 3D
    type GeometryType;
    
    // Convert this object to geometry for rendering
    fn to_geometry(&self, context: &RenderContext) -> Result<Self::GeometryType, RenderError>;
    
    // Get bounding volume for this drawable object
    fn bounds(&self) -> impl BoundingVolume;
    
    // Optional optimization hint for renderers
    fn render_hints(&self) -> RenderHints {
        RenderHints::default()
    }
}

// 2D drawable specialization
pub trait Drawable2D: Drawable<GeometryType = Geometry2D> {
    // 2D-specific methods
}

// 3D drawable specialization
pub trait Drawable3D: Drawable<GeometryType = Geometry3D> {
    // 3D-specific methods
}

// Base operation trait
pub trait Operation {
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError>;
}

// 2D operation implementation
pub struct Draw2D<D: Drawable2D> {
    drawable: D,
    styles: Vec<Box<dyn Style>>,
    effects: Vec<Box<dyn Effect>>,
    transform: Transform,
    clip: Option<Rect>,
}

impl<D: Drawable2D> Draw2D<D> {
    pub fn new(drawable: D) -> Self {
        Self {
            drawable,
            styles: Vec::new(),
            effects: Vec::new(),
            transform: Transform::identity(),
            clip: None,
        }
    }
    
    // Builder methods for fluent operation construction
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
}

impl<D: Drawable2D> Operation for Draw2D<D> {
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Save current state
        renderer.push_state();
        
        // Apply transform and clip
        if let Some(clip) = self.clip {
            renderer.set_clip(clip);
        }
        renderer.set_transform(self.transform);
        
        // Apply pre-effects
        for effect in &self.effects {
            effect.apply_pre(renderer, &self.drawable)?;
        }
        
        // Convert drawable to geometry
        let geometry = self.drawable.to_geometry(renderer.context())?;
        
        // Apply styles
        for style in &self.styles {
            style.apply(renderer, &geometry)?;
        }
        
        // Apply post-effects
        for effect in &self.effects {
            effect.apply_post(renderer, &geometry)?;
        }
        
        // Restore state
        renderer.pop_state();
        
        Ok(())
    }
}

// 3D operation implementation
pub struct Draw3D<M: Mesh> {
    mesh: M,
    material: Material,
    transform: Transform,
    camera: Option<CameraHandle>,
}

impl<M: Mesh> Draw3D<M> {
    pub fn new(mesh: M) -> Self {
        Self {
            mesh,
            material: Material::default(),
            transform: Transform::identity(),
            camera: None,
        }
    }
    
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
    
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    
    pub fn with_camera(mut self, camera: CameraHandle) -> Self {
        self.camera = Some(camera);
        self
    }
}

impl<M: Mesh> Operation for Draw3D<M> {
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Save current state
        renderer.push_state();
        
        // Set transform
        renderer.set_transform(self.transform);
        
        // Convert mesh to geometry
        let geometry = self.mesh.to_geometry(renderer.context())?;
        
        // Set up material and transform
        renderer.set_material(&self.material)?;
        
        // Set camera if provided
        if let Some(camera) = self.camera {
            renderer.set_camera(camera)?;
        }
        
        // Render the geometry
        renderer.draw_geometry(geometry)?;
        
        // Restore state
        renderer.pop_state();
        
        Ok(())
    }
}

// Style trait for fills, strokes, etc.
pub trait Style {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError>;
}

// Fill style implementation
pub struct Fill {
    paint: Paint,
    rule: FillRule,
}

impl Fill {
    // Create a solid color fill
    pub fn solid(color: Color) -> Self {
        Self {
            paint: Paint::Solid(color),
            rule: FillRule::NonZero,
        }
    }
    
    // Create a linear gradient fill
    pub fn linear_gradient(gradient: LinearGradient) -> Self {
        Self {
            paint: Paint::LinearGradient(gradient),
            rule: FillRule::NonZero,
        }
    }
    
    pub fn with_rule(mut self, rule: FillRule) -> Self {
        self.rule = rule;
        self
    }
}

impl Style for Fill {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        renderer.fill_geometry(geometry, &self.paint, self.rule)
    }
}

// Stroke style implementation
pub struct Stroke {
    paint: Paint,
    width: f32,
    line_join: LineJoin,
    line_cap: LineCap,
    dash_pattern: Option<DashPattern>,
}

impl Stroke {
    pub fn new(paint: Paint, width: f32) -> Self {
        Self {
            paint,
            width,
            line_join: LineJoin::Miter,
            line_cap: LineCap::Butt,
            dash_pattern: None,
        }
    }
    
    pub fn with_line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }
    
    pub fn with_line_cap(mut self, cap: LineCap) -> Self {
        self.line_cap = cap;
        self
    }
    
    pub fn with_dash_pattern(mut self, pattern: DashPattern) -> Self {
        self.dash_pattern = Some(pattern);
        self
    }
}

impl Style for Stroke {
    fn apply(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        renderer.stroke_geometry(
            geometry, 
            &self.paint, 
            self.width, 
            self.line_join, 
            self.line_cap,
            self.dash_pattern.as_ref()
        )
    }
}

// Paint types for fills and strokes
#[derive(Clone)]
pub enum Paint {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Texture(TextureHandle),
    Pattern(PatternHandle),
}

// Example usage with the core color system:
// Fill::solid(Color::rgb(1.0, 0.0, 0.0))
// Fill::solid(color_constants::RED)
// Fill::linear_gradient(LinearGradient::new([0.0, 0.0], [100.0, 0.0], Color::rgb(1.0, 0.0, 0.0), Color::rgb(0.0, 0.0, 1.0)))

// Effect trait for shadows, blurs, etc.
pub trait Effect {
    // Apply before the main drawing
    fn apply_pre(&self, renderer: &mut dyn Renderer, drawable: &dyn Drawable) -> Result<(), RenderError> {
        Ok(()) // Default no-op implementation
    }
    
    // Apply after the main drawing
    fn apply_post(&self, renderer: &mut dyn Renderer, geometry: &GeometryData) -> Result<(), RenderError> {
        Ok(()) // Default no-op implementation
    }
}

// Shadow effect implementation
pub struct Shadow {
    color: Color,
    offset: Vec2,
    blur_radius: f32,
}

impl Shadow {
    pub fn new(color: Color, offset: Vec2, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }
}

impl Effect for Shadow {
    fn apply_pre(&self, renderer: &mut dyn Renderer, drawable: &dyn Drawable) -> Result<(), RenderError> {
        // Save the original state
        renderer.push_state();
        
        // Apply shadow transform and settings
        renderer.translate(self.offset.x, self.offset.y);
        
        // Create shadow geometry
        let geometry = drawable.to_geometry(renderer.context())?;
        
        // Apply blur if needed
        if self.blur_radius > 0.0 {
            renderer.set_blur(self.blur_radius);
        }
        
        // Draw shadow with the shadow color
        renderer.fill_geometry(&geometry, &Paint::Solid(self.color), FillRule::NonZero)?;
        
        // Restore original state
        renderer.pop_state();
        
        Ok(())
    }
}

// Command list for executing operations
pub struct CommandList {
    operations: Vec<Box<dyn Operation>>,
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    
    pub fn add<O: Operation + 'static>(&mut self, operation: O) -> &mut Self {
        self.operations.push(Box::new(operation));
        self
    }
    
    pub fn clear(&mut self) {
        self.operations.clear();
    }
    
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        for operation in &self.operations {
            operation.execute(renderer)?;
        }
        Ok(())
    }
}

// Example usage showing operation composition
let renderer = Renderer::new(device)?;
let mut command_list = CommandList::new();

// Create a rectangle with fill, stroke and shadow
let rect_operation = Draw2D::new(Rectangle::new(10.0, 10.0, 100.0, 50.0))
    .with_effect(Shadow::new(Color::rgba(0.0, 0.0, 0.0, 0.5), Vec2::new(2.0, 2.0), 4.0))
    .with_style(Fill::solid(color_constants::BLUE))
    .with_style(Stroke::new(Paint::Solid(color_constants::BLACK), 2.0))
    .with_transform(Transform::rotation(45.0));

// Create a 3D cube with material
let cube_operation = Draw3D::new(Cube::new(1.0))
    .with_material(Material::pbr(Albedo::color(Color::rgb(0.8, 0.2, 0.2)), 0.7, 0.3))
    .with_transform(Transform::from_translation_rotation_scale(
        Vec3::new(0.0, 0.0, -5.0),
        Quaternion::from_euler(0.0, 45.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0)
    ))
    .with_camera(main_camera);

// Add operations to command list
command_list
    .add(rect_operation)
    .add(cube_operation);

// Execute all operations
renderer.begin_frame(width, height);
command_list.execute(&mut renderer)?;
renderer.end_frame();
```

The command system is designed with these key properties:

1. **Composability**: Operations combine drawables, styles, and effects into unified entities
2. **Builder pattern**: Fluent interfaces make creating complex operations intuitive
3. **Separation of concerns**: Drawables handle geometry, styles handle appearance, effects handle special rendering
4. **Extensibility**: New drawables, styles, and effects can be added through the trait system
5. **State management**: Operations manage their own state changes (transform, clip, etc.)
6. **Execution model**: Operations execute in sequence with well-defined lifecycle phases

This operation-centric approach provides a more flexible foundation than command enumeration, allowing for greater extensibility and more intuitive API design. It closely follows the principle of building complex visual elements from simple, composable building blocks.

### 2.2.1 Core Color System

The color system is a fundamental part of the core module, providing a unified approach to color handling across the renderer:

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
        Self { r, g, b, a }
    }
    
    /// Create a new color with RGB components (alpha=1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
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
    
    /// Create a color from a hexadecimal value
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        let a = if hex > 0xFFFFFF { ((hex >> 24) & 0xFF) as u8 } else { 255 };
        
        Self::from_rgba8(r, g, b, a)
    }
    
    /// Multiply this color by another
    pub fn multiply(&self, other: &Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
    
    /// Linear interpolation between colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
    
    /// Apply premultiplied alpha
    pub fn premultiply(&self) -> Self {
        Self {
            r: self.r * self.a,
            g: self.g * self.a,
            b: self.b * self.a,
            a: self.a,
        }
    }
}

/// Common color constants
pub mod constants {
    use super::Color;
    
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    // Additional common colors and gray tones...
}

/// Linear gradient between two or more colors
#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub start_point: [f32; 2],
    pub end_point: [f32; 2],
    pub stops: Vec<GradientStop>,
}

/// A color stop in a gradient
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
                GradientStop {
                    position: 0.0,
                    color: start_color,
                },
                GradientStop {
                    position: 1.0,
                    color: end_color,
                },
            ],
        }
    }
    
    /// Add a color stop to the gradient
    pub fn add_stop(&mut self, position: f32, color: Color) -> &mut Self {
        // Insert sorted by position
        let pos = position.clamp(0.0, 1.0);
        let idx = self.stops.binary_search_by(|s| {
            s.position.partial_cmp(&pos).unwrap()
        }).unwrap_or_else(|e| e);
        
        self.stops.insert(idx, GradientStop {
            position: pos,
            color,
        });
        
        self
    }
}
```

The color system provides a consistent way to work with colors throughout the rendering pipeline. It includes:

1. **Flexible creation methods**: Create colors from various formats (float values, 8-bit integers, hex values)
2. **Color transformations**: Operations like interpolation, premultiplied alpha, and color manipulation
3. **Standard constants**: Common colors available through a constants module
4. **Gradient support**: Linear gradients with multiple color stops
5. **Integration with styles**: Used by Fill, Stroke, and other style components
6. **Integration with effects**: Used by Shadow, Glow, and other visual effects

By making color a central part of the core module, the renderer maintains consistency in color handling across all rendering operations, from simple fills to complex gradients and effects.

Example usage with the operation system:

```rust
// Fill rectangle with solid color
let solid_rect = Operation::new(Rectangle::new(10.0, 10.0, 100.0, 50.0))
    .with_style(Fill::solid(Color::from_hex(0x3366CC))) // Using hex color
    .with_style(Stroke::new(Paint::Solid(color_constants::BLACK), 2.0)); // Using constant

// Create a gradient fill
let gradient = LinearGradient::new(
    [0.0, 0.0],
    [200.0, 0.0],
    Color::from_rgb8(255, 0, 0),   // Red
    Color::from_rgb8(0, 0, 255)    // Blue
);

// Add additional stops
let mut multi_gradient = gradient.clone();
multi_gradient.add_stop(0.5, Color::from_rgb8(0, 255, 0));  // Green in the middle

// Apply gradient to circle
let gradient_circle = Operation::new(Circle::new(150.0, 150.0, 75.0))
    .with_style(Fill::linear_gradient(multi_gradient));
```

### 2.3 Shader Interface

The render crate uses specialized shaders for different rendering tasks:

```rust
// Shader modules for different rendering tasks
pub mod shaders {
    // Vector rendering shaders
    pub mod vector {
        pub static FILL_SHADER: ShaderModule = ShaderModule {
            // Shader details...
        };
        
        pub static STROKE_SHADER: ShaderModule = ShaderModule {
            // Shader details...
        };
    }
    
    // Text rendering shaders
    pub mod text {
        pub static MSDF_SHADER: ShaderModule = ShaderModule {
            // Shader details...
        };
    }
    
    // 3D rendering shaders
    pub mod three_d {
        pub static PBR_SHADER: ShaderModule = ShaderModule {
            // Shader details...
        };
    }
    
    // Effect shaders
    pub mod effects {
        pub static BLUR_SHADER: ShaderModule = ShaderModule {
            // Shader details...
        };
    }
}
```

### 2.4 Integration Between API Tiers

The different API tiers are designed to work together seamlessly:

```rust
// Example of API integration
pub fn render_ui(renderer: &mut Renderer) -> Result<(), RenderError> {
    // Create a vector canvas for 2D rendering
    let mut canvas = VectorCanvas::new();
    
    // Draw background with vector API
    canvas.begin_path()
          .rect(0.0, 0.0, 800.0, 600.0)
          .fill_color(Color::WHITE);
    
    // Draw text with text API
    let text_renderer = TextRenderer::new();
    let font = text_renderer.load_font(include_bytes!("../assets/fonts/roboto.ttf"))?;
    text_renderer.draw_text(&mut canvas, "Hello, World!", 50.0, 50.0, font, 24.0);
    
    // Create a 3D scene
    let mut scene = Scene::new();
    let camera = Camera::perspective(45.0, 1.33, 0.1, 100.0);
    scene.add_camera(camera);
    scene.add_object(cube_mesh, materials.metal, Transform::translation(0.0, 0.0, -5.0));
    
    // Render the 3D scene into a viewport
    canvas.push_state()
          .scissor(Rect::new(500.0, 100.0, 250.0, 250.0));
    scene.render(renderer)?;
    canvas.pop_state();
    
    // Submit the canvas to the renderer
    renderer.submit(canvas.into_draw_list());
    
    Ok(())
}
```

## 3. Resource Layer

The resource layer manages the creation, tracking, and lifecycle of rendering resources.

### 3.1 Resource Types

```rust
// Core resource types with type safety
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GeometryHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FontHandle(pub(crate) ResourceId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MaterialHandle(pub(crate) ResourceId);

// Common resource ID type
type ResourceId = u64;
```

### 3.2 Resource Cache

```rust
// Resource cache for efficient resource management
pub struct ResourceCache {
    geometries: ResourcePool<GeometryResource>,
    textures: ResourcePool<TextureResource>,
    fonts: ResourcePool<FontResource>,
    materials: ResourcePool<MaterialResource>,
    
    // GPU resource mapping
    geometry_buffers: HashMap<GeometryHandle, BufferHandle>,
    texture_handles: HashMap<TextureHandle, TextureId>,
}

impl ResourceCache {
    pub fn new() -> Self {
        // Implementation...
    }
    
    // Create a geometry resource
    pub fn create_geometry(&mut self, vertices: &[Vertex], indices: &[u32]) -> Result<GeometryHandle, RenderError> {
        // Implementation...
    }
    
    // Create a texture resource
    pub fn create_texture(&mut self, width: u32, height: u32, format: PixelFormat, data: &[u8]) -> Result<TextureHandle, RenderError> {
        // Implementation...
    }
    
    // Get GPU resources for a geometry
    pub fn get_geometry_buffers(&self, handle: GeometryHandle) -> Option<(BufferHandle, BufferHandle)> {
        // Implementation...
    }
}
```

### 3.3 Geometry Management

```rust
// Geometry types and management
pub enum GeometryType {
    Static,   // Rarely changes, stored in GPU memory
    Dynamic,  // Changes occasionally, optimized updates
    Streamed, // Changes every frame, optimized for frequent updates
}

pub struct GeometryDesc {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub type_: GeometryType,
}

pub struct GeometryResource {
    desc: GeometryDesc,
    vertex_buffer: Option<BufferHandle>,
    index_buffer: Option<BufferHandle>,
    dirty: bool,
}

impl GeometryResource {
    pub fn new(desc: GeometryDesc) -> Self {
        // Implementation...
    }
    
    pub fn update_vertices(&mut self, vertices: &[Vertex]) {
        self.desc.vertices = vertices.to_vec();
        self.dirty = true;
    }
    
    pub fn upload(&mut self, device: &GpuDevice) -> Result<(), RenderError> {
        if !self.dirty && self.vertex_buffer.is_some() {
            return Ok(());
        }
        
        // Create or update GPU buffers...
        // Set dirty = false when done
        
        Ok(())
    }
}
```

### 3.4 Path Generation

```rust
// Path generation and tessellation
pub struct PathBuilder {
    commands: Vec<PathCommand>,
    current_point: Option<Point>,
}

pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadraticCurveTo { control: Point, end: Point },
    BezierCurveTo { control1: Point, control2: Point, end: Point },
    ArcTo { center: Point, radius: f32, start_angle: f32, end_angle: f32 },
    Close,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_point: None,
        }
    }
    
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        let point = Point::new(x, y);
        self.commands.push(PathCommand::MoveTo(point));
        self.current_point = Some(point);
        self
    }
    
    // Other path building methods...
    
    pub fn build(&self) -> Path {
        Path {
            commands: self.commands.clone(),
            bounds: self.calculate_bounds(),
        }
    }
    
    fn calculate_bounds(&self) -> Rect {
        // Calculate the bounds of the path
        // Implementation...
        Rect::default()
    }
}

pub struct Path {
    commands: Vec<PathCommand>,
    bounds: Rect,
}

impl Path {
    // Tessellate the path into renderable geometry
    pub fn tessellate(&self, fill_rule: FillRule) -> GeometryDesc {
        // Implementation...
        GeometryDesc {
            vertices: Vec::new(),
            indices: Vec::new(),
            type_: GeometryType::Static,
        }
    }
    
    // Generate stroke geometry for the path
    pub fn stroke(&self, width: f32, line_join: LineJoin, line_cap: LineCap) -> GeometryDesc {
        // Implementation...
        GeometryDesc {
            vertices: Vec::new(),
            indices: Vec::new(),
            type_: GeometryType::Static,
        }
    }
}
```

## 4. Pipeline Layer

The pipeline layer manages the specialized rendering pipelines for different rendering tasks.

### 4.1 Vector Rendering Pipeline

```rust
// Vector rendering pipeline
pub struct VectorPipeline {
    fill_pipeline: PipelineHandle,
    stroke_pipeline: PipelineHandle,
    path_pipeline: PipelineHandle,
    stencil_pipeline: PipelineHandle,
}

impl VectorPipeline {
    pub fn new(device: &GpuDevice) -> Result<Self, RenderError> {
        // Create fill pipeline
        let fill_pipeline = device.create_pipeline(PipelineDesc::new()
            .vertex_shader(shaders::vector::FILL_VERTEX)
            .fragment_shader(shaders::vector::FILL_FRAGMENT)
            .vertex_layout(layouts::VECTOR_VERTEX)
            .render_target_format(TextureFormat::RGBA8_UNORM))?;
            
        // Create other pipelines...
        
        Ok(Self {
            fill_pipeline,
            stroke_pipeline,
            path_pipeline,
            stencil_pipeline,
        })
    }
    
    // Render a filled path
    pub fn render_fill(&self, cmd: &mut CommandBuffer, path: &Path, style: &FillStyle) -> Result<(), RenderError> {
        // Implementation...
        Ok(())
    }
    
    // Render a stroked path
    pub fn render_stroke(&self, cmd: &mut CommandBuffer, path: &Path, style: &StrokeStyle) -> Result<(), RenderError> {
        // Implementation...
        Ok(())
    }
}
```

### 4.2 Text Rendering Pipeline

```rust
// Text rendering pipeline
pub struct TextPipeline {
    msdf_pipeline: PipelineHandle,  // Multi-channel signed distance field
    bitmap_pipeline: PipelineHandle, // Bitmap fonts
}

impl TextPipeline {
    pub fn new(device: &GpuDevice) -> Result<Self, RenderError> {
        // Create MSDF pipeline
        let msdf_pipeline = device.create_pipeline(PipelineDesc::new()
            .vertex_shader(shaders::text::MSDF_VERTEX)
            .fragment_shader(shaders::text::MSDF_FRAGMENT)
            .vertex_layout(layouts::TEXT_VERTEX)
            .blend_state(BlendState::ALPHA_BLEND)
            .render_target_format(TextureFormat::RGBA8_UNORM))?;
            
        // Create bitmap pipeline...
        
        Ok(Self {
            msdf_pipeline,
            bitmap_pipeline,
        })
    }
    
    // Render a string using MSDF (multi-channel signed distance field)
    pub fn render_msdf(&self, cmd: &mut CommandBuffer, text: &str, font: &FontResource, position: Point, size: f32, color: Color) -> Result<(), RenderError> {
        // Implementation...
        Ok(())
    }
    
    // Render a pre-calculated text layout
    pub fn render_layout(&self, cmd: &mut CommandBuffer, layout: &TextLayout, position: Point) -> Result<(), RenderError> {
        // Implementation...
        Ok(())
    }
}
```

### 4.3 3D Rendering Pipeline

```rust
// 3D rendering pipeline
pub struct ThreeDPipeline {
    pbr_pipeline: PipelineHandle,  // Physically-based rendering
    shadow_pipeline: PipelineHandle,
    skybox_pipeline: PipelineHandle,
}

impl ThreeDPipeline {
    pub fn new(device: &GpuDevice) -> Result<Self, RenderError> {
        // Create PBR pipeline
        let pbr_pipeline = device.create_pipeline(PipelineDesc::new()
            .vertex_shader(shaders::three_d::PBR_VERTEX)
            .fragment_shader(shaders::three_d::PBR_FRAGMENT)
            .vertex_layout(layouts::MESH_VERTEX)
            .depth_stencil_state(DepthStencilState::DEPTH_TEST_WRITE)
            .render_target_format(TextureFormat::RGBA8_UNORM)
            .depth_stencil_format(TextureFormat::Depth24PlusStencil8))?;
            
        // Create other pipelines...
        
        Ok(Self {
            pbr_pipeline,
            shadow_pipeline,
            skybox_pipeline,
        })
    }
    
    // Render a mesh with a material
    pub fn render_mesh(&self, cmd: &mut CommandBuffer, mesh: &MeshResource, material: &MaterialResource, transform: &Transform, camera: &Camera) -> Result<(), RenderError> {
        // Set pipeline
        cmd.set_pipeline(self.pbr_pipeline);
        
        // Upload uniforms (transform, camera, material properties)
        // Bind vertex and index buffers
        // Draw
        
        Ok(())
    }
}
```

### 4.4 Effect Pipeline

```rust
// Effect pipeline for post-processing
pub struct EffectPipeline {
    blur_pipeline: PipelineHandle,
    shadow_pipeline: PipelineHandle,
    glow_pipeline: PipelineHandle,
}

impl EffectPipeline {
    pub fn new(device: &GpuDevice) -> Result<Self, RenderError> {
        // Create blur pipeline
        let blur_pipeline = device.create_pipeline(PipelineDesc::new()
            .vertex_shader(shaders::effects::BLUR_VERTEX)
            .fragment_shader(shaders::effects::BLUR_FRAGMENT)
            .vertex_layout(layouts::QUAD_VERTEX)
            .render_target_format(TextureFormat::RGBA8_UNORM))?;
            
        // Create other pipelines...
        
        Ok(Self {
            blur_pipeline,
            shadow_pipeline,
            glow_pipeline,
        })
    }
    
    // Apply a blur effect
    pub fn apply_blur(&self, cmd: &mut CommandBuffer, source: TextureHandle, radius: f32) -> Result<TextureHandle, RenderError> {
        // Create temporary texture
        // Set pipeline
        // Draw fullscreen quad with blur shader
        // Return result texture
        
        Ok(TextureHandle(0)) // Placeholder
    }
}
```

## 5. Backend Abstraction

The backend abstraction layer interfaces with the GPU crate, translating Render crate operations to GPU operations.

### 5.1 GPU Interface

```rust
// GPU interface for abstracting the GPU crate
pub struct GpuInterface {
    device: Arc<GpuDevice>,
    command_buffer: Option<CommandBuffer>,
    resource_map: HashMap<ResourceId, GpuResourceInfo>,
    pipeline_cache: HashMap<PipelineKey, PipelineHandle>,
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
    
    // Begin a new frame
    pub fn begin_frame(&mut self, width: u32, height: u32) -> Result<(), RenderError> {
        // Get surface
        let surface = self.device.get_surface()
            .ok_or_else(|| RenderError::InvalidState("No surface available".into()))?;
            
        // Configure surface if needed
        if surface.width() != width || surface.height() != height {
            self.device.configure_surface(
                surface,
                width,
                height,
                PresentMode::Fifo,
                TextureFormat::BGRA8_UNORM
            )?;
        }
        
        // Begin frame and get command buffer
        let cmd = self.device.begin_frame()?;
        self.command_buffer = Some(cmd);
        
        Ok(())
    }
    
    // End frame and present
    pub fn end_frame(&mut self) -> Result<(), RenderError> {
        if let Some(cmd) = self.command_buffer.take() {
            self.device.end_frame(cmd)?;
        }
        
        Ok(())
    }
    
    // Create a buffer on the GPU
    pub fn create_buffer(&mut self, desc: &BufferDesc) -> Result<BufferHandle, RenderError> {
        let buffer = self.device.create_buffer(desc.clone())
            .map_err(|e| RenderError::ResourceCreationFailed(format!("Failed to create buffer: {}", e)))?;
        
        Ok(buffer)
    }
    
    // Write data to a buffer
    pub fn write_buffer<T: bytemuck::Pod>(
        &self,
        buffer: BufferHandle,
        offset: u64,
        data: &[T]
    ) -> Result<(), RenderError> {
        self.device.write_buffer(buffer, offset, bytemuck::cast_slice(data))
            .map_err(|e| RenderError::ResourceUpdateFailed(format!("Failed to write buffer: {}", e)))?;
            
        Ok(())
    }
    
    // Create a texture
    pub fn create_texture(&mut self, desc: &TextureDesc) -> Result<TextureHandle, RenderError> {
        let texture = self.device.create_texture(desc.clone())
            .map_err(|e| RenderError::ResourceCreationFailed(format!("Failed to create texture: {}", e)))?;
            
        Ok(texture)
    }
    
    // Create a pipeline
    pub fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<PipelineHandle, RenderError> {
        let pipeline = self.device.create_render_pipeline(desc.clone())
            .map_err(|e| RenderError::ResourceCreationFailed(format!("Failed to create pipeline: {}", e)))?;
            
        Ok(pipeline)
    }
    
    // Get or create a pipeline from cache
    pub fn get_or_create_pipeline(&mut self, key: PipelineKey) -> Result<PipelineHandle, RenderError> {
        if let Some(pipeline) = self.pipeline_cache.get(&key) {
            return Ok(*pipeline);
        }
        
        // Create new pipeline based on key
        let pipeline_desc = key.to_pipeline_desc();
        let pipeline = self.create_pipeline(&pipeline_desc)?;
        
        // Cache for future use
        self.pipeline_cache.insert(key, pipeline);
        
        Ok(pipeline)
    }
    
    // Execute a command list using the command translator
    pub fn execute_command_list(&mut self, command_list: &CommandList, resources: &ResourceCache) -> Result<(), RenderError> {
        let cmd = self.command_buffer.as_mut()
            .ok_or_else(|| RenderError::InvalidState("No active command buffer".into()))?;
        
        let mut translator = CommandTranslator::new(self);
        translator.translate_command_list(command_list, cmd, resources)?;
        
        Ok(())
    }
    
    // Get current command buffer
    pub fn current_command_buffer(&mut self) -> Result<&mut CommandBuffer, RenderError> {
        self.command_buffer.as_mut()
            .ok_or_else(|| RenderError::InvalidState("No active command buffer".into()))
    }
    
    // Get device reference
    pub fn device(&self) -> &GpuDevice {
        &self.device
    }
}
```

### 5.2 Command Translation

The command translation layer converts high-level operations into GPU commands:

```rust
// Command translation for operations
pub struct CommandTranslator {
    gpu_interface: GpuInterface,
    pipeline_cache: HashMap<PipelineKey, PipelineHandle>,
}

impl CommandTranslator {
    pub fn new(gpu_interface: GpuInterface) -> Self {
        Self {
            gpu_interface,
            pipeline_cache: HashMap::new(),
        }
    }
    
    // Translate a command list to GPU commands
    pub fn translate_command_list(
        &mut self, 
        command_list: &CommandList,
        cmd: &mut CommandBuffer,
        resources: &ResourceCache
    ) -> Result<(), RenderError> {
        // Begin translation
        for operation in &command_list.operations {
            self.translate_operation(operation, cmd, resources)?;
        }
        
        Ok(())
    }
    
    // Translate a single operation to GPU commands
    fn translate_operation(
        &mut self,
        operation: &dyn Operation,
        cmd: &mut CommandBuffer,
        resources: &ResourceCache
    ) -> Result<(), RenderError> {
        // State tracking for the operation
        let mut state = OperationState::new();
        
        // Execute the operation which will call back to this translator
        // through the renderer implementation
        operation.execute(&mut OperationRenderer {
            cmd,
            resources,
            translator: self,
            state: &mut state,
        })?;
        
        Ok(())
    }
    
    // Handle a geometry draw with specific style
    pub fn translate_draw_geometry(
        &mut self,
        cmd: &mut CommandBuffer,
        geometry: &GeometryData,
        style: &dyn Style,
        state: &OperationState,
        resources: &ResourceCache
    ) -> Result<(), RenderError> {
        // Get or create appropriate pipeline for this style
        let pipeline_key = PipelineKey::from_style(style);
        let pipeline = self.get_or_create_pipeline(pipeline_key)?;
        
        // Set pipeline
        cmd.set_pipeline(pipeline);
        
        // Prepare GPU resources
        let (vertex_buffer, index_buffer) = self.prepare_geometry(geometry, resources)?;
        
        // Set uniforms based on operation state
        self.set_operation_uniforms(cmd, state)?;
        
        // Set style-specific uniforms
        style.set_uniforms(cmd)?;
        
        // Bind geometry and draw
        cmd.set_vertex_buffer(0, vertex_buffer);
        cmd.set_index_buffer(index_buffer);
        cmd.draw_indexed(geometry.indices.len() as u32, 1, 0, 0, 0);
        
        Ok(())
    }
    
    // Other translation methods for specific operations...
    
    // Get or create pipeline based on style requirements
    fn get_or_create_pipeline(&mut self, key: PipelineKey) -> Result<PipelineHandle, RenderError> {
        if let Some(pipeline) = self.pipeline_cache.get(&key) {
            return Ok(*pipeline);
        }
        
        // Create new pipeline based on key
        let pipeline_desc = key.to_pipeline_desc();
        let pipeline = self.gpu_interface.create_pipeline(&pipeline_desc)?;
        
        // Cache for future use
        self.pipeline_cache.insert(key, pipeline);
        
        Ok(pipeline)
    }
    
    // Prepare geometry for rendering
    fn prepare_geometry(
        &mut self,
        geometry: &GeometryData, 
        resources: &ResourceCache
    ) -> Result<(BufferHandle, BufferHandle), RenderError> {
        // Check if geometry is already in the GPU
        if let Some(buffers) = resources.get_geometry_buffers(geometry.handle) {
            return Ok(buffers);
        }
        
        // Upload geometry to GPU
        let vertex_buffer = self.gpu_interface.create_buffer(
            &BufferDesc::vertex_buffer(
                geometry.vertices.len() * std::mem::size_of::<Vertex>(),
                BufferUsage::VERTEX
            )
        )?;
        
        self.gpu_interface.write_buffer(
            vertex_buffer,
            0,
            bytemuck::cast_slice(&geometry.vertices)
        )?;
        
        let index_buffer = self.gpu_interface.create_buffer(
            &BufferDesc::index_buffer(
                geometry.indices.len() * std::mem::size_of::<u32>(),
                BufferUsage::INDEX
            )
        )?;
        
        self.gpu_interface.write_buffer(
            index_buffer,
            0,
            bytemuck::cast_slice(&geometry.indices)
        )?;
        
        Ok((vertex_buffer, index_buffer))
    }
    
    // Set operation uniforms (transform, clip, etc.)
    fn set_operation_uniforms(
        &self,
        cmd: &mut CommandBuffer,
        state: &OperationState
    ) -> Result<(), RenderError> {
        // Create uniform buffer with operation state
        let uniform_data = OperationUniforms {
            transform: state.transform.to_mat4(),
            clip_rect: state.clip.map(|r| [r.x, r.y, r.width, r.height]).unwrap_or([0.0, 0.0, 0.0, 0.0]),
            has_clip: state.clip.is_some() as u32,
        };
        
        // Upload uniforms
        cmd.set_push_constants(
            ShaderStage::VERTEX | ShaderStage::FRAGMENT,
            0,
            bytemuck::cast_slice(&[uniform_data])
        );
        
        Ok(())
    }
}

// Renderer implementation that translates operations to commands
struct OperationRenderer<'a> {
    cmd: &'a mut CommandBuffer,
    resources: &'a ResourceCache,
    translator: &'a mut CommandTranslator,
    state: &'a mut OperationState,
}

impl<'a> Renderer for OperationRenderer<'a> {
    // State management methods
    fn push_state(&mut self) {
        self.state.push();
    }
    
    fn pop_state(&mut self) -> Result<(), RenderError> {
        self.state.pop()
    }
    
    fn set_transform(&mut self, transform: Transform) {
        self.state.transform = transform;
    }
    
    fn set_clip(&mut self, clip: Rect) {
        self.state.clip = Some(clip);
    }
    
    // Drawing methods
    fn fill_geometry(
        &mut self,
        geometry: &GeometryData,
        paint: &Paint,
        rule: FillRule
    ) -> Result<(), RenderError> {
        let fill_style = Fill {
            paint: paint.clone(),
            rule,
        };
        
        self.translator.translate_draw_geometry(
            self.cmd,
            geometry,
            &fill_style,
            self.state,
            self.resources
        )
    }
    
    // Other renderer methods for stroke, text, etc.
    // ...
}

// State tracking for operations
struct OperationState {
    transform_stack: Vec<Transform>,
    transform: Transform,
    clip_stack: Vec<Option<Rect>>,
    clip: Option<Rect>,
}

impl OperationState {
    fn new() -> Self {
        Self {
            transform_stack: Vec::new(),
            transform: Transform::identity(),
            clip_stack: Vec::new(),
            clip: None,
        }
    }
    
    fn push(&mut self) {
        self.transform_stack.push(self.transform);
        self.clip_stack.push(self.clip);
    }
    
    fn pop(&mut self) -> Result<(), RenderError> {
        if let Some(transform) = self.transform_stack.pop() {
            self.transform = transform;
        } else {
            return Err(RenderError::StackUnderflow("Transform stack empty".into()));
        }
        
        if let Some(clip) = self.clip_stack.pop() {
            self.clip = clip;
        } else {
            return Err(RenderError::StackUnderflow("Clip stack empty".into()));
        }
        
        Ok(())
    }
}

// Key for pipeline caching
#[derive(Clone, PartialEq, Eq, Hash)]
struct PipelineKey {
    shader_type: ShaderType,
    blend_mode: BlendMode,
    depth_test: bool,
    stencil_mode: StencilMode,
}

impl PipelineKey {
    fn from_style(style: &dyn Style) -> Self {
        // Create key based on style requirements
        // Implementation...
        Self {
            shader_type: ShaderType::Fill, // Default
            blend_mode: BlendMode::Alpha,
            depth_test: false,
            stencil_mode: StencilMode::None,
        }
    }
    
    fn to_pipeline_desc(&self) -> PipelineDesc {
        // Create pipeline description from key
        // Implementation...
        PipelineDesc::default()
    }
}
```

This design translates the high-level operation-based model into GPU commands. Instead of dispatching based on command enum variants, it uses operation execution with callbacks to the renderer, which then delegates to the translator for actual GPU command generation.

### 5.3 State Management

```rust
// State management for the rendering context
pub struct RenderState {
    transform_stack: Vec<Transform>,
    current_transform: Transform,
    scissor_stack: Vec<Option<Rect>>,
    current_scissor: Option<Rect>,
    stencil_level: u32,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            transform_stack: Vec::new(),
            current_transform: Transform::identity(),
            scissor_stack: Vec::new(),
            current_scissor: None,
            stencil_level: 0,
        }
    }
    
    // Push the current transform onto the stack
    pub fn push_transform(&mut self) {
        self.transform_stack.push(self.current_transform);
    }
    
    // Pop transform from the stack
    pub fn pop_transform(&mut self) -> Result<(), RenderError> {
        if let Some(transform) = self.transform_stack.pop() {
            self.current_transform = transform;
            Ok(())
        } else {
            Err(RenderError::StackUnderflow("Transform stack empty".into()))
        }
    }
    
    // Apply current state to GPU commands
    pub fn apply_to_command_buffer(&self, cmd: &mut CommandBuffer) {
        // Apply transform as push constants or uniforms
        
        // Apply scissor if any
        if let Some(scissor) = &self.current_scissor {
            cmd.set_scissor(
                scissor.x as u32,
                scissor.y as u32,
                scissor.width as u32,
                scissor.height as u32
            );
        } else {
            // Reset scissor to full viewport
        }
    }
}
```

### 5.4 Resource Binding

```rust
// Resource binding for shaders
pub struct ResourceBinding {
    pub resource_type: ResourceBindingType,
    pub handle: ResourceId,
    pub binding: u32,
    pub set: u32,
}

pub enum ResourceBindingType {
    Buffer,
    Texture,
    Sampler,
}

// Bind resources to a pipeline
pub fn bind_resources(cmd: &mut CommandBuffer, pipeline: PipelineHandle, bindings: &[ResourceBinding]) -> Result<(), RenderError> {
    cmd.set_pipeline(pipeline);
    
    for binding in bindings {
        match binding.resource_type {
            ResourceBindingType::Buffer => {
                // Bind buffer
            },
            ResourceBindingType::Texture => {
                // Bind texture
            },
            ResourceBindingType::Sampler => {
                // Bind sampler
            },
        }
    }
    
    Ok(())
}
```

## 6. Performance Optimizations

### 6.1 Batching

```rust
// Operation batching system for improved performance
pub struct BatchingSystem {
    batches: HashMap<BatchKey, OperationBatch>,
}

#[derive(PartialEq, Eq, Hash)]
struct BatchKey {
    style_type: StyleType,
    texture: Option<TextureHandle>,
    blend_mode: BlendMode,
}

// Style types for batching
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum StyleType {
    Fill,
    Stroke,
    Text,
    Custom(u32), // For user-defined styles
}

// Batch of operations that can be rendered together
struct OperationBatch {
    // Combined geometry data
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    
    // Operation count for debugging
    operation_count: usize,
    
    // Common style properties
    style_data: Box<dyn Any>,
    
    // Instance data for transforms and properties
    instance_data: Vec<InstanceData>,
}

// Instance data for batched rendering
#[repr(C)]
struct InstanceData {
    transform: [f32; 16],
    color: [f32; 4],
    clip_rect: [f32; 4],
    custom_data: [f32; 4],
}

impl BatchingSystem {
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }
    
    // Try to add an operation to a batch
    pub fn try_batch(
        &mut self, 
        operation: &dyn Operation,
        renderer: &Renderer
    ) -> Result<bool, RenderError> {
        // Check if the operation can be batched
        if let Some(batchable) = operation.as_batchable() {
            // Get or create batch for this operation
            let key = batchable.batch_key();
            let batch = self.batches.entry(key).or_insert_with(|| OperationBatch::new());
            
            // Add operation to batch
            batchable.add_to_batch(batch, renderer)?;
            
            return Ok(true);
        }
        
        Ok(false)
    }
    
    // Execute all batches
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        for (key, batch) in &self.batches {
            // Set up for this batch type
            match key.style_type {
                StyleType::Fill => self.execute_fill_batch(renderer, batch)?,
                StyleType::Stroke => self.execute_stroke_batch(renderer, batch)?,
                StyleType::Text => self.execute_text_batch(renderer, batch)?,
                StyleType::Custom(id) => self.execute_custom_batch(renderer, batch, id)?,
            }
        }
        
        Ok(())
    }
    
    // Execute a fill batch
    fn execute_fill_batch(
        &self, 
        renderer: &mut dyn Renderer, 
        batch: &OperationBatch
    ) -> Result<(), RenderError> {
        // Create combined geometry
        let geometry = GeometryData {
            vertices: batch.vertices.clone(),
            indices: batch.indices.clone(),
            handle: GeometryHandle(0), // Temporary handle
        };
        
        // Extract fill style data
        let fill_data = batch.style_data.downcast_ref::<FillBatchData>()
            .ok_or_else(|| RenderError::InvalidState("Invalid batch data type".into()))?;
        
        // Set up instanced rendering
        renderer.set_instance_data(&batch.instance_data)?;
        
        // Draw the batched geometry
        renderer.fill_geometry_instanced(
            &geometry,
            &fill_data.paint,
            fill_data.rule,
            batch.instance_data.len()
        )?;
        
        Ok(())
    }
    
    // Execute a stroke batch
    fn execute_stroke_batch(
        &self, 
        renderer: &mut dyn Renderer, 
        batch: &OperationBatch
    ) -> Result<(), RenderError> {
        // Similar to fill batch but for strokes
        // Implementation...
        Ok(())
    }
    
    // Execute a text batch
    fn execute_text_batch(
        &self, 
        renderer: &mut dyn Renderer, 
        batch: &OperationBatch
    ) -> Result<(), RenderError> {
        // Implementation for text batching
        // ...
        Ok(())
    }
    
    // Execute a custom batch
    fn execute_custom_batch(
        &self, 
        renderer: &mut dyn Renderer, 
        batch: &OperationBatch,
        id: u32
    ) -> Result<(), RenderError> {
        // Implementation for custom batching
        // ...
        Ok(())
    }
    
    // Reset all batches
    pub fn clear(&mut self) {
        self.batches.clear();
    }
}

// Trait for operations that can be batched
pub trait Batchable {
    // Get a key that identifies this batch type
    fn batch_key(&self) -> BatchKey;
    
    // Add this operation to a batch
    fn add_to_batch(&self, batch: &mut OperationBatch, renderer: &Renderer) -> Result<(), RenderError>;
}

// Allow operations to be batchable
impl<D: Drawable> Operation<D> {
    // Check if this operation can be batched
    pub fn is_batchable(&self) -> bool {
        // Operations can be batched if:
        // 1. They have exactly one style
        // 2. The style is a common type (Fill, Stroke, etc.)
        // 3. They have no effects
        // 4. The drawable is simple enough
        
        if self.styles.len() != 1 || !self.effects.is_empty() {
            return false;
        }
        
        // Check if the style is a batchable type
        self.styles[0].is_batchable() 
            && self.drawable.is_batchable()
    }
}

// Extension for CommandOperation to get batchable interface
trait BatchableOperation {
    fn as_batchable(&self) -> Option<&dyn Batchable>;
}

// Example implementation for a fill style batch
struct FillBatchData {
    paint: Paint,
    rule: FillRule,
}

// Implement Batchable for Fill operations
impl<D: Drawable + 'static> Batchable for Operation<D> 
where
    D: BatchableDrawable,
{
    fn batch_key(&self) -> BatchKey {
        // Create batch key from the operation
        // For simplicity, this example only handles fill operations
        let style = &self.styles[0];
        
        if let Some(fill) = style.as_any().downcast_ref::<Fill>() {
            let texture = match &fill.paint {
                Paint::Solid(_) => None,
                Paint::LinearGradient(_) => None,
                Paint::RadialGradient(_) => None,
                Paint::Texture(texture) => Some(*texture),
                // Other paint types...
            };
            
            BatchKey {
                style_type: StyleType::Fill,
                texture,
                blend_mode: BlendMode::Alpha, // Default
            }
        } else {
            // Fallback for other style types
            BatchKey {
                style_type: StyleType::Custom(0),
                texture: None,
                blend_mode: BlendMode::Alpha,
            }
        }
    }
    
    fn add_to_batch(&self, batch: &mut OperationBatch, renderer: &Renderer) -> Result<(), RenderError> {
        // Get geometry from drawable
        let mut geometry = self.drawable.to_geometry(renderer.context())?;
        
        // Create instance data
        let instance = InstanceData {
            transform: self.transform.to_mat4_array(),
            color: [1.0, 1.0, 1.0, 1.0], // Default
            clip_rect: self.clip
                .map(|r| [r.x, r.y, r.width, r.height])
                .unwrap_or([0.0, 0.0, 0.0, 0.0]),
            custom_data: [0.0, 0.0, 0.0, 0.0],
        };
        
        // Update instance color if using solid fill
        if let Some(fill) = self.styles[0].as_any().downcast_ref::<Fill>() {
            if let Paint::Solid(color) = &fill.paint {
                let [r, g, b, a] = color.to_rgba_array();
                let instance_with_color = InstanceData {
                    color: [r, g, b, a],
                    ..instance
                };
                
                batch.instance_data.push(instance_with_color);
            } else {
                batch.instance_data.push(instance);
            }
            
            // Store fill data if this is the first operation
            if batch.operation_count == 0 {
                batch.style_data = Box::new(FillBatchData {
                    paint: fill.paint.clone(),
                    rule: fill.rule,
                });
            }
        }
        
        // Add geometry to batch
        let index_offset = batch.vertices.len() as u32;
        
        // Adjust indices for the offset
        for index in &mut geometry.indices {
            *index += index_offset;
        }
        
        // Add vertices and indices to batch
        batch.vertices.extend_from_slice(&geometry.vertices);
        batch.indices.extend_from_slice(&geometry.indices);
        
        // Increment operation count
        batch.operation_count += 1;
        
        Ok(())
    }
}

// Trait for drawables that can be batched
pub trait BatchableDrawable {
    fn is_batchable(&self) -> bool;
}

// Example implementation for Rectangle
impl BatchableDrawable for Rectangle {
    fn is_batchable(&self) -> bool {
        true // Rectangles can always be batched
    }
}

// Example implementation for Circle
impl BatchableDrawable for Circle {
    fn is_batchable(&self) -> bool {
        true // Circles can be batched
    }
}

// May not be batchable if complex
impl BatchableDrawable for Path {
    fn is_batchable(&self) -> bool {
        // Paths can be batched if they are not too complex
        self.commands.len() < 100
    }
}
```

This batching system groups compatible operations together to reduce draw calls and improve performance. Operations with similar styles and properties can be batched, while still preserving their individual transformations and other attributes through instanced rendering.

## 7. Error Handling

### 7.1 Error Types

The rendering system uses a structured error type for clear and informative error reporting.

```rust
/// Render error type with detailed information
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to create resource: {0}")]
    ResourceCreationFailed(String),
    
    #[error("Invalid resource handle: {0}")]
    InvalidResourceHandle(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Stack underflow: {0}")]
    StackUnderflow(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Text layout error: {0}")]
    TextLayoutError(String),
    
    #[error("Path tessellation error: {0}")]
    PathTessellationError(String),
    
    #[error("GPU error: {0}")]
    GpuError(#[from] GpuError),
    
    #[error("Unexpected error: {0}")]
    Other(String),
}

// Additional error context
pub struct ErrorContext {
    pub file: &'static str,
    pub line: u32,
    pub operation: &'static str,
}

// Macro for creating detailed errors
#[macro_export]
macro_rules! render_error {
    ($type:expr, $message:expr) => {
        $type(format!("{} (at {}:{})", $message, file!(), line!()))
    };
    
    ($type:expr, $fmt:expr, $($arg:tt)*) => {
        $type(format!("{} (at {}:{})", format!($fmt, $($arg)*), file!(), line!()))
    };
}
```

### 7.2 Recovery Strategies

The error system includes strategies for recovering from errors and continuing rendering when possible.

```rust
/// Error recovery options
pub enum RecoveryAction {
    /// Continue with default/fallback values
    UseFallback,
    
    /// Retry the operation
    Retry,
    
    /// Skip the current operation
    Skip,
    
    /// Abort the entire rendering process
    Abort,
}

/// Context-aware error handling trait
pub trait WithRecovery {
    /// Handle an error with possible recovery
    fn handle_error<T>(
        &self,
        error: RenderError,
        operation: &str,
        recovery: impl FnOnce(RecoveryAction) -> Result<T, RenderError>
    ) -> Result<T, RenderError>;
}

impl WithRecovery for Renderer {
    fn handle_error<T>(
        &self,
        error: RenderError,
        operation: &str,
        recovery: impl FnOnce(RecoveryAction) -> Result<T, RenderError>
    ) -> Result<T, RenderError> {
        // Log the error
        log::error!("Render error in {}: {}", operation, error);
        
        // Determine recovery action based on error type and severity
        let action = match &error {
            RenderError::ResourceCreationFailed(_) => {
                if self.config.allow_fallbacks {
                    RecoveryAction::UseFallback
                } else {
                    RecoveryAction::Abort
                }
            },
            RenderError::InvalidResourceHandle(_) => RecoveryAction::Skip,
            // Other error types...
            _ => RecoveryAction::Abort,
        };
        
        // Execute recovery strategy
        recovery(action)
    }
}

// Example usage
impl Renderer {
    pub fn draw_text(&mut self, text: &str, position: Point, font: FontHandle) -> Result<(), RenderError> {
        let result = self.text_pipeline.render_text(text, position, font);
        
        if let Err(error) = result {
            return self.handle_error(error, "draw_text", |action| {
                match action {
                    RecoveryAction::UseFallback => {
                        // Use fallback font
                        let fallback_font = self.get_fallback_font();
                        self.text_pipeline.render_text(text, position, fallback_font)
                    },
                    RecoveryAction::Skip => Ok(()), // Skip this text
                    _ => Err(error), // Propagate error
                }
            });
        }
        
        Ok(())
    }
}
```

## 8. Feature Flags and Compilation

### 8.1 Feature Flags

Cargo features control which components are compiled into the binary, allowing users to customize the crate for their needs.

```toml
# In Cargo.toml
[features]
default = ["vector", "text", "effect"]

# Core APIs
vector = []
text = []
three_d = []

# Advanced features
effect = []
animation = []
svg = ["vector"]
high_quality = []

# Optimizations
parallel = ["rayon"]
caching = []
small_size = []

# Debug features
debug_visualization = []
profile = []
capture = []
```

### 8.2 Conditional Compilation

The code uses conditional compilation to include only the needed components.

```rust
// Conditional API modules
#[cfg(feature = "vector")]
pub mod vector;

#[cfg(feature = "text")]
pub mod text;

#[cfg(feature = "three_d")]
pub mod three_d;

// Conditional pipeline creation
impl Renderer {
    pub fn new(device: &GpuDevice) -> Result<Self, RenderError> {
        let mut renderer = Self {
            gpu_interface: GpuInterface::new(device.clone()),
            resource_cache: ResourceCache::new(),
            
            #[cfg(feature = "vector")]
            vector_pipeline: VectorPipeline::new(device)?,
            
            #[cfg(feature = "text")]
            text_pipeline: TextPipeline::new(device)?,
            
            #[cfg(feature = "three_d")]
            three_d_pipeline: ThreeDPipeline::new(device)?,
            
            #[cfg(feature = "effect")]
            effect_pipeline: EffectPipeline::new(device)?,
            
            #[cfg(feature = "parallel")]
            parallel_renderer: ParallelRenderer::new(num_cpus::get()),
            
            // Default render state
            render_state: RenderState::new(),
        };
        
        #[cfg(feature = "caching")]
        {
            renderer.setup_caches();
        }
        
        Ok(renderer)
    }
}

// Debug-only functionality
#[cfg(feature = "debug_visualization")]
impl Renderer {
    // Show debug wireframes for geometry
    pub fn show_wireframes(&mut self, enabled: bool) {
        self.debug_options.show_wireframes = enabled;
    }
    
    // Display bounding boxes
    pub fn show_bounds(&mut self, enabled: bool) {
        self.debug_options.show_bounds = enabled;
    }
}

// Size optimizations
#[cfg(feature = "small_size")]
type InternalString = SmallString<[u8; 24]>;

#[cfg(not(feature = "small_size"))]
type InternalString = String;
```

## 9. Project Structure

The complete project structure organizes the code into logical modules that reflect the architecture layers.

```
vectron_render/
├── Cargo.toml          # Package definition with features
├── build.rs            # Build script for shader compilation
│
├── src/
│   ├── lib.rs          # Main entry point and public API
│   │
│   ├── core/           # Core traits and abstractions
│   │   ├── mod.rs      # Core exports
│   │   ├── drawable.rs # Drawable trait implementation
│   │   ├── style.rs    # Style trait and implementations
│   │   ├── effect.rs   # Effect trait and implementations
│   │   ├── color.rs    # Color types and operations
│   │   ├── math/       # Unified math for 2D/3D operations
│   │   │   ├── mod.rs
│   │   │   ├── vector.rs      # 2D/3D vectors
│   │   │   ├── matrix.rs      # Matrix math (2x3, 3x3, 4x4)
│   │   │   ├── quaternion.rs  # Quaternion rotations
│   │   │   └── geometry.rs    # Basic geometric primitives
│   │   │
│   │   ├── transform.rs # Unified 2D/3D transform system
│   │   └── operation.rs # Operation composition system
│   │
│   ├── api/            # API layer modules
│   │   ├── mod.rs      # Exports from all API tiers
│   │   ├── bare/       # Low-level drawing API
│   │   ├── standard/   # Standard rendering API
│   │   ├── vector/     # 2D vector graphics API
│   │   ├── text/       # Text rendering API
│   │   └── three_d/    # 3D rendering API (specialized extensions)
│   │
│   ├── commands/       # Command system implementation
│   │   ├── mod.rs
│   │   ├── list.rs     # Command list management
│   │   ├── execution.rs # Command execution logic
│   │   └── batch.rs    # Command batching optimization
│   │
│   ├── resources/      # Resource management
│   │   ├── mod.rs
│   │   ├── cache.rs    # Resource caching system
│   │   ├── geometry.rs # Generic geometry management (2D & 3D)
│   │   ├── texture.rs  # Texture management
│   │   └── font.rs     # Font management
│   │
│   ├── pipelines/      # Rendering pipelines
│   │   ├── mod.rs
│   │   ├── vector.rs   # Vector rendering pipeline
│   │   ├── text.rs     # Text rendering pipeline
│   │   ├── three_d.rs  # 3D rendering pipeline
│   │   └── effects.rs  # Post-processing effects
│   │
│   ├── backend/        # Backend abstraction
│   │   ├── mod.rs
│   │   ├── gpu.rs      # GPU interface
│   │   ├── commands.rs # Command translation
│   │   ├── state.rs    # State management
│   │   └── binding.rs  # Resource binding
│   │
│   ├── utils/          # Utility modules
│   │   ├── mod.rs
│   │   ├── math.rs     # Math utilities
│   │   ├── path.rs     # Path utilities
│   │   └── profiler.rs # Performance profiling
│   │
│   ├── shaders/        # Shader management
│   │   ├── mod.rs
│   │   ├── vector/     # Vector rendering shaders
│   │   ├── text/       # Text rendering shaders
│   │   ├── three_d/    # 3D rendering shaders
│   │   └── effects/    # Post-processing shaders
│   │
│   └── error.rs        # Error handling system
│
├── examples/           # Example applications
│   ├── simple_shapes.rs
│   ├── text_layout.rs
│   ├── vector_graph.rs
│   ├── command_composition.rs # Demo of the operation composition system
│   ├── three_d_scene.rs
│   └── unified_2d_3d.rs       # Demo showing combined 2D and 3D rendering
│
└── tests/              # Integration tests
    ├── api_tests.rs
    ├── pipeline_tests.rs
    ├── operation_tests.rs # Tests for the operation system
    └── integration_tests.rs
```

## Conclusion

The Vectron Render crate provides a comprehensive, flexible, and high-performance rendering system that builds on the Vectron GPU abstraction. Its tiered API design allows developers to work at their preferred level of abstraction, from low-level drawing commands to high-level vector and 3D rendering.

Key features include:
- Multiple API tiers for different levels of control and abstraction
- Comprehensive vector, text, and 3D rendering capabilities
- Unified math framework supporting both 2D and 3D operations
- Dimension-agnostic core traits with specialized implementations
- Efficient resource management with caching and pooling
- Specialized rendering pipelines optimized for different tasks
- Core color system for consistent color representation across the renderer
- Performance optimizations including batching, culling, and parallelism
- Robust error handling with recovery strategies
- Flexible compilation with feature flags for customization

This architecture provides a solid foundation for building complex rendering applications with consistent, clean APIs that scale from simple 2D interfaces to complex 3D scenes, all using the same underlying rendering system.

