// Pattern generation module
//
// Provides functions for generating common texture patterns

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;

/// Direction for gradients
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    /// Left to right
    Horizontal,
    /// Top to bottom
    Vertical,
    /// Center to edges
    Radial,
    /// Corner to corner
    Diagonal,
}

/// Create a gradient bitmap with the specified colors and direction
pub fn gradient(
    width: u32,
    height: u32,
    start_color: Color,
    end_color: Color,
    direction: GradientDirection,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    let interpolate = |c1: Color, c2: Color, t: f32| -> Color {
        let t = t.max(0.0).min(1.0);
        let r = c1.r * (1.0 - t) + c2.r * t;
        let g = c1.g * (1.0 - t) + c2.g * t;
        let b = c1.b * (1.0 - t) + c2.b * t;
        let a = c1.a * (1.0 - t) + c2.a * t;
        Color::new(r, g, b, a)
    };
    
    for y in 0..height {
        for x in 0..width {
            let t = match direction {
                GradientDirection::Horizontal => x as f32 / (width - 1) as f32,
                GradientDirection::Vertical => y as f32 / (height - 1) as f32,
                GradientDirection::Diagonal => {
                    (x as f32 / (width - 1) as f32 + y as f32 / (height - 1) as f32) * 0.5
                },
                GradientDirection::Radial => {
                    let center_x = width as f32 / 2.0;
                    let center_y = height as f32 / 2.0;
                    let dx = (x as f32 - center_x) / center_x;
                    let dy = (y as f32 - center_y) / center_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    dist.min(1.0)
                },
            };
            
            let color = interpolate(start_color, end_color, t);
            bitmap.set_pixel(x, y, color);
        }
    }
    
    bitmap
}

/// Create a multi-color gradient bitmap
pub fn multi_gradient(
    width: u32,
    height: u32,
    colors: &[Color],
    direction: GradientDirection,
) -> Bitmap {
    if colors.is_empty() {
        return Bitmap::new(width, height); // Empty bitmap
    }
    
    if colors.len() == 1 {
        let mut bitmap = Bitmap::new(width, height);
        bitmap.fill(colors[0]);
        return bitmap;
    }
    
    let mut bitmap = Bitmap::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let t = match direction {
                GradientDirection::Horizontal => x as f32 / (width - 1) as f32,
                GradientDirection::Vertical => y as f32 / (height - 1) as f32,
                GradientDirection::Diagonal => {
                    (x as f32 / (width - 1) as f32 + y as f32 / (height - 1) as f32) * 0.5
                },
                GradientDirection::Radial => {
                    let center_x = width as f32 / 2.0;
                    let center_y = height as f32 / 2.0;
                    let dx = (x as f32 - center_x) / center_x;
                    let dy = (y as f32 - center_y) / center_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    dist.min(1.0)
                },
            };
            
            // Scale to the number of color segments
            let scaled_t = t * (colors.len() - 1) as f32;
            let index = scaled_t.floor() as usize;
            let index_t = scaled_t - index as f32;
            
            // Get the two colors to interpolate between
            let color1 = colors[index];
            let color2 = colors[index.min(colors.len() - 1) + 1.min(colors.len() - index - 1)];
            
            // Interpolate between the two colors
            let r = color1.r * (1.0 - index_t) + color2.r * index_t;
            let g = color1.g * (1.0 - index_t) + color2.g * index_t;
            let b = color1.b * (1.0 - index_t) + color2.b * index_t;
            let a = color1.a * (1.0 - index_t) + color2.a * index_t;
            
            bitmap.set_pixel(x, y, Color::new(r, g, b, a));
        }
    }
    
    bitmap
}

/// Create a checkerboard pattern bitmap
pub fn checkerboard(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    cell_size: u32,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    let cell_size = cell_size.max(1);
    
    for y in 0..height {
        for x in 0..width {
            let cell_x = x / cell_size;
            let cell_y = y / cell_size;
            
            let color = if (cell_x + cell_y) % 2 == 0 {
                color1
            } else {
                color2
            };
            
            bitmap.set_pixel(x, y, color);
        }
    }
    
    bitmap
}

/// Create a grid pattern bitmap
pub fn grid(
    width: u32,
    height: u32,
    background_color: Color,
    line_color: Color,
    cell_size: u32,
    line_width: u32,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    // Fill background
    bitmap.fill(background_color);
    
    // Ensure at least 1px cell size and line width
    let cell_size = cell_size.max(1);
    let line_width = line_width.max(1).min(cell_size / 2);
    
    for y in 0..height {
        for x in 0..width {
            // Check if we're on a grid line
            let on_h_line = y % cell_size < line_width || y % cell_size >= cell_size - line_width;
            let on_v_line = x % cell_size < line_width || x % cell_size >= cell_size - line_width;
            
            if on_h_line || on_v_line {
                bitmap.set_pixel(x, y, line_color);
            }
        }
    }
    
    bitmap
}

