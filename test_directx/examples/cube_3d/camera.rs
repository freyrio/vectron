//! Camera struct for managing view and projection matrices.

use crate::math::{TransformMatrix, Vec3}; // Use items from our math module

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov_y_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,

    // Cached matrices
    view_matrix: TransformMatrix,
    projection_matrix: TransformMatrix,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let mut camera = Self {
            position,
            target,
            up,
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
            view_matrix: TransformMatrix::identity(), // Initialize placeholder
            projection_matrix: TransformMatrix::identity(), // Initialize placeholder
        };
        camera.update_view_matrix();
        camera.update_projection_matrix();
        camera
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = TransformMatrix::look_at_lh(self.position, self.target, self.up);
    }

    fn update_projection_matrix(&mut self) {
        self.projection_matrix = TransformMatrix::perspective_lh(
            self.fov_y_radians,
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update_projection_matrix();
    }

    // --- Getters ---
    pub fn get_view_matrix(&self) -> TransformMatrix {
        self.view_matrix // Return cached copy
    }

    pub fn get_projection_matrix(&self) -> TransformMatrix {
        self.projection_matrix // Return cached copy
    }

    // --- Potential future methods ---
    // pub fn set_position(&mut self, position: Vec3) {
    //     self.position = position;
    //     self.update_view_matrix();
    // }
    //
    // pub fn set_target(&mut self, target: Vec3) {
    //     self.target = target;
    //     self.update_view_matrix();
    // }
}
