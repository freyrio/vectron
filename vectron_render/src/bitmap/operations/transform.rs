// Bitmap transform operations
//
// Provides operations for resizing, rotating, and otherwise transforming bitmaps

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;

/// Resize a bitmap to new dimensions with optional filtering
pub fn resize(
    src: &Bitmap,
    new_width: u32,
    new_height: u32,
    filter: ResizeFilter,
) -> Bitmap {
    // If the size is the same, just clone
    if src.width() == new_width && src.height() == new_height {
        return src.clone();
    }
    
    let mut dst = Bitmap::new(new_width, new_height);
    
    match filter {
        ResizeFilter::Nearest => resize_nearest(src, &mut dst),
        ResizeFilter::Bilinear => resize_bilinear(src, &mut dst),
        ResizeFilter::Bicubic => resize_bicubic(src, &mut dst),
    }
    
    dst
}

/// Filter modes for bitmap resizing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeFilter {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Bilinear filtering (good balance of speed and quality)
    Bilinear,
    /// Bicubic filtering (slower, high quality)
    Bicubic,
}

/// Resize a bitmap using nearest neighbor sampling (fast but low quality)
fn resize_nearest(src: &Bitmap, dst: &mut Bitmap) {
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();
    let dst_height = dst.height();
    
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;
    
    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = (x as f32 * x_ratio).floor() as u32;
            let src_y = (y as f32 * y_ratio).floor() as u32;
            
            if let Some(color) = src.get_pixel(src_x, src_y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
}

/// Resize a bitmap using bilinear filtering (good balance of speed and quality)
fn resize_bilinear(src: &Bitmap, dst: &mut Bitmap) {
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();
    let dst_height = dst.height();
    
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;
    
    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = x as f32 * x_ratio;
            let src_y = y as f32 * y_ratio;
            
            let src_x_floor = src_x.floor();
            let src_y_floor = src_y.floor();
            
            let x1 = src_x_floor as u32;
            let y1 = src_y_floor as u32;
            let x2 = (x1 + 1).min(src_width - 1);
            let y2 = (y1 + 1).min(src_height - 1);
            
            let x_weight = src_x - src_x_floor;
            let y_weight = src_y - src_y_floor;
            
            // Get the four surrounding pixels
            let top_left = src.get_pixel(x1, y1).unwrap_or(Color::TRANSPARENT);
            let top_right = src.get_pixel(x2, y1).unwrap_or(Color::TRANSPARENT);
            let bottom_left = src.get_pixel(x1, y2).unwrap_or(Color::TRANSPARENT);
            let bottom_right = src.get_pixel(x2, y2).unwrap_or(Color::TRANSPARENT);
            
            // Calculate bilinear weighted sum
            let r = bilinear_weight(
                top_left.r, top_right.r,
                bottom_left.r, bottom_right.r,
                x_weight, y_weight
            );
            
            let g = bilinear_weight(
                top_left.g, top_right.g,
                bottom_left.g, bottom_right.g,
                x_weight, y_weight
            );
            
            let b = bilinear_weight(
                top_left.b, top_right.b,
                bottom_left.b, bottom_right.b,
                x_weight, y_weight
            );
            
            let a = bilinear_weight(
                top_left.a, top_right.a,
                bottom_left.a, bottom_right.a,
                x_weight, y_weight
            );
            
            dst.set_pixel(x, y, Color::new(r, g, b, a));
        }
    }
}

/// Helper function for bilinear weight calculation
fn bilinear_weight(
    top_left: f32, top_right: f32,
    bottom_left: f32, bottom_right: f32,
    x_weight: f32, y_weight: f32
) -> f32 {
    let top = top_left * (1.0 - x_weight) + top_right * x_weight;
    let bottom = bottom_left * (1.0 - x_weight) + bottom_right * x_weight;
    top * (1.0 - y_weight) + bottom * y_weight
}

