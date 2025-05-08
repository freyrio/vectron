use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::fmt;

/// A 2D vector type with x and y components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new 2D vector with the given components.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Creates a new 2D vector with all components set to zero.
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Creates a new 2D vector with all components set to one.
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
    
    /// Creates a new 2D vector representing the x-axis unit vector.
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }
    
    /// Creates a new 2D vector representing the y-axis unit vector.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
    
    /// Returns the dot product of this vector and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
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
        }
    }
    
    /// Returns the component-wise maximum of this vector and another vector.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    
    /// Returns the component-wise absolute value of this vector.
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    
    /// Returns the cross product of this vector and another vector.
    /// In 2D, this returns a scalar representing the z-component of the cross product in 3D.
    #[inline]
    pub fn cross(&self, other: &Self) -> f32 {
        self.x * other.y - self.y * other.x
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
    
    /// Returns this vector as an array [x, y].
    #[inline]
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    
    /// Creates a vector from an array [x, y].
    #[inline]
    pub fn from_array(array: [f32; 2]) -> Self {
        Self { x: array[0], y: array[1] }
    }
}

impl Default for Vec2 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vec2 {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

// Component-wise operations
impl Mul for Vec2 {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Div for Vec2 {
    type Output = Self;
    
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// From conversion
impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(arr: [f32; 2]) -> Self {
        Self::from_array(arr)
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(vec: Vec2) -> Self {
        vec.to_array()
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(tuple: (f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(vec: Vec2) -> Self {
        (vec.x, vec.y)
    }
} 