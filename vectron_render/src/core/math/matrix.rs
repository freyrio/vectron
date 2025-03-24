/*!
 * Matrix types for 2D and 3D transformations
 */
use std::ops::{Mul, Index, IndexMut};
use crate::core::math::{Vec2, Vec3, Vec4, to_radians, approx_eq};

/// 3x3 matrix for 2D transformations
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mat3 {
    /// Column-major matrix data [col][row]
    pub data: [[f32; 3]; 3],
}

impl Mat3 {
    /// Create a new matrix from individual elements
    #[inline]
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        Self {
            data: [
                [m00, m01, m02], // Column 0
                [m10, m11, m12], // Column 1
                [m20, m21, m22], // Column 2
            ],
        }
    }
    
    /// Create an identity matrix
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a translation matrix
    #[inline]
    pub fn translation(x: f32, y: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [x, y, 1.0],
            ],
        }
    }
    
    /// Create a scaling matrix
    #[inline]
    pub fn scaling(x: f32, y: f32) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0],
                [0.0, y, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix (angle in degrees)
    #[inline]
    pub fn rotation(angle_degrees: f32) -> Self {
        let angle = to_radians(angle_degrees);
        let cos = angle.cos();
        let sin = angle.sin();
        
        Self {
            data: [
                [cos, sin, 0.0],
                [-sin, cos, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a skew/shear matrix
    #[inline]
    pub fn skew(x: f32, y: f32) -> Self {
        Self {
            data: [
                [1.0, y, 0.0],
                [x, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Transform a 2D point
    #[inline]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        Vec2 {
            x: self.data[0][0] * point.x + self.data[1][0] * point.y + self.data[2][0],
            y: self.data[0][1] * point.x + self.data[1][1] * point.y + self.data[2][1],
        }
    }
    
    /// Transform a 2D vector (ignoring translation)
    #[inline]
    pub fn transform_vector(&self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: self.data[0][0] * vector.x + self.data[1][0] * vector.y,
            y: self.data[0][1] * vector.x + self.data[1][1] * vector.y,
        }
    }
    
    /// Get the determinant of the matrix
    #[inline]
    pub fn determinant(&self) -> f32 {
        let [[a, b, c], [d, e, f], [g, h, i]] = self.data;
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }
    
    /// Get the inverse of the matrix
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        
        if approx_eq(det, 0.0, 1e-6) {
            return None; // Matrix is not invertible
        }
        
        let inv_det = 1.0 / det;
        let [[a, b, c], [d, e, f], [g, h, i]] = self.data;
        
        // Compute adjugate and multiply by 1/det
        Some(Self {
            data: [
                [
                    (e * i - f * h) * inv_det,
                    (c * h - b * i) * inv_det,
                    (b * f - c * e) * inv_det,
                ],
                [
                    (f * g - d * i) * inv_det,
                    (a * i - c * g) * inv_det,
                    (c * d - a * f) * inv_det,
                ],
                [
                    (d * h - e * g) * inv_det,
                    (b * g - a * h) * inv_det,
                    (a * e - b * d) * inv_det,
                ],
            ],
        })
    }
    
    /// Transpose the matrix
    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2]],
            ],
        }
    }
    
    /// Convert to a flat array in column-major order
    #[inline]
    pub fn to_array(&self) -> [f32; 9] {
        [
            self.data[0][0], self.data[0][1], self.data[0][2],
            self.data[1][0], self.data[1][1], self.data[1][2],
            self.data[2][0], self.data[2][1], self.data[2][2],
        ]
    }
    
    /// Convert to a 4x4 matrix for 3D transformations
    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::new(
            self.data[0][0], self.data[0][1], self.data[0][2], 0.0,
            self.data[1][0], self.data[1][1], self.data[1][2], 0.0,
            self.data[2][0], self.data[2][1], self.data[2][2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

// Matrix multiplication
impl Mul for Mat3 {
    type Output = Mat3;
    
    #[inline]
    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut result = Mat3::identity();
        
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = 0.0;
                for k in 0..3 {
                    result.data[i][j] += self.data[k][j] * rhs.data[i][k];
                }
            }
        }
        
        result
    }
}

// Matrix multiplication with Vec3
impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.data[0][0] * rhs.x + self.data[1][0] * rhs.y + self.data[2][0] * rhs.z,
            self.data[0][1] * rhs.x + self.data[1][1] * rhs.y + self.data[2][1] * rhs.z,
            self.data[0][2] * rhs.x + self.data[1][2] * rhs.y + self.data[2][2] * rhs.z,
        )
    }
}

