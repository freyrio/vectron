/*!
 * Unified 2D/3D transformation types and operations
 */
use crate::core::math::{
    Vec2, Vec3, Vec4, Mat3, Mat4, Quaternion, Rect, BoundingBox, deg_to_rad
};

/// Represents a 2D or 3D transformation
#[derive(Clone, Debug, PartialEq)]
pub enum Transform {
    /// 2D transformation using a 3x3 matrix
    Mat3(Mat3),
    /// 3D transformation using a 4x4 matrix
    Mat4(Mat4),
}

impl Transform {
    /// Create identity transform (no transformation)
    #[inline]
    pub fn identity() -> Self {
        Transform::Mat4(Mat4::identity())
    }
    
    /// Create 2D identity transform
    #[inline]
    pub fn identity_2d() -> Self {
        Transform::Mat3(Mat3::identity())
    }
    
    /// Create 3D identity transform
    #[inline]
    pub fn identity_3d() -> Self {
        Transform::Mat4(Mat4::identity())
    }
    
    /// Create a translation transform
    #[inline]
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Transform::Mat4(Mat4::translation(x, y, z))
    }
    
    /// Create a 2D translation transform
    #[inline]
    pub fn translation_2d(x: f32, y: f32) -> Self {
        Transform::Mat3(Mat3::translation(x, y))
    }
    
    /// Create a 3D translation transform
    #[inline]
    pub fn translation_3d(x: f32, y: f32, z: f32) -> Self {
        Transform::Mat4(Mat4::translation(x, y, z))
    }
    
    /// Create a translation transform from a Vec2
    #[inline]
    pub fn from_translation_2d(translation: Vec2) -> Self {
        Transform::Mat3(Mat3::translation(translation.x, translation.y))
    }
    
    /// Create a translation transform from a Vec3
    #[inline]
    pub fn from_translation_3d(translation: Vec3) -> Self {
        Transform::Mat4(Mat4::translation(translation.x, translation.y, translation.z))
    }
    
    /// Create a scaling transform
    #[inline]
    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Transform::Mat4(Mat4::scaling(x, y, z))
    }
    
    /// Create a 2D scaling transform
    #[inline]
    pub fn scaling_2d(x: f32, y: f32) -> Self {
        Transform::Mat3(Mat3::scaling(x, y))
    }
    
    /// Create a 3D scaling transform
    #[inline]
    pub fn scaling_3d(x: f32, y: f32, z: f32) -> Self {
        Transform::Mat4(Mat4::scaling(x, y, z))
    }
    
    /// Create a uniform scaling transform
    #[inline]
    pub fn uniform_scaling(scale: f32) -> Self {
        Transform::Mat4(Mat4::scaling(scale, scale, scale))
    }
    
    /// Create a 2D uniform scaling transform
    #[inline]
    pub fn uniform_scaling_2d(scale: f32) -> Self {
        Transform::Mat3(Mat3::scaling(scale, scale))
    }
    
    /// Create a 3D uniform scaling transform
    #[inline]
    pub fn uniform_scaling_3d(scale: f32) -> Self {
        Transform::Mat4(Mat4::scaling(scale, scale, scale))
    }
    
    /// Create a 2D rotation transform (around z-axis)
    #[inline]
    pub fn rotation_2d(angle_degrees: f32) -> Self {
        Transform::Mat3(Mat3::rotation(deg_to_rad(angle_degrees)))
    }
    
    /// Create a 3D rotation transform around the X axis
    #[inline]
    pub fn rotation_x(angle_degrees: f32) -> Self {
        Transform::Mat4(Mat4::rotation_x(deg_to_rad(angle_degrees)))
    }
    
    /// Create a 3D rotation transform around the Y axis
    #[inline]
    pub fn rotation_y(angle_degrees: f32) -> Self {
        Transform::Mat4(Mat4::rotation_y(deg_to_rad(angle_degrees)))
    }
    
    /// Create a 3D rotation transform around the Z axis
    #[inline]
    pub fn rotation_z(angle_degrees: f32) -> Self {
        Transform::Mat4(Mat4::rotation_z(deg_to_rad(angle_degrees)))
    }
    
    /// Create a 3D rotation transform from Euler angles (degrees)
    #[inline]
    pub fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Self {
        let quat = Quaternion::from_euler(
            deg_to_rad(yaw), 
            deg_to_rad(pitch), 
            deg_to_rad(roll)
        );
        Transform::Mat4(Mat4::from_quaternion(&quat))
    }
    
    /// Create a 3D rotation transform from a quaternion
    #[inline]
    pub fn from_quaternion(quaternion: &Quaternion) -> Self {
        Transform::Mat4(Mat4::from_quaternion(quaternion))
    }
    
    /// Create a 3D rotation transform from an axis and angle (in degrees)
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle_degrees: f32) -> Self {
        let quat = Quaternion::from_axis_angle(axis, deg_to_rad(angle_degrees));
        Transform::Mat4(Mat4::from_quaternion(&quat))
    }
    
    /// Create a look-at view transform
    #[inline]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        Transform::Mat4(Mat4::look_at(eye, target, up))
    }
    
    /// Create a perspective projection transform
    #[inline]
    pub fn perspective(fov_y_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Transform::Mat4(Mat4::perspective(deg_to_rad(fov_y_degrees), aspect_ratio, near, far))
    }
    
    /// Create an orthographic projection transform
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Transform::Mat4(Mat4::orthographic(left, right, bottom, top, near, far))
    }
    
    /// Convert to a Mat3 if possible, or flatten a Mat4 to a Mat3
    #[inline]
    pub fn to_mat3(&self) -> Mat3 {
        match self {
            Transform::Mat3(m) => *m,
            Transform::Mat4(m) => Mat3::new(
                m.get(0, 0), m.get(0, 1), m.get(0, 3),
                m.get(1, 0), m.get(1, 1), m.get(1, 3),
                m.get(3, 0), m.get(3, 1), m.get(3, 3),
            ),
        }
    }
    
    /// Convert to a Mat4
    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        match self {
            Transform::Mat3(m) => Mat4::new(
                m.get(0, 0), m.get(0, 1), 0.0, m.get(0, 2),
                m.get(1, 0), m.get(1, 1), 0.0, m.get(1, 2),
                0.0, 0.0, 1.0, 0.0,
                m.get(2, 0), m.get(2, 1), 0.0, m.get(2, 2),
            ),
            Transform::Mat4(m) => *m,
        }
    }
    
    /// Transform a 2D point
    #[inline]
    pub fn transform_point_2d(&self, point: &Vec2) -> Vec2 {
        match self {
            Transform::Mat3(m) => m.transform_point(point),
            Transform::Mat4(m) => {
                let p = m.transform_point(&Vec3::new(point.x, point.y, 0.0));
                Vec2::new(p.x, p.y)
            }
        }
    }
    
    /// Transform a 3D point
    #[inline]
    pub fn transform_point_3d(&self, point: &Vec3) -> Vec3 {
        match self {
            Transform::Mat3(m) => {
                let p = m.transform_point(&Vec2::new(point.x, point.y));
                Vec3::new(p.x, p.y, point.z)
            },
            Transform::Mat4(m) => m.transform_point(point),
        }
    }
    
    /// Transform a 2D vector (direction)
    #[inline]
    pub fn transform_vector_2d(&self, vector: &Vec2) -> Vec2 {
        match self {
            Transform::Mat3(m) => m.transform_vector(vector),
            Transform::Mat4(m) => {
                let v = m.transform_vector(&Vec3::new(vector.x, vector.y, 0.0));
                Vec2::new(v.x, v.y)
            }
        }
    }
    
    /// Transform a 3D vector (direction)
    #[inline]
    pub fn transform_vector_3d(&self, vector: &Vec3) -> Vec3 {
        match self {
            Transform::Mat3(m) => {
                let v = m.transform_vector(&Vec2::new(vector.x, vector.y));
                Vec3::new(v.x, v.y, vector.z)
            },
            Transform::Mat4(m) => m.transform_vector(vector),
        }
    }
    
    /// Transform a 2D rectangle
    #[inline]
    pub fn transform_rect(&self, rect: &Rect) -> Rect {
        let corners = rect.corners();
        let transformed_corners: [Vec2; 4] = [
            self.transform_point_2d(&corners[0]),
            self.transform_point_2d(&corners[1]),
            self.transform_point_2d(&corners[2]),
            self.transform_point_2d(&corners[3]),
        ];
        
        let mut min_x = transformed_corners[0].x;
        let mut min_y = transformed_corners[0].y;
        let mut max_x = transformed_corners[0].x;
        let mut max_y = transformed_corners[0].y;
        
        for i in 1..4 {
            min_x = min_x.min(transformed_corners[i].x);
            min_y = min_y.min(transformed_corners[i].y);
            max_x = max_x.max(transformed_corners[i].x);
            max_y = max_y.max(transformed_corners[i].y);
        }
        
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
    
    /// Transform a 3D bounding box
    #[inline]
    pub fn transform_bounding_box(&self, bbox: &BoundingBox) -> BoundingBox {
        let corners = bbox.corners();
        let mut min = self.transform_point_3d(&corners[0]);
        let mut max = min;
        
        for i in 1..8 {
            let transformed = self.transform_point_3d(&corners[i]);
            
            min.x = min.x.min(transformed.x);
            min.y = min.y.min(transformed.y);
            min.z = min.z.min(transformed.z);
            
            max.x = max.x.max(transformed.x);
            max.y = max.y.max(transformed.y);
            max.z = max.z.max(transformed.z);
        }
        
        BoundingBox::new(min, max)
    }
    
    /// Get the inverse of this transform
    #[inline]
    pub fn inverse(&self) -> Option<Transform> {
        match self {
            Transform::Mat3(m) => {
                m.inverse().map(Transform::Mat3)
            },
            Transform::Mat4(m) => {
                m.inverse().map(Transform::Mat4)
            },
        }
    }
    
    /// Combine this transform with another (this * other)
    #[inline]
    pub fn combine(&self, other: &Transform) -> Transform {
        match (self, other) {
            (Transform::Mat3(a), Transform::Mat3(b)) => Transform::Mat3(*a * *b),
            (Transform::Mat4(a), Transform::Mat4(b)) => Transform::Mat4(*a * *b),
            (Transform::Mat3(a), Transform::Mat4(b)) => Transform::Mat4(self.to_mat4() * *b),
            (Transform::Mat4(a), Transform::Mat3(b)) => Transform::Mat4(*a * other.to_mat4()),
        }
    }
    
    /// Returns true if this is a 2D transform
    #[inline]
    pub fn is_2d(&self) -> bool {
        matches!(self, Transform::Mat3(_))
    }
    
    /// Returns true if this is a 3D transform
    #[inline]
    pub fn is_3d(&self) -> bool {
        matches!(self, Transform::Mat4(_))
    }
    
    /// Extract the translation component from this transform
    #[inline]
    pub fn get_translation(&self) -> Vec3 {
        match self {
            Transform::Mat3(m) => {
                let translation_2d = m.get_translation();
                Vec3::new(translation_2d.x, translation_2d.y, 0.0)
            },
            Transform::Mat4(m) => m.get_translation(),
        }
    }
    
    /// Extract the 2D translation component from this transform
    #[inline]
    pub fn get_translation_2d(&self) -> Vec2 {
        match self {
            Transform::Mat3(m) => m.get_translation(),
            Transform::Mat4(m) => {
                let translation = m.get_translation();
                Vec2::new(translation.x, translation.y)
            },
        }
    }
    
    /// Extract the rotation as a quaternion (3D only)
    #[inline]
    pub fn get_rotation(&self) -> Option<Quaternion> {
        match self {
            Transform::Mat3(_) => None, // Cannot extract quaternion from 2D transform
            Transform::Mat4(m) => Some(m.to_quaternion()),
        }
    }
    
    /// Extract the 2D rotation angle in radians
    #[inline]
    pub fn get_rotation_angle_2d(&self) -> f32 {
        match self {
            Transform::Mat3(m) => m.get_rotation(),
            Transform::Mat4(m) => {
                // Extract 2D rotation from 3D matrix
                let x_axis = Vec2::new(m.get(0, 0), m.get(1, 0)).normalized();
                f32::atan2(x_axis.y, x_axis.x)
            },
        }
    }
    
    /// Extract the scale component
    #[inline]
    pub fn get_scale(&self) -> Vec3 {
        match self {
            Transform::Mat3(m) => {
                let scale_2d = m.get_scale();
                Vec3::new(scale_2d.x, scale_2d.y, 1.0)
            },
            Transform::Mat4(m) => m.get_scale(),
        }
    }
    
    /// Extract the 2D scale component
    #[inline]
    pub fn get_scale_2d(&self) -> Vec2 {
        match self {
            Transform::Mat3(m) => m.get_scale(),
            Transform::Mat4(m) => {
                let scale = m.get_scale();
                Vec2::new(scale.x, scale.y)
            },
        }
    }
    
    /// Decompose the transform into translation, rotation, and scale
    #[inline]
    pub fn decompose(&self) -> (Vec3, Option<Quaternion>, Vec3) {
        let translation = self.get_translation();
        let rotation = self.get_rotation();
        let scale = self.get_scale();
        
        (translation, rotation, scale)
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Transform::identity()
    }
}

