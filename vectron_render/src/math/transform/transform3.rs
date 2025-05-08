use std::ops::{Mul, MulAssign};
use crate::math::matrix::Mat4;
use crate::math::vector::{Vec3, Vec4};

/// A 3D transformation represented internally as a 4x4 matrix.
/// Uses column-major ordering and homogeneous coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    /// The underlying 4x4 matrix in column-major order.
    /// Each 3D transform is stored as a 4x4 matrix to handle affine transformations.
    matrix: [f32; 16],
}

impl Transform3D {
    /// Creates a new 3D transform from the specified components of a 4x4 matrix.
    /// Components are specified in column-major order.
    #[inline]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32,
    ) -> Self {
        Self {
            matrix: [
                c0r0, c0r1, c0r2, c0r3,
                c1r0, c1r1, c1r2, c1r3,
                c2r0, c2r1, c2r2, c2r3,
                c3r0, c3r1, c3r2, c3r3,
            ],
        }
    }
    
    /// Creates a new transform from a slice of 16 values in column-major order.
    #[inline]
    pub fn from_array(array: [f32; 16]) -> Self {
        Self { matrix: array }
    }
    
    /// Returns the matrix elements as an array in column-major order.
    #[inline]
    pub fn to_array(&self) -> [f32; 16] {
        self.matrix
    }
    
    /// Creates an identity transform.
    #[inline]
    pub fn identity() -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a translation transform.
    #[inline]
    pub fn translation(offset: Vec3) -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                offset.x, offset.y, offset.z, 1.0,
            ],
        }
    }
    
    /// Creates a scaling transform.
    #[inline]
    pub fn scaling(scale: Vec3) -> Self {
        Self {
            matrix: [
                scale.x, 0.0, 0.0, 0.0,
                0.0, scale.y, 0.0, 0.0,
                0.0, 0.0, scale.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a uniform scaling transform.
    #[inline]
    pub fn scaling_uniform(scale: f32) -> Self {
        Self {
            matrix: [
                scale, 0.0, 0.0, 0.0,
                0.0, scale, 0.0, 0.0,
                0.0, 0.0, scale, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a rotation transform around the X axis.
    #[inline]
    pub fn rotation_x(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, cos, sin, 0.0,
                0.0, -sin, cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a rotation transform around the Y axis.
    #[inline]
    pub fn rotation_y(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            matrix: [
                cos, 0.0, -sin, 0.0,
                0.0, 1.0, 0.0, 0.0,
                sin, 0.0, cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a rotation transform around the Z axis.
    #[inline]
    pub fn rotation_z(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            matrix: [
                cos, sin, 0.0, 0.0,
                -sin, cos, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a rotation transform around an arbitrary axis.
    #[inline]
    pub fn rotation_axis(axis: Vec3, radians: f32) -> Self {
        let axis = axis.normalize();
        let (sin, cos) = radians.sin_cos();
        let one_minus_cos = 1.0 - cos;
        
        let xx = axis.x * axis.x;
        let xy = axis.x * axis.y;
        let xz = axis.x * axis.z;
        let yy = axis.y * axis.y;
        let yz = axis.y * axis.z;
        let zz = axis.z * axis.z;
        
        let xs = axis.x * sin;
        let ys = axis.y * sin;
        let zs = axis.z * sin;
        
        Self {
            matrix: [
                cos + xx * one_minus_cos, xy * one_minus_cos + zs, xz * one_minus_cos - ys, 0.0,
                xy * one_minus_cos - zs, cos + yy * one_minus_cos, yz * one_minus_cos + xs, 0.0,
                xz * one_minus_cos + ys, yz * one_minus_cos - xs, cos + zz * one_minus_cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a perspective projection transform.
    #[inline]
    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y_radians * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        
        Self {
            matrix: [
                f / aspect_ratio, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (near + far) * range_inv, -1.0,
                0.0, 0.0, 2.0 * far * near * range_inv, 0.0,
            ],
        }
    }
    
    /// Creates an orthographic projection transform.
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        
        Self {
            matrix: [
                2.0 / width, 0.0, 0.0, 0.0,
                0.0, 2.0 / height, 0.0, 0.0,
                0.0, 0.0, -2.0 / depth, 0.0,
                -(right + left) / width, -(top + bottom) / height, -(far + near) / depth, 1.0,
            ],
        }
    }
    
    /// Creates a view transform looking from 'eye' towards 'target' with 'up' orientation.
    #[inline]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        // Forward (z) is the negative direction from eye to target
        // (in right-handed coordinate system, camera looks down -z)
        let f = (target - eye).normalize();
        let r = f.cross(&up).normalize(); // Right (x) is perpendicular to forward and up
        let u = r.cross(&f); // Up (y) is perpendicular to forward and right
        
        Self {
            matrix: [
                r.x, u.x, -f.x, 0.0,
                r.y, u.y, -f.y, 0.0,
                r.z, u.z, -f.z, 0.0,
                -r.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0,
            ],
        }
    }
    
    /// Transforms a 3D point (homogeneous w=1).
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let x = point.x * self.matrix[0] + point.y * self.matrix[4] + point.z * self.matrix[8] + self.matrix[12];
        let y = point.x * self.matrix[1] + point.y * self.matrix[5] + point.z * self.matrix[9] + self.matrix[13];
        let z = point.x * self.matrix[2] + point.y * self.matrix[6] + point.z * self.matrix[10] + self.matrix[14];
        let w = point.x * self.matrix[3] + point.y * self.matrix[7] + point.z * self.matrix[11] + self.matrix[15];
        
        if w == 1.0 {
            Vec3::new(x, y, z)
        } else {
            let w_inv = 1.0 / w;
            Vec3::new(x * w_inv, y * w_inv, z * w_inv)
        }
    }
    
    /// Transforms a 3D vector as a direction (ignores translation, homogeneous w=0).
    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        Vec3::new(
            vector.x * self.matrix[0] + vector.y * self.matrix[4] + vector.z * self.matrix[8],
            vector.x * self.matrix[1] + vector.y * self.matrix[5] + vector.z * self.matrix[9],
            vector.x * self.matrix[2] + vector.y * self.matrix[6] + vector.z * self.matrix[10],
        )
    }
    
    /// Transforms a 4D vector.
    #[inline]
    pub fn transform_vec4(&self, vector: Vec4) -> Vec4 {
        Vec4::new(
            vector.x * self.matrix[0] + vector.y * self.matrix[4] + vector.z * self.matrix[8] + vector.w * self.matrix[12],
            vector.x * self.matrix[1] + vector.y * self.matrix[5] + vector.z * self.matrix[9] + vector.w * self.matrix[13],
            vector.x * self.matrix[2] + vector.y * self.matrix[6] + vector.z * self.matrix[10] + vector.w * self.matrix[14],
            vector.x * self.matrix[3] + vector.y * self.matrix[7] + vector.z * self.matrix[11] + vector.w * self.matrix[15],
        )
    }
    
    /// Returns the inverse of this transform, or None if the transform is not invertible.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        // Convert to Mat4, invert, and convert back
        Mat4::from_array(self.matrix).inverse().map(|m| Self::from_array(m.to_array()))
    }
    
    /// Converts this transform to a Mat4.
    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_array(self.matrix)
    }
    
    /// Creates a transform from a Mat4.
    #[inline]
    pub fn from_mat4(mat: &Mat4) -> Self {
        Self {
            matrix: mat.to_array(),
        }
    }
    
    /// Gets a specific matrix element.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < 4 && col < 4, "Matrix indices out of bounds");
        self.matrix[col * 4 + row]
    }
    
    /// Sets a specific matrix element.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < 4 && col < 4, "Matrix indices out of bounds");
        self.matrix[col * 4 + row] = value;
    }
    
    /// Gets the translation component of this transform.
    #[inline]
    pub fn get_translation(&self) -> Vec3 {
        Vec3::new(self.matrix[12], self.matrix[13], self.matrix[14])
    }
    
    /// Sets the translation component of this transform.
    #[inline]
    pub fn set_translation(&mut self, translation: Vec3) {
        self.matrix[12] = translation.x;
        self.matrix[13] = translation.y;
        self.matrix[14] = translation.z;
    }
    
    /// Gets the scale components of this transform.
    /// This assumes the transform contains no skew.
    #[inline]
    pub fn get_scale(&self) -> Vec3 {
        Vec3::new(
            Vec3::new(self.matrix[0], self.matrix[1], self.matrix[2]).length(),
            Vec3::new(self.matrix[4], self.matrix[5], self.matrix[6]).length(),
            Vec3::new(self.matrix[8], self.matrix[9], self.matrix[10]).length(),
        )
    }
}

impl Default for Transform3D {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Transform3D {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        // Matrix multiplication in column-major order
        // This implements the operation: self * other
        // Which means: apply other first, then self
        let mat = Mat4::from_array(self.matrix) * Mat4::from_array(other.matrix);
        Self::from_array(mat.to_array())
    }
}

impl MulAssign for Transform3D {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// Implement Mul for Vec3 to allow transform * point operations
impl Mul<Vec3> for Transform3D {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, vector: Vec3) -> Vec3 {
        self.transform_point(vector)
    }
}

// Implement Mul for Vec4 to allow transform * vector operations
impl Mul<Vec4> for Transform3D {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, vector: Vec4) -> Vec4 {
        self.transform_vec4(vector)
    }
} 