// Index operations for easier element access
impl Index<usize> for Mat3 {
    type Output = [f32; 3];
    
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// 4x4 matrix for 3D transformations
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mat4 {
    /// Column-major matrix data [col][row]
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    /// Create a new matrix from individual elements
    #[inline]
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        Self {
            data: [
                [m00, m01, m02, m03], // Column 0
                [m10, m11, m12, m13], // Column 1
                [m20, m21, m22, m23], // Column 2
                [m30, m31, m32, m33], // Column 3
            ],
        }
    }
    
    /// Create an identity matrix
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a translation matrix
    #[inline]
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }
    
    /// Create a scaling matrix
    #[inline]
    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix around the X axis
    #[inline]
    pub fn rotation_x(angle_degrees: f32) -> Self {
        let angle = to_radians(angle_degrees);
        let cos = angle.cos();
        let sin = angle.sin();
        
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix around the Y axis
    #[inline]
    pub fn rotation_y(angle_degrees: f32) -> Self {
        let angle = to_radians(angle_degrees);
        let cos = angle.cos();
        let sin = angle.sin();
        
        Self {
            data: [
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix around the Z axis
    #[inline]
    pub fn rotation_z(angle_degrees: f32) -> Self {
        let angle = to_radians(angle_degrees);
        let cos = angle.cos();
        let sin = angle.sin();
        
        Self {
            data: [
                [cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix from a quaternion
    #[inline]
    pub fn from_quaternion(q: &Quaternion) -> Self {
        let x = q.x;
        let y = q.y;
        let z = q.z;
        let w = q.w;
        
        let xx = x * x;
        let xy = x * y;
        let xz = x * z;
        let xw = x * w;
        
        let yy = y * y;
        let yz = y * z;
        let yw = y * w;
        
        let zz = z * z;
        let zw = z * w;
        
        Self {
            data: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy + zw), 2.0 * (xz - yw), 0.0],
                [2.0 * (xy - zw), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + xw), 0.0],
                [2.0 * (xz + yw), 2.0 * (yz - xw), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a perspective projection matrix
    #[inline]
    pub fn perspective(fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov = to_radians(fov_degrees);
        let f = 1.0 / (fov * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        
        Self {
            data: [
                [f / aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (near + far) * range_inv, -1.0],
                [0.0, 0.0, 2.0 * near * far * range_inv, 0.0],
            ],
        }
    }
    
    /// Create an orthographic projection matrix
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        
        Self {
            data: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, -2.0 / depth, 0.0],
                [-(right + left) / width, -(top + bottom) / height, -(far + near) / depth, 1.0],
            ],
        }
    }
    
    /// Create a look-at view matrix
    #[inline]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalized();
        let s = f.cross(&up).normalized();
        let u = s.cross(&f);
        
        Self {
            data: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-s.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0],
            ],
        }
    }
    
    /// Transform a 3D point (w=1)
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let x = self.data[0][0] * point.x + self.data[1][0] * point.y + self.data[2][0] * point.z + self.data[3][0];
        let y = self.data[0][1] * point.x + self.data[1][1] * point.y + self.data[2][1] * point.z + self.data[3][1];
        let z = self.data[0][2] * point.x + self.data[1][2] * point.y + self.data[2][2] * point.z + self.data[3][2];
        let w = self.data[0][3] * point.x + self.data[1][3] * point.y + self.data[2][3] * point.z + self.data[3][3];
        
        if w != 0.0 && w != 1.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }
    
    /// Transform a 3D vector (w=0)
    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        Vec3::new(
            self.data[0][0] * vector.x + self.data[1][0] * vector.y + self.data[2][0] * vector.z,
            self.data[0][1] * vector.x + self.data[1][1] * vector.y + self.data[2][1] * vector.z,
            self.data[0][2] * vector.x + self.data[1][2] * vector.y + self.data[2][2] * vector.z,
        )
    }
    
    /// Get the determinant of the matrix
    #[inline]
    pub fn determinant(&self) -> f32 {
        let [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self.data;
        
        a * f * k * p - a * f * l * o - a * g * j * p + a * g * l * n + a * h * j * o - a * h * k * n -
        b * e * k * p + b * e * l * o + b * g * i * p - b * g * l * m - b * h * i * o + b * h * k * m +
        c * e * j * p - c * e * l * n - c * f * i * p + c * f * l * m + c * h * i * n - c * h * j * m -
        d * e * j * o + d * e * k * n + d * f * i * o - d * f * k * m - d * g * i * n + d * g * j * m
    }
    
    /// Get the inverse of the matrix
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        // Implementation uses adjugate matrix / determinant
        // This is a bit long for inline implementation, but is standard
        
        let det = self.determinant();
        if approx_eq(det, 0.0, 1e-6) {
            return None; // Matrix is not invertible
        }
        
        let inv_det = 1.0 / det;
        let m = self.data;
        
        // Calculate cofactors, transpose, and multiply by 1/det
        // This calculation is verbose but straightforward
        Some(Self {
            data: [
                [
                    (m[1][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
                     m[1][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) +
                     m[1][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])) * inv_det,
                    
                    -(m[0][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
                      m[0][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) +
                      m[0][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])) * inv_det,
                    
                    (m[0][1] * (m[1][2] * m[3][3] - m[1][3] * m[3][2]) -
                     m[0][2] * (m[1][1] * m[3][3] - m[1][3] * m[3][1]) +
                     m[0][3] * (m[1][1] * m[3][2] - m[1][2] * m[3][1])) * inv_det,
                    
                    -(m[0][1] * (m[1][2] * m[2][3] - m[1][3] * m[2][2]) -
                      m[0][2] * (m[1][1] * m[2][3] - m[1][3] * m[2][1]) +
                      m[0][3] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])) * inv_det,
                ],
                [
                    -(m[1][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
                      m[1][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
                      m[1][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])) * inv_det,
                    
                    (m[0][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
                     m[0][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
                     m[0][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])) * inv_det,
                    
                    -(m[0][0] * (m[1][2] * m[3][3] - m[1][3] * m[3][2]) -
                      m[0][2] * (m[1][0] * m[3][3] - m[1][3] * m[3][0]) +
                      m[0][3] * (m[1][0] * m[3][2] - m[1][2] * m[3][0])) * inv_det,
                    
                    (m[0][0] * (m[1][2] * m[2][3] - m[1][3] * m[2][2]) -
                     m[0][2] * (m[1][0] * m[2][3] - m[1][3] * m[2][0]) +
                     m[0][3] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])) * inv_det,
                ],
                [
                    (m[1][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) -
                     m[1][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
                     m[1][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det,
                    
                    -(m[0][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) -
                      m[0][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
                      m[0][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det,
                    
                    (m[0][0] * (m[1][1] * m[3][3] - m[1][3] * m[3][1]) -
                     m[0][1] * (m[1][0] * m[3][3] - m[1][3] * m[3][0]) +
                     m[0][3] * (m[1][0] * m[3][1] - m[1][1] * m[3][0])) * inv_det,
                    
                    -(m[0][0] * (m[1][1] * m[2][3] - m[1][3] * m[2][1]) -
                      m[0][1] * (m[1][0] * m[2][3] - m[1][3] * m[2][0]) +
                      m[0][3] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])) * inv_det,
                ],
                [
                    -(m[1][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]) -
                      m[1][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]) +
                      m[1][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det,
                    
                    (m[0][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]) -
                     m[0][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]) +
                     m[0][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det,
                    
                    -(m[0][0] * (m[1][1] * m[3][2] - m[1][2] * m[3][1]) -
                      m[0][1] * (m[1][0] * m[3][2] - m[1][2] * m[3][0]) +
                      m[0][2] * (m[1][0] * m[3][1] - m[1][1] * m[3][0])) * inv_det,
                    
                    (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) -
                     m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) +
                     m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])) * inv_det,
                ],
            ],
        })
    }
    
    /// Transpose the matrix
    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2]],
                [self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3]],
            ],
        }
    }
    
    /// Convert to a flat array in column-major order
    #[inline]
    pub fn to_array(&self) -> [f32; 16] {
        [
            self.data[0][0], self.data[0][1], self.data[0][2], self.data[0][3],
            self.data[1][0], self.data[1][1], self.data[1][2], self.data[1][3],
            self.data[2][0], self.data[2][1], self.data[2][2], self.data[2][3],
            self.data[3][0], self.data[3][1], self.data[3][2], self.data[3][3],
        ]
    }
}

// Matrix multiplication
impl Mul for Mat4 {
    type Output = Mat4;
    
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut result = Mat4::identity();
        
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[k][j] * rhs.data[i][k];
                }
            }
        }
        
        result
    }
}

// Matrix multiplication with Vec4
impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0][0] * rhs.x + self.data[1][0] * rhs.y + self.data[2][0] * rhs.z + self.data[3][0] * rhs.w,
            self.data[0][1] * rhs.x + self.data[1][1] * rhs.y + self.data[2][1] * rhs.z + self.data[3][1] * rhs.w,
            self.data[0][2] * rhs.x + self.data[1][2] * rhs.y + self.data[2][2] * rhs.z + self.data[3][2] * rhs.w,
            self.data[0][3] * rhs.x + self.data[1][3] * rhs.y + self.data[2][3] * rhs.z + self.data[3][3] * rhs.w,
        )
    }
}

// Index operations for easier element access
impl Index<usize> for Mat4 {
    type Output = [f32; 4];
    
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Forward-declare Quaternion for from_quaternion method
use crate::core::math::quaternion::Quaternion; 