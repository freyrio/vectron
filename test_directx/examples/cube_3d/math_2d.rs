// Basic 2D math - Transform2D.

use test_directx::AsBytes;
use crate::math::TransformMatrix; // Import the 3D matrix type

/// Represents a 2D transformation (Scale, Rotation, Translation) using a 3x3 matrix.
/// Stored in column-major order suitable for sending to HLSL where it's often treated row-major.
/// Layout: [col0_row0, col0_row1, col0_row2] [col1_row0, col1_row1, col1_row2] [col2_row0, col2_row1, col2_row2]
///   [ m[0][0] m[1][0] m[2][0] ]
///   [ m[0][1] m[1][1] m[2][1] ]
///   [ m[0][2] m[1][2] m[2][2] ] (Often Homogeneous coordinate = 1)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Transform2D {
    pub matrix: [[f32; 3]; 3],
}

impl Transform2D {
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0], // Column 0
                [0.0, 1.0, 0.0], // Column 1
                [0.0, 0.0, 1.0], // Column 2 (Translation)
            ],
        }
    }

    /// Converts the 3x3 2D transform matrix into a 4x4 matrix suitable for graphics APIs.
    /// Assumes the 2D plane is the XY plane.
    pub fn to_matrix4x4(&self) -> TransformMatrix {
        let m = &self.matrix;
        // Map 3x3 (col-major) to 4x4 (col-major)
        TransformMatrix {
            matrix: [
                // Column 0: [X-scale, X-skew, 0, 0]
                [m[0][0], m[0][1], 0.0, 0.0],
                // Column 1: [Y-skew, Y-scale, 0, 0]
                [m[1][0], m[1][1], 0.0, 0.0],
                // Column 2: [0, 0, 1 (Z-scale), 0]
                [0.0, 0.0, 1.0, 0.0],
                // Column 3: [TranslateX, TranslateY, TranslateZ (0), 1]
                [m[2][0], m[2][1], 0.0, 1.0],
            ]
        }
    }

    // Add methods for translation, rotation, scale later if needed
    // pub fn translate(&self, tx: f32, ty: f32) -> Self { ... }
    // pub fn rotate(&self, angle_radians: f32) -> Self { ... }
    // pub fn scale(&self, sx: f32, sy: f32) -> Self { ... }
    // pub fn multiply(&self, other: &Self) -> Self { ... }
}

impl AsBytes for Transform2D {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
} 