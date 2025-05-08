// Bitmap blending operations
//
// Provides various blending modes for combining bitmaps

use crate::bitmap::buffer::Bitmap;
use crate::color::Color;

/// Available blending modes for combining two bitmaps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Simply overwrites destination pixels with source pixels
    Copy,
    /// Alpha blending (standard transparency blending)
    Alpha,
    /// Add the colors together, clamping at 1.0
    Add,
    /// Multiply the colors together
    Multiply,
    /// Screen blending (1 - (1-a) * (1-b))
    Screen,
    /// Overlay blending (combines Multiply and Screen)
    Overlay,
    /// Darken (minimum of both colors)
    Darken,
    /// Lighten (maximum of both colors)
    Lighten,
    /// Color dodge (brightens destination color based on source)
    ColorDodge,
    /// Color burn (darkens destination color based on source)
    ColorBurn,
    /// Hard light (similar to Overlay, but using source to determine operation)
    HardLight,
    /// Soft light (gentler version of Hard Light)
    SoftLight,
    /// Difference (absolute difference between colors)
    Difference,
    /// Exclusion (similar to Difference but lower contrast)
    Exclusion,
}

/// Blend two bitmaps using the specified blend mode
pub fn blend(dst: &mut Bitmap, src: &Bitmap, mode: BlendMode) -> bool {
    blend_rect(dst, src, 0, 0, 0, 0, src.width(), src.height(), mode)
}

/// Blend a rectangular region from source to destination bitmap
pub fn blend_rect(
    dst: &mut Bitmap,
    src: &Bitmap,
    dst_x: u32,
    dst_y: u32,
    src_x: u32,
    src_y: u32,
    width: u32,
    height: u32,
    mode: BlendMode,
) -> bool {
    // Validate source bounds
    if src_x + width > src.width() || src_y + height > src.height() {
        return false;
    }
    
    // Validate destination bounds
    if dst_x + width > dst.width() || dst_y + height > dst.height() {
        return false;
    }
    
    // Select the appropriate blend function
    let blend_fn = match mode {
        BlendMode::Copy => blend_copy,
        BlendMode::Alpha => blend_alpha,
        BlendMode::Add => blend_add,
        BlendMode::Multiply => blend_multiply,
        BlendMode::Screen => blend_screen,
        BlendMode::Overlay => blend_overlay,
        BlendMode::Darken => blend_darken,
        BlendMode::Lighten => blend_lighten,
        BlendMode::ColorDodge => blend_color_dodge,
        BlendMode::ColorBurn => blend_color_burn,
        BlendMode::HardLight => blend_hard_light,
        BlendMode::SoftLight => blend_soft_light,
        BlendMode::Difference => blend_difference,
        BlendMode::Exclusion => blend_exclusion,
    };
    
    // Apply blending pixel by pixel
    for y in 0..height {
        for x in 0..width {
            let src_color = match src.get_pixel(src_x + x, src_y + y) {
                Some(color) => color,
                None => continue,
            };
            
            let dst_color = match dst.get_pixel(dst_x + x, dst_y + y) {
                Some(color) => color,
                None => continue,
            };
            
            let result_color = blend_fn(dst_color, src_color);
            dst.set_pixel(dst_x + x, dst_y + y, result_color);
        }
    }
    
    true
}

/// Blend a bitmap with the specified color using the given blend mode
pub fn blend_color(dst: &mut Bitmap, color: Color, mode: BlendMode) {
    let blend_fn = match mode {
        BlendMode::Copy => blend_copy,
        BlendMode::Alpha => blend_alpha,
        BlendMode::Add => blend_add,
        BlendMode::Multiply => blend_multiply,
        BlendMode::Screen => blend_screen,
        BlendMode::Overlay => blend_overlay,
        BlendMode::Darken => blend_darken,
        BlendMode::Lighten => blend_lighten,
        BlendMode::ColorDodge => blend_color_dodge,
        BlendMode::ColorBurn => blend_color_burn,
        BlendMode::HardLight => blend_hard_light,
        BlendMode::SoftLight => blend_soft_light,
        BlendMode::Difference => blend_difference,
        BlendMode::Exclusion => blend_exclusion,
    };
    
    for y in 0..dst.height() {
        for x in 0..dst.width() {
            if let Some(dst_color) = dst.get_pixel(x, y) {
                let result_color = blend_fn(dst_color, color);
                dst.set_pixel(x, y, result_color);
            }
        }
    }
}

// Individual blend mode implementations

/// Simple copy blend (ignores destination, uses source as-is)
fn blend_copy(dst: Color, src: Color) -> Color {
    // If the source is fully transparent, keep destination
    if src.a <= 0.0 {
        return dst;
    }
    src
}

