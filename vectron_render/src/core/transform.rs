use crate::core::types::{Point2D, Point3D, Vec2, Vec3, Vec4};
use std::ops::Mul;

/// Column-major 3x3 matrix for 2D transformations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    // Stored as a flat array in column-major order
    // [c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2]
    pub elements: [f32; 9],
}

/// Column-major 4x4 matrix for 3D transformations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    // Stored as a flat array in column-major order
    // [c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, ...]
    pub elements: [f32; 16],
}

/// 2D affine transform (internally uses a 3x3 matrix)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    pub matrix: Mat3,
}

/// 3D transform (internally uses a 4x4 matrix)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    pub matrix: Mat4,
}

/// Generic transform that can be either 2D or 3D
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transform {
    Transform2D(Transform2D),
    Transform3D(Transform3D),
}

impl Mat3 {
    /// Create a new 3x3 matrix from individual elements in column-major order
    pub fn new(elements: [f32; 9]) -> Self {
        Self { elements }
    }
    
    /// Create an identity matrix
    pub fn identity() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Get an element at the specified row and column
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.elements[col * 3 + row]
    }
    
    /// Set an element at the specified row and column
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.elements[col * 3 + row] = value;
    }
}

impl Mat4 {
    /// Create a new 4x4 matrix from individual elements in column-major order
    pub fn new(elements: [f32; 16]) -> Self {
        Self { elements }
    }
    
    /// Create an identity matrix
    pub fn identity() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Get an element at the specified row and column
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.elements[col * 4 + row]
    }
    
    /// Set an element at the specified row and column
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.elements[col * 4 + row] = value;
    }
}

impl Mul for Mat3 {
    type Output = Mat3;
    
    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut result = Mat3::new([0.0; 9]);
        
        for row in 0..3 {
            for col in 0..3 {
                let mut sum = 0.0;
                for i in 0..3 {
                    sum += self.get(row, i) * rhs.get(i, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut result = Mat4::new([0.0; 16]);
        
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += self.get(row, i) * rhs.get(i, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl Transform2D {
    /// Create a new 2D transform from a 3x3 matrix
    pub fn new(matrix: Mat3) -> Self {
        Self { matrix }
    }
    
    /// Create an identity transform
    pub fn identity() -> Self {
        Self { matrix: Mat3::identity() }
    }
    
    /// Create a translation transform
    pub fn translation(x: f32, y: f32) -> Self {
        let mut matrix = Mat3::identity();
        matrix.set(0, 2, x);
        matrix.set(1, 2, y);
        Self { matrix }
    }
    
    /// Create a rotation transform
    pub fn rotation(radians: f32) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        
        let mut matrix = Mat3::identity();
        matrix.set(0, 0, cos);
        matrix.set(0, 1, -sin);
        matrix.set(1, 0, sin);
        matrix.set(1, 1, cos);
        
        Self { matrix }
    }
    
    /// Create a scaling transform
    pub fn scaling(x: f32, y: f32) -> Self {
        let mut matrix = Mat3::identity();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        
        Self { matrix }
    }
    
    /// Create a skew transform
    pub fn skew(x: f32, y: f32) -> Self {
        let mut matrix = Mat3::identity();
        matrix.set(0, 1, x.tan());
        matrix.set(1, 0, y.tan());
        
        Self { matrix }
    }
    
    /// Apply this transform to a 2D point
    pub fn transform_point(&self, point: Point2D) -> Point2D {
        let x = point.x * self.matrix.get(0, 0) + point.y * self.matrix.get(0, 1) + self.matrix.get(0, 2);
        let y = point.x * self.matrix.get(1, 0) + point.y * self.matrix.get(1, 1) + self.matrix.get(1, 2);
        
        Point2D::new(x, y)
    }
    
    /// Combine this transform with another (this * other)
    pub fn then(&self, other: &Transform2D) -> Self {
        Self {
            matrix: self.matrix * other.matrix
        }
    }
}

impl Transform3D {
    /// Create a new 3D transform from a 4x4 matrix
    pub fn new(matrix: Mat4) -> Self {
        Self { matrix }
    }
    
    /// Create an identity transform
    pub fn identity() -> Self {
        Self { matrix: Mat4::identity() }
    }
    
    /// Create a translation transform
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Mat4::identity();
        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);
        
        Self { matrix }
    }
    
    /// Create a rotation around the X axis
    pub fn rotation_x(radians: f32) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        
        let mut matrix = Mat4::identity();
        matrix.set(1, 1, cos);
        matrix.set(1, 2, -sin);
        matrix.set(2, 1, sin);
        matrix.set(2, 2, cos);
        
        Self { matrix }
    }
    
    /// Create a rotation around the Y axis
    pub fn rotation_y(radians: f32) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        
        let mut matrix = Mat4::identity();
        matrix.set(0, 0, cos);
        matrix.set(0, 2, sin);
        matrix.set(2, 0, -sin);
        matrix.set(2, 2, cos);
        
        Self { matrix }
    }
    
    /// Create a rotation around the Z axis
    pub fn rotation_z(radians: f32) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        
        let mut matrix = Mat4::identity();
        matrix.set(0, 0, cos);
        matrix.set(0, 1, -sin);
        matrix.set(1, 0, sin);
        matrix.set(1, 1, cos);
        
        Self { matrix }
    }
    
