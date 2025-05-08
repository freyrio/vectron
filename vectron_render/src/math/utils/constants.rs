/// Mathematical constants for the rendering engine

/// Pi
pub const PI: f32 = std::f32::consts::PI;

/// Pi multiplied by 2
pub const TWO_PI: f32 = 2.0 * PI;

/// Pi divided by 2
pub const HALF_PI: f32 = PI / 2.0;

/// Pi divided by 4
pub const QUARTER_PI: f32 = PI / 4.0;

/// 1.0 divided by Pi
pub const INV_PI: f32 = 1.0 / PI;

/// 1.0 divided by (2.0 * Pi)
pub const INV_TWO_PI: f32 = 1.0 / TWO_PI;

/// Square root of 2
pub const SQRT_2: f32 = std::f32::consts::SQRT_2;

/// Square root of 3
pub const SQRT_3: f32 = 1.7320508075688772;

/// Square root of 0.5 (same as 1.0 / sqrt(2.0))
pub const SQRT_HALF: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// Euler's number (e)
pub const E: f32 = std::f32::consts::E;

/// Conversion factor from degrees to radians
pub const DEG_TO_RAD: f32 = PI / 180.0;

/// Conversion factor from radians to degrees
pub const RAD_TO_DEG: f32 = 180.0 / PI;

/// A very small value often used for floating-point comparisons
pub const EPSILON: f32 = 1e-6;

/// Positive infinity
pub const INFINITY: f32 = std::f32::INFINITY;

/// Negative infinity
pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;

/// Not a number
pub const NAN: f32 = std::f32::NAN;

/// The golden ratio
pub const GOLDEN_RATIO: f32 = 1.6180339887498948; 