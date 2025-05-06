/// Fundamental type definitions for the rendering engine

/// Common vector types
pub type Vec2 = [f32; 2];
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];

/// 2D Point representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

/// 3D Point representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 2D Size representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// 2D Rectangle representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 3D bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}

/// Enumeration of possible bounding volume types
#[derive(Debug, Clone, PartialEq)]
pub enum BoundingVolume {
    /// 2D rectangle
    Rect(Rect),
    /// 3D box
    Box(BoundingBox),
    /// Empty bounding volume
    Empty,
}

/// RGBA Color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Fill rule for determining inside/outside of shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillRule {
    /// Non-zero winding rule
    NonZero,
    /// Even-odd rule
    EvenOdd,
}

/// Line join style for strokes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    /// Miter join (sharp corner)
    Miter,
    /// Round join
    Round,
    /// Bevel join
    Bevel,
}

/// Line cap style for strokes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    /// Butt cap (flat end)
    Butt,
    /// Round cap
    Round,
    /// Square cap
    Square,
}

/// Dash pattern for stroked paths
#[derive(Debug, Clone, PartialEq)]
pub struct DashPattern {
    /// Array of dash and gap lengths
    pub segments: Vec<f32>,
    /// Offset to start the pattern
    pub offset: f32,
}

/// Blur direction options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurDirection {
    /// Horizontal blur only
    Horizontal,
    /// Vertical blur only
    Vertical,
    /// Both horizontal and vertical (full) blur
    Both,
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Left align text
    Left,
    /// Center align text
    Center,
    /// Right align text
    Right,
}

/// Handle types for resources
pub type GeometryHandle = u64;
pub type TextureHandle = u64;
pub type BufferHandle = u64;
pub type FontHandle = u64;
pub type MaterialHandle = u64;
pub type PipelineHandle = u64;
pub type PatternHandle = u64;

impl Point2D {
    /// Create a new 2D point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Convert to a 3D point with the given z coordinate
    pub fn to_3d(&self, z: f32) -> Point3D {
        Point3D::new(self.x, self.y, z)
    }
    
    /// Convert to a Vec2
    pub fn to_vec2(&self) -> Vec2 {
        [self.x, self.y]
    }
}

impl Point3D {
    /// Create a new 3D point
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    /// Convert to a 2D point (discarding z)
    pub fn to_2d(&self) -> Point2D {
        Point2D::new(self.x, self.y)
    }
    
    /// Convert to a Vec3
    pub fn to_vec3(&self) -> Vec3 {
        [self.x, self.y, self.z]
    }
}

impl Size {
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    /// Create a rectangle from two points
    pub fn from_points(p1: Point2D, p2: Point2D) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs();
        let height = (p1.y - p2.y).abs();
        Self { x, y, width, height }
    }
    
    /// Get the top-left point of the rectangle
    pub fn origin(&self) -> Point2D {
        Point2D::new(self.x, self.y)
    }
    
    /// Get the width and height as a Size
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    
    /// Check if this rectangle contains a point
    pub fn contains(&self, point: Point2D) -> bool {
        point.x >= self.x && 
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }
}

impl Color {
    /// Create a new RGBA color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a new RGB color with alpha=1.0
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
    
    /// Black color (0,0,0,1)
    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }
    
    /// White color (1,1,1,1)
    pub fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }
    
    /// Transparent color (0,0,0,0)
    pub fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
} 