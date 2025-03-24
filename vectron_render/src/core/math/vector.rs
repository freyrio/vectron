/*!
 * Vector types for 2D and 3D operations
 */
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::core::math::{approx_eq, lerp};

/// 2D vector
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Create a new 2D vector
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Create a zero vector (0,0)
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Create a unit vector (1,1)
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
    
    /// Create a unit X vector (1,0)
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }
    
    /// Create a unit Y vector (0,1)
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
    
    /// Dot product of two vectors
    #[inline]
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    /// Cross product of two 2D vectors (returns scalar)
    #[inline]
    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
    
    /// Length squared of the vector
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    /// Length of the vector
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    /// Distance squared between two points
    #[inline]
    pub fn distance_squared(&self, other: &Vec2) -> f32 {
        (*self - *other).length_squared()
    }
    
    /// Distance between two points
    #[inline]
    pub fn distance(&self, other: &Vec2) -> f32 {
        (*self - *other).length()
    }
    
    /// Normalize the vector (make it unit length)
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
        }
        self
    }
    
    /// Return a normalized copy of the vector
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }
    
    /// Linear interpolation between two vectors
    #[inline]
    pub fn lerp(&self, other: &Vec2, t: f32) -> Self {
        Self {
            x: lerp(self.x, other.x, t),
            y: lerp(self.y, other.y, t),
        }
    }
    
    /// Check if two vectors are approximately equal
    #[inline]
    pub fn approx_eq(&self, other: &Vec2, epsilon: f32) -> bool {
        approx_eq(self.x, other.x, epsilon) && approx_eq(self.y, other.y, epsilon)
    }
    
    /// Convert to array [x, y]
    #[inline]
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    
    /// Convert to tuple (x, y)
    #[inline]
    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    
    /// Convert to Vec3 with z=0
    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

// Operator implementations for Vec2
impl Add for Vec2 {
    type Output = Vec2;
    
    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    
    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    
    #[inline]
    fn div(self, rhs: f32) -> Vec2 {
        Vec2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    
    #[inline]
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// From conversions
impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(v: [f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

/// 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Create a new 3D vector
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    /// Create a zero vector (0,0,0)
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Create a unit vector (1,1,1)
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }
    
    /// Create a unit X vector (1,0,0)
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    
    /// Create a unit Y vector (0,1,0)
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    
    /// Create a unit Z vector (0,0,1)
    #[inline]
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    
    /// Dot product of two vectors
    #[inline]
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    /// Cross product of two vectors
    #[inline]
    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Length squared of the vector
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    /// Length of the vector
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    /// Distance squared between two points
    #[inline]
    pub fn distance_squared(&self, other: &Vec3) -> f32 {
        (*self - *other).length_squared()
    }
    
    /// Distance between two points
    #[inline]
    pub fn distance(&self, other: &Vec3) -> f32 {
        (*self - *other).length()
    }
    
    /// Normalize the vector (make it unit length)
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
        self
    }
    
    /// Return a normalized copy of the vector
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }
    
    /// Linear interpolation between two vectors
    #[inline]
    pub fn lerp(&self, other: &Vec3, t: f32) -> Self {
        Self {
            x: lerp(self.x, other.x, t),
            y: lerp(self.y, other.y, t),
            z: lerp(self.z, other.z, t),
        }
    }
    
    /// Check if two vectors are approximately equal
    #[inline]
    pub fn approx_eq(&self, other: &Vec3, epsilon: f32) -> bool {
        approx_eq(self.x, other.x, epsilon) && 
        approx_eq(self.y, other.y, epsilon) && 
        approx_eq(self.z, other.z, epsilon)
    }
    
    /// Convert to array [x, y, z]
    #[inline]
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    
    /// Convert to tuple (x, y, z)
    #[inline]
    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
    
    /// Convert to Vec2, dropping the z component
    #[inline]
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    /// Convert to Vec4 with w=1.0 (as a point)
    #[inline]
    pub fn to_point(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
    }
    
    /// Convert to Vec4 with w=0.0 (as a direction)
    #[inline]
    pub fn to_direction(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 0.0)
    }
}

// Operator implementations for Vec3
impl Add for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// From conversions
impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(v: (f32, f32, f32)) -> Self {
        Self { x: v.0, y: v.1, z: v.2 }
    }
}

/// 4D vector or homogeneous coordinates (x, y, z, w)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Create a new 4D vector
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    /// Create a zero vector (0,0,0,0)
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    
    /// Create a unit vector (1,1,1,1)
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
    
    /// Create a point from xyz coordinates (w=1)
    #[inline]
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    
    /// Create a direction from xyz coordinates (w=0)
    #[inline]
    pub fn direction(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    
    /// Dot product of two vectors
    #[inline]
    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    
    /// Length squared of the vector
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    
    /// Length of the vector
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    /// Normalize the vector (make it unit length)
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }
        self
    }
    
    /// Return a normalized copy of the vector
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }
    
    /// Linear interpolation between two vectors
    #[inline]
    pub fn lerp(&self, other: &Vec4, t: f32) -> Self {
        Self {
            x: lerp(self.x, other.x, t),
            y: lerp(self.y, other.y, t),
            z: lerp(self.z, other.z, t),
            w: lerp(self.w, other.w, t),
        }
    }
    
    /// Convert to array [x, y, z, w]
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
    
    /// Convert to Vec3, dividing by w if it's a point
    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        if self.w != 0.0 && self.w != 1.0 {
            // Homogeneous coordinate, divide by w
            Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
        } else {
            Vec3::new(self.x, self.y, self.z)
        }
    }
}

// Operator implementations for Vec4
impl Add for Vec4 {
    type Output = Vec4;
    
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z, 
            w: self.w + rhs.w 
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4 { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z, 
            w: self.w - rhs.w 
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, rhs: f32) -> Vec4 {
        Vec4 { 
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs, 
            w: self.w * rhs 
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 { 
            x: self * rhs.x, 
            y: self * rhs.y, 
            z: self * rhs.z, 
            w: self * rhs.w 
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    
    #[inline]
    fn div(self, rhs: f32) -> Vec4 {
        Vec4 { 
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs, 
            w: self.w / rhs 
        }
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    
    #[inline]
    fn neg(self) -> Vec4 {
        Vec4 { 
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
            w: -self.w 
        }
    }
}

// From conversions
impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(v: [f32; 4]) -> Self {
        Self { x: v[0], y: v[1], z: v[2], w: v[3] }
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self { x: v.0, y: v.1, z: v.2, w: v.3 }
    }
}

impl From<Vec3> for Vec4 {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w: 1.0 } // Create as point by default
    }
} 