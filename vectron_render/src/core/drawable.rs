/*!
 * Core traits for drawable objects in the rendering system
 */
use std::any::Any;
use crate::core::color::Color;
use crate::core::math::{Vec2, Vec3, Rect, BoundingBox, LineSegment};
use crate::core::transform::Transform;

/// Context for rendering operations
pub struct RenderContext {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub device_pixel_ratio: f32,
}

/// Hints for rendering optimization
#[derive(Clone, Debug, Default)]
pub struct RenderHints {
    pub can_batch: bool,
    pub needs_alpha_blending: bool,
    pub can_use_shader_transform: bool,
}

/// Core trait for all drawable objects
pub trait Drawable {
    /// Get the type name of this drawable
    fn type_name(&self) -> &'static str;
    
    /// Get rendering hints for optimization
    fn get_render_hints(&self) -> RenderHints {
        RenderHints::default()
    }
    
    /// Get the 2D bounding rectangle of this drawable
    fn bounds(&self) -> Rect;
    
    /// Convert to geometry for rendering
    fn to_geometry(&self) -> Vec<Vec2>;
    
    /// Convert this drawable to a specific type via downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Convert this drawable to a mutable specific type via downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    /// Check if this drawable is a 2D object
    fn is_2d(&self) -> bool;
    
    /// Check if this drawable is a 3D object
    fn is_3d(&self) -> bool;
    
    /// Get this drawable as a Drawable2D if it is one
    fn as_drawable_2d(&self) -> Option<&dyn Drawable2D> {
        if self.is_2d() {
            // Safety: We've verified this is a 2D drawable with is_2d()
            unsafe {
                Some(&*(self as *const dyn Drawable as *const dyn Drawable2D))
            }
        } else {
            None
        }
    }
    
    /// Get this drawable as a mutable Drawable2D if it is one
    fn as_drawable_2d_mut(&mut self) -> Option<&mut dyn Drawable2D> {
        if self.is_2d() {
            // Safety: We've verified this is a 2D drawable with is_2d()
            unsafe {
                Some(&mut *(self as *mut dyn Drawable as *mut dyn Drawable2D))
            }
        } else {
            None
        }
    }
    
    /// Get this drawable as a Drawable3D if it is one
    fn as_drawable_3d(&self) -> Option<&dyn Drawable3D> {
        if self.is_3d() {
            // Safety: We've verified this is a 3D drawable with is_3d()
            unsafe {
                Some(&*(self as *const dyn Drawable as *const dyn Drawable3D))
            }
        } else {
            None
        }
    }
    
    /// Get this drawable as a mutable Drawable3D if it is one
    fn as_drawable_3d_mut(&mut self) -> Option<&mut dyn Drawable3D> {
        if self.is_3d() {
            // Safety: We've verified this is a 3D drawable with is_3d()
            unsafe {
                Some(&mut *(self as *mut dyn Drawable as *mut dyn Drawable3D))
            }
        } else {
            None
        }
    }
    
    /// Create a cloned version of this drawable
    fn clone_drawable(&self) -> Box<dyn Drawable>;
}

/// Specialized trait for 2D drawables
pub trait Drawable2D: Drawable {
    /// Get the bounding rectangle of this drawable
    fn get_bounds(&self) -> Rect;
    
    /// Get the geometry for this drawable
    fn get_geometry(&self) -> Vec<Vec2>;
    
    /// Get the triangulation for this drawable if available
    fn get_triangulation(&self) -> Option<Vec<u16>> {
        None
    }
    
    /// Check if this drawable contains a point
    fn contains_point(&self, point: Vec2) -> bool;
    
    /// Apply a 2D transform to this drawable
    fn transform(&mut self, transform: &Transform);
    
    /// Get a transformed copy of this drawable
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable2D>;
}

/// Specialized trait for 3D drawables
pub trait Drawable3D: Drawable {
    /// Get the bounding box of this drawable
    fn get_bounds(&self) -> BoundingBox;
    
    /// Get the vertices for this drawable
    fn get_vertices(&self) -> Vec<Vec3>;
    
