# Refining the D2 Module and API for 2D Rendering

Similar to our approach with the D3 API, we can design the D2 module and API to be inspired by established 2D libraries while maintaining Rust idioms. For this, we can draw inspiration from popular 2D libraries like Paper.js, Pixi.js, and the Canvas API, while incorporating elements from Rust libraries like Nannou and Druid.

## D2 Module Structure and Core Concepts

Let's revise the D2 module structure to support a comprehensive 2D rendering system:

```
elements/
├── d2/
│   ├── mod.rs                 # Module exports
│   ├── canvas.rs              # Canvas definition (viewport)
│   ├── layer.rs               # Layer for grouping elements
│   │
│   ├── path/                  # Path elements
│   │   ├── mod.rs             # Path module exports
│   │   ├── path.rs            # Core path definition
│   │   ├── curve.rs           # Curve segments (Bezier, etc.)
│   │   ├── segment.rs         # Path segments
│   │   ├── commands.rs        # Path commands (MoveTo, LineTo, etc.)
│   │   └── tessellation.rs    # Path tessellation
│   │
│   ├── shapes/                # Basic shapes
│   │   ├── mod.rs             # Shapes module exports
│   │   ├── rectangle.rs       # Rectangle shape
│   │   ├── ellipse.rs         # Ellipse/circle shape
│   │   ├── polygon.rs         # Polygon shape
│   │   ├── line.rs            # Line shape
│   │   └── star.rs            # Star shape
│   │
│   ├── text/                  # Text elements
│   │   ├── mod.rs             # Text module exports
│   │   ├── text.rs            # Text element
│   │   ├── text_span.rs       # Styled text span
│   │   ├── paragraph.rs       # Text paragraph
│   │   ├── font.rs            # Font handling
│   │   └── layout.rs          # Text layout engine
│   │
│   ├── image/                 # Image handling
│   │   ├── mod.rs             # Image module exports
│   │   ├── bitmap.rs          # Bitmap image
│   │   ├── svg.rs             # SVG image
│   │   └── filters.rs         # Image filters
│   │
│   ├── composite/             # Composite elements
│   │   ├── mod.rs             # Composite module exports
│   │   ├── group.rs           # Group element
│   │   ├── mask.rs            # Mask element
│   │   └── clip_path.rs       # Clipping path
│   │
│   ├── style/                 # Style definitions
│   │   ├── mod.rs             # Style module exports
│   │   ├── fill.rs            # Fill style
│   │   ├── stroke.rs          # Stroke style
│   │   ├── gradient.rs        # Gradient definitions
│   │   ├── pattern.rs         # Pattern definitions
│   │   └── shadow.rs          # Shadow style
│   │
│   ├── animation/             # Animation support
│   │   ├── mod.rs             # Animation module exports
│   │   ├── tween.rs           # Tween animation
│   │   ├── easing.rs          # Easing functions
│   │   └── timeline.rs        # Animation timeline
│   │
│   ├── interactions/          # Interactive elements
│   │   ├── mod.rs             # Interactions module exports
│   │   ├── hitbox.rs          # Hit testing
│   │   └── events.rs          # Event handling
│   │
│   └── camera.rs              # 2D camera (pan/zoom)
```

## Core Abstractions for D2

Let's define some of the key types and traits for the D2 module:

```rust
// Basic geometry primitives
pub struct Point {
    pub x: Unit<f32>,
    pub y: Unit<f32>,
}

pub struct Size {
    pub width: Unit<f32>,
    pub height: Unit<f32>,
}

pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

// Path definitions
pub struct PathSegment {
    pub kind: SegmentKind,
    pub points: Vec<Point>,
}

pub enum SegmentKind {
    MoveTo,
    LineTo,
    CurveTo,
    QuadTo,
    ArcTo,
    Close,
}

pub struct Path {
    pub segments: Vec<PathSegment>,
    pub fill_rule: FillRule,
    pub origin: Point,
}

// Style definitions
pub struct Fill {
    pub paint: Paint,
    pub rule: FillRule,
    pub alpha: f32,
    pub blend_mode: BlendMode,
}

pub struct Stroke {
    pub paint: Paint,
    pub width: Unit<f32>,
    pub line_join: LineJoin,
    pub line_cap: LineCap,
    pub dash_pattern: Option<DashPattern>,
    pub alpha: f32,
    pub blend_mode: BlendMode,
}

pub enum Paint {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    ConicGradient(ConicGradient),
    Pattern(PatternHandle),
}

// Text support
pub struct TextRun {
    pub text: String,
    pub font: FontHandle,
    pub font_size: Unit<f32>,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub fill: Fill,
    pub stroke: Option<Stroke>,
    pub origin: Point,
    pub anchor: TextAnchor,
    pub baseline: TextBaseline,
}

// Canvas and layers
pub struct Canvas {
    pub size: Size,
    pub background: Option<Paint>,
    pub layers: Vec<Layer>,
    pub view_transform: Transform,
}

pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub alpha: f32,
    pub blend_mode: BlendMode,
    pub elements: Vec<Box<dyn Renderable2D>>,
}
```

