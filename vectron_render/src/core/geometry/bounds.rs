use std::fmt::Debug;
use crate::core::geometry::point::{Point, Point2D, Point3D};
use crate::core::types::Dimensionality;

/// Trait for representing a bounding volume in space.
pub trait BoundingVolume: Debug {
    /// Check if this bounding volume contains a point.
    fn contains_point(&self, point: &Point) -> bool;
    
    /// Check if this bounding volume intersects with another bounding volume.
    fn intersects(&self, other: &dyn BoundingVolume) -> bool;
    
    /// Compute the union of this bounding volume with another bounding volume.
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume>;
    
    /// Get the minimum and maximum points of this bounding volume.
    fn get_min_max(&self) -> (Point, Point);
    
    /// Get the dimensionality of this bounding volume.
    fn dimensionality(&self) -> Dimensionality;
    
    /// Clone this bounding volume.
    fn clone_volume(&self) -> Box<dyn BoundingVolume>;
    
    /// Returns self as Any for downcast operations.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// An axis-aligned bounding box in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2D {
    /// The minimum point of the AABB.
    pub min: Point2D,
    
    /// The maximum point of the AABB.
    pub max: Point2D,
}

/// An axis-aligned bounding box in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB3D {
    /// The minimum point of the AABB.
    pub min: Point3D,
    
    /// The maximum point of the AABB.
    pub max: Point3D,
}

impl AABB2D {
    /// Creates a new 2D AABB from minimum and maximum points.
    #[inline]
    pub fn new(min: Point2D, max: Point2D) -> Self {
        Self { min, max }
    }
    
    /// Creates a new 2D AABB from individual coordinates.
    #[inline]
    pub fn new_from_coords(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Point2D::new(min_x, min_y),
            max: Point2D::new(max_x, max_y),
        }
    }
    
    /// Creates an AABB encompassing a set of points.
    #[inline]
    pub fn from_points(points: &[Point2D]) -> Self {
        assert!(!points.is_empty(), "Points array must not be empty");
        
        let mut min = points[0];
        let mut max = points[0];
        
        for point in points.iter().skip(1) {
            min = min.min(point);
            max = max.max(point);
        }
        
        Self { min, max }
    }
    
    /// Creates an empty AABB.
    #[inline]
    pub fn empty() -> Self {
        Self {
            min: Point2D::new(f32::MAX, f32::MAX),
            max: Point2D::new(f32::MIN, f32::MIN),
        }
    }
    
    /// Returns whether this AABB is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x() > self.max.x() || self.min.y() > self.max.y()
    }
    
    /// Expands this AABB to include a point.
    #[inline]
    pub fn expand(&mut self, point: &Point2D) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }
    
    /// Expands this AABB by a fixed amount in all directions.
    #[inline]
    pub fn expand_by(&mut self, amount: f32) {
        self.min = Point2D::new(self.min.x() - amount, self.min.y() - amount);
        self.max = Point2D::new(self.max.x() + amount, self.max.y() + amount);
    }
    
    /// Returns the center of this AABB.
    #[inline]
    pub fn center(&self) -> Point2D {
        Point2D::new(
            (self.min.x() + self.max.x()) * 0.5,
            (self.min.y() + self.max.y()) * 0.5,
        )
    }
    
    /// Returns the size of this AABB.
    #[inline]
    pub fn size(&self) -> Point2D {
        Point2D::new(
            self.max.x() - self.min.x(),
            self.max.y() - self.min.y(),
        )
    }
    
    /// Returns the width of this AABB.
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x() - self.min.x()
    }
    
    /// Returns the height of this AABB.
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y() - self.min.y()
    }
    
    /// Returns the area of this AABB.
    #[inline]
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }
    
    /// Returns the perimeter of this AABB.
    #[inline]
    pub fn perimeter(&self) -> f32 {
        2.0 * (self.width() + self.height())
    }
    
    /// Check if this AABB contains a 2D point.
    #[inline]
    pub fn contains_point(&self, point: &Point2D) -> bool {
        point.x() >= self.min.x() && point.x() <= self.max.x() &&
        point.y() >= self.min.y() && point.y() <= self.max.y()
    }
    
    /// Check if this AABB intersects with another AABB.
    #[inline]
    pub fn intersects(&self, other: &AABB2D) -> bool {
        self.min.x() <= other.max.x() && self.max.x() >= other.min.x() &&
        self.min.y() <= other.max.y() && self.max.y() >= other.min.y()
    }
    
    /// Compute the union of this AABB with another AABB.
    #[inline]
    pub fn union(&self, other: &AABB2D) -> AABB2D {
        AABB2D {
            min: self.min.min(&other.min),
            max: self.max.max(&other.max),
        }
    }
    
    /// Compute the intersection of this AABB with another AABB.
    #[inline]
    pub fn intersection(&self, other: &AABB2D) -> Option<AABB2D> {
        let min_x = self.min.x().max(other.min.x());
        let min_y = self.min.y().max(other.min.y());
        let max_x = self.max.x().min(other.max.x());
        let max_y = self.max.y().min(other.max.y());
        
        if min_x <= max_x && min_y <= max_y {
            Some(AABB2D {
                min: Point2D::new(min_x, min_y),
                max: Point2D::new(max_x, max_y),
            })
        } else {
            None
        }
    }
}

