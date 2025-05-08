// Basic bitmap operations
//
// Provides fundamental bitmap manipulation operations like
// setting/getting pixels, filling regions, and copying

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;

/// Fill an entire bitmap with a specific color
pub fn fill(bitmap: &mut Bitmap, color: Color) {
    bitmap.fill(color);
}

/// Clear a bitmap (fill with transparent color)
pub fn clear(bitmap: &mut Bitmap) {
    bitmap.fill(Color::TRANSPARENT);
}

/// Fill a rectangular region of the bitmap with a specific color
pub fn fill_rect(bitmap: &mut Bitmap, x: u32, y: u32, width: u32, height: u32, color: Color) {
    bitmap.fill_rect(x, y, width, height, color);
}

/// Copy a rectangular region from source to destination bitmap
pub fn copy_rect(
    src: &Bitmap,
    dst: &mut Bitmap,
    src_x: u32,
    src_y: u32,
    dst_x: u32,
    dst_y: u32,
    width: u32,
    height: u32,
) -> bool {
    // Validate source bounds
    if src_x + width > src.width() || src_y + height > src.height() {
        return false;
    }
    
    // Validate destination bounds
    if dst_x + width > dst.width() || dst_y + height > dst.height() {
        return false;
    }
    
    // Copy pixel by pixel
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = src.get_pixel(src_x + x, src_y + y) {
                dst.set_pixel(dst_x + x, dst_y + y, color);
            }
        }
    }
    
    true
}

/// Draw a single pixel line between two points using Bresenham's algorithm
pub fn draw_line(bitmap: &mut Bitmap, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    
    let mut x = x0;
    let mut y = y0;
    
    loop {
        if x >= 0 && y >= 0 && x < bitmap.width() as i32 && y < bitmap.height() as i32 {
            bitmap.set_pixel(x as u32, y as u32, color);
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            if x == x1 {
                break;
            }
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 {
                break;
            }
            err += dx;
            y += sy;
        }
    }
}

/// Draw an empty rectangle (outline only)
pub fn draw_rect(bitmap: &mut Bitmap, x: u32, y: u32, width: u32, height: u32, color: Color) {
    let x1 = (x + width).saturating_sub(1);
    let y1 = (y + height).saturating_sub(1);
    
    // Draw horizontal lines
    for cx in x..=x1 {
        bitmap.set_pixel(cx, y, color);
        bitmap.set_pixel(cx, y1, color);
    }
    
    // Draw vertical lines
    for cy in y+1..y1 {
        bitmap.set_pixel(x, cy, color);
        bitmap.set_pixel(x1, cy, color);
    }
}

/// Draw a filled circle using the midpoint circle algorithm
pub fn fill_circle(bitmap: &mut Bitmap, center_x: u32, center_y: u32, radius: u32, color: Color) {
    let cx = center_x as i32;
    let cy = center_y as i32;
    let r = radius as i32;
    
    let mut x = 0;
    let mut y = r;
    let mut d = 1 - r;
    
    // Helper to draw horizontal line in the circle
    let draw_horizontal_line = |bitmap: &mut Bitmap, x0: i32, x1: i32, y: i32, color: Color| {
        let start_x = x0.max(0) as u32;
        let end_x = x1.min(bitmap.width() as i32 - 1) as u32;
        let y = y.max(0).min(bitmap.height() as i32 - 1) as u32;
        
        for x in start_x..=end_x {
            bitmap.set_pixel(x, y, color);
        }
    };
    
    while y >= x {
        // Draw horizontal lines for each octant
        draw_horizontal_line(bitmap, cx - y, cx + y, cy + x, color);
        draw_horizontal_line(bitmap, cx - y, cx + y, cy - x, color);
        
        if x > 0 {
            draw_horizontal_line(bitmap, cx - x, cx + x, cy + y, color);
            draw_horizontal_line(bitmap, cx - x, cx + x, cy - y, color);
        }
        
        // Update midpoint circle values
        if d < 0 {
            d += 2 * x + 3;
        } else {
            d += 2 * (x - y) + 5;
            y -= 1;
        }
        x += 1;
    }
}