## D2 API Design

Now let's create a 2D API layer inspired by libraries like Paper.js but with Rust idioms:

```rust
// In src/api/mod.rs
pub mod d2; // D2 API module

// In src/api/d2/mod.rs
//! Paper.js-inspired API for 2D rendering in Rust

use crate::core::{self, Transform, Units};
use crate::elements::d2 as elements;
use crate::renderer::RenderContext;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Main entry point
pub struct Project {
    inner: elements::Canvas,
    active_layer: usize,
}

impl Project {
    pub fn new(width: impl Into<Units<f32>>, height: impl Into<Units<f32>>) -> Self {
        let size = elements::Size { 
            width: width.into(), 
            height: height.into() 
        };
        
        let mut canvas = elements::Canvas::new(size);
        // Create a default layer
        canvas.layers.push(elements::Layer::new("Layer 1"));
        
        Self {
            inner: canvas,
            active_layer: 0,
        }
    }
    
    pub fn layer(&self) -> &Layer {
        &self.inner.layers[self.active_layer]
    }
    
    pub fn layer_mut(&mut self) -> &mut Layer {
        &mut self.inner.layers[self.active_layer]
    }
    
    pub fn add_layer(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        self.inner.layers.push(elements::Layer::new(name));
        let index = self.inner.layers.len() - 1;
        self.active_layer = index;
        index
    }
    
    pub fn activate_layer(&mut self, index: usize) {
        if index < self.inner.layers.len() {
            self.active_layer = index;
        }
    }
    
    pub fn view(&self) -> View {
        View {
            transform: self.inner.view_transform.clone(),
        }
    }
    
    pub fn view_mut(&mut self) -> ViewMut {
        ViewMut {
            transform: &mut self.inner.view_transform,
        }
    }
}

// Layer management
pub struct Layer {
    inner: elements::Layer,
}

impl Layer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            inner: elements::Layer::new(name.into()),
        }
    }
    
    pub fn add_item(&mut self, item: impl Into<Item>) {
        let item = item.into();
        self.inner.elements.push(item.into_renderable());
    }
    
    pub fn clear(&mut self) {
        self.inner.elements.clear();
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.inner.visible = visible;
    }
    
    pub fn set_opacity(&mut self, opacity: f32) {
        self.inner.alpha = opacity.clamp(0.0, 1.0);
    }
}

// View management
pub struct View {
    transform: Transform,
}

impl View {
    pub fn center(&self) -> Point {
        // Implementation
        Point::new(0.0, 0.0)
    }
    
    pub fn zoom(&self) -> f32 {
        // Extract scale from transform
        1.0
    }
}

pub struct ViewMut<'a> {
    transform: &'a mut Transform,
}

impl<'a> ViewMut<'a> {
    pub fn set_center(&mut self, x: f32, y: f32) {
        // Update transform to center view
    }
    
    pub fn set_zoom(&mut self, zoom: f32) {
        // Update transform scale
    }
    
    pub fn translate(&mut self, dx: f32, dy: f32) {
        // Update transform translation
    }
    
    pub fn rotate(&mut self, angle: f32) {
        // Update transform rotation
    }
}

// Point type
pub struct Point {
    inner: elements::Point,
}

impl Point {
    pub fn new(x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> Self {
        Self {
            inner: elements::Point::new(x.into(), y.into()),
        }
    }
    
    pub fn x(&self) -> f32 {
        self.inner.x.value
    }
    
    pub fn y(&self) -> f32 {
        self.inner.y.value
    }
    
    pub fn distance_to(&self, other: &Point) -> f32 {
        // Calculate distance
        0.0
    }
}

// Size type
pub struct Size {
    inner: elements::Size,
}

impl Size {
    pub fn new(width: impl Into<Units<f32>>, height: impl Into<Units<f32>>) -> Self {
        Self {
            inner: elements::Size::new(width.into(), height.into()),
        }
    }
    
    pub fn width(&self) -> f32 {
        self.inner.width.value
    }
    
    pub fn height(&self) -> f32 {
        self.inner.height.value
    }
}

// Rectangle
pub struct Rectangle {
    inner: elements::shapes::Rectangle,
}

impl Rectangle {
    pub fn new(x: impl Into<Units<f32>>, y: impl Into<Units<f32>>, 
               width: impl Into<Units<f32>>, height: impl Into<Units<f32>>) -> Self {
        Self {
            inner: elements::shapes::Rectangle::new(
                x.into(), y.into(), width.into(), height.into()
            ),
        }
    }
    
    pub fn with_rounded_corners(mut self, radius: impl Into<Units<f32>>) -> Self {
        self.inner.set_corner_radius(radius.into());
        self
    }
    
    pub fn set_size(&mut self, width: impl Into<Units<f32>>, height: impl Into<Units<f32>>) {
        self.inner.size.width = width.into();
        self.inner.size.height = height.into();
    }
    
    pub fn center(&self) -> Point {
        // Calculate center
        Point::new(0.0, 0.0)
    }
}

// Circle
pub struct Circle {
    inner: elements::shapes::Ellipse,
}

impl Circle {
    pub fn new(x: impl Into<Units<f32>>, y: impl Into<Units<f32>>, radius: impl Into<Units<f32>>) -> Self {
        let radius = radius.into();
        Self {
            inner: elements::shapes::Ellipse::new(
                x.into(), y.into(), radius, radius
            ),
        }
    }
    
    pub fn radius(&self) -> f32 {
        self.inner.radius_x.value
    }
    
    pub fn set_radius(&mut self, radius: impl Into<Units<f32>>) {
        let radius = radius.into();
        self.inner.radius_x = radius;
        self.inner.radius_y = radius;
    }
}

// Path
pub struct Path {
    inner: elements::path::Path,
}

impl Path {
    pub fn new() -> Self {
        Self {
            inner: elements::path::Path::new(),
        }
    }
    
    pub fn move_to(&mut self, x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> &mut Self {
        self.inner.move_to(x.into(), y.into());
        self
    }
    
    pub fn line_to(&mut self, x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> &mut Self {
        self.inner.line_to(x.into(), y.into());
        self
    }
    
    pub fn curve_to(&mut self, 
                   cp1x: impl Into<Units<f32>>, cp1y: impl Into<Units<f32>>,
                   cp2x: impl Into<Units<f32>>, cp2y: impl Into<Units<f32>>,
                   x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> &mut Self {
        self.inner.curve_to(
            cp1x.into(), cp1y.into(),
            cp2x.into(), cp2y.into(),
            x.into(), y.into()
        );
        self
    }
    
    pub fn close(&mut self) -> &mut Self {
        self.inner.close();
        self
    }
    
    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }
}

// Path builder
pub struct PathBuilder {
    path: Path,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            path: Path::new(),
        }
    }
    
    pub fn move_to(mut self, x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> Self {
        self.path.move_to(x, y);
        self
    }
    
    pub fn line_to(mut self, x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> Self {
        self.path.line_to(x, y);
        self
    }
    
    pub fn curve_to(mut self, 
                   cp1x: impl Into<Units<f32>>, cp1y: impl Into<Units<f32>>,
                   cp2x: impl Into<Units<f32>>, cp2y: impl Into<Units<f32>>,
                   x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> Self {
        self.path.curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
        self
    }
    
    pub fn close(mut self) -> Self {
        self.path.close();
        self
    }
    
    pub fn build(self) -> Path {
        self.path
    }
}

// Text
pub struct Text {
    inner: elements::text::TextRun,
}

impl Text {
    pub fn new(content: impl Into<String>, x: impl Into<Units<f32>>, y: impl Into<Units<f32>>) -> Self {
        Self {
            inner: elements::text::TextRun::new(content.into(), x.into(), y.into()),
        }
    }
    
    pub fn with_font_size(mut self, size: impl Into<Units<f32>>) -> Self {
        self.inner.font_size = size.into();
        self
    }
    
    pub fn with_font(mut self, font: impl Into<String>) -> Self {
        self.inner.font = elements::text::FontHandle::new(font.into());
        self
    }
    
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.inner.fill.paint = elements::style::Paint::Solid(color.into().inner);
        self
    }
    
    pub fn content(&self) -> &str {
        &self.inner.text
    }
    
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.inner.text = content.into();
    }
}

// Style types
pub struct Color {
    inner: elements::style::Color,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            inner: elements::style::Color::rgb(r, g, b),
        }
    }
    
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: elements::style::Color::rgba(r, g, b, a),
        }
    }
    
    pub fn hex(hex: &str) -> Result<Self, ColorError> {
        let inner = elements::style::Color::from_hex(hex)?;
        Ok(Self { inner })
    }
}

pub struct FillStyle {
    inner: elements::style::Fill,
}

impl FillStyle {
    pub fn color(color: impl Into<Color>) -> Self {
        Self {
            inner: elements::style::Fill::new(elements::style::Paint::Solid(color.into().inner)),
        }
    }
    
    pub fn linear_gradient(gradient: LinearGradient) -> Self {
        Self {
            inner: elements::style::Fill::new(elements::style::Paint::LinearGradient(gradient.inner)),
        }
    }
}

pub struct StrokeStyle {
    inner: elements::style::Stroke,
}

impl StrokeStyle {
    pub fn new(color: impl Into<Color>, width: impl Into<Units<f32>>) -> Self {
        Self {
            inner: elements::style::Stroke::new(
                elements::style::Paint::Solid(color.into().inner),
                width.into(),
            ),
        }
    }
    
    pub fn with_line_join(mut self, join: LineJoin) -> Self {
        self.inner.line_join = join;
        self
    }
    
    pub fn with_line_cap(mut self, cap: LineCap) -> Self {
        self.inner.line_cap = cap;
        self
    }
    
    pub fn with_dash_pattern(mut self, pattern: &[f32]) -> Self {
        self.inner.dash_pattern = Some(elements::style::DashPattern::new(pattern.to_vec()));
        self
    }
}

// Common type for all drawable items
pub enum Item {
    Path(Path),
    Rectangle(Rectangle),
    Circle(Circle),
    Text(Text),
    // ...other types
}

impl Item {
    fn into_renderable(self) -> Box<dyn elements::Renderable2D> {
        match self {
            Item::Path(path) => Box::new(path.inner),
            Item::Rectangle(rect) => Box::new(rect.inner),
            Item::Circle(circle) => Box::new(circle.inner),
            Item::Text(text) => Box::new(text.inner),
        }
    }
}

// Implement conversions from specific types to Item
impl From<Path> for Item {
    fn from(path: Path) -> Self {
        Item::Path(path)
    }
}

impl From<Rectangle> for Item {
    fn from(rect: Rectangle) -> Self {
        Item::Rectangle(rect)
    }
}

impl From<Circle> for Item {
    fn from(circle: Circle) -> Self {
        Item::Circle(circle)
    }
}

impl From<Text> for Item {
    fn from(text: Text) -> Self {
        Item::Text(text)
    }
}

// Renderer
pub struct Renderer {
    context: RenderContext,
}

impl Renderer {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }
    
    pub fn render(&mut self, project: &Project) {
        // Implementation
    }
}
```