impl AABB3D {
    /// Creates a new 3D AABB from minimum and maximum points.
    #[inline]
    pub fn new(min: Point3D, max: Point3D) -> Self {
        Self { min, max }
    }
    
    /// Creates a new 3D AABB from individual coordinates.
    #[inline]
    pub fn new_from_coords(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
        Self {
            min: Point3D::new(min_x, min_y, min_z),
            max: Point3D::new(max_x, max_y, max_z),
        }
    }
    
    /// Creates an AABB encompassing a set of points.
    #[inline]
    pub fn from_points(points: &[Point3D]) -> Self {
        assert!(!points.is_empty(), "Points array must not be empty");
        
        let mut min = points[0];
        let mut max = points[0];
        
        for point in points.iter().skip(1) {
            min = min.min(point);
            max = max.max(point);
        }
        
        Self { min, max }
    }
    
    /// Creates an empty AABB.
    #[inline]
    pub fn empty() -> Self {
        Self {
            min: Point3D::new(f32::MAX, f32::MAX, f32::MAX),
            max: Point3D::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }
    
    /// Returns whether this AABB is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x() > self.max.x() || self.min.y() > self.max.y() || self.min.z() > self.max.z()
    }
    
    /// Expands this AABB to include a point.
    #[inline]
    pub fn expand(&mut self, point: &Point3D) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }
    
    /// Expands this AABB by a fixed amount in all directions.
    #[inline]
    pub fn expand_by(&mut self, amount: f32) {
        self.min = Point3D::new(self.min.x() - amount, self.min.y() - amount, self.min.z() - amount);
        self.max = Point3D::new(self.max.x() + amount, self.max.y() + amount, self.max.z() + amount);
    }
    
    /// Returns the center of this AABB.
    #[inline]
    pub fn center(&self) -> Point3D {
        Point3D::new(
            (self.min.x() + self.max.x()) * 0.5,
            (self.min.y() + self.max.y()) * 0.5,
            (self.min.z() + self.max.z()) * 0.5,
        )
    }
    
    /// Returns the size of this AABB.
    #[inline]
    pub fn size(&self) -> Point3D {
        Point3D::new(
            self.max.x() - self.min.x(),
            self.max.y() - self.min.y(),
            self.max.z() - self.min.z(),
        )
    }
    
    /// Returns the width of this AABB.
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x() - self.min.x()
    }
    
    /// Returns the height of this AABB.
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y() - self.min.y()
    }
    
    /// Returns the depth of this AABB.
    #[inline]
    pub fn depth(&self) -> f32 {
        self.max.z() - self.min.z()
    }
    
    /// Returns the volume of this AABB.
    #[inline]
    pub fn volume(&self) -> f32 {
        self.width() * self.height() * self.depth()
    }
    
    /// Returns the surface area of this AABB.
    #[inline]
    pub fn surface_area(&self) -> f32 {
        let width = self.width();
        let height = self.height();
        let depth = self.depth();
        
        2.0 * (width * height + width * depth + height * depth)
    }
    
    /// Check if this AABB contains a 3D point.
    #[inline]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        point.x() >= self.min.x() && point.x() <= self.max.x() &&
        point.y() >= self.min.y() && point.y() <= self.max.y() &&
        point.z() >= self.min.z() && point.z() <= self.max.z()
    }
    
    /// Check if this AABB intersects with another AABB.
    #[inline]
    pub fn intersects(&self, other: &AABB3D) -> bool {
        self.min.x() <= other.max.x() && self.max.x() >= other.min.x() &&
        self.min.y() <= other.max.y() && self.max.y() >= other.min.y() &&
        self.min.z() <= other.max.z() && self.max.z() >= other.min.z()
    }
    
    /// Compute the union of this AABB with another AABB.
    #[inline]
    pub fn union(&self, other: &AABB3D) -> AABB3D {
        AABB3D {
            min: self.min.min(&other.min),
            max: self.max.max(&other.max),
        }
    }
    
    /// Compute the intersection of this AABB with another AABB.
    #[inline]
    pub fn intersection(&self, other: &AABB3D) -> Option<AABB3D> {
        let min_x = self.min.x().max(other.min.x());
        let min_y = self.min.y().max(other.min.y());
        let min_z = self.min.z().max(other.min.z());
        let max_x = self.max.x().min(other.max.x());
        let max_y = self.max.y().min(other.max.y());
        let max_z = self.max.z().min(other.max.z());
        
        if min_x <= max_x && min_y <= max_y && min_z <= max_z {
            Some(AABB3D {
                min: Point3D::new(min_x, min_y, min_z),
                max: Point3D::new(max_x, max_y, max_z),
            })
        } else {
            None
        }
    }
}

