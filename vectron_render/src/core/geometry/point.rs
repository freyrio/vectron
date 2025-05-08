use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt;
use std::convert::From;
use crate::core::types::Dimensionality;
use crate::math::vector::{Vec2, Vec3 };

/// A generic point in D-dimensional space.
/// 
/// Points represent positions in space, while Vectors (Vec2, Vec3) represent directions.
/// This is a core distinction in computer graphics and maintains separation between
/// the "what" (geometry) and "how" (math) layers of the engine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point {
    // A 2D point in space
    D2(Point2D),
    // A 3D point in space
    D3(Point3D),
}

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    // Internal Vec2 for storage and calculations
    vec: Vec2,
}

/// A point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    // Internal Vec3 for storage and calculations
    vec: Vec3,
}

impl Point2D {
    /// Creates a new 2D point.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2::new(x, y) }
    }
    
    /// Creates a 2D point at the origin (0,0).
    #[inline]
    pub fn origin() -> Self {
        Self { vec: Vec2::zero() }
    }
    
    /// Gets the x-coordinate of the point.
    #[inline]
    pub fn x(&self) -> f32 {
        self.vec.x
    }
    
    /// Gets the y-coordinate of the point.
    #[inline]
    pub fn y(&self) -> f32 {
        self.vec.y
    }
    
    /// Sets the x-coordinate of the point.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.vec.x = x;
    }
    
    /// Sets the y-coordinate of the point.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.vec.y = y;
    }
    
    /// Returns the vector from the origin to this point.
    #[inline]
    pub fn to_vec2(&self) -> Vec2 {
        self.vec
    }
    
    /// Calculates the distance between this point and another point.
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        (self.vec - other.vec).length()
    }
    
    /// Calculates the squared distance between this point and another point.
    #[inline]
    pub fn distance_squared(&self, other: &Self) -> f32 {
        (self.vec - other.vec).length_squared()
    }
    
    /// Returns the linear interpolation between this point and another point.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self { vec: self.vec.lerp(&other.vec, t) }
    }
    
    /// Returns the component-wise minimum of this point and another point.
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self { vec: self.vec.min(&other.vec) }
    }
    
    /// Returns the component-wise maximum of this point and another point.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self { vec: self.vec.max(&other.vec) }
    }
}

impl Point3D {
    /// Creates a new 3D point.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { vec: Vec3::new(x, y, z) }
    }
    
    /// Creates a 3D point at the origin (0,0,0).
    #[inline]
    pub fn origin() -> Self {
        Self { vec: Vec3::zero() }
    }
    
    /// Gets the x-coordinate of the point.
    #[inline]
    pub fn x(&self) -> f32 {
        self.vec.x
    }
    
    /// Gets the y-coordinate of the point.
    #[inline]
    pub fn y(&self) -> f32 {
        self.vec.y
    }
    
    /// Gets the z-coordinate of the point.
    #[inline]
    pub fn z(&self) -> f32 {
        self.vec.z
    }
    
    /// Sets the x-coordinate of the point.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.vec.x = x;
    }
    
    /// Sets the y-coordinate of the point.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.vec.y = y;
    }
    
    /// Sets the z-coordinate of the point.
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        self.vec.z = z;
    }
    
    /// Returns the vector from the origin to this point.
    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        self.vec
    }
    
    /// Calculates the distance between this point and another point.
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        (self.vec - other.vec).length()
    }
    
    /// Calculates the squared distance between this point and another point.
    #[inline]
    pub fn distance_squared(&self, other: &Self) -> f32 {
        (self.vec - other.vec).length_squared()
    }
    
    /// Calculates the cross product between the vectors from origin to these points.
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self { vec: self.vec.cross(&other.vec) }
    }
    
    /// Returns the linear interpolation between this point and another point.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self { vec: self.vec.lerp(&other.vec, t) }
    }
    
    /// Returns the component-wise minimum of this point and another point.
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self { vec: self.vec.min(&other.vec) }
    }
    
    /// Returns the component-wise maximum of this point and another point.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self { vec: self.vec.max(&other.vec) }
    }
}