    /// Create a scaling transform
    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Mat4::identity();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);
        
        Self { matrix }
    }
    
    /// Create a look-at view transform
    pub fn look_at(eye: Point3D, target: Point3D, up: Vec3) -> Self {
        // Calculate forward (z), right (x), and up (y) vectors
        let forward = [
            eye.x - target.x,
            eye.y - target.y,
            eye.z - target.z,
        ];
        
        // Normalize forward
        let len = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
        let forward = [forward[0] / len, forward[1] / len, forward[2] / len];
        
        // Calculate right vector with cross product
        let right = [
            up[1] * forward[2] - up[2] * forward[1],
            up[2] * forward[0] - up[0] * forward[2],
            up[0] * forward[1] - up[1] * forward[0],
        ];
        
        // Normalize right
        let len = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
        let right = [right[0] / len, right[1] / len, right[2] / len];
        
        // Recalculate up vector
        let up = [
            forward[1] * right[2] - forward[2] * right[1],
            forward[2] * right[0] - forward[0] * right[2],
            forward[0] * right[1] - forward[1] * right[0],
        ];
        
        // Create view matrix
        let mut matrix = Mat4::identity();
        matrix.set(0, 0, right[0]);
        matrix.set(1, 0, right[1]);
        matrix.set(2, 0, right[2]);
        
        matrix.set(0, 1, up[0]);
        matrix.set(1, 1, up[1]);
        matrix.set(2, 1, up[2]);
        
        matrix.set(0, 2, forward[0]);
        matrix.set(1, 2, forward[1]);
        matrix.set(2, 2, forward[2]);
        
        matrix.set(0, 3, -right[0] * eye.x - right[1] * eye.y - right[2] * eye.z);
        matrix.set(1, 3, -up[0] * eye.x - up[1] * eye.y - up[2] * eye.z);
        matrix.set(2, 3, -forward[0] * eye.x - forward[1] * eye.y - forward[2] * eye.z);
        
        Self { matrix }
    }
    
    /// Create a perspective projection transform
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y / 2.0).tan();
        let range_inv = 1.0 / (near - far);
        
        let mut matrix = Mat4::new([0.0; 16]);
        matrix.set(0, 0, f / aspect);
        matrix.set(1, 1, f);
        matrix.set(2, 2, (near + far) * range_inv);
        matrix.set(2, 3, 2.0 * near * far * range_inv);
        matrix.set(3, 2, -1.0);
        
        Self { matrix }
    }
    
    /// Create an orthographic projection transform
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut matrix = Mat4::identity();
        matrix.set(0, 0, 2.0 / (right - left));
        matrix.set(1, 1, 2.0 / (top - bottom));
        matrix.set(2, 2, -2.0 / (far - near));
        
        matrix.set(0, 3, -(right + left) / (right - left));
        matrix.set(1, 3, -(top + bottom) / (top - bottom));
        matrix.set(2, 3, -(far + near) / (far - near));
        
        Self { matrix }
    }
    
    /// Apply this transform to a 3D point
    pub fn transform_point(&self, point: Point3D) -> Point3D {
        let x = point.x * self.matrix.get(0, 0) + 
               point.y * self.matrix.get(0, 1) + 
               point.z * self.matrix.get(0, 2) + 
               self.matrix.get(0, 3);
               
        let y = point.x * self.matrix.get(1, 0) + 
               point.y * self.matrix.get(1, 1) + 
               point.z * self.matrix.get(1, 2) + 
               self.matrix.get(1, 3);
               
        let z = point.x * self.matrix.get(2, 0) + 
               point.y * self.matrix.get(2, 1) + 
               point.z * self.matrix.get(2, 2) + 
               self.matrix.get(2, 3);
               
        let w = point.x * self.matrix.get(3, 0) + 
               point.y * self.matrix.get(3, 1) + 
               point.z * self.matrix.get(3, 2) + 
               self.matrix.get(3, 3);
               
        let w_inv = if w != 0.0 { 1.0 / w } else { 1.0 };
        
        Point3D::new(x * w_inv, y * w_inv, z * w_inv)
    }
    
    /// Combine this transform with another (this * other)
    pub fn then(&self, other: &Transform3D) -> Self {
        Self {
            matrix: self.matrix * other.matrix
        }
    }
}

impl Transform {
    /// Create an identity transform (2D by default)
    pub fn identity() -> Self {
        Self::Transform2D(Transform2D::identity())
    }
    
    /// Create a 2D transform from a Transform2D
    pub fn from_2d(transform: Transform2D) -> Self {
        Self::Transform2D(transform)
    }
    
    /// Create a 3D transform from a Transform3D
    pub fn from_3d(transform: Transform3D) -> Self {
        Self::Transform3D(transform)
    }
    
    /// Check if this is a 2D transform
    pub fn is_2d(&self) -> bool {
        matches!(self, Self::Transform2D(_))
    }
    
    /// Check if this is a 3D transform
    pub fn is_3d(&self) -> bool {
        matches!(self, Self::Transform3D(_))
    }
    
    /// Convert to a 2D transform
    pub fn to_2d(&self) -> Option<Transform2D> {
        match self {
            Self::Transform2D(transform) => Some(*transform),
            Self::Transform3D(_) => None,
        }
    }
    
    /// Convert to a 3D transform
    pub fn to_3d(&self) -> Option<Transform3D> {
        match self {
            Self::Transform2D(_) => None,
            Self::Transform3D(transform) => Some(*transform),
        }
    }
} 