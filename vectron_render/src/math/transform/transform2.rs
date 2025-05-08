use std::ops::{Mul, MulAssign};
use crate::math::matrix::Mat3;
use crate::math::vector::Vec2;

/// A 2D transformation represented internally as a 3x3 matrix.
/// Uses column-major ordering and homogeneous coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    /// The underlying 3x3 matrix in column-major order.
    /// Each 2D transform is stored as a 3x3 matrix to handle affine transformations.
    matrix: [f32; 9],
}

impl Transform2D {
    /// Creates a new 2D transform from the specified components of a 3x3 matrix.
    /// Components are specified in column-major order.
    #[inline]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32,
        c1r0: f32, c1r1: f32, c1r2: f32,
        c2r0: f32, c2r1: f32, c2r2: f32,
    ) -> Self {
        Self {
            matrix: [
                c0r0, c0r1, c0r2,
                c1r0, c1r1, c1r2,
                c2r0, c2r1, c2r2,
            ],
        }
    }
    
    /// Creates a new transform from a slice of 9 values in column-major order.
    #[inline]
    pub fn from_array(array: [f32; 9]) -> Self {
        Self { matrix: array }
    }
    
    /// Returns the matrix elements as an array in column-major order.
    #[inline]
    pub fn to_array(&self) -> [f32; 9] {
        self.matrix
    }
    
    /// Creates an identity transform.
    #[inline]
    pub fn identity() -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a translation transform.
    #[inline]
    pub fn translation(offset: Vec2) -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                offset.x, offset.y, 1.0,
            ],
        }
    }
    
    /// Creates a scaling transform.
    #[inline]
    pub fn scaling(scale: Vec2) -> Self {
        Self {
            matrix: [
                scale.x, 0.0, 0.0,
                0.0, scale.y, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a uniform scaling transform.
    #[inline]
    pub fn scaling_uniform(scale: f32) -> Self {
        Self {
            matrix: [
                scale, 0.0, 0.0,
                0.0, scale, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a rotation transform around the origin.
    /// Positive rotation is clockwise in 2D screen space.
    #[inline]
    pub fn rotation(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            matrix: [
                cos, sin, 0.0,
                -sin, cos, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a skew transform.
    #[inline]
    pub fn skew(x: f32, y: f32) -> Self {
        Self {
            matrix: [
                1.0, y, 0.0,
                x, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Transforms a 2D point.
    #[inline]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        let x = point.x * self.matrix[0] + point.y * self.matrix[3] + self.matrix[6];
        let y = point.x * self.matrix[1] + point.y * self.matrix[4] + self.matrix[7];
        let w = point.x * self.matrix[2] + point.y * self.matrix[5] + self.matrix[8];
        
        if w == 1.0 {
            Vec2::new(x, y)
        } else {
            let w_inv = 1.0 / w;
            Vec2::new(x * w_inv, y * w_inv)
        }
    }
    
    /// Transforms a 2D vector as a direction (ignores translation).
    #[inline]
    pub fn transform_vector(&self, vector: Vec2) -> Vec2 {
        Vec2::new(
            vector.x * self.matrix[0] + vector.y * self.matrix[3],
            vector.x * self.matrix[1] + vector.y * self.matrix[4],
        )
    }
    
    /// Returns the inverse of this transform, or None if the transform is not invertible.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        // Calculate the determinant of the 2x2 upper-left matrix
        let det = self.matrix[0] * self.matrix[4] - self.matrix[1] * self.matrix[3];
        if det == 0.0 {
            return None;
        }
        
        let inv_det = 1.0 / det;
        
        // Calculate the inverse
        let a = self.matrix[4] * inv_det;
        let b = -self.matrix[1] * inv_det;
        let c = -self.matrix[3] * inv_det;
        let d = self.matrix[0] * inv_det;
        
        // Calculate the translation part
        let tx = -a * self.matrix[6] - c * self.matrix[7];
        let ty = -b * self.matrix[6] - d * self.matrix[7];
        
        Some(Self {
            matrix: [
                a, b, 0.0,
                c, d, 0.0,
                tx, ty, 1.0,
            ],
        })
    }
    
    /// Converts this transform to a Mat3.
    #[inline]
    pub fn to_mat3(&self) -> Mat3 {
        Mat3::from_array(self.matrix)
    }
    
    /// Creates a transform from a Mat3.
    #[inline]
    pub fn from_mat3(mat: &Mat3) -> Self {
        Self {
            matrix: mat.to_array(),
        }
    }
    
    /// Gets a specific matrix element.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < 3 && col < 3, "Matrix indices out of bounds");
        self.matrix[col * 3 + row]
    }
    
    /// Sets a specific matrix element.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < 3 && col < 3, "Matrix indices out of bounds");
        self.matrix[col * 3 + row] = value;
    }
    
    /// Gets the translation component of this transform.
    #[inline]
    pub fn get_translation(&self) -> Vec2 {
        Vec2::new(self.matrix[6], self.matrix[7])
    }
    
    /// Sets the translation component of this transform.
    #[inline]
    pub fn set_translation(&mut self, translation: Vec2) {
        self.matrix[6] = translation.x;
        self.matrix[7] = translation.y;
    }
    
    /// Gets the rotation angle in radians of this transform.
    /// This assumes the transform contains no skew.
    #[inline]
    pub fn get_rotation(&self) -> f32 {
        // Extract rotation angle from the matrix
        // We can use atan2 on the elements of the first column
        // to get the rotation angle
        f32::atan2(self.matrix[1], self.matrix[0])
    }
    
    /// Gets the scale components of this transform.
    /// This assumes the transform contains no skew.
    #[inline]
    pub fn get_scale(&self) -> Vec2 {
        Vec2::new(
            f32::sqrt(self.matrix[0] * self.matrix[0] + self.matrix[1] * self.matrix[1]),
            f32::sqrt(self.matrix[3] * self.matrix[3] + self.matrix[4] * self.matrix[4]),
        )
    }
}

impl Default for Transform2D {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Transform2D {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        // Matrix multiplication in column-major order
        // This implements the operation: self * other
        // Which means: apply other first, then self
        
        let a = &self.matrix;
        let b = &other.matrix;
        
        // First row
        let c00 = a[0] * b[0] + a[3] * b[1] + a[6] * b[2];
        let c01 = a[0] * b[3] + a[3] * b[4] + a[6] * b[5];
        let c02 = a[0] * b[6] + a[3] * b[7] + a[6] * b[8];
        
        // Second row
        let c10 = a[1] * b[0] + a[4] * b[1] + a[7] * b[2];
        let c11 = a[1] * b[3] + a[4] * b[4] + a[7] * b[5];
        let c12 = a[1] * b[6] + a[4] * b[7] + a[7] * b[8];
        
        // Third row
        let c20 = a[2] * b[0] + a[5] * b[1] + a[8] * b[2];
        let c21 = a[2] * b[3] + a[5] * b[4] + a[8] * b[5];
        let c22 = a[2] * b[6] + a[5] * b[7] + a[8] * b[8];
        
        Self {
            matrix: [
                c00, c10, c20,
                c01, c11, c21,
                c02, c12, c22,
            ],
        }
    }
}

impl MulAssign for Transform2D {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// Implement Mul for Vec2 to allow transform * point operations
impl Mul<Vec2> for Transform2D {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, vector: Vec2) -> Vec2 {
        self.transform_point(vector)
    }
} 