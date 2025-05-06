/*!
 * Geometric primitives and bounding volumes
 */
use crate::core::math::{Vec2, Vec3, approx_eq};
use std::any::Any;

/// Trait for all objects that can provide a bounding volume
pub trait BoundingVolume {
    /// Check if this volume contains a point
    fn contains_point(&self, point: &Vec3) -> bool;
    
    /// Check if this volume completely contains another volume
    fn contains(&self, other: &dyn BoundingVolume) -> bool;
    
    /// Check if this volume intersects with another volume
    fn intersects(&self, other: &dyn BoundingVolume) -> bool;
    
    /// Calculate the union of this volume with another
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume>;
    
    /// Clone this volume (required because Box<dyn Trait> doesn't implement Clone)
    fn clone_box(&self) -> Box<dyn BoundingVolume>;
    
    /// Get this object as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// 2D Rectangle (axis-aligned)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle
    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    /// Create a rectangle from min and max points
    #[inline]
    pub fn from_points(min: Vec2, max: Vec2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            width: max.x - min.x,
            height: max.y - min.y,
        }
    }
    
    /// Get the left edge of the rectangle
    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }
    
    /// Get the right edge of the rectangle
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }
    
    /// Get the top edge of the rectangle
    #[inline]
    pub fn top(&self) -> f32 {
        self.y
    }
    
    /// Get the bottom edge of the rectangle
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
    
    /// Get the center of the rectangle
    #[inline]
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
    
    /// Check if the rectangle contains a point
    #[inline]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        point.x >= self.x && 
        point.x <= self.x + self.width && 
        point.y >= self.y && 
        point.y <= self.y + self.height
    }
    
    /// Check if this rectangle completely contains another
    #[inline]
    pub fn contains(&self, other: &Rect) -> bool {
        self.x <= other.x && 
        self.y <= other.y && 
        self.x + self.width >= other.x + other.width && 
        self.y + self.height >= other.y + other.height
    }
    
    /// Check if this rectangle intersects with another
    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width && 
        self.x + self.width > other.x && 
        self.y < other.y + other.height && 
        self.y + self.height > other.y
    }
    
    /// Calculate the intersection of this rectangle with another
    #[inline]
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }
        
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = (self.x + self.width).min(other.x + other.width) - x;
        let height = (self.y + self.height).min(other.y + other.height) - y;
        
        if width <= 0.0 || height <= 0.0 {
            return None;
        }
        
        Some(Rect::new(x, y, width, height))
    }
    
    /// Calculate the union of this rectangle with another
    #[inline]
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let width = (self.x + self.width).max(other.x + other.width) - x;
        let height = (self.y + self.height).max(other.y + other.height) - y;
        
        Rect::new(x, y, width, height)
    }
    
    /// Expand the rectangle to include a point
    #[inline]
    pub fn expand_to_include(&mut self, point: &Vec2) {
        let right = self.x + self.width;
        let bottom = self.y + self.height;
        
        if point.x < self.x {
            self.width += self.x - point.x;
            self.x = point.x;
        } else if point.x > right {
            self.width = point.x - self.x;
        }
        
        if point.y < self.y {
            self.height += self.y - point.y;
            self.y = point.y;
        } else if point.y > bottom {
            self.height = point.y - self.y;
        }
    }
    
    /// Inflate the rectangle by a specified amount in all directions
    #[inline]
    pub fn inflate(&mut self, dx: f32, dy: f32) {
        self.x -= dx;
        self.y -= dy;
        self.width += dx * 2.0;
        self.height += dy * 2.0;
    }
    
    /// Convert to array [x, y, width, height]
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.width, self.height]
    }
    
    /// Get all four corners of the rectangle
    #[inline]
    pub fn corners(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.x, self.y),
            Vec2::new(self.x + self.width, self.y),
            Vec2::new(self.x + self.width, self.y + self.height),
            Vec2::new(self.x, self.y + self.height),
        ]
    }
}