## Example Usage

With this API, users could create 2D graphics in an intuitive, Rust-idiomatic way:

```rust
use render_engine::api::d2::{self, Project, Path, Rectangle, Circle, Text, Color, FillStyle, StrokeStyle};

// Create a new project (canvas)
let mut project = Project::new(800.0, 600.0);

// Create a rectangle
let rect = Rectangle::new(100.0, 100.0, 200.0, 150.0)
    .with_rounded_corners(10.0);

// Create a circle
let circle = Circle::new(400.0, 300.0, 80.0);

// Create a path
let path = Path::builder()
    .move_to(300.0, 200.0)
    .line_to(500.0, 200.0)
    .line_to(500.0, 400.0)
    .curve_to(450.0, 450.0, 350.0, 450.0, 300.0, 400.0)
    .close()
    .build();

// Create text
let text = Text::new("Hello, Render Engine!", 250.0, 100.0)
    .with_font_size(24.0)
    .with_font("Arial")
    .with_color(Color::rgb(0.1, 0.1, 0.8));

// Add items to the active layer
project.layer_mut().add_item(rect);
project.layer_mut().add_item(circle);
project.layer_mut().add_item(path);
project.layer_mut().add_item(text);

// Create a new layer
let layer_index = project.add_layer("Foreground");

// Add content to the new layer
let small_circle = Circle::new(150.0, 150.0, 30.0);
project.layer_mut().add_item(small_circle);

// Pan and zoom
project.view_mut().set_center(400.0, 300.0);
project.view_mut().set_zoom(1.2);

// Create renderer and render
let mut renderer = d2::Renderer::new(render_context);
renderer.render(&project);
```