impl Point {
    /// Get the dimensionality of the point
    pub fn dimensionality(&self) -> Dimensionality {
        match self {
            Point::D2(_) => Dimensionality::D2,
            Point::D3(_) => Dimensionality::D3,
        }
    }
    
    /// Creates a new 2D point
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self::D2(Point2D::new(x, y))
    }
    
    /// Creates a new 3D point
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self::D3(Point3D::new(x, y, z))
    }
    
    /// Creates a point at the origin in the specified dimensionality
    pub fn origin(dim: Dimensionality) -> Self {
        match dim {
            Dimensionality::D2 => Self::D2(Point2D::origin()),
            Dimensionality::D3 => Self::D3(Point3D::origin()),
        }
    }
    
    /// Returns the x-coordinate of the point
    pub fn x(&self) -> f32 {
        match self {
            Point::D2(p) => p.x(),
            Point::D3(p) => p.x(),
        }
    }
    
    /// Returns the y-coordinate of the point
    pub fn y(&self) -> f32 {
        match self {
            Point::D2(p) => p.y(),
            Point::D3(p) => p.y(),
        }
    }
    
    /// Returns the z-coordinate of the point (0 for 2D points)
    pub fn z(&self) -> f32 {
        match self {
            Point::D2(_) => 0.0,
            Point::D3(p) => p.z(),
        }
    }
}

// From/Into implementations for clean conversions
impl From<Vec2> for Point2D {
    #[inline]
    fn from(vec: Vec2) -> Self {
        Self { vec }
    }
}

impl From<Point2D> for Vec2 {
    #[inline]
    fn from(point: Point2D) -> Self {
        point.vec
    }
}

impl From<Vec3> for Point3D {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self { vec }
    }
}

impl From<Point3D> for Vec3 {
    #[inline]
    fn from(point: Point3D) -> Self {
        point.vec
    }
}

// Operator implementations for Point2D
impl Add for Point2D {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self { vec: self.vec + other.vec }
    }
}

impl Sub for Point2D {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self { vec: self.vec - other.vec }
    }
}

impl Mul<f32> for Point2D {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self { vec: self.vec * scalar }
    }
}

impl Div<f32> for Point2D {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self { vec: self.vec / scalar }
    }
}

impl AddAssign for Point2D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.vec += other.vec;
    }
}

impl SubAssign for Point2D {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.vec -= other.vec;
    }
}

impl MulAssign<f32> for Point2D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.vec *= scalar;
    }
}

impl DivAssign<f32> for Point2D {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.vec /= scalar;
    }
}

impl Neg for Point2D {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self { vec: -self.vec }
    }
}

// Operator implementations for Point3D
impl Add for Point3D {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self { vec: self.vec + other.vec }
    }
}

impl Sub for Point3D {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self { vec: self.vec - other.vec }
    }
}

impl Mul<f32> for Point3D {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self { vec: self.vec * scalar }
    }
}

impl Div<f32> for Point3D {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self { vec: self.vec / scalar }
    }
}

impl AddAssign for Point3D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.vec += other.vec;
    }
}

impl SubAssign for Point3D {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.vec -= other.vec;
    }
}

impl MulAssign<f32> for Point3D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.vec *= scalar;
    }
}

impl DivAssign<f32> for Point3D {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.vec /= scalar;
    }
}

impl Neg for Point3D {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self { vec: -self.vec }
    }
}

impl Default for Point2D {
    #[inline]
    fn default() -> Self {
        Self::origin()
    }
}

impl Default for Point3D {
    #[inline]
    fn default() -> Self {
        Self::origin()
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2D({}, {})", self.x(), self.y())
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point3D({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Point::D2(p) => write!(f, "{}", p),
            Point::D3(p) => write!(f, "{}", p),
        }
    }
} 