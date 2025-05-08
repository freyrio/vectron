use crate::math::vector::Vec3;
use crate::math::quaternion::Quat;

/// Represents a 3D rotation using either a quaternion, Euler angles, or a matrix.
/// Provides convenient conversions between different representations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation3D {
    /// Rotation represented as a quaternion
    Quaternion(Quat),
    
    /// Rotation represented as Euler angles (roll, pitch, yaw) in radians
    EulerAngles(f32, f32, f32),
    
    /// Rotation represented as an axis and angle in radians
    AxisAngle(Vec3, f32),
}

impl Rotation3D {
    /// Creates a new rotation from a quaternion.
    #[inline]
    pub fn from_quaternion(quat: Quat) -> Self {
        Self::Quaternion(quat.normalize())
    }
    
    /// Creates a new rotation from Euler angles (roll, pitch, yaw) in radians.
    #[inline]
    pub fn from_euler(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self::EulerAngles(roll, pitch, yaw)
    }
    
    /// Creates a new rotation from an axis and angle in radians.
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        Self::AxisAngle(axis.normalize(), angle)
    }
    
    /// Converts this rotation to a quaternion.
    #[inline]
    pub fn to_quaternion(&self) -> Quat {
        match *self {
            Self::Quaternion(quat) => quat,
            Self::EulerAngles(roll, pitch, yaw) => Quat::from_euler(roll, pitch, yaw),
            Self::AxisAngle(axis, angle) => Quat::from_axis_angle(axis, angle),
        }
    }
    
    /// Converts this rotation to Euler angles (roll, pitch, yaw) in radians.
    #[inline]
    pub fn to_euler(&self) -> (f32, f32, f32) {
        match *self {
            Self::Quaternion(quat) => quat.to_euler(),
            Self::EulerAngles(roll, pitch, yaw) => (roll, pitch, yaw),
            Self::AxisAngle(axis, angle) => {
                Quat::from_axis_angle(axis, angle).to_euler()
            },
        }
    }
    
    /// Converts this rotation to an axis and angle in radians.
    #[inline]
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        match *self {
            Self::Quaternion(quat) => quat.to_axis_angle(),
            Self::EulerAngles(roll, pitch, yaw) => {
                Quat::from_euler(roll, pitch, yaw).to_axis_angle()
            },
            Self::AxisAngle(axis, angle) => (axis, angle),
        }
    }
    
    /// Rotates a vector by this rotation.
    #[inline]
    pub fn rotate_vector(&self, vector: Vec3) -> Vec3 {
        self.to_quaternion().rotate_vector(vector)
    }
    
    /// Spherically interpolates between this rotation and another.
    #[inline]
    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let q1 = self.to_quaternion();
        let q2 = other.to_quaternion();
        Self::Quaternion(q1.slerp(&q2, t))
    }
    
    /// Returns the angle in radians between this rotation and another.
    #[inline]
    pub fn angle_between(&self, other: &Self) -> f32 {
        let q1 = self.to_quaternion();
        let q2 = other.to_quaternion();
        q1.angle_between(&q2)
    }
}

impl Default for Rotation3D {
    #[inline]
    fn default() -> Self {
        Self::Quaternion(Quat::identity())
    }
}

/// Interpolates between a sequence of rotations using spherical linear interpolation.
#[derive(Debug, Clone)]
pub struct RotationSequence {
    /// The sequence of rotations to interpolate between.
    rotations: Vec<Quat>,
    
    /// The time points for each rotation (must be sorted in ascending order).
    times: Vec<f32>,
}

impl RotationSequence {
    /// Creates a new rotation sequence.
    ///
    /// # Arguments
    ///
    /// * `rotations` - The sequence of rotations to interpolate between.
    /// * `times` - The time points for each rotation (must be sorted in ascending order).
    ///
    /// # Panics
    ///
    /// Panics if `rotations` and `times` have different lengths or are empty.
    #[inline]
    pub fn new(rotations: Vec<Quat>, times: Vec<f32>) -> Self {
        assert!(!rotations.is_empty(), "Rotations must not be empty");
        assert_eq!(rotations.len(), times.len(), "Rotations and times must have the same length");
        
        // Check if times are in ascending order
        for i in 1..times.len() {
            assert!(times[i] >= times[i - 1], "Times must be in ascending order");
        }
        
        // Normalize all quaternions
        let rotations = rotations.into_iter().map(|q| q.normalize()).collect();
        
        Self { rotations, times }
    }
    
    /// Evaluates the rotation at the given time.
    ///
    /// If the time is before the first time point, returns the first rotation.
    /// If the time is after the last time point, returns the last rotation.
    /// Otherwise, spherically interpolates between the two surrounding rotations.
    #[inline]
    pub fn evaluate(&self, time: f32) -> Quat {
        let n = self.times.len();
        
        // Handle edge cases
        if n == 1 || time <= self.times[0] {
            return self.rotations[0];
        }
        
        if time >= self.times[n - 1] {
            return self.rotations[n - 1];
        }
        
        // Find the surrounding indices
        let mut index = 0;
        for i in 0..n - 1 {
            if time >= self.times[i] && time <= self.times[i + 1] {
                index = i;
                break;
            }
        }
        
        // Interpolate between the surrounding rotations
        let t_segment = (time - self.times[index]) / (self.times[index + 1] - self.times[index]);
        self.rotations[index].slerp(&self.rotations[index + 1], t_segment)
    }
} 