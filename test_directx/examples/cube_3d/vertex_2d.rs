// Basic 2D vertex definition.

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub color: [f32; 4], // Add color directly for simplicity now
}

impl Vertex2D {
    pub fn new(pos: [f32; 2], col: [f32; 4]) -> Self {
        Self { position: pos, color: col }
    }
}

// We need AsBytes to send vertices to the GPU buffer
use test_directx::AsBytes;
impl AsBytes for Vertex2D {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
} 