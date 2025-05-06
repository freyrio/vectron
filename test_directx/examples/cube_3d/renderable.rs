//! Defines the Renderable trait and a basic Object3D implementing it.

use crate::math::TransformMatrix;
use crate::material::Material; // Import Material from the new module
use test_directx::types::{BufferId, IndexFormat};
use test_directx::AsBytes; // Add AsBytes import

/// Trait for objects that can be rendered.
pub trait Renderable {
    /// Gets the object's current model-to-world transformation matrix.
    fn get_model_matrix(&self) -> TransformMatrix;

    /// Gets the ID of the vertex buffer resource.
    fn get_vertex_buffer(&self) -> BufferId;

    /// Gets the ID of the index buffer resource.
    fn get_index_buffer(&self) -> BufferId;

    /// Gets the number of indices to draw.
    fn get_index_count(&self) -> u32;

    /// Gets the format of the indices (e.g., Uint16, Uint32).
    fn get_index_format(&self) -> IndexFormat;

    /// Gets the material properties for the object.
    fn get_material(&self) -> &Material;
}

/// Represents a basic 3D object in the scene.
pub struct Object3D {
    pub model_matrix: TransformMatrix,
    pub vertex_buffer: BufferId,
    pub index_buffer: BufferId,
    pub index_count: u32,
    pub index_format: IndexFormat,
    pub material: Material, // Add material field
}

impl Object3D {
    /// Creates a new Object3D.
    pub fn new(
        vertex_buffer: BufferId,
        index_buffer: BufferId,
        index_count: u32,
        index_format: IndexFormat,
        material: Material, // Add material parameter
    ) -> Self {
        Self {
            model_matrix: TransformMatrix::identity(), // Start at origin, no rotation/scale
            vertex_buffer,
            index_buffer,
            index_count,
            index_format,
            material, // Store material
        }
    }
}

impl Renderable for Object3D {
    fn get_model_matrix(&self) -> TransformMatrix {
        self.model_matrix // Simply return the stored matrix
    }

    fn get_vertex_buffer(&self) -> BufferId {
        self.vertex_buffer
    }

    fn get_index_buffer(&self) -> BufferId {
        self.index_buffer
    }

    fn get_index_count(&self) -> u32 {
        self.index_count
    }

    fn get_index_format(&self) -> IndexFormat {
        self.index_format
    }

    fn get_material(&self) -> &Material {
        &self.material // Return a reference to the stored material
    }
} 