/// Resize a bitmap using bicubic filtering (higher quality but slower)
fn resize_bicubic(src: &Bitmap, dst: &mut Bitmap) {
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();
    let dst_height = dst.height();
    
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;
    
    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = x as f32 * x_ratio;
            let src_y = y as f32 * y_ratio;
            
            let x0 = src_x.floor() as i32 - 1;
            let y0 = src_y.floor() as i32 - 1;
            
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut a_sum = 0.0;
            let mut weight_sum = 0.0;
            
            // Sample 4x4 grid of pixels
            for cy in 0..4 {
                for cx in 0..4 {
                    let sample_x = (x0 + cx) as u32;
                    let sample_y = (y0 + cy) as u32;
                    
                    // Skip if outside source bounds
                    if sample_x >= src_width || sample_y >= src_height {
                        continue;
                    }
                    
                    let color = src.get_pixel(sample_x, sample_y).unwrap_or(Color::TRANSPARENT);
                    
                    let dx = src_x - sample_x as f32;
                    let dy = src_y - sample_y as f32;
                    
                    // Bicubic kernel weight
                    let weight = bicubic_weight(dx) * bicubic_weight(dy);
                    
                    r_sum += color.r * weight;
                    g_sum += color.g * weight;
                    b_sum += color.b * weight;
                    a_sum += color.a * weight;
                    weight_sum += weight;
                }
            }
            
            // Normalize by total weight
            if weight_sum > 0.0 {
                r_sum /= weight_sum;
                g_sum /= weight_sum;
                b_sum /= weight_sum;
                a_sum /= weight_sum;
            }
            
            // Clamp values to valid range
            let r = r_sum.max(0.0).min(1.0);
            let g = g_sum.max(0.0).min(1.0);
            let b = b_sum.max(0.0).min(1.0);
            let a = a_sum.max(0.0).min(1.0);
            
            dst.set_pixel(x, y, Color::new(r, g, b, a));
        }
    }
}

/// Cubic interpolation weight function
fn bicubic_weight(x: f32) -> f32 {
    let x = x.abs();
    let a = -0.5; // Commonly used value for bicubic interpolation
    
    if x < 1.0 {
        return ((a + 2.0) * x * x * x) - ((a + 3.0) * x * x) + 1.0;
    } else if x < 2.0 {
        return (a * x * x * x) - (5.0 * a * x * x) + (8.0 * a * x) - (4.0 * a);
    } else {
        return 0.0;
    }
}

/// Rotate a bitmap by the specified angle in degrees with optional filtering
pub fn rotate(
    src: &Bitmap,
    angle_degrees: f32,
    filter: ResizeFilter,
) -> Bitmap {
    // Normalize angle to 0-360 degrees
    let angle = angle_degrees % 360.0;
    
    // Fast path for common rotations
    if angle.abs() < 0.001 {
        return src.clone(); // No rotation
    } else if (angle - 90.0).abs() < 0.001 {
        return rotate_90(src);
    } else if (angle - 180.0).abs() < 0.001 {
        return rotate_180(src);
    } else if (angle - 270.0).abs() < 0.001 {
        return rotate_270(src);
    }
    
    // Convert angle to radians
    let angle_rad = angle.to_radians();
    
    // Calculate the size of the rotated image
    let width = src.width();
    let height = src.height();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();
    
    // Calculate bounds of rotated image
    let x1 = (-(height as f32) * sin_angle).abs();
    let x2 = (width as f32 * cos_angle).abs();
    let x3 = (width as f32 * cos_angle - height as f32 * sin_angle).abs();
    let x4 = (width as f32 * cos_angle + 0.0 * sin_angle).abs();
    
    let y1 = (height as f32 * cos_angle).abs();
    let y2 = (width as f32 * sin_angle).abs();
    let y3 = (width as f32 * sin_angle + height as f32 * cos_angle).abs();
    let y4 = (0.0 * cos_angle + height as f32 * cos_angle).abs();
    
    let new_width = x1.max(x2).max(x3).max(x4).ceil() as u32;
    let new_height = y1.max(y2).max(y3).max(y4).ceil() as u32;
    
    let mut dst = Bitmap::new(new_width, new_height);
    
    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;
    let half_new_width = new_width as f32 / 2.0;
    let half_new_height = new_height as f32 / 2.0;
    
    // Rotation matrix transformation
    for y in 0..new_height {
        for x in 0..new_width {
            // Translate to center
            let dx = x as f32 - half_new_width;
            let dy = y as f32 - half_new_height;
            
            // Rotate using rotation matrix
            let src_x = dx * cos_angle + dy * sin_angle + half_width;
            let src_y = -dx * sin_angle + dy * cos_angle + half_height;
            
            // Check if coordinates are within source bounds
            if src_x < 0.0 || src_x >= width as f32 || src_y < 0.0 || src_y >= height as f32 {
                continue;
            }
            
            // Sample using the selected filter
            let color = match filter {
                ResizeFilter::Nearest => {
                    let sx = src_x.round() as u32;
                    let sy = src_y.round() as u32;
                    src.get_pixel(sx, sy).unwrap_or(Color::TRANSPARENT)
                },
                ResizeFilter::Bilinear => {
                    sample_bilinear(src, src_x, src_y)
                },
                ResizeFilter::Bicubic => {
                    sample_bicubic(src, src_x, src_y)
                },
            };
            
            dst.set_pixel(x, y, color);
        }
    }
    
    dst
}

