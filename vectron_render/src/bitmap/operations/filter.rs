// Bitmap filter operations
//
// Provides various filters and adjustments for bitmap manipulation

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;
use std::f32::consts::PI;

/// Apply a Gaussian blur filter to the bitmap
pub fn blur(src: &Bitmap, radius: f32) -> Bitmap {
    if radius <= 0.0 {
        return src.clone();
    }
    
    // Calculate kernel size based on radius (odd number)
    let kernel_size = (radius * 2.0 + 1.0).ceil() as usize;
    if kernel_size <= 1 {
        return src.clone();
    }
    
    // Create Gaussian kernel
    let sigma = radius / 3.0; // Standard deviation
    let kernel = create_gaussian_kernel(kernel_size, sigma);
    
    // Apply separable convolution for efficiency (horizontal then vertical)
    let horizontal = apply_horizontal_kernel(src, &kernel);
    let result = apply_vertical_kernel(&horizontal, &kernel);
    
    result
}

/// Create a Gaussian kernel for filtering
fn create_gaussian_kernel(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = vec![0.0; size];
    let center = size as f32 / 2.0;
    let factor = 1.0 / (2.0 * PI * sigma * sigma).sqrt();
    
    let mut sum = 0.0;
    
    for i in 0..size {
        let x = i as f32 - center;
        let value = factor * (-x * x / (2.0 * sigma * sigma)).exp();
        kernel[i] = value;
        sum += value;
    }
    
    // Normalize kernel so it sums to 1.0
    for i in 0..size {
        kernel[i] /= sum;
    }
    
    kernel
}

/// Apply a 1D kernel horizontally to a bitmap
fn apply_horizontal_kernel(src: &Bitmap, kernel: &[f32]) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    let radius = kernel.len() / 2;
    
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut a_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for (ki, k) in kernel.iter().enumerate() {
                let sample_x = x as i32 + ki as i32 - radius as i32;
                
                // Skip if outside image bounds
                if sample_x < 0 || sample_x >= width as i32 {
                    continue;
                }
                
                let color = src.get_pixel(sample_x as u32, y).unwrap_or(Color::TRANSPARENT);
                let weight = *k;
                
                r_sum += color.r * weight;
                g_sum += color.g * weight;
                b_sum += color.b * weight;
                a_sum += color.a * weight;
                weight_sum += weight;
            }
            
            // Normalize by weight sum (if non-zero)
            if weight_sum > 0.0 {
                r_sum /= weight_sum;
                g_sum /= weight_sum;
                b_sum /= weight_sum;
                a_sum /= weight_sum;
            }
            
            dst.set_pixel(x, y, Color::new(r_sum, g_sum, b_sum, a_sum));
        }
    }
    
    dst
}

/// Apply a 1D kernel vertically to a bitmap
fn apply_vertical_kernel(src: &Bitmap, kernel: &[f32]) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    let radius = kernel.len() / 2;
    
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut a_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for (ki, k) in kernel.iter().enumerate() {
                let sample_y = y as i32 + ki as i32 - radius as i32;
                
                // Skip if outside image bounds
                if sample_y < 0 || sample_y >= height as i32 {
                    continue;
                }
                
                let color = src.get_pixel(x, sample_y as u32).unwrap_or(Color::TRANSPARENT);
                let weight = *k;
                
                r_sum += color.r * weight;
                g_sum += color.g * weight;
                b_sum += color.b * weight;
                a_sum += color.a * weight;
                weight_sum += weight;
            }
            
            // Normalize by weight sum (if non-zero)
            if weight_sum > 0.0 {
                r_sum /= weight_sum;
                g_sum /= weight_sum;
                b_sum /= weight_sum;
                a_sum /= weight_sum;
            }
            
            dst.set_pixel(x, y, Color::new(r_sum, g_sum, b_sum, a_sum));
        }
    }
    
    dst
}

