// Defines a simple 2D rectangle shape.

use super::tessellatable::{Tessellatable, TessellationData};
use crate::vertex_2d::Vertex2D;

/// Represents an axis-aligned rectangle.
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4], // Add a color field for the solid rectangle
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) -> Self {
        assert!(width >= 0.0, "Width cannot be negative");
        assert!(height >= 0.0, "Height cannot be negative");
        Self { x, y, width, height, color }
    }
}

impl Tessellatable for Rectangle {
    /// Tessellates the rectangle into two triangles (four vertices, six indices).
    fn tessellate(&self, _tolerance: f32) -> Result<TessellationData, String> {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;
        let c = self.color;

        let vertices = vec![
            Vertex2D::new([x1, y1], c), // Top-left
            Vertex2D::new([x2, y1], c), // Top-right
            Vertex2D::new([x2, y2], c), // Bottom-right
            Vertex2D::new([x1, y2], c), // Bottom-left
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        Ok(TessellationData { vertices, indices })
    }
} 