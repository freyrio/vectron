use std::any::Any;
use crate::core::geometry::point::{Point, Point2D, Point3D};
use crate::core::geometry::bounds::BoundingVolume;
use crate::core::geometry::transform::Transform;
use crate::core::types::Dimensionality;

/// The core trait for any renderable element in the rendering system.
/// 
/// This trait defines the fundamental properties and operations for objects 
/// that can be rendered, regardless of their dimensionality or other characteristics.
/// Implements the "Renderable Trait System" described in the architecture blueprint.
pub trait Renderable: Any + Send + Sync {
    /// Get the dimensionality of this renderable (2D or 3D)
    fn dimensionality(&self) -> Dimensionality;
    
    /// Get the local origin point
    fn origin(&self) -> Point;
    
    /// Set the local origin point
    fn set_origin(&mut self, origin: Point);
    
    /// Get local bounds without any transforms applied
    fn local_bounds(&self) -> Box<dyn BoundingVolume>;
    
    /// Get bounds with current transform applied
    fn world_bounds(&self, transform: &Transform) -> Box<dyn BoundingVolume>;
    
    /// Perform a transform relative to the origin
    fn transform_local(&mut self, transform: &Transform);
    
    /// Create a clone of this renderable
    fn clone_renderable(&self) -> Box<dyn Renderable>;
    
    /// Returns self as Any for downcast operations
    fn as_any(&self) -> &dyn Any;
    
    /// Returns mutable self as Any for downcast operations
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Extension trait for 2D renderable objects
pub trait Object2D: Renderable {
    /// Get the 2D bounding rectangle in local space
    fn local_rect(&self) -> (Point2D, Point2D);
    
    /// Get the 2D bounding rectangle in world space
    fn world_rect(&self, transform: &Transform) -> (Point2D, Point2D);
    
    /// Get the 2D origin point
    fn origin_2d(&self) -> Point2D {
        match self.origin() {
            Point::D2(p) => p,
            _ => panic!("Expected 2D point for 2D renderable"),
        }
    }
}

/// Extension trait for 3D renderable objects
pub trait Object3D: Renderable {
    /// Get the 3D bounding box in local space
    fn local_box(&self) -> (Point3D, Point3D);
    
    /// Get the 3D bounding box in world space
    fn world_box(&self, transform: &Transform) -> (Point3D, Point3D);
    
    /// Get the 3D origin point
    fn origin_3d(&self) -> Point3D {
        match self.origin() {
            Point::D3(p) => p,
            _ => panic!("Expected 3D point for 3D renderable"),
        }
    }
} 