use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::fmt;
use super::vec2::Vec2;

/// A 3D vector type with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Creates a new 3D vector with the given components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    /// Creates a new 3D vector from a 2D vector and a z component.
    #[inline]
    pub fn from_vec2(v: Vec2, z: f32) -> Self {
        Self { x: v.x, y: v.y, z }
    }
    
    /// Creates a new 3D vector with all components set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Creates a new 3D vector with all components set to one.
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }
    
    /// Creates a new 3D vector representing the x-axis unit vector.
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    
    /// Creates a new 3D vector representing the y-axis unit vector.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    
    /// Creates a new 3D vector representing the z-axis unit vector.
    #[inline]
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    
    /// Returns the dot product of this vector and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Returns the squared length of this vector.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    
    /// Returns the length of this vector.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    /// Returns the distance between this vector and another vector.
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }
    
    /// Returns the squared distance between this vector and another vector.
    #[inline]
    pub fn distance_squared(&self, other: &Self) -> f32 {
        (*self - *other).length_squared()
    }
    
    /// Returns a normalized version of this vector (unit length).
    #[inline]
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > 0.0 {
            *self / length
        } else {
            *self
        }
    }
    
    /// Normalizes this vector in-place (sets it to unit length).
    #[inline]
    pub fn normalize_mut(&mut self) {
        let length = self.length();
        if length > 0.0 {
            *self /= length;
        }
    }
    
    /// Returns the cross product of this vector and another vector.
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Reflects this vector about a normal vector.
    #[inline]
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - (2.0 * self.dot(normal) * *normal)
    }
    
    /// Linearly interpolates between this vector and another vector.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        *self + ((*other - *self) * t)
    }
    
    /// Returns the component-wise minimum of this vector and another vector.
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    
    /// Returns the component-wise maximum of this vector and another vector.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
    
    /// Returns the component-wise absolute value of this vector.
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    
    /// Returns a vector with the same direction as this vector, but with the specified length.
    #[inline]
    pub fn with_length(&self, length: f32) -> Self {
        self.normalize() * length
    }
    
    /// Returns whether this vector has a length close to 1.0 within a tolerance.
    #[inline]
    pub fn is_normalized(&self, epsilon: f32) -> bool {
        (self.length_squared() - 1.0).abs() < epsilon
    }
    
    /// Returns this vector as an array [x, y, z].
    #[inline]
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    
    /// Creates a vector from an array [x, y, z].
    #[inline]
    pub fn from_array(array: [f32; 3]) -> Self {
        Self { x: array[0], y: array[1], z: array[2] }
    }
    
    /// Returns the xy components as a Vec2.
    #[inline]
    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    /// Returns the xz components as a Vec2.
    #[inline]
    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
    
    /// Returns the yz components as a Vec2.
    #[inline]
    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vec3 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

// Component-wise operations
impl Mul for Vec3 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Div for Vec3 {
    type Output = Self;
    
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

// From conversion
impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(arr: [f32; 3]) -> Self {
        Self::from_array(arr)
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(vec: Vec3) -> Self {
        vec.to_array()
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(vec: Vec3) -> Self {
        (vec.x, vec.y, vec.z)
    }
} 