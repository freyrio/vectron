// Trait for basic 2D drawable objects in the example.

use test_directx::types::{BufferId, IndexFormat};
use test_directx::Dx11Backend; // Need backend to create buffers
use test_directx::resources::BufferDesc; // For buffer creation
use test_directx::resources::BufferUsage; // For buffer creation
use test_directx::AsBytes; // For sending data to buffers

use crate::math_2d::Transform2D; // Uncommented
use crate::shapes::Tessellatable; // To accept tessellatable shapes

pub trait Drawable2D {
    fn get_vertex_buffer(&self) -> BufferId;
    fn get_index_buffer(&self) -> BufferId;
    fn get_index_count(&self) -> u32;
    fn get_index_format(&self) -> IndexFormat;
    fn get_transform(&self) -> Transform2D; // Uncommented
    fn set_transform(&mut self, transform: Transform2D); // Add a setter for transforms
    // Add methods for color/texture later
}

/// Concrete drawable 2D shape holding GPU resources.
pub struct DrawableShape2D {
    vertex_buffer: BufferId,
    index_buffer: BufferId,
    index_count: u32,
    index_format: IndexFormat,
    pub transform: Transform2D, // Make transform public or add getters/setters
}

impl DrawableShape2D {
    /// Creates a new drawable shape by tessellating the input shape
    /// and creating GPU buffers.
    pub fn new<S: Tessellatable + ?Sized>(
        shape: &S,
        app: &mut Dx11Backend, // Pass mutable reference to backend
    ) -> Result<Self, Box<dyn std::error::Error>> {
        
        // 1. Tessellate the shape
        let tess_data = shape.tessellate(0.0)?; // Use default tolerance
        
        // 2. Create Vertex Buffer
        let vb_desc = BufferDesc {
            size: std::mem::size_of_val(&tess_data.vertices[..]),
            stride: std::mem::size_of::<crate::vertex_2d::Vertex2D>(),
            usage: BufferUsage::Vertex,
            dynamic: false, // Assume static geometry for now
        };
        let vertex_buffer = app.create_buffer(&vb_desc, Some(tess_data.vertices.as_bytes()))?;

        // 3. Create Index Buffer
        let ib_desc = BufferDesc {
            size: std::mem::size_of_val(&tess_data.indices[..]),
            stride: 0, 
            usage: BufferUsage::Index,
            dynamic: false,
        };
        let index_buffer = app.create_buffer(&ib_desc, Some(tess_data.indices.as_bytes()))?;

        Ok(Self {
            vertex_buffer,
            index_buffer,
            index_count: tess_data.indices.len() as u32,
            index_format: IndexFormat::Uint16, // Assuming u16 based on TessellationData
            transform: Transform2D::identity(), // Default transform
        })
    }
}

impl Drawable2D for DrawableShape2D {
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

    fn get_transform(&self) -> Transform2D {
        self.transform
    }
    
    fn set_transform(&mut self, transform: Transform2D) {
        self.transform = transform;
    }
} 