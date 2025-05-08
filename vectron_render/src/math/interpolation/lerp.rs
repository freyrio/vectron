use crate::math::vector::{Vec2, Vec3, Vec4};
use crate::math::utils::functions;

/// Trait for types that can be linearly interpolated.
pub trait Lerp {
    /// Linearly interpolate between two values.
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

/// Linear interpolation implementation for f32.
impl Lerp for f32 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        functions::lerp(*self, *other, t)
    }
}

/// Linear interpolation implementation for Vec2.
impl Lerp for Vec2 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Vec2::new(
            self.x.lerp(&other.x, t),
            self.y.lerp(&other.y, t),
        )
    }
}

/// Linear interpolation implementation for Vec3.
impl Lerp for Vec3 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Vec3::new(
            self.x.lerp(&other.x, t),
            self.y.lerp(&other.y, t),
            self.z.lerp(&other.z, t),
        )
    }
}

/// Linear interpolation implementation for Vec4.
impl Lerp for Vec4 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Vec4::new(
            self.x.lerp(&other.x, t),
            self.y.lerp(&other.y, t),
            self.z.lerp(&other.z, t),
            self.w.lerp(&other.w, t),
        )
    }
}

/// A generic linear interpolator that can interpolate between a sequence of values.
#[derive(Debug, Clone)]
pub struct LerpSequence<T: Lerp + Clone> {
    /// The sequence of values to interpolate between.
    values: Vec<T>,
    /// The time points for each value (must be sorted in ascending order).
    times: Vec<f32>,
}

impl<T: Lerp + Clone> LerpSequence<T> {
    /// Creates a new linear interpolation sequence.
    ///
    /// # Arguments
    ///
    /// * `values` - The sequence of values to interpolate between.
    /// * `times` - The time points for each value (must be sorted in ascending order).
    ///
    /// # Panics
    ///
    /// Panics if `values` and `times` have different lengths or are empty.
    #[inline]
    pub fn new(values: Vec<T>, times: Vec<f32>) -> Self {
        assert!(!values.is_empty(), "Values must not be empty");
        assert_eq!(values.len(), times.len(), "Values and times must have the same length");
        
        // Check if times are in ascending order
        for i in 1..times.len() {
            assert!(times[i] >= times[i - 1], "Times must be in ascending order");
        }
        
        Self { values, times }
    }
    
    /// Evaluates the interpolation at the given time.
    ///
    /// If the time is before the first time point, returns the first value.
    /// If the time is after the last time point, returns the last value.
    /// Otherwise, linearly interpolates between the two surrounding values.
    #[inline]
    pub fn evaluate(&self, time: f32) -> T {
        let n = self.times.len();
        
        // Handle edge cases
        if n == 1 || time <= self.times[0] {
            return self.values[0].clone();
        }
        
        if time >= self.times[n - 1] {
            return self.values[n - 1].clone();
        }
        
        // Find the surrounding indices
        let mut index = 0;
        for i in 0..n - 1 {
            if time >= self.times[i] && time <= self.times[i + 1] {
                index = i;
                break;
            }
        }
        
        // Interpolate between the surrounding values
        let t_segment = (time - self.times[index]) / (self.times[index + 1] - self.times[index]);
        self.values[index].lerp(&self.values[index + 1], t_segment)
    }
    
    /// Returns the number of values in the sequence.
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Returns true if the sequence is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// Returns a reference to the values in the sequence.
    #[inline]
    pub fn values(&self) -> &[T] {
        &self.values
    }
    
    /// Returns a reference to the times in the sequence.
    #[inline]
    pub fn times(&self) -> &[f32] {
        &self.times
    }
} 