impl Default for Rect {
    #[inline]
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

/// 3D Axis-Aligned Bounding Box (AABB)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    /// Create a new bounding box from min and max points
    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    
    /// Create a bounding box from a center point and half-extents
    #[inline]
    pub fn from_center(center: Vec3, half_extents: Vec3) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }
    
    /// Create a bounding box from a list of points
    #[inline]
    pub fn from_points(points: &[Vec3]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        
        let mut min = points[0];
        let mut max = points[0];
        
        for point in points.iter().skip(1) {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);
            
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
        }
        
        Some(Self { min, max })
    }
    
    /// Get the center of the bounding box
    #[inline]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
    
    /// Get the extents (dimensions) of the bounding box
    #[inline]
    pub fn extents(&self) -> Vec3 {
        self.max - self.min
    }
    
    /// Get the half-extents of the bounding box
    #[inline]
    pub fn half_extents(&self) -> Vec3 {
        self.extents() * 0.5
    }
    
    /// Check if the bounding box is valid (min <= max)
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }
    
    /// Get the volume of the bounding box
    #[inline]
    pub fn volume(&self) -> f32 {
        let extents = self.extents();
        extents.x * extents.y * extents.z
    }
    
    /// Expand the bounding box to include a point
    #[inline]
    pub fn expand_to_include(&mut self, point: &Vec3) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }
    
    /// Expand the bounding box by merging with another
    #[inline]
    pub fn merge(&mut self, other: &BoundingBox) {
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);
        
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);
    }
    
    /// Calculate the union of this bounding box with another
    #[inline]
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }
    
    /// Check if the bounding box contains a point
    #[inline]
    pub fn contains_point(&self, point: &Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
    
    /// Check if this bounding box completely contains another
    #[inline]
    pub fn contains(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.min.x && self.max.x >= other.max.x &&
        self.min.y <= other.min.y && self.max.y >= other.max.y &&
        self.min.z <= other.min.z && self.max.z >= other.max.z
    }
    
    /// Check if this bounding box intersects with another
    #[inline]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
    
    /// Get all eight corners of the bounding box
    #[inline]
    pub fn corners(&self) -> [Vec3; 8] {
        [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ]
    }
}

impl Default for BoundingBox {
    #[inline]
    fn default() -> Self {
        Self {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
        }
    }
}

impl BoundingVolume for BoundingBox {
    fn contains_point(&self, point: &Vec3) -> bool {
        self.contains_point(point)
    }
    
