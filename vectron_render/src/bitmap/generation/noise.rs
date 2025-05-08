// Noise generation module
//
// Provides algorithms for generating various types of noise textures

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;

/// Types of noise generation algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseType {
    /// Value noise (simple and fast)
    Value,
    /// Perlin noise (smooth gradient noise)
    Perlin,
    /// Simplex noise (improved Perlin noise)
    Simplex,
    /// Worley noise (cellular/Voronoi-like patterns)
    Worley,
    /// Fractal Brownian Motion - layered noise
    Fbm,
}

/// Generator for procedural noise textures
pub struct NoiseGenerator {
    /// Random seed value
    seed: u32,
    /// Number of octaves for fractal noise
    octaves: u32,
    /// Persistence factor for octave amplitude
    persistence: f32,
    /// Lacunarity factor for octave frequency
    lacunarity: f32,
    /// Noise type to generate
    noise_type: NoiseType,
}

impl NoiseGenerator {
    /// Create a new noise generator with the specified seed
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.0,
            noise_type: NoiseType::Perlin,
        }
    }
    
    /// Set the number of octaves for fractal noise
    pub fn with_octaves(mut self, octaves: u32) -> Self {
        self.octaves = octaves.max(1);
        self
    }
    
    /// Set the persistence for fractal noise
    pub fn with_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence.max(0.0).min(1.0);
        self
    }
    
    /// Set the lacunarity for fractal noise
    pub fn with_lacunarity(mut self, lacunarity: f32) -> Self {
        self.lacunarity = lacunarity.max(1.0);
        self
    }
    
    /// Set the noise type to generate
    pub fn with_noise_type(mut self, noise_type: NoiseType) -> Self {
        self.noise_type = noise_type;
        self
    }
    
    /// Generate a grayscale noise bitmap with the specified dimensions and scale
    pub fn generate(&self, width: u32, height: u32, scale: f32) -> Bitmap {
        let mut bitmap = Bitmap::new(width, height);
        
        match self.noise_type {
            NoiseType::Value => bitmap = self.generate_value_noise(bitmap, scale),
            NoiseType::Perlin => bitmap = self.generate_perlin_noise(bitmap, scale),
            NoiseType::Simplex => bitmap = self.generate_simplex_noise(bitmap, scale),
            NoiseType::Worley => bitmap = self.generate_worley_noise(bitmap, scale),
            NoiseType::Fbm => bitmap = self.generate_fbm_noise(bitmap, scale),
        }
        
        bitmap
    }
    
    /// Generate a colored noise bitmap with the specified dimensions and scale
    pub fn generate_colored(&self, width: u32, height: u32, scale: f32, gradient: &[Color]) -> Bitmap {
        let grayscale = self.generate(width, height, scale);
        let mut result = Bitmap::new(width, height);
        
        if gradient.is_empty() {
            return grayscale; // Return grayscale if no gradient provided
        }
        
        for y in 0..height {
            for x in 0..width {
                if let Some(gray_color) = grayscale.get_pixel(x, y) {
                    // Use the red channel value as the gradient position
                    let pos = gray_color.r;
                    let color = sample_gradient(gradient, pos);
                    result.set_pixel(x, y, color);
                }
            }
        }
        
        result
    }
    
    /// Generate value noise
    fn generate_value_noise(&self, mut bitmap: Bitmap, scale: f32) -> Bitmap {
        let width = bitmap.width();
        let height = bitmap.height();
        let seed = self.seed;
        
        for y in 0..height {
            for x in 0..width {
                let nx = x as f32 * scale / width as f32;
                let ny = y as f32 * scale / height as f32;
                
                let value = value_noise(nx, ny, seed);
                let value = (value + 1.0) * 0.5; // Normalize to 0-1
                
                bitmap.set_pixel(x, y, Color::new(value, value, value, 1.0));
            }
        }
        
        bitmap
    }
    
    /// Generate Perlin noise
    fn generate_perlin_noise(&self, mut bitmap: Bitmap, scale: f32) -> Bitmap {
        let width = bitmap.width();
        let height = bitmap.height();
        let seed = self.seed;
        
        for y in 0..height {
            for x in 0..width {
                let nx = x as f32 * scale / width as f32;
                let ny = y as f32 * scale / height as f32;
                
                let value = perlin_noise(nx, ny, seed);
                let value = (value + 1.0) * 0.5; // Normalize to 0-1
                
                bitmap.set_pixel(x, y, Color::new(value, value, value, 1.0));
            }
        }
        
        bitmap
    }
    
    /// Generate Simplex noise
    fn generate_simplex_noise(&self, mut bitmap: Bitmap, scale: f32) -> Bitmap {
        let width = bitmap.width();
        let height = bitmap.height();
        let seed = self.seed;
        
        for y in 0..height {
            for x in 0..width {
                let nx = x as f32 * scale / width as f32;
                let ny = y as f32 * scale / height as f32;
                
                let value = simplex_noise(nx, ny, seed);
                let value = (value + 1.0) * 0.5; // Normalize to 0-1
                
                bitmap.set_pixel(x, y, Color::new(value, value, value, 1.0));
            }
        }
        
        bitmap
    }
    
    /// Generate Worley (cellular) noise
    fn generate_worley_noise(&self, mut bitmap: Bitmap, scale: f32) -> Bitmap {
        let width = bitmap.width();
        let height = bitmap.height();
        let seed = self.seed;
        
        // Generate random feature points
        let num_points = 32;
        let mut points = Vec::with_capacity(num_points);
        
        let mut rng = SimpleRng::new(seed);
        for _ in 0..num_points {
            let x = rng.next_f32();
            let y = rng.next_f32();
            points.push((x, y));
        }
        
        for y in 0..height {
            for x in 0..width {
                let nx = x as f32 / width as f32;
                let ny = y as f32 / height as f32;
                
                // Calculate distance to closest feature point
                let mut min_dist = f32::MAX;
                for (px, py) in &points {
                    // Calculate distance in torus topology (wrapping around edges)
                    let dx = (nx - px).abs().min(1.0 - (nx - px).abs());
                    let dy = (ny - py).abs().min(1.0 - (ny - py).abs());
                    
                    // Apply scale factor
                    let dx = dx * scale;
                    let dy = dy * scale;
                    
                    // Euclidean distance
                    let dist = (dx * dx + dy * dy).sqrt();
                    min_dist = min_dist.min(dist);
                }
                
                // Normalize distance
                let value = min_dist.min(1.0);
                
                bitmap.set_pixel(x, y, Color::new(value, value, value, 1.0));
            }
        }
        
        bitmap
    }
    
    /// Generate Fractal Brownian Motion (FBM) noise
    fn generate_fbm_noise(&self, mut bitmap: Bitmap, scale: f32) -> Bitmap {
        let width = bitmap.width();
        let height = bitmap.height();
        let seed = self.seed;
        let octaves = self.octaves;
        let persistence = self.persistence;
        let lacunarity = self.lacunarity;
        
        for y in 0..height {
            for x in 0..width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut total = 0.0;
                let mut max_value = 0.0;
                
                let mut nx = x as f32 / width as f32;
                let mut ny = y as f32 / height as f32;
                
                // Sum multiple octaves of noise
                for i in 0..octaves {
                    nx = x as f32 * scale * frequency / width as f32;
                    ny = y as f32 * scale * frequency / height as f32;
                    
                    let noise_value = perlin_noise(nx, ny, seed + i as u32);
                    total += noise_value * amplitude;
                    
                    max_value += amplitude;
                    amplitude *= persistence;
                    frequency *= lacunarity;
                }
                
                // Normalize
                let value = (total / max_value + 1.0) * 0.5;
                let value = value.max(0.0).min(1.0);
                
                bitmap.set_pixel(x, y, Color::new(value, value, value, 1.0));
            }
        }
        
        bitmap
    }
}