## Advanced Features

### SVG Support

```rust
// SVG path parsing
pub fn path_from_svg(svg_path: &str) -> Result<Path, ParseError> {
    // Parse SVG path commands
    let mut path = Path::new();
    // Implementation
    Ok(path)
}

// SVG import
pub fn load_svg(path: &std::path::Path) -> Result<Group, LoadError> {
    // Load and parse SVG file
    // Create appropriate elements
    // Return as a group
    unimplemented!()
}
```

### Interactive Elements

```rust
// Hit testing
pub trait Interactive {
    fn contains_point(&self, point: &Point) -> bool;
    fn distance_to_point(&self, point: &Point) -> f32;
}

impl Interactive for Path {
    fn contains_point(&self, point: &Point) -> bool {
        // Implementation
        false
    }
    
    fn distance_to_point(&self, point: &Point) -> f32 {
        // Implementation
        0.0
    }
}

// Event handling
pub struct EventHandler {
    on_click: Option<Box<dyn Fn(&Point)>>,
    on_hover: Option<Box<dyn Fn(&Point, bool)>>,
    on_drag: Option<Box<dyn Fn(&Point, &Point)>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            on_click: None,
            on_hover: None,
            on_drag: None,
        }
    }
    
    pub fn on_click<F: Fn(&Point) + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    
    pub fn on_hover<F: Fn(&Point, bool) + 'static>(mut self, handler: F) -> Self {
        self.on_hover = Some(Box::new(handler));
        self
    }
    
    pub fn on_drag<F: Fn(&Point, &Point) + 'static>(mut self, handler: F) -> Self {
        self.on_drag = Some(Box::new(handler));
        self
    }
}

// Extensions for interactive elements
impl Rectangle {
    pub fn with_event_handler(mut self, handler: EventHandler) -> Self {
        // Attach handler
        self
    }
}
```