    /// Get the normals for this drawable if available
    fn get_normals(&self) -> Option<Vec<Vec3>> {
        None
    }
    
    /// Get the texture coordinates for this drawable if available
    fn get_texture_coords(&self) -> Option<Vec<Vec2>> {
        None
    }
    
    /// Get the indices for this drawable if available (triangles)
    fn get_indices(&self) -> Option<Vec<u32>> {
        None
    }
    
    /// Apply a 3D transform to this drawable
    fn transform(&mut self, transform: &Transform);
    
    /// Get a transformed copy of this drawable
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable3D>;
}

/// A rectangle shape
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self { x, y, width, height, color }
    }
}

impl Drawable for Rectangle {
    fn type_name(&self) -> &'static str {
        "Rectangle"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: true,
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        vec![
            Vec2::new(self.x, self.y),
            Vec2::new(self.x + self.width, self.y),
            Vec2::new(self.x + self.width, self.y + self.height),
            Vec2::new(self.x, self.y + self.height),
        ]
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        true
    }
    
    fn is_3d(&self) -> bool {
        false
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable2D for Rectangle {
    fn get_bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
    
    fn get_geometry(&self) -> Vec<Vec2> {
        vec![
            Vec2::new(self.x, self.y),
            Vec2::new(self.x + self.width, self.y),
            Vec2::new(self.x + self.width, self.y + self.height),
            Vec2::new(self.x, self.y + self.height),
        ]
    }
    
    fn get_triangulation(&self) -> Option<Vec<u16>> {
        Some(vec![0, 1, 2, 0, 2, 3])
    }
    
    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    fn transform(&mut self, transform: &Transform) {
        let bounds = self.get_bounds();
        let new_bounds = transform.transform_rect(&bounds);
        self.x = new_bounds.x;
        self.y = new_bounds.y;
        self.width = new_bounds.width;
        self.height = new_bounds.height;
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable2D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// A circle shape
#[derive(Clone, Debug)]
pub struct Circle {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
    pub color: Color,
}

impl Circle {
    /// Create a new circle
    pub fn new(center_x: f32, center_y: f32, radius: f32, color: Color) -> Self {
        Self { center_x, center_y, radius, color }
    }
}

impl Drawable for Circle {
    fn type_name(&self) -> &'static str {
        "Circle"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: true,
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        Rect::new(
            self.center_x - self.radius,
            self.center_y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        // Approximate with 24 points
        let segments = 24;
        let mut points = Vec::with_capacity(segments);
        
        for i in 0..segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = self.center_x + self.radius * angle.cos();
            let y = self.center_y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }
        
        points
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        true
    }
    
    fn is_3d(&self) -> bool {
        false
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable2D for Circle {
    fn get_bounds(&self) -> Rect {
        Rect::new(
            self.center_x - self.radius,
            self.center_y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
    
    fn get_geometry(&self) -> Vec<Vec2> {
        // Approximate with 24 points
        let segments = 24;
        let mut points = Vec::with_capacity(segments);
        
        for i in 0..segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = self.center_x + self.radius * angle.cos();
            let y = self.center_y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }
        
        points
    }
    
    fn contains_point(&self, point: Vec2) -> bool {
        let dx = point.x - self.center_x;
        let dy = point.y - self.center_y;
        (dx * dx + dy * dy) <= (self.radius * self.radius)
    }
    
    fn transform(&mut self, transform: &Transform) {
        let center = Vec2::new(self.center_x, self.center_y);
        let transformed_center = transform.transform_point_2d(&center);
        
        // For the radius, we need to consider the scale
        let scale = transform.get_scale_2d();
        let avg_scale = (scale.x.abs() + scale.y.abs()) * 0.5;
        
        self.center_x = transformed_center.x;
        self.center_y = transformed_center.y;
        self.radius *= avg_scale;
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable2D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// Text shape
#[derive(Clone, Debug)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub font_size: f32,
    pub color: Color,
}

impl Text {
    /// Create new text
    pub fn new(x: f32, y: f32, text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self { x, y, text: text.into(), font_size, color }
    }
}

impl Drawable for Text {
    fn type_name(&self) -> &'static str {
        "Text"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: false, // Text typically can't be batched with regular geometry
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        // This is approximate since real bounds depend on font metrics
        Rect::new(
            self.x,
            self.y - self.font_size,
            self.text.len() as f32 * self.font_size * 0.6,
            self.font_size * 1.2,
        )
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        // Text geometry is typically generated by the backend
        vec![
            Vec2::new(self.x, self.y),
        ]
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        true
    }
    
    fn is_3d(&self) -> bool {
        false
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable2D for Text {
    fn get_bounds(&self) -> Rect {
        // This is approximate since real bounds depend on font metrics
        Rect::new(
            self.x,
            self.y - self.font_size,
            self.text.len() as f32 * self.font_size * 0.6,
            self.font_size * 1.2,
        )
    }
    
    fn get_geometry(&self) -> Vec<Vec2> {
        // Text geometry is typically generated by the backend
        vec![
            Vec2::new(self.x, self.y),
        ]
    }
    
    fn contains_point(&self, point: Vec2) -> bool {
        // Simple box check
        let bounds = self.get_bounds();
        bounds.contains_point(&point)
    }
    
    fn transform(&mut self, transform: &Transform) {
        let pos = Vec2::new(self.x, self.y);
        let transformed_pos = transform.transform_point_2d(&pos);
        
        // Extract scaling for font size
        let scale = transform.get_scale_2d();
        let avg_scale = (scale.x.abs() + scale.y.abs()) * 0.5;
        
        self.x = transformed_pos.x;
        self.y = transformed_pos.y;
        self.font_size *= avg_scale;
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable2D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// 3D Box shape (cube)
#[derive(Clone, Debug)]
pub struct Box3D {
    pub center: Vec3,
    pub size: Vec3,
    pub color: Color,
}

impl Box3D {
    /// Create a new 3D box
    pub fn new(center: Vec3, size: Vec3, color: Color) -> Self {
        Self { center, size, color }
    }
}

impl Drawable for Box3D {
    fn type_name(&self) -> &'static str {
        "Box3D"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: true,
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        // Project 3D bounds to 2D
        let bbox = self.as_drawable_3d().unwrap().get_bounds();
        Rect::new(
            bbox.min.x,
            bbox.min.y,
            bbox.max.x - bbox.min.x,
            bbox.max.y - bbox.min.y,
        )
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        // Project 3D vertices to 2D
        let vertices = self.as_drawable_3d().unwrap().get_vertices();
        vertices.iter().map(|v| Vec2::new(v.x, v.y)).collect()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        false
    }
    
    fn is_3d(&self) -> bool {
        true
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable3D for Box3D {
    fn get_bounds(&self) -> BoundingBox {
        BoundingBox::from_center(
            self.center,
            self.size * 0.5,
        )
    }
    
    fn get_vertices(&self) -> Vec<Vec3> {
        let half_size = self.size * 0.5;
        vec![
            // Front face
            Vec3::new(self.center.x - half_size.x, self.center.y - half_size.y, self.center.z + half_size.z),
            Vec3::new(self.center.x + half_size.x, self.center.y - half_size.y, self.center.z + half_size.z),
            Vec3::new(self.center.x + half_size.x, self.center.y + half_size.y, self.center.z + half_size.z),
            Vec3::new(self.center.x - half_size.x, self.center.y + half_size.y, self.center.z + half_size.z),
            // Back face
            Vec3::new(self.center.x - half_size.x, self.center.y - half_size.y, self.center.z - half_size.z),
            Vec3::new(self.center.x + half_size.x, self.center.y - half_size.y, self.center.z - half_size.z),
            Vec3::new(self.center.x + half_size.x, self.center.y + half_size.y, self.center.z - half_size.z),
            Vec3::new(self.center.x - half_size.x, self.center.y + half_size.y, self.center.z - half_size.z),
        ]
    }
    
    fn get_normals(&self) -> Option<Vec<Vec3>> {
        Some(vec![
            // Front face
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            // Back face
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ])
    }
    
    fn get_indices(&self) -> Option<Vec<u32>> {
        Some(vec![
            // Front face
            0, 1, 2, 0, 2, 3,
            // Back face
            4, 7, 6, 4, 6, 5,
            // Top face
            3, 2, 6, 3, 6, 7,
            // Bottom face
            0, 4, 5, 0, 5, 1,
            // Right face
            1, 5, 6, 1, 6, 2,
            // Left face
            0, 3, 7, 0, 7, 4,
        ])
    }
    
    fn transform(&mut self, transform: &Transform) {
        self.center = transform.transform_point_3d(&self.center);
        
        // For size, we need to consider non-uniform scaling
        let scale = transform.get_scale();
        self.size.x *= scale.x.abs();
        self.size.y *= scale.y.abs();
        self.size.z *= scale.z.abs();
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable3D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// 3D Sphere shape
#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub segments: u32,
}

impl Sphere {
    /// Create a new sphere
    pub fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Self { center, radius, color, segments: 16 }
    }
    
    /// Create a new sphere with custom segments
    pub fn new_with_segments(center: Vec3, radius: f32, color: Color, segments: u32) -> Self {
        Self { center, radius, color, segments }
    }
}

impl Drawable for Sphere {
    fn type_name(&self) -> &'static str {
        "Sphere"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: true,
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        // Project 3D bounds to 2D
        Rect::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        // Project 3D sphere to 2D circle
        let segments = self.segments as usize;
        let mut points = Vec::with_capacity(segments);
        
        for i in 0..segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }
        
        points
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        false
    }
    
    fn is_3d(&self) -> bool {
        true
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable3D for Sphere {
    fn get_bounds(&self) -> BoundingBox {
        BoundingBox::from_center(
            self.center,
            Vec3::new(self.radius, self.radius, self.radius),
        )
    }
    
    fn get_vertices(&self) -> Vec<Vec3> {
        let mut vertices = Vec::new();
        let segments = self.segments as usize;
        
        // Generate vertices for UV sphere
        for i in 0..segments + 1 {
            let phi = std::f32::consts::PI * (i as f32) / (segments as f32);
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            
            for j in 0..segments * 2 {
                let theta = std::f32::consts::PI * 2.0 * (j as f32) / (segments as f32 * 2.0);
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
                
                let x = self.center.x + self.radius * sin_phi * cos_theta;
                let y = self.center.y + self.radius * cos_phi;
                let z = self.center.z + self.radius * sin_phi * sin_theta;
                
                vertices.push(Vec3::new(x, y, z));
            }
        }
        
        vertices
    }
    
    fn get_normals(&self) -> Option<Vec<Vec3>> {
        // For a sphere, the normals are just the normalized direction from center to vertex
        let vertices = self.get_vertices();
        let mut normals = Vec::with_capacity(vertices.len());
        
        for vertex in &vertices {
            let normal = (*vertex - self.center).normalized();
            normals.push(normal);
        }
        
        Some(normals)
    }
    
    fn get_indices(&self) -> Option<Vec<u32>> {
        let segments = self.segments as u32;
        let mut indices = Vec::new();
        
        // Generate indices for sphere triangles
        for i in 0..segments {
            for j in 0..segments * 2 {
                let next_j = (j + 1) % (segments * 2);
                let current_row = i * (segments * 2);
                let next_row = (i + 1) * (segments * 2);
                
                // Upper triangle
                indices.push(current_row + j);
                indices.push(next_row + j);
                indices.push(next_row + next_j);
                
                // Lower triangle
                indices.push(current_row + j);
                indices.push(next_row + next_j);
                indices.push(current_row + next_j);
            }
        }
        
        Some(indices)
    }
    
    fn transform(&mut self, transform: &Transform) {
        self.center = transform.transform_point_3d(&self.center);
        
        // For radius, use the average scale
        let scale = transform.get_scale();
        let avg_scale = (scale.x.abs() + scale.y.abs() + scale.z.abs()) / 3.0;
        self.radius *= avg_scale;
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable3D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// A path shape for complex 2D drawing
#[derive(Clone, Debug)]
pub struct Path {
    pub points: Vec<Vec2>,
    pub closed: bool,
    pub color: Color,
}

impl Path {
    /// Create a new empty path
    pub fn new(color: Color) -> Self {
        Self {
            points: Vec::new(),
            closed: false,
            color,
        }
    }
    
    /// Create a path with initial points
    pub fn with_points(points: Vec<Vec2>, color: Color) -> Self {
        Self {
            points,
            closed: false,
            color,
        }
    }
    
    /// Create a closed path
    pub fn closed(mut self) -> Self {
        self.closed = true;
        self
    }
    
    /// Add a point to the path
    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(Vec2::new(x, y));
    }
    
    /// Add multiple points to the path
    pub fn add_points(&mut self, points: &[Vec2]) {
        self.points.extend_from_slice(points);
    }
    
    /// Get the line segments that make up this path
    pub fn get_segments(&self) -> Vec<LineSegment> {
        if self.points.len() < 2 {
            return Vec::new();
        }
        
        let mut segments = Vec::with_capacity(
            if self.closed { self.points.len() } else { self.points.len() - 1 }
        );
        
        for i in 0..self.points.len() - 1 {
            segments.push(LineSegment::new(self.points[i], self.points[i + 1]));
        }
        
        if self.closed && self.points.len() >= 3 {
            segments.push(LineSegment::new(
                *self.points.last().unwrap(),
                self.points[0]
            ));
        }
        
        segments
    }
}

impl Drawable for Path {
    fn type_name(&self) -> &'static str {
        "Path"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: true,
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        if self.points.is_empty() {
            return Rect::default();
        }
        
        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = min_x;
        let mut max_y = min_y;
        
        for point in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        self.points.clone()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        true
    }
    
    fn is_3d(&self) -> bool {
        false
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable2D for Path {
    fn get_bounds(&self) -> Rect {
        self.bounds()
    }
    
    fn get_geometry(&self) -> Vec<Vec2> {
        self.points.clone()
    }
    
    fn get_triangulation(&self) -> Option<Vec<u16>> {
        // For a basic path, we don't provide built-in triangulation
        // This would require a complex triangulation algorithm for concave shapes
        // The renderer would typically handle this using techniques like
        // the stencil buffer for fills
        None
    }
    
    fn contains_point(&self, point: Vec2) -> bool {
        // This is a non-trivial computation for complex paths
        // Implementation of the point-in-polygon algorithm
        // Ray casting algorithm: count how many times a ray from the
        // point crosses the path edges
        
        if self.points.len() < 3 || !self.closed {
            return false; // Open paths or paths with fewer than 3 points don't have an interior
        }
        
        let segments = self.get_segments();
        let ray_end = Vec2::new(point.x + 10000.0, point.y); // Create a ray to the right
        let ray = LineSegment::new(point, ray_end);
        
        let mut intersections = 0;
        for segment in &segments {
            if ray.intersects(segment) {
                intersections += 1;
            }
        }
        
        // If number of intersections is odd, the point is inside
        intersections % 2 == 1
    }
    
    fn transform(&mut self, transform: &Transform) {
        for point in &mut self.points {
            *point = transform.transform_point_2d(point);
        }
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable2D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
}

/// A 3D mesh with vertices, faces, normals, and optional texture coordinates
#[derive(Clone, Debug)]
pub struct Mesh3D {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Option<Vec<Vec3>>,
    pub texture_coords: Option<Vec<Vec2>>,
    pub colors: Option<Vec<Color>>,
    pub color: Color, // Default color if per-vertex colors aren't provided
}

impl Mesh3D {
    /// Create a new empty mesh
    pub fn new(color: Color) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: None,
            texture_coords: None,
            colors: None,
            color,
        }
    }
    
    /// Create a mesh with vertices and indices
    pub fn with_vertices_and_indices(vertices: Vec<Vec3>, indices: Vec<u32>, color: Color) -> Self {
        Self {
            vertices,
            indices,
            normals: None,
            texture_coords: None,
            colors: None,
            color,
        }
    }
    
    /// Add normals to the mesh
    pub fn with_normals(mut self, normals: Vec<Vec3>) -> Self {
        self.normals = Some(normals);
        self
    }
    
    /// Add texture coordinates to the mesh
    pub fn with_texture_coords(mut self, texture_coords: Vec<Vec2>) -> Self {
        self.texture_coords = Some(texture_coords);
        self
    }
    
    /// Add per-vertex colors to the mesh
    pub fn with_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = Some(colors);
        self
    }
    
    /// Calculate normals for the mesh if they aren't already present
    pub fn calculate_normals(&mut self) {
        if self.normals.is_some() {
            return;
        }
        
        let mut normals = vec![Vec3::zero(); self.vertices.len()];
        
        // Calculate normals for each triangle and add them to the vertices
        for i in (0..self.indices.len()).step_by(3) {
            if i + 2 >= self.indices.len() {
                break;
            }
            
            let idx1 = self.indices[i] as usize;
            let idx2 = self.indices[i + 1] as usize;
            let idx3 = self.indices[i + 2] as usize;
            
            if idx1 >= self.vertices.len() || idx2 >= self.vertices.len() || idx3 >= self.vertices.len() {
                continue;
            }
            
            let v1 = self.vertices[idx1];
            let v2 = self.vertices[idx2];
            let v3 = self.vertices[idx3];
            
            let edge1 = v2 - v1;
            let edge2 = v3 - v1;
            let normal = edge1.cross(&edge2).normalized();
            
            normals[idx1] += normal;
            normals[idx2] += normal;
            normals[idx3] += normal;
        }
        
        // Normalize all normals
        for normal in &mut normals {
            if normal.length_squared() > 0.0 {
                *normal = normal.normalized();
            } else {
                *normal = Vec3::new(0.0, 1.0, 0.0); // Default normal if calculation failed
            }
        }
        
        self.normals = Some(normals);
    }
    
    /// Calculate the bounding box of the mesh
    pub fn calculate_bounds(&self) -> BoundingBox {
        if self.vertices.is_empty() {
            return BoundingBox::default();
        }
        
        let mut min = self.vertices[0];
        let mut max = self.vertices[0];
        
        for vertex in &self.vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }
        
        BoundingBox::new(min, max)
    }
}

impl Drawable for Mesh3D {
    fn type_name(&self) -> &'static str {
        "Mesh3D"
    }
    
    fn get_render_hints(&self) -> RenderHints {
        RenderHints {
            can_batch: false, // Meshes are typically complex and handled individually
            needs_alpha_blending: self.color.a < 1.0,
            can_use_shader_transform: true,
        }
    }
    
    fn bounds(&self) -> Rect {
        // Project 3D bounds to 2D
        let bbox = self.as_drawable_3d().unwrap().get_bounds();
        Rect::new(
            bbox.min.x,
            bbox.min.y,
            bbox.max.x - bbox.min.x,
            bbox.max.y - bbox.min.y,
        )
    }
    
    fn to_geometry(&self) -> Vec<Vec2> {
        // Project 3D vertices to 2D
        self.vertices.iter().map(|v| Vec2::new(v.x, v.y)).collect()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn is_2d(&self) -> bool {
        false
    }
    
    fn is_3d(&self) -> bool {
        true
    }
    
    fn clone_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Drawable3D for Mesh3D {
    fn get_bounds(&self) -> BoundingBox {
        self.calculate_bounds()
    }
    
    fn get_vertices(&self) -> Vec<Vec3> {
        self.vertices.clone()
    }
    
    fn get_normals(&self) -> Option<Vec<Vec3>> {
        self.normals.clone()
    }
    
    fn get_texture_coords(&self) -> Option<Vec<Vec2>> {
        self.texture_coords.clone()
    }
    
    fn get_indices(&self) -> Option<Vec<u32>> {
        Some(self.indices.clone())
    }
    
    fn transform(&mut self, transform: &Transform) {
        // Transform vertices
        for vertex in &mut self.vertices {
            *vertex = transform.transform_point_3d(vertex);
        }
        
        // Transform normals if they exist
        if let Some(normals) = &mut self.normals {
            for normal in normals {
                *normal = transform.transform_vector_3d(normal).normalized();
            }
        }
    }
    
    fn transformed(&self, transform: &Transform) -> Box<dyn Drawable3D> {
        let mut clone = self.clone();
        clone.transform(transform);
        Box::new(clone)
    }
} 