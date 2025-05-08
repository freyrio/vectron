use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::fmt;
use crate::math::vector::Vec3;
use crate::math::matrix::Mat4;
use crate::math::utils::constants::*;

/// A quaternion for representing 3D rotations.
/// 
/// Stored as [x, y, z, w] where (x, y, z) is the vector (imaginary) part
/// and w is the scalar (real) part.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    /// Creates a new quaternion with the given components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    /// Creates a quaternion from a vector part (imaginary) and scalar part (real).
    #[inline]
    pub fn from_parts(vector: Vec3, scalar: f32) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: scalar,
        }
    }
    
    /// Creates a new identity quaternion representing no rotation.
    #[inline]
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
    
    /// Creates a quaternion from an axis-angle rotation.
    /// The axis should be normalized.
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self {
            x: axis.x * sin,
            y: axis.y * sin,
            z: axis.z * sin,
            w: cos,
        }
    }
    
    /// Creates a quaternion from Euler angles in radians (roll, pitch, yaw).
    /// The rotation order is: roll (z), then pitch (x), then yaw (y).
    #[inline]
    pub fn from_euler(roll: f32, pitch: f32, yaw: f32) -> Self {
        // Roll (Z)
        let (sr, cr) = (roll * 0.5).sin_cos();
        // Pitch (X)
        let (sp, cp) = (pitch * 0.5).sin_cos();
        // Yaw (Y)
        let (sy, cy) = (yaw * 0.5).sin_cos();
        
        Self {
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
            w: cr * cp * cy + sr * sp * sy,
        }
    }
    
    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    pub fn from_rotation_matrix(m: &[f32; 9]) -> Self {
        // Compute trace and discriminant
        let trace = m[0] + m[4] + m[8];
        
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Self {
                x: (m[5] - m[7]) * s,
                y: (m[6] - m[2]) * s,
                z: (m[1] - m[3]) * s,
                w: 0.25 / s,
            }
        } else if m[0] > m[4] && m[0] > m[8] {
            let s = 2.0 * (1.0 + m[0] - m[4] - m[8]).sqrt();
            Self {
                x: 0.25 * s,
                y: (m[1] + m[3]) / s,
                z: (m[6] + m[2]) / s,
                w: (m[5] - m[7]) / s,
            }
        } else if m[4] > m[8] {
            let s = 2.0 * (1.0 + m[4] - m[0] - m[8]).sqrt();
            Self {
                x: (m[1] + m[3]) / s,
                y: 0.25 * s,
                z: (m[5] + m[7]) / s,
                w: (m[6] - m[2]) / s,
            }
        } else {
            let s = 2.0 * (1.0 + m[8] - m[0] - m[4]).sqrt();
            Self {
                x: (m[6] + m[2]) / s,
                y: (m[5] + m[7]) / s,
                z: 0.25 * s,
                w: (m[1] - m[3]) / s,
            }
        }
    }
    
    /// Returns the vector part (imaginary) of the quaternion.
    #[inline]
    pub fn vector_part(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    
    /// Returns the scalar part (real) of the quaternion.
    #[inline]
    pub fn scalar_part(&self) -> f32 {
        self.w
    }
    
    /// Returns the squared length of the quaternion.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    
    /// Returns the length of the quaternion.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    /// Returns a normalized copy of the quaternion.
    #[inline]
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > EPSILON {
            *self / length
        } else {
            Self::identity()
        }
    }
    
    /// Normalizes this quaternion in-place.
    #[inline]
    pub fn normalize_mut(&mut self) {
        let length = self.length();
        if length > EPSILON {
            *self /= length;
        } else {
            *self = Self::identity();
        }
    }
    
    /// Returns the conjugate of this quaternion.
    #[inline]
    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    
    /// Returns the inverse of this quaternion.
    #[inline]
    pub fn inverse(&self) -> Self {
        let length_sq = self.length_squared();
        if length_sq > EPSILON {
            self.conjugate() / length_sq
        } else {
            Self::identity()
        }
    }
    
    /// Returns the dot product of this quaternion and another.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    
    /// Rotates a vector by this quaternion.
    #[inline]
    pub fn rotate_vector(&self, vector: Vec3) -> Vec3 {
        // Convert vector to pure quaternion (w=0)
        let v = Quat::new(vector.x, vector.y, vector.z, 0.0);
        
        // Apply rotation: q * v * q^-1
        let result = *self * v * self.inverse();
        
        // Extract vector part
        Vec3::new(result.x, result.y, result.z)
    }
    
    /// Converts this quaternion to a 4x4 rotation matrix.
    #[inline]
    pub fn to_matrix4(&self) -> Mat4 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;
        
        Mat4::new(
            1.0 - 2.0 * (y2 + z2), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
            2.0 * (xy - wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz + wx), 0.0,
            2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (x2 + y2), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    /// Returns the angle in radians between this quaternion and another.
    #[inline]
    pub fn angle_between(&self, other: &Self) -> f32 {
        let dot = self.dot(other).abs().min(1.0);
        2.0 * dot.acos()
    }
    
    /// Spherically interpolates between this quaternion and another.
    #[inline]
    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let mut dot = self.dot(other);
        
        // If the dot product is negative, slerp won't take the shorter path.
        // Fix by reversing one quaternion.
        let mut q1 = *other;
        if dot < 0.0 {
            q1 = -q1;
            dot = -dot;
        }
        
        if dot > 0.9995 {
            // If the quaternions are very close, use linear interpolation
            let result = *self + (q1 - *self) * t;
            return result.normalize();
        }
        
        let angle = dot.acos();
        let sin_angle = angle.sin();
        
        let factor0 = ((1.0 - t) * angle).sin() / sin_angle;
        let factor1 = (t * angle).sin() / sin_angle;
        
        *self * factor0 + q1 * factor1
    }
    
    /// Linearly interpolates between this quaternion and another.
    /// The resulting quaternion is normalized.
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let result = *self * (1.0 - t) + *other * t;
        result.normalize()
    }
    
    /// Returns the rotation axis and angle in radians represented by this quaternion.
    #[inline]
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let q = self.normalize();
        let angle = 2.0 * q.w.acos();
        
        let sin_half_angle = (1.0 - q.w * q.w).sqrt();
        
        let axis = if sin_half_angle > EPSILON {
            Vec3::new(q.x / sin_half_angle, q.y / sin_half_angle, q.z / sin_half_angle)
        } else {
            // Arbitrary axis if quaternion is almost identity
            Vec3::unit_x()
        };
        
        (axis, angle)
    }
    
    /// Returns the Euler angles (roll, pitch, yaw) in radians represented by this quaternion.
    #[inline]
    pub fn to_euler(&self) -> (f32, f32, f32) {
        // Roll (x-axis rotation)
        let roll = f32::atan2(
            2.0 * (self.w * self.x + self.y * self.z),
            1.0 - 2.0 * (self.x * self.x + self.y * self.y),
        );
        
        // Pitch (y-axis rotation)
        let pitch = f32::asin(
            (2.0 * (self.w * self.y - self.z * self.x)).clamp(-1.0, 1.0),
        );
        
        // Yaw (z-axis rotation)
        let yaw = f32::atan2(
            2.0 * (self.w * self.z + self.x * self.y),
            1.0 - 2.0 * (self.y * self.y + self.z * self.z),
        );
        
        (roll, pitch, yaw)
    }
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Add for Quat {
    type Output = Self;
    
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Quat {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Quat {
    type Output = Self;
    
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Quat {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl MulAssign<f32> for Quat {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl Mul<Quat> for f32 {
    type Output = Quat;
    
    #[inline]
    fn mul(self, quat: Quat) -> Quat {
        quat * self
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    
    #[inline]
    fn div(self, scalar: f32) -> Self {
        let inv_scalar = 1.0 / scalar;
        Self {
            x: self.x * inv_scalar,
            y: self.y * inv_scalar,
            z: self.z * inv_scalar,
            w: self.w * inv_scalar,
        }
    }
}

impl DivAssign<f32> for Quat {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        let inv_scalar = 1.0 / scalar;
        self.x *= inv_scalar;
        self.y *= inv_scalar;
        self.z *= inv_scalar;
        self.w *= inv_scalar;
    }
}

impl Neg for Quat {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul for Quat {
    type Output = Self;
    
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl MulAssign for Quat {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quat({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// From conversion
impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(array: [f32; 4]) -> Self {
        Self::new(array[0], array[1], array[2], array[3])
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(quat: Quat) -> Self {
        [quat.x, quat.y, quat.z, quat.w]
    }
} 