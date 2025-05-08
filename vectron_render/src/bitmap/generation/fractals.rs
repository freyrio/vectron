// Fractal pattern generation
//
// Provides functionality for generating various fractal patterns as bitmaps

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;
use std::f32::consts::PI;

/// Types of fractal patterns that can be generated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractalType {
    /// Mandelbrot set
    Mandelbrot,
    /// Julia set
    Julia,
    /// Burning ship fractal
    BurningShip,
    /// Newton fractal
    Newton,
    /// Sierpinski triangle
    Sierpinski,
}

/// Generator for various fractal patterns
pub struct FractalGenerator {
    /// Maximum number of iterations for escape-time fractals
    iterations: u32,
    /// Escape radius for escape-time fractals
    escape_radius: f32,
    /// Complex parameter for Julia sets
    julia_c: (f32, f32),
    /// Color mapping options
    color_mode: ColorMode,
}

/// Color mapping options for fractal visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Simple grayscale based on iteration count
    Grayscale,
    /// HSL color mapping based on iteration count
    Hsl,
    /// Smooth coloring algorithm
    Smooth,
    /// Black and white (binary) based on escape
    Binary,
}

impl FractalGenerator {
    /// Create a new fractal generator with default parameters
    pub fn new() -> Self {
        Self {
            iterations: 100,
            escape_radius: 2.0,
            julia_c: (-0.7, 0.27015),
            color_mode: ColorMode::Hsl,
        }
    }
    