/// Sample a color from a gradient based on a position (0-1)
fn sample_gradient(gradient: &[Color], position: f32) -> Color {
    if gradient.is_empty() {
        return Color::BLACK;
    }
    
    if gradient.len() == 1 {
        return gradient[0];
    }
    
    let pos = position.max(0.0).min(1.0);
    let scaled_pos = pos * (gradient.len() - 1) as f32;
    let index = scaled_pos.floor() as usize;
    let t = scaled_pos - index as f32;
    
    let next_index = (index + 1).min(gradient.len() - 1);
    
    let c1 = gradient[index];
    let c2 = gradient[next_index];
    
    // Linear interpolation between colors
    let r = c1.r * (1.0 - t) + c2.r * t;
    let g = c1.g * (1.0 - t) + c2.g * t;
    let b = c1.b * (1.0 - t) + c2.b * t;
    let a = c1.a * (1.0 - t) + c2.a * t;
    
    Color::new(r, g, b, a)
}

/// Simple random number generator
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    
    fn next_u32(&mut self) -> u32 {
        // XorShift algorithm
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
    
    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
}

/// Hash function for noise generation
fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let h = ((x as u32).wrapping_mul(1619) 
           ^ (y as u32).wrapping_mul(31337) 
           ^ seed.wrapping_mul(1013)) & 0x7fffffff;
    h % 1024
}

