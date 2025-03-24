// Core renderer types and traits
pub mod drawable;
pub mod style;
pub mod effect;
pub mod operation;
pub mod color;
pub mod math;
pub mod transform;

// Re-export commonly used types
pub use drawable::{Drawable, Drawable2D, Drawable3D, Rectangle, Circle, Text, Box3D, Sphere, Path, Mesh3D, RenderContext, RenderHints};
pub use style::{Style, Fill, Stroke};
pub use effect::{Effect, Shadow, Glow, Blur};
pub use operation::Operation;
pub use color::Color;
pub use transform::{Transform, TransformStack};
pub use math::{Vec2, Vec3, Vec4, Mat3, Mat4, Quaternion, Rect, BoundingBox, BoundingSphere, BoundingVolume, LineSegment, Point};