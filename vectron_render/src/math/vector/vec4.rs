use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::fmt;
use super::vec2::Vec2;
use super::vec3::Vec3;

/// A 4D vector type with x, y, z, and w components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Creates a new 4D vector with the given components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    /// Creates a new 4D vector from a 3D vector and a w component.
    #[inline]
    pub fn from_vec3(v: Vec3, w: f32) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w }
    }
    
    /// Creates a new 4D vector from a 2D vector and z, w components.
    #[inline]
    pub fn from_vec2(v: Vec2, z: f32, w: f32) -> Self {
        Self { x: v.x, y: v.y, z, w }
    }
    
    /// Creates a new 4D vector with all components set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    
    /// Creates a new 4D vector with all components set to one.
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
    
    /// Creates a new 4D vector representing the x-axis unit vector.
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    
    /// Creates a new 4D vector representing the y-axis unit vector.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 }
    }
    
    /// Creates a new 4D vector representing the z-axis unit vector.
    #[inline]
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 }
    }
    
    /// Creates a new 4D vector representing the w-axis unit vector.
    #[inline]
    pub fn unit_w() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
    
    /// Creates a new 4D point (w=1.0).
    #[inline]
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    
    /// Creates a new 4D direction vector (w=0.0).
    #[inline]
    pub fn direction(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    
    /// Returns the dot product of this vector and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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
            w: self.w.min(other.w),
        }
    }
    
    /// Returns the component-wise maximum of this vector and another vector.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }
    
    /// Returns the component-wise absolute value of this vector.
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
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
    
    /// Returns this vector as an array [x, y, z, w].
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
    
    /// Creates a vector from an array [x, y, z, w].
    #[inline]
    pub fn from_array(array: [f32; 4]) -> Self {
        Self { x: array[0], y: array[1], z: array[2], w: array[3] }
    }
    
    /// Returns the xyz components as a Vec3.
    #[inline]
    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    
    /// Returns the xy components as a Vec2.
    #[inline]
    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Default for Vec4 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vec4 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, vector: Vec4) -> Vec4 {
        Vec4 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
            w: self * vector.w,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl Neg for Vec4 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// Component-wise operations
impl Mul for Vec4 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl MulAssign for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl Div for Vec4 {
    type Output = Self;
    
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl DivAssign for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

// From conversion
impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(arr: [f32; 4]) -> Self {
        Self::from_array(arr)
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(vec: Vec4) -> Self {
        vec.to_array()
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(vec: Vec4) -> Self {
        (vec.x, vec.y, vec.z, vec.w)
    }
} 