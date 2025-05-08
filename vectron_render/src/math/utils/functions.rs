use super::constants::*;

/// Linearly interpolates between two values.
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Performs a smoothstep interpolation.
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Performs a more gradual smoothstep interpolation (smootherstep).
#[inline]
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Clamps a value between a minimum and maximum.
#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { min } else if x > max { max } else { x }
}

/// Determines if two floating-point values are approximately equal within epsilon.
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() <= epsilon
}

/// Converts an angle from degrees to radians.
#[inline]
pub fn to_radians(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD
}

/// Converts an angle from radians to degrees.
#[inline]
pub fn to_degrees(radians: f32) -> f32 {
    radians * RAD_TO_DEG
}

/// Maps a value from one range to another.
#[inline]
pub fn map(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

/// Returns the sign of a value (-1.0, 0.0, or 1.0).
#[inline]
pub fn sign(x: f32) -> f32 {
    if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { 0.0 }
}

/// Performs a step function: 0.0 if x < edge, 1.0 otherwise.
#[inline]
pub fn step(edge: f32, x: f32) -> f32 {
    if x < edge { 0.0 } else { 1.0 }
}

/// Performs a sine cardinal (sinc) function.
#[inline]
pub fn sinc(x: f32) -> f32 {
    if x.abs() < EPSILON {
        1.0
    } else {
        (x * PI).sin() / (x * PI)
    }
}

/// Wraps an angle to the range [0, 2π].
#[inline]
pub fn wrap_angle(angle: f32) -> f32 {
    let mut result = angle % TWO_PI;
    if result < 0.0 {
        result += TWO_PI;
    }
    result
}

/// Normalizes an angle to the range [-π, π].
#[inline]
pub fn normalize_angle(angle: f32) -> f32 {
    let mut result = angle % TWO_PI;
    if result > PI {
        result -= TWO_PI;
    } else if result < -PI {
        result += TWO_PI;
    }
    result
}

/// Gets the shortest angular distance between two angles.
#[inline]
pub fn angle_distance(a: f32, b: f32) -> f32 {
    let delta = (b - a) % TWO_PI;
    (delta + PI) % TWO_PI - PI
}

/// Performs a power function that preserves the sign of the input.
#[inline]
pub fn signed_pow(x: f32, exp: f32) -> f32 {
    if x >= 0.0 {
        x.powf(exp)
    } else {
        -((-x).powf(exp))
    }
}

/// Hermite interpolation between two values.
#[inline]
pub fn hermite(a: f32, b: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let h1 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h2 = -2.0 * t3 + 3.0 * t2;
    a * h1 + b * h2
}

/// Cubic Hermite interpolation between two values with specified tangents.
#[inline]
pub fn cubic_hermite(a: f32, b: f32, tan_a: f32, tan_b: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;
    
    a * h00 + tan_a * h10 + b * h01 + tan_b * h11
}

/// Returns the fractional part of a number.
#[inline]
pub fn fract(x: f32) -> f32 {
    x - x.floor()
}

/// Performs a modulo operation that works for negative values.
#[inline]
pub fn mod_positive(x: f32, y: f32) -> f32 {
    ((x % y) + y) % y
}

/// Calculates the square of a value.
#[inline]
pub fn square(x: f32) -> f32 {
    x * x
}

/// Calculates the cube of a value.
#[inline]
pub fn cube(x: f32) -> f32 {
    x * x * x
} 