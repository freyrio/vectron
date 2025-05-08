use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use std::fmt;
use crate::math::vector::{Vec2, Vec3};

/// A 3x3 matrix stored in column-major order.
/// 
/// Stored as [c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2]
/// Where c = column and r = row.
#[derive(Clone, Copy, PartialEq)]
pub struct Mat3 {
    /// Matrix elements stored in column-major order
    pub data: [f32; 9],
}

impl Mat3 {
    /// Creates a new 3x3 matrix with the specified elements.
    /// Elements are specified in column-major order.
    #[inline]
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32,
        c1r0: f32, c1r1: f32, c1r2: f32,
        c2r0: f32, c2r1: f32, c2r2: f32,
    ) -> Self {
        Self {
            data: [
                c0r0, c0r1, c0r2,
                c1r0, c1r1, c1r2,
                c2r0, c2r1, c2r2,
            ],
        }
    }
    
    /// Creates a new 3x3 matrix from three column vectors.
    #[inline]
    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Self {
            data: [
                c0.x, c0.y, c0.z,
                c1.x, c1.y, c1.z,
                c2.x, c2.y, c2.z,
            ],
        }
    }
    
    /// Creates a new 3x3 matrix from an array of 9 elements.
    /// The elements are assumed to be in column-major order.
    #[inline]
    pub fn from_array(array: [f32; 9]) -> Self {
        Self { data: array }
    }
    
    /// Returns the elements as an array in column-major order.
    #[inline]
    pub fn to_array(&self) -> [f32; 9] {
        self.data
    }
    
    /// Creates a 3x3 identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 3x3 zero matrix (all elements are zero).
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [0.0; 9],
        }
    }
    
    /// Creates a 3x3 scaling matrix.
    #[inline]
    pub fn scaling(scale: Vec2) -> Self {
        Self {
            data: [
                scale.x, 0.0, 0.0,
                0.0, scale.y, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 3x3 uniform scaling matrix.
    #[inline]
    pub fn scaling_uniform(scale: f32) -> Self {
        Self {
            data: [
                scale, 0.0, 0.0,
                0.0, scale, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 3x3 translation matrix.
    #[inline]
    pub fn translation(translation: Vec2) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                translation.x, translation.y, 1.0,
            ],
        }
    }
    
    /// Creates a 3x3 rotation matrix around the z-axis.
    /// Positive rotation is clockwise in 2D screen space.
    #[inline]
    pub fn rotation(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            data: [
                cos, sin, 0.0,
                -sin, cos, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }
    
    /// Transforms a 2D point, treating it as a Vec3 with z=1.
    #[inline]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        let x = point.x * self.data[0] + point.y * self.data[3] + self.data[6];
        let y = point.x * self.data[1] + point.y * self.data[4] + self.data[7];
        let w = point.x * self.data[2] + point.y * self.data[5] + self.data[8];
        
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
            vector.x * self.data[0] + vector.y * self.data[3],
            vector.x * self.data[1] + vector.y * self.data[4],
        )
    }
    
    /// Transforms a 3D vector.
    #[inline]
    pub fn transform_vec3(&self, vector: Vec3) -> Vec3 {
        Vec3::new(
            vector.x * self.data[0] + vector.y * self.data[3] + vector.z * self.data[6],
            vector.x * self.data[1] + vector.y * self.data[4] + vector.z * self.data[7],
            vector.x * self.data[2] + vector.y * self.data[5] + vector.z * self.data[8],
        )
    }
    
    /// Returns the transpose of this matrix.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                self.data[0], self.data[3], self.data[6],
                self.data[1], self.data[4], self.data[7],
                self.data[2], self.data[5], self.data[8],
            ],
        }
    }
    
    /// Returns the determinant of this matrix.
    #[inline]
    pub fn determinant(&self) -> f32 {
        let m = &self.data;
        
        m[0] * (m[4] * m[8] - m[5] * m[7]) -
        m[3] * (m[1] * m[8] - m[2] * m[7]) +
        m[6] * (m[1] * m[5] - m[2] * m[4])
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
        let c00 = m[4] * m[8] - m[5] * m[7];
        let c01 = -(m[3] * m[8] - m[5] * m[6]);
        let c02 = m[3] * m[7] - m[4] * m[6];
        
        let c10 = -(m[1] * m[8] - m[2] * m[7]);
        let c11 = m[0] * m[8] - m[2] * m[6];
        let c12 = -(m[0] * m[7] - m[1] * m[6]);
        
        let c20 = m[1] * m[5] - m[2] * m[4];
        let c21 = -(m[0] * m[5] - m[2] * m[3]);
        let c22 = m[0] * m[4] - m[1] * m[3];
        
        Some(Self {
            data: [
                c00 * inv_det, c10 * inv_det, c20 * inv_det,
                c01 * inv_det, c11 * inv_det, c21 * inv_det,
                c02 * inv_det, c12 * inv_det, c22 * inv_det,
            ],
        })
    }
    
    /// Gets a specific column of the matrix as a Vec3.
    #[inline]
    pub fn col(&self, index: usize) -> Vec3 {
        assert!(index < 3, "Matrix column index out of bounds");
        let offset = index * 3;
        Vec3::new(
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
        )
    }
    
    /// Gets a specific row of the matrix as a Vec3.
    #[inline]
    pub fn row(&self, index: usize) -> Vec3 {
        assert!(index < 3, "Matrix row index out of bounds");
        Vec3::new(
            self.data[index],
            self.data[index + 3],
            self.data[index + 6],
        )
    }
    
    /// Gets a specific element of the matrix.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < 3 && col < 3, "Matrix indices out of bounds");
        self.data[col * 3 + row]
    }
    
    /// Sets a specific element of the matrix.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < 3 && col < 3, "Matrix indices out of bounds");
        self.data[col * 3 + row] = value;
    }
}

impl Default for Mat3 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Add for Mat3 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..9 {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl AddAssign for Mat3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..9 {
            self.data[i] += other.data[i];
        }
    }
}

impl Sub for Mat3 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..9 {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl SubAssign for Mat3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        for i in 0..9 {
            self.data[i] -= other.data[i];
        }
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..9 {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl MulAssign<f32> for Mat3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        for i in 0..9 {
            self.data[i] *= scalar;
        }
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();
        
        for row in 0..3 {
            for col in 0..3 {
                let mut sum = 0.0;
                for i in 0..3 {
                    sum += self.get(row, i) * other.get(i, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, vector: Vec3) -> Vec3 {
        self.transform_vec3(vector)
    }
}

impl Neg for Mat3 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        let mut result = Self::zero();
        for i in 0..9 {
            result.data[i] = -self.data[i];
        }
        result
    }
}

impl fmt::Debug for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat3 [\n")?;
        for row in 0..3 {
            write!(f, "  ")?;
            for col in 0..3 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat3 [\n")?;
        for row in 0..3 {
            write!(f, "  ")?;
            for col in 0..3 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

// From conversion
impl From<[f32; 9]> for Mat3 {
    #[inline]
    fn from(array: [f32; 9]) -> Self {
        Self::from_array(array)
    }
}

impl From<Mat3> for [f32; 9] {
    #[inline]
    fn from(mat: Mat3) -> Self {
        mat.data
    }
} 