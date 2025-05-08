use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use std::fmt;
use crate::math::vector::Vec2;

/// A 2x2 matrix stored in column-major order.
/// 
/// Stored as [c0r0, c0r1, c1r0, c1r1]
/// Where c = column and r = row.
#[derive(Clone, Copy, PartialEq)]
pub struct Mat2 {
    /// Matrix elements stored in column-major order
    pub data: [f32; 4],
}

impl Mat2 {
    /// Creates a new 2x2 matrix with the specified elements.
    /// Elements are specified in column-major order.
    #[inline]
    pub fn new(
        c0r0: f32, c0r1: f32,
        c1r0: f32, c1r1: f32,
    ) -> Self {
        Self {
            data: [
                c0r0, c0r1,
                c1r0, c1r1,
            ],
        }
    }
    
    /// Creates a new 2x2 matrix from two column vectors.
    #[inline]
    pub fn from_cols(c0: Vec2, c1: Vec2) -> Self {
        Self {
            data: [
                c0.x, c0.y,
                c1.x, c1.y,
            ],
        }
    }
    
    /// Creates a new 2x2 matrix from an array of 4 elements.
    /// The elements are assumed to be in column-major order.
    #[inline]
    pub fn from_array(array: [f32; 4]) -> Self {
        Self { data: array }
    }
    
    /// Returns the elements as an array in column-major order.
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        self.data
    }
    
    /// Creates a 2x2 identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0,
                0.0, 1.0,
            ],
        }
    }
    
    /// Creates a 2x2 zero matrix (all elements are zero).
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [0.0; 4],
        }
    }
    
    /// Creates a 2x2 scaling matrix.
    #[inline]
    pub fn scaling(scale: Vec2) -> Self {
        Self {
            data: [
                scale.x, 0.0,
                0.0, scale.y,
            ],
        }
    }
    
    /// Creates a 2x2 uniform scaling matrix.
    #[inline]
    pub fn scaling_uniform(scale: f32) -> Self {
        Self {
            data: [
                scale, 0.0,
                0.0, scale,
            ],
        }
    }
    
    /// Creates a 2x2 rotation matrix.
    /// Positive rotation is clockwise in 2D screen space.
    #[inline]
    pub fn rotation(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self {
            data: [
                cos, sin,
                -sin, cos,
            ],
        }
    }
    
    /// Transforms a 2D vector.
    #[inline]
    pub fn transform_vec2(&self, vector: Vec2) -> Vec2 {
        Vec2::new(
            vector.x * self.data[0] + vector.y * self.data[2],
            vector.x * self.data[1] + vector.y * self.data[3],
        )
    }
    
    /// Returns the transpose of this matrix.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                self.data[0], self.data[2],
                self.data[1], self.data[3],
            ],
        }
    }
    
    /// Returns the determinant of this matrix.
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
    
    /// Returns the inverse of this matrix, or None if the matrix is not invertible.
    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        
        let inv_det = 1.0 / det;
        
        Some(Self {
            data: [
                self.data[3] * inv_det, -self.data[1] * inv_det,
                -self.data[2] * inv_det, self.data[0] * inv_det,
            ],
        })
    }
    
    /// Gets a specific column of the matrix as a Vec2.
    #[inline]
    pub fn col(&self, index: usize) -> Vec2 {
        assert!(index < 2, "Matrix column index out of bounds");
        let offset = index * 2;
        Vec2::new(
            self.data[offset],
            self.data[offset + 1],
        )
    }
    
    /// Gets a specific row of the matrix as a Vec2.
    #[inline]
    pub fn row(&self, index: usize) -> Vec2 {
        assert!(index < 2, "Matrix row index out of bounds");
        Vec2::new(
            self.data[index],
            self.data[index + 2],
        )
    }
    
    /// Gets a specific element of the matrix.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < 2 && col < 2, "Matrix indices out of bounds");
        self.data[col * 2 + row]
    }
    
    /// Sets a specific element of the matrix.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < 2 && col < 2, "Matrix indices out of bounds");
        self.data[col * 2 + row] = value;
    }
}

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Add for Mat2 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl AddAssign for Mat2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..4 {
            self.data[i] += other.data[i];
        }
    }
}

impl Sub for Mat2 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl SubAssign for Mat2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        for i in 0..4 {
            self.data[i] -= other.data[i];
        }
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl MulAssign<f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        for i in 0..4 {
            self.data[i] *= scalar;
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();
        
        for row in 0..2 {
            for col in 0..2 {
                let mut sum = 0.0;
                for i in 0..2 {
                    sum += self.get(row, i) * other.get(i, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, vector: Vec2) -> Vec2 {
        self.transform_vec2(vector)
    }
}

impl Neg for Mat2 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result.data[i] = -self.data[i];
        }
        result
    }
}

impl fmt::Debug for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat2 [\n")?;
        for row in 0..2 {
            write!(f, "  ")?;
            for col in 0..2 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mat2 [\n")?;
        for row in 0..2 {
            write!(f, "  ")?;
            for col in 0..2 {
                write!(f, "{:8.3} ", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

// From conversion
impl From<[f32; 4]> for Mat2 {
    #[inline]
    fn from(array: [f32; 4]) -> Self {
        Self::from_array(array)
    }
}

impl From<Mat2> for [f32; 4] {
    #[inline]
    fn from(mat: Mat2) -> Self {
        mat.data
    }
} 