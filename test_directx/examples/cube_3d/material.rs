//! Material definition for 3D objects.

use test_directx::AsBytes;

/// Enum to define the type of color calculation.
#[repr(i32)] // Ensure this matches HLSL int
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorType {
    Solid = 0,
    LinearGradient = 1,
    // Add other types like RadialGradient later
}

/// Material properties, designed for direct mapping to HLSL constant buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// Type of color calculation (0=Solid, 1=LinearGradient, ...)
    pub color_type: i32,
    /// Padding to align the next float4.
    pub _padding1: [f32; 3],

    // --- Data for Solid Color --- (Used if color_type == 0)
    pub solid_color: [f32; 4], // RGBA

    // --- Data for Linear Gradient --- (Used if color_type == 1)
    pub gradient_color_start: [f32; 4], // RGBA
    pub gradient_color_end: [f32; 4],   // RGBA
    /// Normalized direction vector for the gradient.
    pub gradient_direction: [f32; 3],
    /// Padding to align struct to 16-byte boundary if needed.
    pub _padding2: f32,
    // Add fields for other gradient types or properties here later
}

impl Material {
    /// Creates a new solid color material.
    pub fn new_solid(color: [f32; 4]) -> Self {
        Self {
            color_type: ColorType::Solid as i32,
            solid_color: color,
            ..Default::default() // Initialize other fields to default/zero
        }
    }

    /// Creates a new linear gradient material.
    pub fn new_linear_gradient(
        start_color: [f32; 4],
        end_color: [f32; 4],
        direction: [f32; 3], // Should be normalized
    ) -> Self {
        // Normalize direction just in case
        let len_sq = direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2];
        let norm_direction = if len_sq > f32::EPSILON {
            let inv_len = 1.0 / len_sq.sqrt();
            [direction[0] * inv_len, direction[1] * inv_len, direction[2] * inv_len]
        } else {
            [1.0, 0.0, 0.0] // Default to X direction if input is zero
        };

        Self {
            color_type: ColorType::LinearGradient as i32,
            gradient_color_start: start_color,
            gradient_color_end: end_color,
            gradient_direction: norm_direction,
            ..Default::default()
        }
    }
}

impl Default for Material {
    /// Default material (white solid color).
    fn default() -> Self {
        Self {
            color_type: ColorType::Solid as i32,
            _padding1: [0.0; 3],
            solid_color: [1.0, 1.0, 1.0, 1.0],
            gradient_color_start: [0.0; 4],
            gradient_color_end: [0.0; 4],
            gradient_direction: [0.0; 3],
            _padding2: 0.0,
        }
    }
}

impl AsBytes for Material {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
} 