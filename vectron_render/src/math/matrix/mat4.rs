use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use std::fmt;
use crate::math::vector::{Vec3, Vec4};

/// A 4x4 matrix stored in column-major order.
/// 
/// Stored as [c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2, c3r3]
/// Where c = column and r = row.
#[derive(Clone, Copy, PartialEq)]
pub struct Mat4 {
    /// Matrix elements stored in column-major order
    pub data: [f32; 16],
}

impl Mat4 {
    /// Creates a new 4x4 matrix with the specified elements.
    /// Elements are specified in column-major order.
    #[inline]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32,
    ) -> Self {
        Self {
            data: [
                c0r0, c0r1, c0r2, c0r3,
                c1r0, c1r1, c1r2, c1r3,
                c2r0, c2r1, c2r2, c2r3,
                c3r0, c3r1, c3r2, c3r3,
            ],
        }
    }
    
    /// Creates a new 4x4 matrix from four column vectors.
    #[inline]
    pub fn from_cols(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Self {
        Self {
            data: [
                c0.x, c0.y, c0.z, c0.w,
                c1.x, c1.y, c1.z, c1.w,
                c2.x, c2.y, c2.z, c2.w,
                c3.x, c3.y, c3.z, c3.w,
            ],
        }
    }
    
    /// Creates a new 4x4 matrix from an array of 16 elements.
    /// The elements are assumed to be in column-major order.
    #[inline]
    pub fn from_array(array: [f32; 16]) -> Self {
        Self { data: array }
    }
    
    /// Returns the elements as an array in column-major order.
    #[inline]
    pub fn to_array(&self) -> [f32; 16] {
        self.data
    }
    
    /// Creates a 4x4 identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 zero matrix (all elements are zero).
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [0.0; 16],
        }
    }
    
    /// Creates a 4x4 scaling matrix.
    #[inline]
    pub fn scaling(scale: Vec3) -> Self {
        Self {
            data: [
                scale.x, 0.0, 0.0, 0.0,
                0.0, scale.y, 0.0, 0.0,
                0.0, 0.0, scale.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 uniform scaling matrix.
    #[inline]
    pub fn scaling_uniform(scale: f32) -> Self {
        Self {
            data: [
                scale, 0.0, 0.0, 0.0,
                0.0, scale, 0.0, 0.0,
                0.0, 0.0, scale, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 translation matrix.
    #[inline]
    pub fn translation(translation: Vec3) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                translation.x, translation.y, translation.z, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 rotation matrix around the X axis.
    #[inline]
    pub fn rotation_x(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, cos, sin, 0.0,
                0.0, -sin, cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 rotation matrix around the Y axis.
    #[inline]
    pub fn rotation_y(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            data: [
                cos, 0.0, -sin, 0.0,
                0.0, 1.0, 0.0, 0.0,
                sin, 0.0, cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 rotation matrix around the Z axis.
    #[inline]
    pub fn rotation_z(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            data: [
                cos, sin, 0.0, 0.0,
                -sin, cos, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 4x4 rotation matrix from axis and angle.
    #[inline]
    pub fn rotation_axis_angle(axis: Vec3, radians: f32) -> Self {
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
            data: [
                cos + xx * one_minus_cos, xy * one_minus_cos + zs, xz * one_minus_cos - ys, 0.0,
                xy * one_minus_cos - zs, cos + yy * one_minus_cos, yz * one_minus_cos + xs, 0.0,
                xz * one_minus_cos + ys, yz * one_minus_cos - xs, cos + zz * one_minus_cos, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a perspective projection matrix.
    #[inline]
    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y_radians * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        
        Self {
            data: [
                f / aspect_ratio, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (near + far) * range_inv, -1.0,
                0.0, 0.0, 2.0 * far * near * range_inv, 0.0,
            ],
        }
    }
    
    /// Creates an orthographic projection matrix.
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        
        Self {
            data: [
                2.0 / width, 0.0, 0.0, 0.0,
                0.0, 2.0 / height, 0.0, 0.0,
                0.0, 0.0, -2.0 / depth, 0.0,
                -(right + left) / width, -(top + bottom) / height, -(far + near) / depth, 1.0,
            ],
        }
    }
    
    /// Creates a look-at view matrix.
    #[inline]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);
        
        Self {
            data: [
                s.x, u.x, -f.x, 0.0,
                s.y, u.y, -f.y, 0.0,
                s.z, u.z, -f.z, 0.0,
                -s.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0,
            ],
        }
    }
    
    /// Transforms a 3D vector as a point (w = 1).
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let x = point.x * self.data[0] + point.y * self.data[4] + point.z * self.data[8] + self.data[12];
        let y = point.x * self.data[1] + point.y * self.data[5] + point.z * self.data[9] + self.data[13];
        let z = point.x * self.data[2] + point.y * self.data[6] + point.z * self.data[10] + self.data[14];
        let w = point.x * self.data[3] + point.y * self.data[7] + point.z * self.data[11] + self.data[15];
        
        if w == 1.0 {
            Vec3::new(x, y, z)
        } else {
            let w_inv = 1.0 / w;
            Vec3::new(x * w_inv, y * w_inv, z * w_inv)
        }
    }
    
    /// Transforms a 3D vector as a direction (w = 0).
    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        Vec3::new(
            vector.x * self.data[0] + vector.y * self.data[4] + vector.z * self.data[8],
            vector.x * self.data[1] + vector.y * self.data[5] + vector.z * self.data[9],
            vector.x * self.data[2] + vector.y * self.data[6] + vector.z * self.data[10],
        )
    }
    
    /// Transforms a 4D vector.
    #[inline]
    pub fn transform_vec4(&self, vector: Vec4) -> Vec4 {
        Vec4::new(
            vector.x * self.data[0] + vector.y * self.data[4] + vector.z * self.data[8] + vector.w * self.data[12],
            vector.x * self.data[1] + vector.y * self.data[5] + vector.z * self.data[9] + vector.w * self.data[13],
            vector.x * self.data[2] + vector.y * self.data[6] + vector.z * self.data[10] + vector.w * self.data[14],
            vector.x * self.data[3] + vector.y * self.data[7] + vector.z * self.data[11] + vector.w * self.data[15],
        )
    }
    
    /// Returns the transpose of this matrix.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                self.data[0], self.data[4], self.data[8], self.data[12],
                self.data[1], self.data[5], self.data[9], self.data[13],
                self.data[2], self.data[6], self.data[10], self.data[14],
                self.data[3], self.data[7], self.data[11], self.data[15],
            ],
        }
    }
    
    /// Returns the determinant of this matrix.
    #[inline]
    pub fn determinant(&self) -> f32 {
        let m = &self.data;
        
        let a = m[0] * m[5] - m[1] * m[4];
        let b = m[0] * m[6] - m[2] * m[4];
        let c = m[0] * m[7] - m[3] * m[4];
        let d = m[1] * m[6] - m[2] * m[5];
        let e = m[1] * m[7] - m[3] * m[5];
        let f = m[2] * m[7] - m[3] * m[6];
        let g = m[8] * m[13] - m[9] * m[12];
        let h = m[8] * m[14] - m[10] * m[12];
        let i = m[8] * m[15] - m[11] * m[12];
        let j = m[9] * m[14] - m[10] * m[13];
        let k = m[9] * m[15] - m[11] * m[13];
        let l = m[10] * m[15] - m[11] * m[14];
        
        a * l - b * k + c * j + d * i - e * h + f * g
    }
    
    /// Returns the inverse of this matrix, or None if the matrix is not invertible.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        
        let inv_det = 1.0 / det;
        let m = &self.data;
        
        // Calculate cofactors
        let c0 = m[5] * (m[10] * m[15] - m[11] * m[14]) - m[9] * (m[6] * m[15] - m[7] * m[14]) + m[13] * (m[6] * m[11] - m[7] * m[10]);
        let c1 = -(m[4] * (m[10] * m[15] - m[11] * m[14]) - m[8] * (m[6] * m[15] - m[7] * m[14]) + m[12] * (m[6] * m[11] - m[7] * m[10]));
        let c2 = m[4] * (m[9] * m[15] - m[11] * m[13]) - m[8] * (m[5] * m[15] - m[7] * m[13]) + m[12] * (m[5] * m[11] - m[7] * m[9]);
        let c3 = -(m[4] * (m[9] * m[14] - m[10] * m[13]) - m[8] * (m[5] * m[14] - m[6] * m[13]) + m[12] * (m[5] * m[10] - m[6] * m[9]));
        
        let c4 = -(m[1] * (m[10] * m[15] - m[11] * m[14]) - m[9] * (m[2] * m[15] - m[3] * m[14]) + m[13] * (m[2] * m[11] - m[3] * m[10]));
        let c5 = m[0] * (m[10] * m[15] - m[11] * m[14]) - m[8] * (m[2] * m[15] - m[3] * m[14]) + m[12] * (m[2] * m[11] - m[3] * m[10]);
        let c6 = -(m[0] * (m[9] * m[15] - m[11] * m[13]) - m[8] * (m[1] * m[15] - m[3] * m[13]) + m[12] * (m[1] * m[11] - m[3] * m[9]));
        let c7 = m[0] * (m[9] * m[14] - m[10] * m[13]) - m[8] * (m[1] * m[14] - m[2] * m[13]) + m[12] * (m[1] * m[10] - m[2] * m[9]);
        
        let c8 = m[1] * (m[6] * m[15] - m[7] * m[14]) - m[5] * (m[2] * m[15] - m[3] * m[14]) + m[13] * (m[2] * m[7] - m[3] * m[6]);
        let c9 = -(m[0] * (m[6] * m[15] - m[7] * m[14]) - m[4] * (m[2] * m[15] - m[3] * m[14]) + m[12] * (m[2] * m[7] - m[3] * m[6]));
        let c10 = m[0] * (m[5] * m[15] - m[7] * m[13]) - m[4] * (m[1] * m[15] - m[3] * m[13]) + m[12] * (m[1] * m[7] - m[3] * m[5]);
        let c11 = -(m[0] * (m[5] * m[14] - m[6] * m[13]) - m[4] * (m[1] * m[14] - m[2] * m[13]) + m[12] * (m[1] * m[6] - m[2] * m[5]));
        
        let c12 = -(m[1] * (m[6] * m[11] - m[7] * m[10]) - m[5] * (m[2] * m[11] - m[3] * m[10]) + m[9] * (m[2] * m[7] - m[3] * m[6]));
        let c13 = m[0] * (m[6] * m[11] - m[7] * m[10]) - m[4] * (m[2] * m[11] - m[3] * m[10]) + m[8] * (m[2] * m[7] - m[3] * m[6]);
        let c14 = -(m[0] * (m[5] * m[11] - m[7] * m[9]) - m[4] * (m[1] * m[11] - m[3] * m[9]) + m[8] * (m[1] * m[7] - m[3] * m[5]));
        let c15 = m[0] * (m[5] * m[10] - m[6] * m[9]) - m[4] * (m[1] * m[10] - m[2] * m[9]) + m[8] * (m[1] * m[6] - m[2] * m[5]);
        
        Some(Self {
            data: [
                c0 * inv_det, c4 * inv_det, c8 * inv_det, c12 * inv_det,
                c1 * inv_det, c5 * inv_det, c9 * inv_det, c13 * inv_det,
                c2 * inv_det, c6 * inv_det, c10 * inv_det, c14 * inv_det,
                c3 * inv_det, c7 * inv_det, c11 * inv_det, c15 * inv_det,
            ],
        })
    }
    
    /// Gets a specific column of the matrix as a Vec4.
    #[inline]
    pub fn col(&self, index: usize) -> Vec4 {
        assert!(index < 4, "Matrix column index out of bounds");
        let offset = index * 4;
        Vec4::new(
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        )
    }
    
    /// Gets a specific row of the matrix as a Vec4.
    #[inline]
    pub fn row(&self, index: usize) -> Vec4 {
        assert!(index < 4, "Matrix row index out of bounds");
        Vec4::new(
            self.data[index],
            self.data[index + 4],
            self.data[index + 8],
            self.data[index + 12],
        )
    }
    
    /// Gets a specific element of the matrix.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < 4 && col < 4, "Matrix indices out of bounds");
        self.data[col * 4 + row]
    }
    
    /// Sets a specific element of the matrix.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < 4 && col < 4, "Matrix indices out of bounds");
        self.data[col * 4 + row] = value;
    }
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Add for Mat4 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..16 {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl AddAssign for Mat4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..16 {
            self.data[i] += other.data[i];
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..16 {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl SubAssign for Mat4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        for i in 0..16 {
            self.data[i] -= other.data[i];
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..16 {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl MulAssign<f32> for Mat4 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        for i in 0..16 {
            self.data[i] *= scalar;
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();
        
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += self.get(row, i) * other.get(i, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, vector: Vec4) -> Vec4 {
        self.transform_vec4(vector)
    }
}

impl Neg for Mat4 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        let mut result = Self::zero();
        for i in 0..16 {
            result.data[i] = -self.data[i];
        }
        result
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat4 [\n")?;
        for row in 0..4 {
            write!(f, "  ")?;
            for col in 0..4 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat4 [\n")?;
        for row in 0..4 {
            write!(f, "  ")?;
            for col in 0..4 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

// From conversion
impl From<[f32; 16]> for Mat4 {
    #[inline]
    fn from(array: [f32; 16]) -> Self {
        Self::from_array(array)
    }
}

impl From<Mat4> for [f32; 16] {
    #[inline]
    fn from(mat: Mat4) -> Self {
        mat.data
    }
} 