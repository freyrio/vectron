use crate::core::geometry::point::{Point, Point2D, Point3D};
use crate::core::types::Dimensionality;
use crate::math::transform::{Transform2D, Transform3D};

/// A high-level transform interface for geometry types.
/// 
/// This module provides wrapper functionality to bridge between the
/// math module's transform implementations and the core geometry types.
/// It follows the conventions defined in the blueprint:
/// - All matrices are stored in column-major format
/// - Right-handed coordinate system for 3D
/// - 2D coordinate system with Y pointing down (screen space)
/// - Positive rotation is clockwise in 2D
/// - Transformations are applied in right-to-left order

/// A type to represent transforms that can operate in 2D or 3D space.
#[derive(Debug, Clone, Copy)]
pub enum Transform {
    D2(Transform2D),
    D3(Transform3D),
}

/// Transforms a Point using a Transform.
/// 
/// This is a high-level function to bridge between the math module's
/// transform types and the core geometry types.
#[inline]
pub fn transform_point(point: &Point, transform: &Transform) -> Option<Point> {
    match (point, transform) {
        (Point::D2(p), Transform::D2(t)) => {
            // Convert Point2D to Vec2, transform, and convert back
            let vec2 = p.to_vec2();
            let result = t.transform_point(vec2);
            Some(Point::D2(Point2D::from(result)))
        },
        (Point::D3(p), Transform::D3(t)) => {
            // Convert Point3D to Vec3, transform, and convert back
            let vec3 = p.to_vec3();
            let result = t.transform_point(vec3);
            Some(Point::D3(Point3D::from(result)))
        },
        _ => None // Mismatched dimensions
    }
}

/// Creates an identity transform for the specified dimensionality.
#[inline]
pub fn identity_transform(dim: Dimensionality) -> Transform {
    match dim {
        Dimensionality::D2 => Transform::D2(Transform2D::identity()),
        Dimensionality::D3 => Transform::D3(Transform3D::identity()),
    }
}

/// Creates a translation transform.
#[inline]
pub fn translation_transform(point: &Point) -> Transform {
    match point {
        Point::D2(p) => {
            Transform::D2(Transform2D::translation(p.to_vec2()))
        },
        Point::D3(p) => {
            Transform::D3(Transform3D::translation(p.to_vec3()))
        },
    }
}

/// Creates a scaling transform.
#[inline]
pub fn scaling_transform(scale: &Point) -> Transform {
    match scale {
        Point::D2(s) => {
            Transform::D2(Transform2D::scaling(s.to_vec2()))
        },
        Point::D3(s) => {
            Transform::D3(Transform3D::scaling(s.to_vec3()))
        },
    }
}

/// Creates a uniform scaling transform.
#[inline]
pub fn uniform_scaling_transform(scale: f32, dim: Dimensionality) -> Transform {
    match dim {
        Dimensionality::D2 => Transform::D2(Transform2D::scaling_uniform(scale)),
        Dimensionality::D3 => Transform::D3(Transform3D::scaling_uniform(scale)),
    }
}

/// Creates a 2D rotation transform (rotation around Z axis).
/// Positive rotation is clockwise in 2D screen space.
#[inline]
pub fn rotation_transform_2d(radians: f32) -> Transform {
    Transform::D2(Transform2D::rotation(radians))
}

/// Creates a 3D rotation transform around the X axis.
#[inline]
pub fn rotation_transform_x(radians: f32) -> Transform {
    Transform::D3(Transform3D::rotation_x(radians))
}

/// Creates a 3D rotation transform around the Y axis.
#[inline]
pub fn rotation_transform_y(radians: f32) -> Transform {
    Transform::D3(Transform3D::rotation_y(radians))
}

/// Creates a 3D rotation transform around the Z axis.
#[inline]
pub fn rotation_transform_z(radians: f32) -> Transform {
    Transform::D3(Transform3D::rotation_z(radians))
}

/// Creates a 3D rotation transform around an arbitrary axis.
#[inline]
pub fn rotation_transform_axis(axis: &Point3D, radians: f32) -> Transform {
    Transform::D3(Transform3D::rotation_axis(axis.to_vec3(), radians))
}

/// Creates a 3D perspective projection transform.
#[inline]
pub fn perspective_transform(fov_y_radians: f32, aspect_ratio: f32, near: f32, far: f32) -> Transform {
    Transform::D3(Transform3D::perspective(fov_y_radians, aspect_ratio, near, far))
}

/// Creates a 3D orthographic projection transform.
#[inline]
pub fn orthographic_transform(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Transform {
    Transform::D3(Transform3D::orthographic(left, right, bottom, top, near, far))
}

/// Creates a 3D look-at transform.
#[inline]
pub fn look_at_transform(eye: &Point3D, target: &Point3D, up: &Point3D) -> Transform {
    Transform::D3(Transform3D::look_at(
        eye.to_vec3(),
        target.to_vec3(),
        up.to_vec3()
    ))
}

/// Combines two transforms by multiplying them.
/// The resulting transform represents applying transform2 first, then transform1.
#[inline]
pub fn combine_transforms(transform1: &Transform, transform2: &Transform) -> Option<Transform> {
    match (transform1, transform2) {
        (Transform::D2(t1), Transform::D2(t2)) => {
            Some(Transform::D2(*t1 * *t2))
        },
        (Transform::D3(t1), Transform::D3(t2)) => {
            Some(Transform::D3(*t1 * *t2))
        },
        _ => None // Mismatched dimensions
    }
}

/// Returns the inverse of a transform, or None if not invertible.
#[inline]
pub fn inverse_transform(transform: &Transform) -> Option<Transform> {
    match transform {
        Transform::D2(t) => {
            t.inverse().map(Transform::D2)
        },
        Transform::D3(t) => {
            t.inverse().map(Transform::D3)
        },
    }
} 