    /// Set maximum iterations
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }
    
    /// Set escape radius
    pub fn with_escape_radius(mut self, radius: f32) -> Self {
        self.escape_radius = radius;
        self
    }
    
    /// Set Julia set parameter
    pub fn with_julia_c(mut self, real: f32, imag: f32) -> Self {
        self.julia_c = (real, imag);
        self
    }
    
    /// Set color mapping mode
    pub fn with_color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }
    
    /// Generate a fractal pattern with the specified parameters
    pub fn generate(
        &self,
        width: u32,
        height: u32,
        fractal_type: FractalType,
        center_x: f32,
        center_y: f32,
        zoom: f32,
    ) -> Bitmap {
        let mut bitmap = Bitmap::new(width, height);
        
        match fractal_type {
            FractalType::Mandelbrot => self.generate_mandelbrot(&mut bitmap, center_x, center_y, zoom),
            FractalType::Julia => self.generate_julia(&mut bitmap, center_x, center_y, zoom),
            FractalType::BurningShip => self.generate_burning_ship(&mut bitmap, center_x, center_y, zoom),
            FractalType::Newton => self.generate_newton(&mut bitmap, center_x, center_y, zoom),
            FractalType::Sierpinski => self.generate_sierpinski(&mut bitmap, center_x, center_y, zoom),
        }
        
        bitmap
    }
    
    /// Generate Mandelbrot set
    fn generate_mandelbrot(&self, bitmap: &mut Bitmap, center_x: f32, center_y: f32, zoom: f32) {
        let width = bitmap.width();
        let height = bitmap.height();
        let aspect = width as f32 / height as f32;
        
        let scale_x = 3.0 / (zoom * width as f32);
        let scale_y = 3.0 / (zoom * height as f32 * aspect);
        
        for y in 0..height {
            for x in 0..width {
                let cx = (x as f32 - width as f32 / 2.0) * scale_x + center_x;
                let cy = (y as f32 - height as f32 / 2.0) * scale_y + center_y;
                
                let iter = self.mandelbrot_iteration(cx, cy);
                let color = self.map_color(iter, self.iterations);
                
                bitmap.set_pixel(x, y, color);
            }
        }
    }
    
    /// Calculate Mandelbrot iteration for a point
    fn mandelbrot_iteration(&self, cx: f32, cy: f32) -> u32 {
        let mut zx = 0.0;
        let mut zy = 0.0;
        let mut iter = 0;
        
        while zx * zx + zy * zy < self.escape_radius * self.escape_radius && iter < self.iterations {
            let tmp = zx * zx - zy * zy + cx;
            zy = 2.0 * zx * zy + cy;
            zx = tmp;
            iter += 1;
        }
        
        iter
    }
    
    /// Generate Julia set
    fn generate_julia(&self, bitmap: &mut Bitmap, center_x: f32, center_y: f32, zoom: f32) {
        let width = bitmap.width();
        let height = bitmap.height();
        let aspect = width as f32 / height as f32;
        
        let scale_x = 3.0 / (zoom * width as f32);
        let scale_y = 3.0 / (zoom * height as f32 * aspect);
        
        let (julia_real, julia_imag) = self.julia_c;
        
        for y in 0..height {
            for x in 0..width {
                let mut zx = (x as f32 - width as f32 / 2.0) * scale_x + center_x;
                let mut zy = (y as f32 - height as f32 / 2.0) * scale_y + center_y;
                
                let mut iter = 0;
                while zx * zx + zy * zy < self.escape_radius * self.escape_radius && iter < self.iterations {
                    let tmp = zx * zx - zy * zy + julia_real;
                    zy = 2.0 * zx * zy + julia_imag;
                    zx = tmp;
                    iter += 1;
                }
                
                let color = self.map_color(iter, self.iterations);
                bitmap.set_pixel(x, y, color);
            }
        }
    }
    
    /// Generate Burning Ship fractal
    fn generate_burning_ship(&self, bitmap: &mut Bitmap, center_x: f32, center_y: f32, zoom: f32) {
        let width = bitmap.width();
        let height = bitmap.height();
        let aspect = width as f32 / height as f32;
        
        let scale_x = 3.0 / (zoom * width as f32);
        let scale_y = 3.0 / (zoom * height as f32 * aspect);
        
        for y in 0..height {
            for x in 0..width {
                let cx = (x as f32 - width as f32 / 2.0) * scale_x + center_x;
                let cy = (y as f32 - height as f32 / 2.0) * scale_y + center_y;
                
                let mut zx = 0.0;
                let mut zy = 0.0;
                let mut iter = 0;
                
                while zx * zx + zy * zy < self.escape_radius * self.escape_radius && iter < self.iterations {
                    // The burning ship uses absolute values before squaring
                    zx = zx.abs();
                    zy = zy.abs();
                    
                    let tmp = zx * zx - zy * zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = tmp;
                    iter += 1;
                }
                
                let color = self.map_color(iter, self.iterations);
                bitmap.set_pixel(x, y, color);
            }
        }
    }
    
    /// Generate Newton fractal
    fn generate_newton(&self, bitmap: &mut Bitmap, center_x: f32, center_y: f32, zoom: f32) {
        let width = bitmap.width();
        let height = bitmap.height();
        let aspect = width as f32 / height as f32;
        
        let scale_x = 3.0 / (zoom * width as f32);
        let scale_y = 3.0 / (zoom * height as f32 * aspect);
        
        // Define the roots of the polynomial z^3 - 1
        let roots = [
            (1.0, 0.0),                                // 1
            (-0.5, 0.866),                             // -0.5 + 0.866i
            (-0.5, -0.866),                            // -0.5 - 0.866i
        ];
        
        let root_colors = [
            Color::new(1.0, 0.0, 0.0, 1.0),   // Red
            Color::new(0.0, 1.0, 0.0, 1.0),   // Green
            Color::new(0.0, 0.0, 1.0, 1.0),   // Blue
        ];
        
        for y in 0..height {
            for x in 0..width {
                let mut zx = (x as f32 - width as f32 / 2.0) * scale_x + center_x;
                let mut zy = (y as f32 - height as f32 / 2.0) * scale_y + center_y;
                
                // Skip points very close to zero to avoid division by zero
                if zx.abs() < 1e-10 && zy.abs() < 1e-10 {
                    zx = 1e-10;
                }
                
                let mut iter = 0;
                let tolerance = 1e-6;
                let mut converged = false;
                let mut root_idx = 0;
                
                while iter < self.iterations && !converged {
                    // Newton's method for z^3 - 1 = 0
                    // z_next = z - (z^3 - 1) / (3 * z^2)
                    
                    // Calculate z^2
                    let z2x = zx * zx - zy * zy;
                    let z2y = 2.0 * zx * zy;
                    
                    // Calculate z^3
                    let z3x = z2x * zx - z2y * zy;
                    let z3y = z2x * zy + z2y * zx;
                    
                    // Calculate z^3 - 1
                    let numerator_x = z3x - 1.0;
                    let numerator_y = z3y;
                    
                    // Calculate 3 * z^2
                    let denominator_x = 3.0 * z2x;
                    let denominator_y = 3.0 * z2y;
                    
                    // Calculate (z^3 - 1) / (3 * z^2)
                    let denominator_square = denominator_x * denominator_x + denominator_y * denominator_y;
                    let div_x = (numerator_x * denominator_x + numerator_y * denominator_y) / denominator_square;
                    let div_y = (numerator_y * denominator_x - numerator_x * denominator_y) / denominator_square;
                    
                    // Update z
                    zx = zx - div_x;
                    zy = zy - div_y;
                    
                    // Check if we've converged to a root
                    for (i, &(rx, ry)) in roots.iter().enumerate() {
                        let dx = zx - rx;
                        let dy = zy - ry;
                        if dx * dx + dy * dy < tolerance {
                            converged = true;
                            root_idx = i;
                            break;
                        }
                    }
                    
                    iter += 1;
                }
                
                // Color based on which root we converged to and how quickly
                let color = if converged {
                    // Use the color of the root with intensity based on iteration count
                    let base_color = root_colors[root_idx];
                    let intensity = 0.5 + 0.5 * (self.iterations - iter) as f32 / self.iterations as f32;
                    Color::new(
                        base_color.r * intensity,
                        base_color.g * intensity,
                        base_color.b * intensity,
                        1.0
                    )
                } else {
                    // Black for points that didn't converge
                    Color::new(0.0, 0.0, 0.0, 1.0)
                };
                
                bitmap.set_pixel(x, y, color);
            }
        }
    }
    
    /// Generate Sierpinski triangle fractal
    fn generate_sierpinski(&self, bitmap: &mut Bitmap, _center_x: f32, _center_y: f32, zoom: f32) {
        let width = bitmap.width();
        let height = bitmap.height();
        
        // Clear bitmap
        for y in 0..height {
            for x in 0..width {
                bitmap.set_pixel(x, y, Color::new(0.0, 0.0, 0.0, 1.0));
            }
        }
        
        // Determine size based on zoom
        let size = (height as f32 * zoom).min(width as f32) as u32;
        let offset_x = (width - size) / 2;
        let offset_y = (height - size) / 2;
        
        // For Sierpinski, we'll use a simple recursive division algorithm
        // This is much more efficient than iteration for this particular fractal
        draw_sierpinski(bitmap, offset_x, offset_y + size, size, 8, Color::new(1.0, 1.0, 1.0, 1.0));
    }
    
    /// Map iteration count to color based on the selected color mode
    fn map_color(&self, iter: u32, max_iter: u32) -> Color {
        match self.color_mode {
            ColorMode::Grayscale => {
                // Simple grayscale mapping
                if iter == max_iter {
                    Color::new(0.0, 0.0, 0.0, 1.0) // Black for points in the set
                } else {
                    let v = iter as f32 / max_iter as f32;
                    Color::new(v, v, v, 1.0)
                }
            },
            ColorMode::Hsl => {
                // HSL color mapping
                if iter == max_iter {
                    Color::new(0.0, 0.0, 0.0, 1.0) // Black for points in the set
                } else {
                    // Map iteration to hue (cycling through colors)
                    let hue = 360.0 * (iter as f32 / 50.0).fract();
                    let saturation = 0.8;
                    let lightness = 0.6;
                    
                    // Simple HSL to RGB conversion
                    let c = (1.0 - f32::abs(2.0 * lightness - 1.0)) * saturation;
                    let x = c * (1.0 - f32::abs((hue / 60.0) % 2.0 - 1.0));
                    let m = lightness - c / 2.0;
                    
                    let (r, g, b) = if hue < 60.0 {
                        (c, x, 0.0)
                    } else if hue < 120.0 {
                        (x, c, 0.0)
                    } else if hue < 180.0 {
                        (0.0, c, x)
                    } else if hue < 240.0 {
                        (0.0, x, c)
                    } else if hue < 300.0 {
                        (x, 0.0, c)
                    } else {
                        (c, 0.0, x)
                    };
                    
                    Color::new(r + m, g + m, b + m, 1.0)
                }
            },
            ColorMode::Smooth => {
                // Smooth coloring algorithm based on logarithmic scaling
                if iter == max_iter {
                    Color::new(0.0, 0.0, 0.0, 1.0) // Black for points in the set
                } else {
                    // Use a simpler formula without zx/zy variables
                    let smooth_iter = iter as f32 + 1.0 - (iter as f32 / max_iter as f32).ln() / 2.0f32.ln();
                    let v = smooth_iter / max_iter as f32;
                    
                    // Map to a color palette
                    let r = 0.5 + 0.5 * (3.0 * v).sin();
                    let g = 0.5 + 0.5 * (3.0 * v + 2.0).sin();
                    let b = 0.5 + 0.5 * (3.0 * v + 4.0).sin();
                    
                    Color::new(r, g, b, 1.0)
                }
            },
            ColorMode::Binary => {
                // Binary (black/white) coloring
                if iter == max_iter {
                    Color::new(0.0, 0.0, 0.0, 1.0) // Black for points in the set
                } else {
                    Color::new(1.0, 1.0, 1.0, 1.0) // White for points outside
                }
            }
        }
    }
}