/// Convert hash to normalized float
fn hash_to_float(hash: u32) -> f32 {
    (hash as f32 / 1024.0) * 2.0 - 1.0
}

/// Generate gradient vector from hash
fn hash_to_gradient(hash: u32) -> (f32, f32) {
    let angle = (hash as f32 / 1024.0) * 2.0 * std::f32::consts::PI;
    let (sin, cos) = angle.sin_cos();
    (cos, sin)
}

/// Smooth interpolation function
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Value noise function
fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;
    
    let sx = smoothstep(fx);
    let sy = smoothstep(fy);
    
    let n00 = hash_to_float(hash(x0, y0, seed));
    let n10 = hash_to_float(hash(x1, y0, seed));
    let n01 = hash_to_float(hash(x0, y1, seed));
    let n11 = hash_to_float(hash(x1, y1, seed));
    
    let nx0 = n00 * (1.0 - sx) + n10 * sx;
    let nx1 = n01 * (1.0 - sx) + n11 * sx;
    
    nx0 * (1.0 - sy) + nx1 * sy
}

/// Dot product for gradient noise
fn dot_grad(hash: u32, x: f32, y: f32) -> f32 {
    let (gx, gy) = hash_to_gradient(hash);
    x * gx + y * gy
}

/// Perlin noise function
fn perlin_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    
    let dx0 = x - x0 as f32;
    let dy0 = y - y0 as f32;
    let dx1 = dx0 - 1.0;
    let dy1 = dy0 - 1.0;
    
    let g00 = hash(x0, y0, seed);
    let g10 = hash(x1, y0, seed);
    let g01 = hash(x0, y1, seed);
    let g11 = hash(x1, y1, seed);
    
    let n00 = dot_grad(g00, dx0, dy0);
    let n10 = dot_grad(g10, dx1, dy0);
    let n01 = dot_grad(g01, dx0, dy1);
    let n11 = dot_grad(g11, dx1, dy1);
    
    let sx = smoothstep(dx0);
    let sy = smoothstep(dy0);
    
    let nx0 = n00 * (1.0 - sx) + n10 * sx;
    let nx1 = n01 * (1.0 - sx) + n11 * sx;
    
    nx0 * (1.0 - sy) + nx1 * sy
}

/// Simplex noise function (based on Perlin's improved noise)
fn simplex_noise(x: f32, y: f32, seed: u32) -> f32 {
    // Skew input space to determine which simplex cell we're in
    const F2: f32 = 0.366025404; // (sqrt(3) - 1) / 2
    const G2: f32 = 0.211324865; // (3 - sqrt(3)) / 6
    
    let s = (x + y) * F2;
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;
    
    let t = (i + j) as f32 * G2;
    let X0 = i as f32 - t;
    let Y0 = j as f32 - t;
    let x0 = x - X0;
    let y0 = y - Y0;
    
    // Determine which simplex we're in
    let i1 = if x0 > y0 { 1 } else { 0 };
    let j1 = if x0 > y0 { 0 } else { 1 };
    
    let x1 = x0 - i1 as f32 + G2;
    let y1 = y0 - j1 as f32 + G2;
    let x2 = x0 - 1.0 + 2.0 * G2;
    let y2 = y0 - 1.0 + 2.0 * G2;
    
    // Calculate the contribution from the three corners
    let g0 = hash(i, j, seed);
    let g1 = hash(i + i1, j + j1, seed);
    let g2 = hash(i + 1, j + 1, seed);
    
    let mut n0 = 0.0;
    let t0 = 0.5 - x0 * x0 - y0 * y0;
    if t0 > 0.0 {
        n0 = t0 * t0 * t0 * t0 * dot_grad(g0, x0, y0);
    }
    
    let mut n1 = 0.0;
    let t1 = 0.5 - x1 * x1 - y1 * y1;
    if t1 > 0.0 {
        n1 = t1 * t1 * t1 * t1 * dot_grad(g1, x1, y1);
    }
    
    let mut n2 = 0.0;
    let t2 = 0.5 - x2 * x2 - y2 * y2;
    if t2 > 0.0 {
        n2 = t2 * t2 * t2 * t2 * dot_grad(g2, x2, y2);
    }
    
    // The result is scaled to return a value in the range [-1, 1]
    70.0 * (n0 + n1 + n2)
} 