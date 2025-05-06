// Trait for shapes that can be converted into vertex and index data.

use crate::vertex_2d::Vertex2D; // Use the 2D vertex definition

/// Represents the output of tessellation.
pub struct TessellationData {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>, // Using u16 for 2D indices is often sufficient
}

/// Trait for geometric shapes that can be tessellated into triangles.
pub trait Tessellatable {
    /// Generates vertex and index data for the shape.
    /// 
    /// Args:
    ///     tolerance: A parameter that might influence the level of detail 
    ///                (e.g., for curved shapes). Not used for simple shapes like rectangles.
    /// 
    /// Returns:
    ///     A Result containing the tessellated data or an error.
    fn tessellate(&self, tolerance: f32) -> Result<TessellationData, String>;
}