/// Helper function for Sierpinski triangle drawing
/// Recursively draws a Sierpinski triangle
fn draw_sierpinski(bitmap: &mut Bitmap, x: u32, y: u32, size: u32, depth: u32, color: Color) {
    if depth == 0 || size <= 1 {
        return;
    }
    
    let half_size = size / 2;
    
    // Draw the bottom-left triangle
    draw_triangle(bitmap, x, y, x + half_size, y, x + half_size / 2, y - half_size, color);
    
    // Recursive calls for the three smaller triangles
    draw_sierpinski(bitmap, x, y, half_size, depth - 1, color);
    draw_sierpinski(bitmap, x + half_size, y, half_size, depth - 1, color);
    draw_sierpinski(bitmap, x + half_size / 2, y - half_size, half_size, depth - 1, color);
}

/// Draw a triangle by connecting three points
fn draw_triangle(bitmap: &mut Bitmap, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, color: Color) {
    // Simple triangle rasterization
    // For simplicity, we'll just draw the outline
    draw_line(bitmap, x1, y1, x2, y2, color);
    draw_line(bitmap, x2, y2, x3, y3, color);
    draw_line(bitmap, x3, y3, x1, y1, color);
}

/// Draw a line between two points using Bresenham's algorithm
fn draw_line(bitmap: &mut Bitmap, mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, color: Color) {
    let steep = (y1 as i32 - y0 as i32).abs() > (x1 as i32 - x0 as i32).abs();
    
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    
    let dx = x1 - x0;
    let dy = (y1 as i32 - y0 as i32).abs() as u32;
    let mut err = dx / 2;
    let mut y = y0;
    let ystep = if y0 < y1 { 1 } else { -1i32 as u32 };
    
    for x in x0..=x1 {
        if steep {
            bitmap.set_pixel(y, x, color);
        } else {
            bitmap.set_pixel(x, y, color);
        }
        
        err = err - dy;
        if err as i32 <= 0 {
            y = ((y as i32) + ystep as i32) as u32;
            err += dx;
        }
    }
}

/// Generate a Mandelbrot set bitmap with default parameters
pub fn mandelbrot(width: u32, height: u32, center_x: f32, center_y: f32, zoom: f32) -> Bitmap {
    FractalGenerator::new()
        .generate(width, height, FractalType::Mandelbrot, center_x, center_y, zoom)
}

/// Generate a Julia set bitmap with default parameters
pub fn julia(width: u32, height: u32, center_x: f32, center_y: f32, zoom: f32, c_real: f32, c_imag: f32) -> Bitmap {
    FractalGenerator::new()
        .with_julia_c(c_real, c_imag)
        .generate(width, height, FractalType::Julia, center_x, center_y, zoom)
} 