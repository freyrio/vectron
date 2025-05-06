//! Math utilities: TransformMatrix, Vec3, and helpers.

// We need access to the AsBytes trait defined in the parent (main.rs or lib.rs)
// This might need adjustment depending on your project structure.
// If AsBytes is in test_directx/src/lib.rs, you might use:
// use test_directx::AsBytes;
// If it's defined directly in main.rs (less ideal), this is tricky.
// Let's assume it's accessible via the crate root for now.
use test_directx::AsBytes;


// Reintroduce TransformMatrix struct and AsBytes impl
#[repr(C)]
#[derive(Clone, Copy, Debug)] // Add Clone, Copy, Debug for convenience
pub struct TransformMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl AsBytes for TransformMatrix {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

// Define Vec3 type alias outside the impl block
pub type Vec3 = [f32; 3];

// --- Matrix Methods ---
impl TransformMatrix {
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Standard matrix multiplication (row-major A * B)
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 { // row of result
            for j in 0..4 { // column of result
                for k in 0..4 { // terms
                    result[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Self { matrix: result }
    }

    // Create a left-handed perspective projection matrix (DirectX style)
    pub fn perspective_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
         if aspect_ratio <= 0.0 || z_near <= 0.0 || z_far <= z_near || fov_y_radians <= 0.0 {
            // Return identity or panic for invalid inputs
            // panic!("Invalid arguments for perspective_lh");
             return Self::identity();
         }
        let mut m = [[0.0; 4]; 4];
        let f = 1.0 / (fov_y_radians / 2.0).tan();
         if aspect_ratio.abs() < f32::EPSILON {
            // Avoid division by zero
             return Self::identity();
        }
        // Recreate DirectX standard LH projection matrix (row-major)
        m[0][0] = f / aspect_ratio;
        m[1][1] = f;
        // Near is mapped to 0, far is mapped to 1 in DirectX-style Z-mapping
        m[2][2] = z_far / (z_far - z_near);
        m[3][2] = -(z_near * z_far) / (z_far - z_near);
        m[2][3] = 1.0; 
        // m[3][3] = 0.0; // Already 0 from initialization
        Self { matrix: m }
    }

    // Create a left-handed view matrix (look-at)
    pub fn look_at_lh(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let z_axis = normalize([target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]]);
        // Check if up and z_axis are parallel
         if cross_product(up, z_axis).iter().all(|&x| x.abs() < f32::EPSILON) {
            // Handle parallel vectors, perhaps use a default 'right' vector if up is parallel to view direction
            // For simplicity, returning identity might be acceptable for debugging
             return Self::identity(); // Or panic
         }
        let x_axis = normalize(cross_product(up, z_axis));
        let y_axis = cross_product(z_axis, x_axis); // Should be normalized due to x/z being ortho-normal

        let mut m = Self::identity().matrix;
        // Construct the rotation part - row-major DirectX convention
        // Row 0 - Right vector (x_axis)
        m[0][0] = x_axis[0]; 
        m[0][1] = x_axis[1]; 
        m[0][2] = x_axis[2];
        
        // Row 1 - Up vector (y_axis)
        m[1][0] = y_axis[0]; 
        m[1][1] = y_axis[1]; 
        m[1][2] = y_axis[2];
        
        // Row 2 - Forward vector (z_axis)
        m[2][0] = z_axis[0]; 
        m[2][1] = z_axis[1]; 
        m[2][2] = z_axis[2];

        // Construct the translation part (dot products with negated eye position)
        m[3][0] = -dot_product(x_axis, eye);
        m[3][1] = -dot_product(y_axis, eye);
        m[3][2] = -dot_product(z_axis, eye);
        // m[3][3] = 1.0; // Already 1.0 from identity

        Self { matrix: m }
    }

    // Create a rotation matrix around the Y axis
    pub fn rotation_y(angle_radians: f32) -> Self {
        let mut m = Self::identity().matrix;
        let (sin, cos) = angle_radians.sin_cos();
        m[0][0] = cos;  m[0][2] = -sin; // Correct rotation
        m[2][0] = sin;  m[2][2] = cos;
        // m[1][1] = 1.0; // Already 1.0 from identity
        // m[3][3] = 1.0; // Already 1.0 from identity
        Self { matrix: m }
    }

    // Transpose the matrix
    pub fn transpose(&self) -> Self {
        let mut m = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = self.matrix[j][i];
            }
        }
        Self { matrix: m }
    }

    /// Print the matrix in readable format for debugging
    pub fn print(&self) {
        // Remove the previous println! calls
        // Method remains, but does nothing now unless reimplemented with logging
    }
}

// --- Helper functions for look_at_lh ---
fn cross_product(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot_product(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(v: Vec3) -> Vec3 {
    let len_sq = dot_product(v, v);
    if len_sq > f32::EPSILON {
        let len_inv = 1.0 / len_sq.sqrt();
        [v[0] * len_inv, v[1] * len_inv, v[2] * len_inv]
    } else {
        [0.0, 0.0, 0.0] // Return zero vector if length is near zero
    }
}