/// Create a dot pattern bitmap
pub fn dots(
    width: u32,
    height: u32,
    background_color: Color,
    dot_color: Color,
    cell_size: u32,
    dot_radius: u32,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    // Fill background
    bitmap.fill(background_color);
    
    // Ensure at least 1px cell size
    let cell_size = cell_size.max(1);
    let dot_radius = dot_radius.max(1).min(cell_size / 2);
    
    for cy in 0..(height / cell_size + 1) {
        for cx in 0..(width / cell_size + 1) {
            let center_x = cx * cell_size + cell_size / 2;
            let center_y = cy * cell_size + cell_size / 2;
            
            // Skip if the center is outside the bitmap
            if center_x >= width || center_y >= height {
                continue;
            }
            
            // Draw the dot
            for y in center_y.saturating_sub(dot_radius)..=(center_y + dot_radius).min(height - 1) {
                for x in center_x.saturating_sub(dot_radius)..=(center_x + dot_radius).min(width - 1) {
                    let dx = (x as i32 - center_x as i32).abs() as u32;
                    let dy = (y as i32 - center_y as i32).abs() as u32;
                    let dist_squared = dx * dx + dy * dy;
                    
                    if dist_squared <= dot_radius * dot_radius {
                        bitmap.set_pixel(x, y, dot_color);
                    }
                }
            }
        }
    }
    
    bitmap
}

/// Create a striped pattern bitmap
pub fn stripes(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    stripe_width: u32,
    angle_degrees: f32,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    // Ensure at least 1px stripe width
    let stripe_width = stripe_width.max(1);
    
    // Convert angle to radians
    let angle_rad = angle_degrees * std::f32::consts::PI / 180.0;
    let (sin, cos) = angle_rad.sin_cos();
    
    for y in 0..height {
        for x in 0..width {
            // Project the point onto the stripe direction vector
            let projected = (x as f32 * cos + y as f32 * sin) as i32;
            
            // Determine the stripe index
            let stripe_index = (projected / stripe_width as i32).abs();
            
            let color = if stripe_index % 2 == 0 {
                color1
            } else {
                color2
            };
            
            bitmap.set_pixel(x, y, color);
        }
    }
    
    bitmap
}

/// Create a radial stripe pattern bitmap (like a sunburst)
pub fn radial_stripes(
    width: u32,
    height: u32,
    color1: Color,
    color2: Color,
    num_stripes: u32,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    // Ensure at least 2 stripes
    let num_stripes = num_stripes.max(2);
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    
    for y in 0..height {
        for x in 0..width {
            // Calculate angle from center
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let angle = dy.atan2(dx);
            
            // Normalize angle to 0-1 range
            let normalized_angle = (angle / (2.0 * std::f32::consts::PI) + 0.5) % 1.0;
            
            // Determine the stripe index
            let stripe_index = (normalized_angle * num_stripes as f32) as u32;
            
            let color = if stripe_index % 2 == 0 {
                color1
            } else {
                color2
            };
            
            bitmap.set_pixel(x, y, color);
        }
    }
    
    bitmap
}

/// Create a repeating pattern from a smaller bitmap
pub fn repeat_pattern(
    width: u32,
    height: u32,
    pattern: &Bitmap,
    tile_mode: TileMode,
) -> Bitmap {
    let mut bitmap = Bitmap::new(width, height);
    
    let pattern_width = pattern.width();
    let pattern_height = pattern.height();
    
    // Don't attempt to use an empty pattern
    if pattern_width == 0 || pattern_height == 0 {
        return bitmap;
    }
    
    for y in 0..height {
        for x in 0..width {
            // Calculate source coordinates
            let (src_x, src_y) = match tile_mode {
                TileMode::Repeat => {
                    (x % pattern_width, y % pattern_height)
                },
                TileMode::MirrorX => {
                    let cell_x = x / pattern_width;
                    let offset_x = x % pattern_width;
                    let mirrored_x = if cell_x % 2 == 0 {
                        offset_x
                    } else {
                        pattern_width - 1 - offset_x
                    };
                    (mirrored_x, y % pattern_height)
                },
                TileMode::MirrorY => {
                    let cell_y = y / pattern_height;
                    let offset_y = y % pattern_height;
                    let mirrored_y = if cell_y % 2 == 0 {
                        offset_y
                    } else {
                        pattern_height - 1 - offset_y
                    };
                    (x % pattern_width, mirrored_y)
                },
                TileMode::MirrorXY => {
                    let cell_x = x / pattern_width;
                    let cell_y = y / pattern_height;
                    let offset_x = x % pattern_width;
                    let offset_y = y % pattern_height;
                    let mirrored_x = if cell_x % 2 == 0 {
                        offset_x
                    } else {
                        pattern_width - 1 - offset_x
                    };
                    let mirrored_y = if cell_y % 2 == 0 {
                        offset_y
                    } else {
                        pattern_height - 1 - offset_y
                    };
                    (mirrored_x, mirrored_y)
                },
            };
            
            if let Some(color) = pattern.get_pixel(src_x, src_y) {
                bitmap.set_pixel(x, y, color);
            }
        }
    }
    
    bitmap
}

/// Tiling modes for repeating patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileMode {
    /// Simple repeat the pattern
    Repeat,
    /// Mirror the pattern horizontally
    MirrorX,
    /// Mirror the pattern vertically
    MirrorY,
    /// Mirror the pattern both horizontally and vertically
    MirrorXY,
} 