/// Sample a bitmap using bilinear interpolation
fn sample_bilinear(src: &Bitmap, x: f32, y: f32) -> Color {
    let x1 = x.floor() as u32;
    let y1 = y.floor() as u32;
    let x2 = (x1 + 1).min(src.width() - 1);
    let y2 = (y1 + 1).min(src.height() - 1);
    
    let x_weight = x - x.floor();
    let y_weight = y - y.floor();
    
    // Get the four surrounding pixels
    let top_left = src.get_pixel(x1, y1).unwrap_or(Color::TRANSPARENT);
    let top_right = src.get_pixel(x2, y1).unwrap_or(Color::TRANSPARENT);
    let bottom_left = src.get_pixel(x1, y2).unwrap_or(Color::TRANSPARENT);
    let bottom_right = src.get_pixel(x2, y2).unwrap_or(Color::TRANSPARENT);
    
    // Calculate bilinear weighted sum
    let r = bilinear_weight(
        top_left.r, top_right.r,
        bottom_left.r, bottom_right.r,
        x_weight, y_weight
    );
    
    let g = bilinear_weight(
        top_left.g, top_right.g,
        bottom_left.g, bottom_right.g,
        x_weight, y_weight
    );
    
    let b = bilinear_weight(
        top_left.b, top_right.b,
        bottom_left.b, bottom_right.b,
        x_weight, y_weight
    );
    
    let a = bilinear_weight(
        top_left.a, top_right.a,
        bottom_left.a, bottom_right.a,
        x_weight, y_weight
    );
    
    Color::new(r, g, b, a)
}

/// Sample a bitmap using bicubic interpolation
fn sample_bicubic(src: &Bitmap, x: f32, y: f32) -> Color {
    let x0 = x.floor() as i32 - 1;
    let y0 = y.floor() as i32 - 1;
    
    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    let mut a_sum = 0.0;
    let mut weight_sum = 0.0;
    
    // Sample 4x4 grid of pixels
    for cy in 0..4 {
        for cx in 0..4 {
            let sample_x = (x0 + cx) as u32;
            let sample_y = (y0 + cy) as u32;
            
            // Skip if outside source bounds
            if sample_x >= src.width() || sample_y >= src.height() {
                continue;
            }
            
            let color = src.get_pixel(sample_x, sample_y).unwrap_or(Color::TRANSPARENT);
            
            let dx = x - sample_x as f32;
            let dy = y - sample_y as f32;
            
            // Bicubic kernel weight
            let weight = bicubic_weight(dx) * bicubic_weight(dy);
            
            r_sum += color.r * weight;
            g_sum += color.g * weight;
            b_sum += color.b * weight;
            a_sum += color.a * weight;
            weight_sum += weight;
        }
    }
    
    // Normalize by total weight
    if weight_sum > 0.0 {
        r_sum /= weight_sum;
        g_sum /= weight_sum;
        b_sum /= weight_sum;
        a_sum /= weight_sum;
    }
    
    // Clamp values to valid range
    let r = r_sum.max(0.0).min(1.0);
    let g = g_sum.max(0.0).min(1.0);
    let b = b_sum.max(0.0).min(1.0);
    let a = a_sum.max(0.0).min(1.0);
    
    Color::new(r, g, b, a)
}

/// Rotate a bitmap 90 degrees clockwise
fn rotate_90(src: &Bitmap) -> Bitmap {
    let width = src.height();
    let height = src.width();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let src_x = y;
            let src_y = width - 1 - x;
            if let Some(color) = src.get_pixel(src_x, src_y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
    
    dst
}

/// Rotate a bitmap 180 degrees
fn rotate_180(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let src_x = width - 1 - x;
            let src_y = height - 1 - y;
            if let Some(color) = src.get_pixel(src_x, src_y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
    
    dst
}

/// Rotate a bitmap 270 degrees clockwise (90 degrees counter-clockwise)
fn rotate_270(src: &Bitmap) -> Bitmap {
    let width = src.height();
    let height = src.width();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let src_x = height - 1 - y;
            let src_y = x;
            if let Some(color) = src.get_pixel(src_x, src_y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
    
    dst
}

/// Flip a bitmap horizontally (left to right)
pub fn flip_horizontal(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let src_x = width - 1 - x;
            if let Some(color) = src.get_pixel(src_x, y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
    
    dst
}

/// Flip a bitmap vertically (top to bottom)
pub fn flip_vertical(src: &Bitmap) -> Bitmap {
    let width = src.width();
    let height = src.height();
    let mut dst = Bitmap::new(width, height);
    
    for y in 0..height {
        let src_y = height - 1 - y;
        for x in 0..width {
            if let Some(color) = src.get_pixel(x, src_y) {
                dst.set_pixel(x, y, color);
            }
        }
    }
    
    dst
}

/// Crop a bitmap to the specified region
pub fn crop(src: &Bitmap, x: u32, y: u32, width: u32, height: u32) -> Option<Bitmap> {
    src.crop(x, y, width, height)
} 