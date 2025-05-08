// Blending modes for Vectron Render
//
// This module provides implementations of various blending modes for colors,
// following standard compositing operations.

use crate::color::Color;

/// Represents a blending mode for compositing colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// Normal blending (standard alpha compositing)
    Normal,
    /// Multiply blend mode
    Multiply,
    /// Screen blend mode
    Screen,
    /// Overlay blend mode
    Overlay,
    /// Darken blend mode (selects the darker color)
    Darken,
    /// Lighten blend mode (selects the lighter color)
    Lighten,
    /// Color Dodge blend mode (brightens the base color)
    ColorDodge,
    /// Color Burn blend mode (darkens the base color)
    ColorBurn,
    /// Hard Light blend mode
    HardLight,
    /// Soft Light blend mode
    SoftLight,
    /// Difference blend mode
    Difference,
    /// Exclusion blend mode
    Exclusion,
    /// Hue blend mode (preserves luminosity and saturation of base color, but uses hue of blend color)
    Hue,
    /// Saturation blend mode (preserves luminosity and hue of base color, but uses saturation of blend color)
    Saturation,
    /// Color blend mode (preserves luminosity of base color, but uses hue and saturation of blend color)
    Color,
    /// Luminosity blend mode (preserves hue and saturation of base color, but uses luminosity of blend color)
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl BlendMode {
    /// Get the name of this blend mode as a string.
    pub fn name(&self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::HardLight => "Hard Light",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Try to parse a blend mode name into a BlendMode.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        match name.as_str() {
            "normal" => Some(BlendMode::Normal),
            "multiply" => Some(BlendMode::Multiply),
            "screen" => Some(BlendMode::Screen),
            "overlay" => Some(BlendMode::Overlay),
            "darken" => Some(BlendMode::Darken),
            "lighten" => Some(BlendMode::Lighten),
            "color-dodge" | "colordodge" | "color dodge" => Some(BlendMode::ColorDodge),
            "color-burn" | "colorburn" | "color burn" => Some(BlendMode::ColorBurn),
            "hard-light" | "hardlight" | "hard light" => Some(BlendMode::HardLight),
            "soft-light" | "softlight" | "soft light" => Some(BlendMode::SoftLight),
            "difference" => Some(BlendMode::Difference),
            "exclusion" => Some(BlendMode::Exclusion),
            "hue" => Some(BlendMode::Hue),
            "saturation" => Some(BlendMode::Saturation),
            "color" => Some(BlendMode::Color),
            "luminosity" => Some(BlendMode::Luminosity),
            _ => None,
        }
    }

    /// Blend two colors according to this blend mode.
    ///
    /// * `base` - The base (backdrop) color
    /// * `blend` - The blend (source) color to composite over the base color
    pub fn apply(self, base: Color, blend: Color) -> Color {
        // For non-separable blend modes (hue, saturation, color, luminosity)
        // we need to convert to HSL or other color spaces
        use crate::color::spaces::{HSL, RGB};

        // Extract the color components
        let br = base.r;
        let bg = base.g;
        let bb = base.b;
        let ba = base.a;
        
        let sr = blend.r;
        let sg = blend.g;
        let sb = blend.b;
        let sa = blend.a;
        
        // Perform the blend operation according to the blend mode
        let (r, g, b) = match self {
            BlendMode::Normal => {
                // Simple alpha compositing
                (sr, sg, sb)
            },
            BlendMode::Multiply => {
                (br * sr, bg * sg, bb * sb)
            },
            BlendMode::Screen => {
                (br + sr - br * sr, bg + sg - bg * sg, bb + sb - bb * sb)
            },
            BlendMode::Overlay => {
                (
                    if br <= 0.5 { 2.0 * br * sr } else { 1.0 - 2.0 * (1.0 - br) * (1.0 - sr) },
                    if bg <= 0.5 { 2.0 * bg * sg } else { 1.0 - 2.0 * (1.0 - bg) * (1.0 - sg) },
                    if bb <= 0.5 { 2.0 * bb * sb } else { 1.0 - 2.0 * (1.0 - bb) * (1.0 - sb) }
                )
            },
            BlendMode::Darken => {
                (br.min(sr), bg.min(sg), bb.min(sb))
            },
            BlendMode::Lighten => {
                (br.max(sr), bg.max(sg), bb.max(sb))
            },
            BlendMode::ColorDodge => {
                (
                    if sr == 1.0 { 1.0 } else if sr == 0.0 { 0.0 } else { (br / (1.0 - sr)).min(1.0) },
                    if sg == 1.0 { 1.0 } else if sg == 0.0 { 0.0 } else { (bg / (1.0 - sg)).min(1.0) },
                    if sb == 1.0 { 1.0 } else if sb == 0.0 { 0.0 } else { (bb / (1.0 - sb)).min(1.0) }
                )
            },
            BlendMode::ColorBurn => {
                (
                    if sr == 0.0 { 0.0 } else if sr == 1.0 { 1.0 } else { 1.0 - ((1.0 - br) / sr).min(1.0) },
                    if sg == 0.0 { 0.0 } else if sg == 1.0 { 1.0 } else { 1.0 - ((1.0 - bg) / sg).min(1.0) },
                    if sb == 0.0 { 0.0 } else if sb == 1.0 { 1.0 } else { 1.0 - ((1.0 - bb) / sb).min(1.0) }
                )
            },
            BlendMode::HardLight => {
                (
                    if sr <= 0.5 { 2.0 * br * sr } else { 1.0 - 2.0 * (1.0 - br) * (1.0 - sr) },
                    if sg <= 0.5 { 2.0 * bg * sg } else { 1.0 - 2.0 * (1.0 - bg) * (1.0 - sg) },
                    if sb <= 0.5 { 2.0 * bb * sb } else { 1.0 - 2.0 * (1.0 - bb) * (1.0 - sb) }
                )
            },
            BlendMode::SoftLight => {
                // Using the Adobe Photoshop formula for Soft Light
                let soft_light = |base: f32, blend: f32| -> f32 {
                    if blend <= 0.5 {
                        base - (1.0 - 2.0 * blend) * base * (1.0 - base)
                    } else {
                        let d = if base <= 0.25 {
                            ((16.0 * base - 12.0) * base + 4.0) * base
                        } else {
                            base.sqrt()
                        };
                        base + (2.0 * blend - 1.0) * (d - base)
                    }
                };
                
                (
                    soft_light(br, sr),
                    soft_light(bg, sg),
                    soft_light(bb, sb)
                )
            },
            BlendMode::Difference => {
                (
                    (br - sr).abs(),
                    (bg - sg).abs(),
                    (bb - sb).abs()
                )
            },
            BlendMode::Exclusion => {
                (
                    br + sr - 2.0 * br * sr,
                    bg + sg - 2.0 * bg * sg,
                    bb + sb - 2.0 * bb * sb
                )
            },
            BlendMode::Hue => {
                // Convert to HSL
                let base_rgb = RGB::new(br, bg, bb);
                let base_hsl = HSL::from(base_rgb);
                
                let blend_rgb = RGB::new(sr, sg, sb);
                let blend_hsl = HSL::from(blend_rgb);
                
                // Take hue from blend, saturation and lightness from base
                let result_hsl = HSL::new(blend_hsl.h, base_hsl.s, base_hsl.l);
                
                // Convert back to RGB
                let result_rgb = RGB::from(result_hsl);
                (result_rgb.r, result_rgb.g, result_rgb.b)
            },
            BlendMode::Saturation => {
                // Convert to HSL
                let base_rgb = RGB::new(br, bg, bb);
                let base_hsl = HSL::from(base_rgb);
                
                let blend_rgb = RGB::new(sr, sg, sb);
                let blend_hsl = HSL::from(blend_rgb);
                
                // Take saturation from blend, hue and lightness from base
                let result_hsl = HSL::new(base_hsl.h, blend_hsl.s, base_hsl.l);
                
                // Convert back to RGB
                let result_rgb = RGB::from(result_hsl);
                (result_rgb.r, result_rgb.g, result_rgb.b)
            },
            BlendMode::Color => {
                // Convert to HSL
                let base_rgb = RGB::new(br, bg, bb);
                let base_hsl = HSL::from(base_rgb);
                
                let blend_rgb = RGB::new(sr, sg, sb);
                let blend_hsl = HSL::from(blend_rgb);
                
                // Take hue and saturation from blend, lightness from base
                let result_hsl = HSL::new(blend_hsl.h, blend_hsl.s, base_hsl.l);
                
                // Convert back to RGB
                let result_rgb = RGB::from(result_hsl);
                (result_rgb.r, result_rgb.g, result_rgb.b)
            },
            BlendMode::Luminosity => {
                // Convert to HSL
                let base_rgb = RGB::new(br, bg, bb);
                let base_hsl = HSL::from(base_rgb);
                
                let blend_rgb = RGB::new(sr, sg, sb);
                let blend_hsl = HSL::from(blend_rgb);
                
                // Take luminosity from blend, hue and saturation from base
                let result_hsl = HSL::new(base_hsl.h, base_hsl.s, blend_hsl.l);
                
                // Convert back to RGB
                let result_rgb = RGB::from(result_hsl);
                (result_rgb.r, result_rgb.g, result_rgb.b)
            },
        };
        
        // Compute final alpha (standard alpha compositing)
        let a = sa + ba * (1.0 - sa);
        
        // Return the blended color
        if a < f32::EPSILON {
            // Avoid division by zero if the result is fully transparent
            Color::TRANSPARENT
        } else {
            // Perform alpha compositing
            let alpha_ratio = sa / a;
            let inverse_alpha_ratio = ba * (1.0 - sa) / a;
            
            Color::new(
                r * alpha_ratio + br * inverse_alpha_ratio,
                g * alpha_ratio + bg * inverse_alpha_ratio,
                b * alpha_ratio + bb * inverse_alpha_ratio,
                a
            )
        }
    }
}

/// Trait for types that can be blended with colors.
pub trait Blendable {
    /// Blend this value with another using the specified blend mode.
    fn blend(&self, other: &Self, mode: BlendMode) -> Self;
    
    /// Blend this value with another using the normal blend mode.
    fn blend_normal(&self, other: &Self) -> Self;
}

impl Blendable for Color {
    fn blend(&self, other: &Self, mode: BlendMode) -> Self {
        mode.apply(*self, *other)
    }
    
    fn blend_normal(&self, other: &Self) -> Self {
        self.blend(other, BlendMode::Normal)
    }
} 