/// Apply a sharpening filter to the bitmap
pub fn sharpen(src: &Bitmap, amount: f32) -> Bitmap {
    if amount <= 0.0 {
        return src.clone();
    }
    
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    // Create a simple sharpening kernel
    let kernel_size = 3;
    let center = kernel_size / 2;
    
    // Example: for amount=1.0, kernel is:
    // [0, -1, 0]
    // [-1, 5, -1]
    // [0, -1, 0]
    let mut kernel = vec![vec![0.0; kernel_size]; kernel_size];
    kernel[center][center] = 1.0 + 4.0 * amount;
    kernel[center - 1][center] = -amount;
    kernel[center + 1][center] = -amount;
    kernel[center][center - 1] = -amount;
    kernel[center][center + 1] = -amount;
    
    // Apply convolution
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut a_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let sample_x = x as i32 + kx as i32 - center as i32;
                    let sample_y = y as i32 + ky as i32 - center as i32;
                    
                    // Skip if outside image bounds
                    if sample_x < 0 || sample_x >= width as i32 || 
                       sample_y < 0 || sample_y >= height as i32 {
                        continue;
                    }
                    
                    let color = src.get_pixel(sample_x as u32, sample_y as u32)
                        .unwrap_or(Color::TRANSPARENT);
                    let weight = kernel[ky][kx];
                    
                    r_sum += color.r * weight;
                    g_sum += color.g * weight;
                    b_sum += color.b * weight;
                    a_sum += color.a;  // Don't sharpen alpha
                    weight_sum += weight.abs();
                }
            }
            
            // Clamp to valid range
            let r = f32::max(r_sum, 0.0).min(1.0);
            let g = f32::max(g_sum, 0.0).min(1.0);
            let b = f32::max(b_sum, 0.0).min(1.0);
            let a = f32::max(a_sum / weight_sum, 0.0).min(1.0);
            
            dst.set_pixel(x, y, Color::new(r, g, b, a));
        }
    }
    
    dst
}

/// Adjust the brightness of a bitmap
pub fn adjust_brightness(src: &Bitmap, amount: f32) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                let r = f32::max(color.r + amount, 0.0).min(1.0);
                let g = f32::max(color.g + amount, 0.0).min(1.0);
                let b = f32::max(color.b + amount, 0.0).min(1.0);
                
                dst.set_pixel(x, y, Color::new(r, g, b, color.a));
            }
        }
    }
    
    dst
}

/// Adjust the contrast of a bitmap
pub fn adjust_contrast(src: &Bitmap, amount: f32) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    // Calculate factor from amount
    let factor = amount + 1.0;
    let midpoint = 0.5;
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                // Apply contrast formula: (c - 0.5) * factor + 0.5
                let r = f32::max((color.r - midpoint) * factor + midpoint, 0.0).min(1.0);
                let g = f32::max((color.g - midpoint) * factor + midpoint, 0.0).min(1.0);
                let b = f32::max((color.b - midpoint) * factor + midpoint, 0.0).min(1.0);
                
                dst.set_pixel(x, y, Color::new(r, g, b, color.a));
            }
        }
    }
    
    dst
}

/// Convert a bitmap to grayscale
pub fn grayscale(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                // Use standard luminance formula
                let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
                dst.set_pixel(x, y, Color::new(gray, gray, gray, color.a));
            }
        }
    }
    
    dst
}

/// Invert the colors of a bitmap
pub fn invert(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                let r = 1.0 - color.r;
                let g = 1.0 - color.g;
                let b = 1.0 - color.b;
                dst.set_pixel(x, y, Color::new(r, g, b, color.a));
            }
        }
    }
    
    dst
}

/// Apply a sepia tone filter to the bitmap
pub fn sepia(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                // Convert to grayscale first
                let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
                
                // Apply sepia tone
                let r = f32::min(gray * 1.351, 1.0);
                let g = f32::min(gray * 1.203, 1.0);
                let b = f32::min(gray * 0.937, 1.0);
                
                dst.set_pixel(x, y, Color::new(r, g, b, color.a));
            }
        }
    }
    
    dst
}