/// A transform stack for hierarchical transformations
#[derive(Clone, Debug, Default)]
pub struct TransformStack {
    /// The stack of transforms
    stack: Vec<Transform>,
}

impl TransformStack {
    /// Create a new transform stack with an identity transform
    #[inline]
    pub fn new() -> Self {
        Self {
            stack: vec![Transform::identity()],
        }
    }
    
    /// Create a new 2D transform stack with an identity transform
    #[inline]
    pub fn new_2d() -> Self {
        Self {
            stack: vec![Transform::identity_2d()],
        }
    }
    
    /// Create a new 3D transform stack with an identity transform
    #[inline]
    pub fn new_3d() -> Self {
        Self {
            stack: vec![Transform::identity_3d()],
        }
    }
    
    /// Reset the stack to a single identity transform
    #[inline]
    pub fn reset(&mut self) {
        self.stack.clear();
        self.stack.push(Transform::identity());
    }
    
    /// Reset the stack to a single 2D identity transform
    #[inline]
    pub fn reset_2d(&mut self) {
        self.stack.clear();
        self.stack.push(Transform::identity_2d());
    }
    
    /// Reset the stack to a single 3D identity transform
    #[inline]
    pub fn reset_3d(&mut self) {
        self.stack.clear();
        self.stack.push(Transform::identity_3d());
    }
    