/// Draw the outline of a circle using the midpoint circle algorithm
pub fn draw_circle(bitmap: &mut Bitmap, center_x: u32, center_y: u32, radius: u32, color: Color) {
    let cx = center_x as i32;
    let cy = center_y as i32;
    let r = radius as i32;
    
    let mut x = 0;
    let mut y = r;
    let mut d = 1 - r;
    
    // Helper to plot a point if it's within bitmap bounds
    let plot = |bitmap: &mut Bitmap, x: i32, y: i32, color: Color| {
        if x >= 0 && y >= 0 && x < bitmap.width() as i32 && y < bitmap.height() as i32 {
            bitmap.set_pixel(x as u32, y as u32, color);
        }
    };
    
    while y >= x {
        // Plot points in all octants
        plot(bitmap, cx + x, cy + y, color);
        plot(bitmap, cx - x, cy + y, color);
        plot(bitmap, cx + x, cy - y, color);
        plot(bitmap, cx - x, cy - y, color);
        plot(bitmap, cx + y, cy + x, color);
        plot(bitmap, cx - y, cy + x, color);
        plot(bitmap, cx + y, cy - x, color);
        plot(bitmap, cx - y, cy - x, color);
        
        // Update midpoint circle values
        if d < 0 {
            d += 2 * x + 3;
        } else {
            d += 2 * (x - y) + 5;
            y -= 1;
        }
        x += 1;
    }
}

/// Draw a quadratic Bezier curve
pub fn draw_quadratic_bezier(
    bitmap: &mut Bitmap,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: Color,
    steps: u32,
) {
    let steps = steps.max(10); // Ensure minimum number of steps
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let t_inv = 1.0 - t;
        
        let x = (t_inv * t_inv * x0 as f32 + 2.0 * t_inv * t * x1 as f32 + t * t * x2 as f32) as i32;
        let y = (t_inv * t_inv * y0 as f32 + 2.0 * t_inv * t * y1 as f32 + t * t * y2 as f32) as i32;
        
        if x >= 0 && y >= 0 && x < bitmap.width() as i32 && y < bitmap.height() as i32 {
            bitmap.set_pixel(x as u32, y as u32, color);
        }
    }
}

/// Draw a cubic Bezier curve
pub fn draw_cubic_bezier(
    bitmap: &mut Bitmap,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: Color,
    steps: u32,
) {
    let steps = steps.max(20); // Ensure minimum number of steps
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let t_inv = 1.0 - t;
        
        let x = (t_inv * t_inv * t_inv * x0 as f32 +
                3.0 * t_inv * t_inv * t * x1 as f32 +
                3.0 * t_inv * t * t * x2 as f32 +
                t * t * t * x3 as f32) as i32;
                
        let y = (t_inv * t_inv * t_inv * y0 as f32 +
                3.0 * t_inv * t_inv * t * y1 as f32 +
                3.0 * t_inv * t * t * y2 as f32 +
                t * t * t * y3 as f32) as i32;
        
        if x >= 0 && y >= 0 && x < bitmap.width() as i32 && y < bitmap.height() as i32 {
            bitmap.set_pixel(x as u32, y as u32, color);
        }
    }
}

/// Fill a closed polygon defined by a list of points
pub fn fill_polygon(bitmap: &mut Bitmap, points: &[(i32, i32)], color: Color) {
    if points.len() < 3 {
        return; // Need at least 3 points for a polygon
    }
    
    // Find the bounds of the polygon
    let mut min_y = bitmap.height() as i32;
    let mut max_y = 0;
    
    for (_, y) in points {
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    
    // Clamp to bitmap boundaries
    min_y = min_y.max(0);
    max_y = max_y.min(bitmap.height() as i32 - 1);
    
    // Scan each row and find intersections
    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        
        // Find intersections with all edges
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            
            // Check if the edge crosses the current scanline
            if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                // Calculate x-coordinate of intersection
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }
        
        // Sort intersections by x-coordinate
        intersections.sort_unstable();
        
        // Fill between pairs of intersections
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let start_x = intersections[i].max(0);
                let end_x = intersections[i + 1].min(bitmap.width() as i32 - 1);
                
                // Draw horizontal line
                for x in start_x..=end_x {
                    bitmap.set_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
} 