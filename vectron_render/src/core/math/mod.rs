/*!
 * Math module for the rendering system
 */

// Basic modules
pub mod vector;
pub mod matrix;
pub mod quaternion;
pub mod geometry;

// Re-export common types
pub use vector::{Vec2, Vec3, Vec4};
pub use matrix::{Mat3, Mat4};
pub use quaternion::Quaternion;
pub use geometry::{Rect, BoundingBox, BoundingSphere, BoundingVolume, LineSegment, Point};

// Re-export math constants
pub use std::f32::consts::{PI, TAU, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8};

/// Convert degrees to radians
#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Convert radians to degrees
#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / PI
}

/// Linear interpolation between two values
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Check if two floating point values are approximately equal
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() <= epsilon
} 