// BoundingVolume implementations
impl BoundingVolume for AABB2D {
    fn contains_point(&self, point: &Point) -> bool {
        match point {
            Point::D2(p) => self.contains_point(p),
            _ => false, // Cannot contain a point of different dimensionality
        }
    }
    
    fn intersects(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_aabb) = other.as_any().downcast_ref::<AABB2D>() {
            self.intersects(other_aabb)
        } else {
            false // Different types or dimensionalities
        }
    }
    
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume> {
        if let Some(other_aabb) = other.as_any().downcast_ref::<AABB2D>() {
            Box::new(self.union(other_aabb))
        } else {
            // Return self if dimensionalities don't match
            Box::new(*self)
        }
    }
    
    fn get_min_max(&self) -> (Point, Point) {
        (Point::D2(self.min), Point::D2(self.max))
    }
    
    fn dimensionality(&self) -> Dimensionality {
        Dimensionality::D2
    }
    
    fn clone_volume(&self) -> Box<dyn BoundingVolume> {
        Box::new(*self)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BoundingVolume for AABB3D {
    fn contains_point(&self, point: &Point) -> bool {
        match point {
            Point::D3(p) => self.contains_point(p),
            _ => false, // Cannot contain a point of different dimensionality
        }
    }
    
    fn intersects(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_aabb) = other.as_any().downcast_ref::<AABB3D>() {
            self.intersects(other_aabb)
        } else {
            false // Different types or dimensionalities
        }
    }
    
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume> {
        if let Some(other_aabb) = other.as_any().downcast_ref::<AABB3D>() {
            Box::new(self.union(other_aabb))
        } else {
            // Return self if dimensionalities don't match
            Box::new(*self)
        }
    }
    
    fn get_min_max(&self) -> (Point, Point) {
        (Point::D3(self.min), Point::D3(self.max))
    }
    
    fn dimensionality(&self) -> Dimensionality {
        Dimensionality::D3
    }
    
    fn clone_volume(&self) -> Box<dyn BoundingVolume> {
        Box::new(*self)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// A factory for creating and converting between different bounding volumes.
pub struct BoundingVolumeFactory;

impl BoundingVolumeFactory {
    /// Creates a new 2D AABB.
    pub fn new_aabb_2d(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Box<dyn BoundingVolume> {
        Box::new(AABB2D::new_from_coords(min_x, min_y, max_x, max_y))
    }
    
    /// Creates a new 3D AABB.
    pub fn new_aabb_3d(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Box<dyn BoundingVolume> {
        Box::new(AABB3D::new_from_coords(min_x, min_y, min_z, max_x, max_y, max_z))
    }
    
    /// Creates a bounding volume that contains a point.
    pub fn from_point(point: &Point) -> Box<dyn BoundingVolume> {
        match point {
            Point::D2(p) => {
                Box::new(AABB2D::new(*p, *p))
            },
            Point::D3(p) => {
                Box::new(AABB3D::new(*p, *p))
            },
        }
    }
} 