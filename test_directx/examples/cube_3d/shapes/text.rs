// Defines a simple text shape.

use super::tessellatable::{Tessellatable, TessellationData};
use crate::vertex_2d::Vertex2D;
// Add rusttype imports
use rusttype::{point, Font, Scale, PositionedGlyph};

// Embed the font data (adjust the path if needed)
const FONT_DATA: &[u8] = include_bytes!("../shaders/DejaVuSans.ttf"); // Assumes font is in shaders/

/// Represents a simple text string to be rendered.
pub struct Text {
    pub content: String,
    pub x: f32,
    pub y: f32,
    pub size: f32, // Font size in pixels
    pub color: [f32; 4],
}

impl Text {
    pub fn new(content: String, x: f32, y: f32, size: f32, color: [f32; 4]) -> Self {
        Self { content, x, y, size, color }
    }
}

impl Tessellatable for Text {
    /// Tessellates the text into a series of quads using rusttype.
    fn tessellate(&self, _tolerance: f32) -> Result<TessellationData, String> {
        // Load the font
        let font = Font::try_from_bytes(FONT_DATA)
            .ok_or_else(|| "Failed to load font data".to_string())?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let scale = Scale::uniform(self.size);
        let start_point = point(self.x, self.y); // Starting point for the text baseline

        // Layout the glyphs
        let glyphs: Vec<PositionedGlyph> = font
            .layout(&self.content, scale, start_point)
            .collect();

        let mut base_index: u16 = 0;

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                // Calculate texture coordinates (not used here, but useful for textured fonts)
                // let uv_rect = glyph.unpositioned().h_metrics(); // Example, needs texture atlas

                // Calculate screen coordinates for the quad
                let x1 = bb.min.x as f32;
                let y1 = bb.min.y as f32; // Rusttype Y grows downwards, screen Y often too
                let x2 = bb.max.x as f32;
                let y2 = bb.max.y as f32;
                let c = self.color;

                // Add vertices for the quad (adjust winding order if needed)
                // Top-left, Top-right, Bottom-right, Bottom-left
                vertices.push(Vertex2D::new([x1, y1], c));
                vertices.push(Vertex2D::new([x2, y1], c));
                vertices.push(Vertex2D::new([x2, y2], c));
                vertices.push(Vertex2D::new([x1, y2], c));

                // Add indices for the two triangles of the quad
                indices.push(base_index + 0);
                indices.push(base_index + 1);
                indices.push(base_index + 2);
                indices.push(base_index + 0);
                indices.push(base_index + 2);
                indices.push(base_index + 3);

                base_index += 4;
            }
        }

        if vertices.is_empty() {
            // Handle empty string or no visible glyphs
            return Ok(TessellationData { vertices: vec![], indices: vec![] });
        }

        Ok(TessellationData { vertices, indices })
    }
} 