### Effects and Filters

```rust
pub struct Shadow {
    inner: elements::style::Shadow,
}

impl Shadow {
    pub fn new(color: impl Into<Color>, x_offset: f32, y_offset: f32, blur: f32) -> Self {
        Self {
            inner: elements::style::Shadow::new(
                color.into().inner,
                [x_offset, y_offset].into(),
                blur,
            ),
        }
    }
}

pub struct Blur {
    inner: elements::style::Blur,
}

impl Blur {
    pub fn new(radius: f32) -> Self {
        Self {
            inner: elements::style::Blur::new(radius),
        }
    }
}

// Apply effects to items
impl Rectangle {
    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        // Apply shadow effect
        self
    }
    
    pub fn with_blur(mut self, blur: Blur) -> Self {
        // Apply blur effect
        self
    }
}
```

## Conclusion

This D2 API design provides:

1. A Paper.js/Canvas-inspired API that feels familiar to web developers
2. Rust idioms like builder patterns, strong typing, and ownership semantics
3. A comprehensive set of 2D drawing capabilities
4. A layer-based approach for organizing elements
5. Flexible styling options with gradients, patterns, and effects
6. Support for interactive elements
7. A clear separation between the public API and the internal elements

The design balances ease of use with performance considerations, leveraging the core engine's capabilities while providing a more ergonomic interface for 2D graphics programming. This approach allows developers to work at a higher level of abstraction while still having access to the full power of the underlying rendering engine when needed.