    /// Push a transform onto the stack, combining it with the current transform
    #[inline]
    pub fn push(&mut self, transform: Transform) {
        let current = self.current();
        let combined = current.combine(&transform);
        self.stack.push(combined);
    }
    
    /// Pop a transform from the stack
    #[inline]
    pub fn pop(&mut self) -> Option<Transform> {
        if self.stack.len() <= 1 {
            return None;
        }
        
        self.stack.pop()
    }
    
    /// Get the current transform (top of the stack)
    #[inline]
    pub fn current(&self) -> Transform {
        self.stack.last().cloned().unwrap_or_else(Transform::identity)
    }
    
    /// Get the current inverse transform
    #[inline]
    pub fn current_inverse(&self) -> Option<Transform> {
        self.current().inverse()
    }
    
    /// Apply a translation
    #[inline]
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.push(Transform::translation(x, y, z));
    }
    
    /// Apply a 2D translation
    #[inline]
    pub fn translate_2d(&mut self, x: f32, y: f32) {
        self.push(Transform::translation_2d(x, y));
    }
    
    /// Apply a 3D translation
    #[inline]
    pub fn translate_3d(&mut self, x: f32, y: f32, z: f32) {
        self.push(Transform::translation_3d(x, y, z));
    }
    
    /// Apply a scaling
    #[inline]
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.push(Transform::scaling(x, y, z));
    }
    
    /// Apply a 2D scaling
    #[inline]
    pub fn scale_2d(&mut self, x: f32, y: f32) {
        self.push(Transform::scaling_2d(x, y));
    }
    
    /// Apply a 3D scaling
    #[inline]
    pub fn scale_3d(&mut self, x: f32, y: f32, z: f32) {
        self.push(Transform::scaling_3d(x, y, z));
    }
    
    /// Apply a uniform scaling
    #[inline]
    pub fn scale_uniform(&mut self, scale: f32) {
        self.push(Transform::uniform_scaling(scale));
    }
    
    /// Apply a 2D rotation (around Z axis)
    #[inline]
    pub fn rotate_2d(&mut self, angle_degrees: f32) {
        self.push(Transform::rotation_2d(angle_degrees));
    }
    
    /// Apply a rotation around the X axis
    #[inline]
    pub fn rotate_x(&mut self, angle_degrees: f32) {
        self.push(Transform::rotation_x(angle_degrees));
    }
    
    /// Apply a rotation around the Y axis
    #[inline]
    pub fn rotate_y(&mut self, angle_degrees: f32) {
        self.push(Transform::rotation_y(angle_degrees));
    }
    
    /// Apply a rotation around the Z axis
    #[inline]
    pub fn rotate_z(&mut self, angle_degrees: f32) {
        self.push(Transform::rotation_z(angle_degrees));
    }
    
    /// Apply a rotation from Euler angles
    #[inline]
    pub fn rotate_euler(&mut self, yaw: f32, pitch: f32, roll: f32) {
        self.push(Transform::from_euler(yaw, pitch, roll));
    }
    
    /// Apply a rotation from a quaternion
    #[inline]
    pub fn rotate_quaternion(&mut self, quaternion: &Quaternion) {
        self.push(Transform::from_quaternion(quaternion));
    }
    
    /// Transform a 2D point using the current transform
    #[inline]
    pub fn transform_point_2d(&self, point: &Vec2) -> Vec2 {
        self.current().transform_point_2d(point)
    }
    
    /// Transform a 3D point using the current transform
    #[inline]
    pub fn transform_point_3d(&self, point: &Vec3) -> Vec3 {
        self.current().transform_point_3d(point)
    }
    
    /// Transform a 2D vector using the current transform
    #[inline]
    pub fn transform_vector_2d(&self, vector: &Vec2) -> Vec2 {
        self.current().transform_vector_2d(vector)
    }
    
    /// Transform a 3D vector using the current transform
    #[inline]
    pub fn transform_vector_3d(&self, vector: &Vec3) -> Vec3 {
        self.current().transform_vector_3d(vector)
    }
    
    /// Transform a rectangle using the current transform
    #[inline]
    pub fn transform_rect(&self, rect: &Rect) -> Rect {
        self.current().transform_rect(rect)
    }
    
    /// Transform a bounding box using the current transform
    #[inline]
    pub fn transform_bounding_box(&self, bbox: &BoundingBox) -> BoundingBox {
        self.current().transform_bounding_box(bbox)
    }
} 