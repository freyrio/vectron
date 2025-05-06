/*!
 * Quaternion implementation for 3D rotations
 */
use std::ops::{Mul, Add, Sub, Neg, MulAssign, AddAssign, SubAssign};
use crate::core::math::{Vec3, Vec4, to_radians, to_degrees, approx_eq, lerp};

/// Quaternion for representing 3D rotations
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    /// Create a new quaternion with components (x, y, z, w)
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    /// Create an identity quaternion (no rotation)
    #[inline]
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
    
    /// Create a quaternion from axis-angle representation
    /// axis: normalized rotation axis
    /// angle_degrees: rotation angle in degrees
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle_degrees: f32) -> Self {
        let half_angle = to_radians(angle_degrees) * 0.5;
        let s = half_angle.sin();
        let normalized_axis = axis.normalized();
        
        Self {
            x: normalized_axis.x * s,
            y: normalized_axis.y * s,
            z: normalized_axis.z * s,
            w: half_angle.cos(),
        }
    }
    
    /// Create a quaternion from euler angles (in degrees)
    /// Order of rotations is YXZ (yaw, pitch, roll)
    #[inline]
    pub fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Self {
        // Convert to radians and halve
        let yaw = to_radians(yaw) * 0.5;
        let pitch = to_radians(pitch) * 0.5;
        let roll = to_radians(roll) * 0.5;
        
        // Precompute sine and cosine values
        let cy = yaw.cos();
        let sy = yaw.sin();
        let cp = pitch.cos();
        let sp = pitch.sin();
        let cr = roll.cos();
        let sr = roll.sin();
        
        // YXZ rotation order
        Self {
            x: cy * sp * cr + sy * cp * sr,
            y: sy * cp * cr - cy * sp * sr,
            z: cy * cp * sr - sy * sp * cr,
            w: cy * cp * cr + sy * sp * sr,
        }
    }
    
    /// Create a quaternion from a rotation around the X axis
    #[inline]
    pub fn from_rotation_x(angle_degrees: f32) -> Self {
        let half_angle = to_radians(angle_degrees) * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        
        Self { x: s, y: 0.0, z: 0.0, w: c }
    }
    
    /// Create a quaternion from a rotation around the Y axis
    #[inline]
    pub fn from_rotation_y(angle_degrees: f32) -> Self {
        let half_angle = to_radians(angle_degrees) * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        
        Self { x: 0.0, y: s, z: 0.0, w: c }
    }
    
    /// Create a quaternion from a rotation around the Z axis
    #[inline]
    pub fn from_rotation_z(angle_degrees: f32) -> Self {
        let half_angle = to_radians(angle_degrees) * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        
        Self { x: 0.0, y: 0.0, z: s, w: c }
    }
    
    /// Create a quaternion to rotate from one direction to another
    #[inline]
    pub fn from_rotation_between(from: Vec3, to: Vec3) -> Self {
        let from_normalized = from.normalized();
        let to_normalized = to.normalized();
        
        // Check if vectors are exactly opposite
        let dot = from_normalized.dot(&to_normalized);
        
        if approx_eq(dot, -1.0, 1e-6) {
            // Vectors are opposite, find an orthogonal vector
            let mut orthogonal = Vec3::unit_x();
            if from_normalized.dot(&orthogonal).abs() > 0.9 {
                orthogonal = Vec3::unit_y();
            }
            
            // Rotate 180 degrees around the orthogonal vector
            return Self::from_axis_angle(
                from_normalized.cross(&orthogonal).normalized(), 
                180.0
            );
        }
        
        if approx_eq(dot, 1.0, 1e-6) {
            // Vectors are nearly identical, return identity
            return Self::identity();
        }
        
        // Standard case - vectors have an angle between them
        let cross = from_normalized.cross(&to_normalized);
        
        Self {
            x: cross.x,
            y: cross.y,
            z: cross.z,
            w: 1.0 + dot,
        }.normalized()
    }
    
    /// Calculate the magnitude (length) of the quaternion
    #[inline]
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    
    /// Calculate the square magnitude of the quaternion
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    
    /// Normalize the quaternion
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            self.x *= inv_mag;
            self.y *= inv_mag;
            self.z *= inv_mag;
            self.w *= inv_mag;
        }
        self
    }
    
    /// Return a normalized copy of the quaternion
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut result = *self;
        result.normalize();
        result
    }
    
    /// Conjugate of the quaternion (inverse rotation)
    #[inline]
    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    
    /// Inverse of the quaternion
    #[inline]
    pub fn inverse(&self) -> Self {
        let mag_squared = self.magnitude_squared();
        if mag_squared > 0.0 {
            let inv_mag_squared = 1.0 / mag_squared;
            Self {
                x: -self.x * inv_mag_squared,
                y: -self.y * inv_mag_squared,
                z: -self.z * inv_mag_squared,
                w: self.w * inv_mag_squared,
            }
        } else {
            *self // Return the original if magnitude is zero
        }
    }
    
    /// Calculate the dot product between two quaternions
    #[inline]
    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    
    /// Spherical linear interpolation between quaternions
    #[inline]
    pub fn slerp(&self, other: &Quaternion, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mut dot = self.dot(other);
        
        // Determine direction and adjust dot product
        let mut target = *other;
        if dot < 0.0 {
            dot = -dot;
            target = -*other;
        }
        
        // If quaternions are very close, use linear interpolation
        if dot > 0.9995 {
            return Self {
                x: lerp(self.x, target.x, t),
                y: lerp(self.y, target.y, t),
                z: lerp(self.z, target.z, t),
                w: lerp(self.w, target.w, t),
            }.normalized();
        }
        
        // Calculate angle and sine values
        let theta = dot.clamp(-1.0, 1.0).acos();
        let sin_theta = theta.sin();
        
        // Calculate weights
        let w1 = ((1.0 - t) * theta).sin() / sin_theta;
        let w2 = (t * theta).sin() / sin_theta;
        
        // Calculate interpolated quaternion
        Self {
            x: w1 * self.x + w2 * target.x,
            y: w1 * self.y + w2 * target.y,
            z: w1 * self.z + w2 * target.z,
            w: w1 * self.w + w2 * target.w,
        }
    }
    
    /// Non-spherical linear interpolation (nlerp)
    /// Faster but less accurate than slerp for large angles
    #[inline]
    pub fn nlerp(&self, other: &Quaternion, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let dot = self.dot(other);
        
        // Determine direction
        let target = if dot < 0.0 { -*other } else { *other };
        
        // Linear interpolation
        Self {
            x: lerp(self.x, target.x, t),
            y: lerp(self.y, target.y, t),
            z: lerp(self.z, target.z, t),
            w: lerp(self.w, target.w, t),
        }.normalized()
    }
    
    /// Rotate a vector by this quaternion
    #[inline]
    pub fn rotate_vec3(&self, v: Vec3) -> Vec3 {
        // v' = q * v * q^-1 but optimized for pure quaternion
        
        // Extract the vector part of the quaternion
        let q_vec = Vec3::new(self.x, self.y, self.z);
        
        // Calculate intermediate results
        let cross1 = q_vec.cross(&v);
        let cross2 = q_vec.cross(&cross1);
        
        // Final calculation
        v + cross1 * (2.0 * self.w) + cross2 * 2.0
    }
    
    /// Convert to a normalized axis and angle in degrees
    #[inline]
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let normalized = self.normalized();
        let angle = 2.0 * normalized.w.acos();
        
        if approx_eq(angle, 0.0, 1e-6) {
            // No rotation, return arbitrary axis
            return (Vec3::unit_x(), 0.0);
        }
        
        let sin_half_angle = (1.0 - normalized.w * normalized.w).sqrt();
        
        // Calculate axis (normalize the vector part)
        let axis = if sin_half_angle > 1e-6 {
            Vec3::new(
                normalized.x / sin_half_angle,
                normalized.y / sin_half_angle,
                normalized.z / sin_half_angle,
            )
        } else {
            Vec3::unit_x() // Arbitrary axis for very small angles
        };
        
        (axis, to_degrees(angle))
    }
    
    /// Convert to euler angles (in degrees)
    /// Order of rotations is YXZ (yaw, pitch, roll)
    #[inline]
    pub fn to_euler(&self) -> (f32, f32, f32) {
        let q = self.normalized();
        
        // Extract components for convenience
        let x = q.x;
        let y = q.y;
        let z = q.z;
        let w = q.w;
        
        // Calculate pitch (x-axis rotation)
        let pitch = (-2.0 * (y * z - w * x)).asin();
        
        // Check for gimbal lock
        if approx_eq((y * z - w * x).abs(), 0.5, 1e-6) {
            // Gimbal lock case
            let yaw = 2.0 * (x * z + w * y).atan2(w * w - x * x - y * y + z * z);
            let roll = 0.0; // In gimbal lock, roll loses a degree of freedom
            
            return (
                to_degrees(yaw),
                to_degrees(pitch),
                to_degrees(roll),
            );
        }
        
        // Normal case (no gimbal lock)
        let yaw = (2.0 * (x * y + w * z)).atan2(w * w + x * x - y * y - z * z);
        let roll = (2.0 * (x * z + w * y)).atan2(w * w - x * x - y * y + z * z);
        
        (
            to_degrees(yaw),
            to_degrees(pitch),
            to_degrees(roll),
        )
    }
    
    /// Convert to Vec4 representation
    #[inline]
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
    
    /// Convert to array [x, y, z, w]
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

// Quaternion multiplication
impl Mul for Quaternion {
    type Output = Quaternion;
    
    #[inline]
    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

// Quaternion-vector multiplication (rotation)
impl Mul<Vec3> for Quaternion {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.rotate_vec3(rhs)
    }
}

// Addition
impl Add for Quaternion {
    type Output = Quaternion;
    
    #[inline]
    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// Subtraction
impl Sub for Quaternion {
    type Output = Quaternion;
    
    #[inline]
    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// Negation
impl Neg for Quaternion {
    type Output = Quaternion;
    
    #[inline]
    fn neg(self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Compound assignment operators
impl MulAssign for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self = *self * rhs;
    }
}

impl AddAssign for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: Quaternion) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: Quaternion) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// From conversions
impl From<[f32; 4]> for Quaternion {
    #[inline]
    fn from(v: [f32; 4]) -> Self {
        Self { x: v[0], y: v[1], z: v[2], w: v[3] }
    }
}

impl From<Vec4> for Quaternion {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w: v.w }
    }
} 