/// Standard alpha blending with premultiplied alpha
fn blend_alpha(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    // If source is completely opaque, return source
    if src_a >= 1.0 {
        return src;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Calculate blended color components
    let out_r = (src.r * src_a + dst.r * dst_a * (1.0 - src_a)) / out_a;
    let out_g = (src.g * src_a + dst.g * dst_a * (1.0 - src_a)) / out_a;
    let out_b = (src.b * src_a + dst.b * dst_a * (1.0 - src_a)) / out_a;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Add the colors together, clamping at 1.0
fn blend_add(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Perform additive blending with alpha
    let out_a = (src_a + dst_a).min(1.0);
    let src_factor = src_a;
    let dst_factor = dst_a;
    
    let out_r = (src.r * src_factor + dst.r * dst_factor).min(1.0);
    let out_g = (src.g * src_factor + dst.g * dst_factor).min(1.0);
    let out_b = (src.b * src_factor + dst.b * dst_factor).min(1.0);
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Multiply the colors together
fn blend_multiply(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Blend with multiply and alpha
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = src_a * dst_a * (1.0 - src_factor - dst_factor) / out_a;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + (src.r * dst.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + (src.g * dst.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + (src.b * dst.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Screen blending (1 - (1-a) * (1-b))
fn blend_screen(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Screen blend formula: 1 - (1-a) * (1-b)
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let screen = |a: f32, b: f32| -> f32 {
        1.0 - (1.0 - a) * (1.0 - b)
    };
    
    let out_r = src.r * src_factor + dst.r * dst_factor + screen(src.r, dst.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + screen(src.g, dst.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + screen(src.b, dst.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Overlay blending (combines Multiply and Screen)
fn blend_overlay(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Overlay blend formula depends on the destination color
    let overlay = |a: f32, b: f32| -> f32 {
        if b <= 0.5 {
            // Use multiply formula for darker colors
            2.0 * a * b
        } else {
            // Use screen formula for lighter colors
            1.0 - 2.0 * (1.0 - a) * (1.0 - b)
        }
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + overlay(src.r, dst.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + overlay(src.g, dst.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + overlay(src.b, dst.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Darken (minimum of both colors)
fn blend_darken(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Darken blend formula is min(a, b)
    let darken = |a: f32, b: f32| -> f32 {
        a.min(b)
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + darken(src.r, dst.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + darken(src.g, dst.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + darken(src.b, dst.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Lighten (maximum of both colors)
fn blend_lighten(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Lighten blend formula is max(a, b)
    let lighten = |a: f32, b: f32| -> f32 {
        a.max(b)
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + lighten(src.r, dst.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + lighten(src.g, dst.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + lighten(src.b, dst.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Color dodge (brightens destination color based on source)
fn blend_color_dodge(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Color dodge formula: dst / (1 - src)
    let color_dodge = |a: f32, b: f32| -> f32 {
        if b >= 1.0 {
            1.0
        } else {
            (a / (1.0 - b)).min(1.0)
        }
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + color_dodge(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + color_dodge(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + color_dodge(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Color burn (darkens destination color based on source)
fn blend_color_burn(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Color burn formula: 1 - (1 - dst) / src
    let color_burn = |a: f32, b: f32| -> f32 {
        if b <= 0.0 {
            0.0
        } else {
            1.0 - ((1.0 - a) / b).min(1.0)
        }
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + color_burn(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + color_burn(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + color_burn(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Hard light (similar to Overlay, but using source to determine operation)
fn blend_hard_light(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Hard light formula depends on the source color
    let hard_light = |a: f32, b: f32| -> f32 {
        if b <= 0.5 {
            // Use multiply formula for darker colors
            2.0 * a * b
        } else {
            // Use screen formula for lighter colors
            1.0 - 2.0 * (1.0 - a) * (1.0 - b)
        }
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + hard_light(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + hard_light(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + hard_light(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Soft light (gentler version of Hard Light)
fn blend_soft_light(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Soft light formula
    let soft_light = |a: f32, b: f32| -> f32 {
        if b <= 0.5 {
            // Darker version
            a * (2.0 * b + a * (1.0 - 2.0 * b))
        } else {
            // Lighter version
            a + (2.0 * b - 1.0) * (a.sqrt() - a)
        }
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + soft_light(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + soft_light(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + soft_light(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Difference (absolute difference between colors)
fn blend_difference(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Difference formula: |a - b|
    let difference = |a: f32, b: f32| -> f32 {
        (a - b).abs()
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + difference(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + difference(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + difference(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
}

/// Exclusion (similar to Difference but lower contrast)
fn blend_exclusion(dst: Color, src: Color) -> Color {
    let src_a = src.a;
    
    // If source is completely transparent, return destination unchanged
    if src_a <= 0.0 {
        return dst;
    }
    
    let dst_a = dst.a;
    
    // If destination is completely transparent, return source
    if dst_a <= 0.0 {
        return src;
    }
    
    // Calculate resulting alpha
    let out_a = src_a + dst_a * (1.0 - src_a);
    
    // If resulting alpha is effectively zero, return transparent color
    if out_a < 0.001 {
        return Color::TRANSPARENT;
    }
    
    // Exclusion formula: a + b - 2 * a * b
    let exclusion = |a: f32, b: f32| -> f32 {
        a + b - 2.0 * a * b
    };
    
    let src_factor = src_a / out_a;
    let dst_factor = dst_a * (1.0 - src_a) / out_a;
    let src_dst_factor = 1.0 - src_factor - dst_factor;
    
    let out_r = src.r * src_factor + dst.r * dst_factor + exclusion(dst.r, src.r) * src_dst_factor;
    let out_g = src.g * src_factor + dst.g * dst_factor + exclusion(dst.g, src.g) * src_dst_factor;
    let out_b = src.b * src_factor + dst.b * dst_factor + exclusion(dst.b, src.b) * src_dst_factor;
    
    Color::new(out_r, out_g, out_b, out_a)
} 