    fn contains(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_box) = other.as_any().downcast_ref::<BoundingBox>() {
            self.contains(other_box)
        } else if let Some(sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            // A box contains a sphere if it contains the center and the center +/- radius in each axis
            self.contains_point(&sphere.center) &&
            self.contains_point(&Vec3::new(sphere.center.x - sphere.radius, sphere.center.y, sphere.center.z)) &&
            self.contains_point(&Vec3::new(sphere.center.x + sphere.radius, sphere.center.y, sphere.center.z)) &&
            self.contains_point(&Vec3::new(sphere.center.x, sphere.center.y - sphere.radius, sphere.center.z)) &&
            self.contains_point(&Vec3::new(sphere.center.x, sphere.center.y + sphere.radius, sphere.center.z)) &&
            self.contains_point(&Vec3::new(sphere.center.x, sphere.center.y, sphere.center.z - sphere.radius)) &&
            self.contains_point(&Vec3::new(sphere.center.x, sphere.center.y, sphere.center.z + sphere.radius))
        } else {
            false
        }
    }
    
    fn intersects(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_box) = other.as_any().downcast_ref::<BoundingBox>() {
            self.intersects(other_box)
        } else if let Some(sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            // Find closest point on the box to the sphere center
            let closest = Vec3::new(
                sphere.center.x.clamp(self.min.x, self.max.x),
                sphere.center.y.clamp(self.min.y, self.max.y),
                sphere.center.z.clamp(self.min.z, self.max.z),
            );
            
            // If the closest point is within the sphere radius, they intersect
            sphere.center.distance_squared(&closest) <= sphere.radius * sphere.radius
        } else {
            false
        }
    }
    
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume> {
        if let Some(other_box) = other.as_any().downcast_ref::<BoundingBox>() {
            Box::new(self.union(other_box))
        } else if let Some(sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            // Create box that encloses the sphere
            let sphere_box = BoundingBox {
                min: Vec3::new(
                    sphere.center.x - sphere.radius,
                    sphere.center.y - sphere.radius,
                    sphere.center.z - sphere.radius,
                ),
                max: Vec3::new(
                    sphere.center.x + sphere.radius,
                    sphere.center.y + sphere.radius,
                    sphere.center.z + sphere.radius,
                ),
            };
            
            // Union of the two boxes
            Box::new(self.union(&sphere_box))
        } else {
            // Default to just cloning this volume
            self.clone_box()
        }
    }
    
    fn clone_box(&self) -> Box<dyn BoundingVolume> {
        Box::new(*self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Bounding Sphere
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    /// Create a new bounding sphere from center and radius
    #[inline]
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
    
    /// Create a bounding sphere from a list of points
    #[inline]
    pub fn from_points(points: &[Vec3]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        
        // Calculate center (average of points)
        let mut center = Vec3::zero();
        for point in points {
            center += *point;
        }
        center = center / points.len() as f32;
        
        // Find radius (max distance from center)
        let mut radius_squared = 0.0;
        for point in points {
            let dist_squared = center.distance_squared(point);
            if dist_squared > radius_squared {
                radius_squared = dist_squared;
            }
        }
        
        Some(Self {
            center,
            radius: radius_squared.sqrt(),
        })
    }
    
    /// Create a bounding sphere from a bounding box
    #[inline]
    pub fn from_bounding_box(box_: &BoundingBox) -> Self {
        let center = box_.center();
        let radius = box_.min.distance(&box_.max) * 0.5;
        
        Self { center, radius }
    }
    
    /// Check if the sphere contains a point
    #[inline]
    pub fn contains_point(&self, point: &Vec3) -> bool {
        self.center.distance_squared(point) <= self.radius * self.radius
    }
    
    /// Check if this sphere completely contains another
    #[inline]
    pub fn contains(&self, other: &BoundingSphere) -> bool {
        let distance = self.center.distance(&other.center);
        distance + other.radius <= self.radius
    }
    
    /// Check if this sphere intersects with another
    #[inline]
    pub fn intersects(&self, other: &BoundingSphere) -> bool {
        let distance_squared = self.center.distance_squared(&other.center);
        let radii_sum = self.radius + other.radius;
        distance_squared <= radii_sum * radii_sum
    }
    
    /// Calculate the union of this sphere with another
    #[inline]
    pub fn union(&self, other: &BoundingSphere) -> BoundingSphere {
        let direction = other.center - self.center;
        let distance = direction.length();
        
        // If one sphere contains the other, return the larger one
        if distance + other.radius <= self.radius {
            return *self;
        }
        
        if distance + self.radius <= other.radius {
            return *other;
        }
        
        // Otherwise, create a new sphere that encloses both
        let normalized_dir = if distance > 0.0 {
            direction / distance
        } else {
            Vec3::unit_x() // arbitrary direction if spheres share same center
        };
        
        let p1 = self.center - normalized_dir * self.radius;
        let p2 = other.center + normalized_dir * other.radius;
        
        // New sphere's center is halfway between the extreme points
        let center = (p1 + p2) * 0.5;
        // New radius is half the distance between extreme points
        let radius = p1.distance(&p2) * 0.5;
        
        BoundingSphere { center, radius }
    }
}

impl Default for BoundingSphere {
    #[inline]
    fn default() -> Self {
        Self {
            center: Vec3::zero(),
            radius: 0.0,
        }
    }
}

impl BoundingVolume for BoundingSphere {
    fn contains_point(&self, point: &Vec3) -> bool {
        self.contains_point(point)
    }
    
    fn contains(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            self.contains(other_sphere)
        } else if let Some(box_) = other.as_any().downcast_ref::<BoundingBox>() {
            // A sphere contains a box if it contains all corners
            let corners = box_.corners();
            corners.iter().all(|corner| self.contains_point(corner))
        } else {
            false
        }
    }
    
    fn intersects(&self, other: &dyn BoundingVolume) -> bool {
        if let Some(other_sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            self.intersects(other_sphere)
        } else if let Some(box_) = other.as_any().downcast_ref::<BoundingBox>() {
            // Find closest point on the box to the sphere center
            let closest = Vec3::new(
                self.center.x.clamp(box_.min.x, box_.max.x),
                self.center.y.clamp(box_.min.y, box_.max.y),
                self.center.z.clamp(box_.min.z, box_.max.z),
            );
            
            // If the closest point is within the sphere radius, they intersect
            self.center.distance_squared(&closest) <= self.radius * self.radius
        } else {
            false
        }
    }
    
    fn union(&self, other: &dyn BoundingVolume) -> Box<dyn BoundingVolume> {
        if let Some(other_sphere) = other.as_any().downcast_ref::<BoundingSphere>() {
            Box::new(self.union(other_sphere))
        } else if let Some(box_) = other.as_any().downcast_ref::<BoundingBox>() {
            // Create a sphere from the box
            let box_sphere = BoundingSphere::from_bounding_box(box_);
            
            // Union of the two spheres
            Box::new(self.union(&box_sphere))
        } else {
            // Default to just cloning this volume
            self.clone_box()
        }
    }
    
    fn clone_box(&self) -> Box<dyn BoundingVolume> {
        Box::new(*self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 2D Point type (for consistent naming)
pub type Point = Vec2;

/// 2D Line segment
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    /// Create a new line segment from start and end points
    #[inline]
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
    
    /// Get the length of the line segment
    #[inline]
    pub fn length(&self) -> f32 {
        self.start.distance(&self.end)
    }
    
    /// Get the length squared of the line segment
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.start.distance_squared(&self.end)
    }
    
    /// Get the direction vector of the line segment
    #[inline]
    pub fn direction(&self) -> Vec2 {
        (self.end - self.start).normalized()
    }
    
    /// Check if the line segment intersects with another
    #[inline]
    pub fn intersects(&self, other: &LineSegment) -> bool {
        // Check if the line segments intersect using cross products
        let p1 = self.start;
        let p2 = self.end;
        let p3 = other.start;
        let p4 = other.end;
        
        let d1 = (p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x);
        let d2 = (p4.x - p3.x) * (p2.y - p3.y) - (p4.y - p3.y) * (p2.x - p3.x);
        let d3 = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
        let d4 = (p2.x - p1.x) * (p4.y - p1.y) - (p2.y - p1.y) * (p4.x - p1.x);
        
        return (d1 * d2 <= 0.0) && (d3 * d4 <= 0.0);
    }
    
    /// Calculate the intersection point with another line segment
    #[inline]
    pub fn intersection_point(&self, other: &LineSegment) -> Option<Point> {
        if !self.intersects(other) {
            return None;
        }
        
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;
        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;
        
        let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
        
        if approx_eq(denominator, 0.0, 1e-6) {
            return None; // Lines are parallel
        }
        
        let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
        
        let x = x1 + ua * (x2 - x1);
        let y = y1 + ua * (y2 - y1);
        
        Some(Point::new(x, y))
    }
    
    /// Find the closest point on the line segment to a given point
    #[inline]
    pub fn closest_point(&self, point: &Point) -> Point {
        let segment_vector = self.end - self.start;
        let segment_length_squared = segment_vector.length_squared();
        
        if segment_length_squared == 0.0 {
            return self.start; // Both points are the same
        }
        
        // Calculate projection of point onto line segment
        let t = ((point.x - self.start.x) * segment_vector.x + 
                (point.y - self.start.y) * segment_vector.y) / 
                segment_length_squared;
        
        if t < 0.0 {
            return self.start; // Closest to start point
        } else if t > 1.0 {
            return self.end; // Closest to end point
        }
        
        // Projected point on line segment
        Point::new(
            self.start.x + t * segment_vector.x,
            self.start.y + t * segment_vector.y
        )
    }
    
    /// Get the distance from the line segment to a point
    #[inline]
    pub fn distance_to_point(&self, point: &Point) -> f32 {
        let closest = self.closest_point(point);
        closest.distance(point)
    }
}