/// Apply a threshold filter to convert to black and white
pub fn threshold(src: &Bitmap, threshold: f32) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    // Clamp threshold to valid range
    let threshold = threshold.max(0.0).min(1.0);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                // Convert to grayscale
                let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
                
                // Apply threshold
                let value = if gray >= threshold { 1.0 } else { 0.0 };
                
                dst.set_pixel(x, y, Color::new(value, value, value, color.a));
            }
        }
    }
    
    dst
}

/// Apply edge detection filter
pub fn edge_detect(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    // First convert to grayscale
    let grayscale_bitmap = grayscale(src);
    
    // Sobel operator kernels
    let sobel_x = [
        [-1.0, 0.0, 1.0],
        [-2.0, 0.0, 2.0],
        [-1.0, 0.0, 1.0]
    ];
    
    let sobel_y = [
        [-1.0, -2.0, -1.0],
        [0.0, 0.0, 0.0],
        [1.0, 2.0, 1.0]
    ];
    
    let kernel_size = 3;
    let radius = kernel_size / 2;
    
    for y in radius..height.saturating_sub(radius) {
        for x in radius..width.saturating_sub(radius) {
            let mut gx = 0.0;
            let mut gy = 0.0;
            
            // Apply both kernels
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let sample_x = x + kx as u32 - radius as u32;
                    let sample_y = y + ky as u32 - radius as u32;
                    
                    if let Some(color) = grayscale_bitmap.get_pixel(sample_x, sample_y) {
                        // Use red channel as grayscale value
                        let value = color.r;
                        gx += value * sobel_x[ky as usize][kx as usize];
                        gy += value * sobel_y[ky as usize][kx as usize];
                    }
                }
            }
            
            // Calculate magnitude
            let magnitude = f32::sqrt(gx * gx + gy * gy);
            
            // Normalize to 0-1 range (approximation)
            let edge_value = (magnitude / 4.0).min(1.0);
            
            // Get original alpha
            let alpha = src.get_pixel(x, y).map_or(1.0, |c| c.a);
            
            dst.set_pixel(x, y, Color::new(edge_value, edge_value, edge_value, alpha));
        }
    }
    
    dst
}

/// Apply a color matrix transformation to the bitmap
pub fn apply_color_matrix(src: &Bitmap, matrix: [[f32; 4]; 4]) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                let r = color.r;
                let g = color.g;
                let b = color.b;
                let a = color.a;
                
                // Apply matrix transformation
                let new_r = matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b + matrix[0][3] * a;
                let new_g = matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b + matrix[1][3] * a;
                let new_b = matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b + matrix[2][3] * a;
                let new_a = matrix[3][0] * r + matrix[3][1] * g + matrix[3][2] * b + matrix[3][3] * a;
                
                // Clamp to valid range
                let new_r = new_r.max(0.0).min(1.0);
                let new_g = new_g.max(0.0).min(1.0);
                let new_b = new_b.max(0.0).min(1.0);
                let new_a = new_a.max(0.0).min(1.0);
                
                dst.set_pixel(x, y, Color::new(new_r, new_g, new_b, new_a));
            }
        }
    }
    
    dst
}

/// Adjust the saturation of a bitmap
pub fn adjust_saturation(src: &Bitmap, amount: f32) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    // Calculate saturation factor (1.0 = normal, 0.0 = grayscale, >1.0 = increased saturation)
    let saturation = amount + 1.0;
    
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, y) {
                // Calculate luma using standard coefficients
                let luma = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
                
                // Interpolate between grayscale and original color
                let r = luma + saturation * (color.r - luma);
                let g = luma + saturation * (color.g - luma);
                let b = luma + saturation * (color.b - luma);
                
                // Clamp to valid range
                let r = r.max(0.0).min(1.0);
                let g = g.max(0.0).min(1.0);
                let b = b.max(0.0).min(1.0);
                
                dst.set_pixel(x, y, Color::new(r, g, b, color.a));
            }
        }
    }
    
    dst
} 