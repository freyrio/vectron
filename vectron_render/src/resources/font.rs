/*!
 * Font resource management
 */

use crate::api::bare::resources::{TextureHandle, ResourceId};
use crate::api::bare::state::Rect;

/// Font metrics for text layout
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    /// Line height of the font
    pub line_height: f32,
    
    /// Ascent (distance from baseline to top)
    pub ascent: f32,
    
    /// Descent (distance from baseline to bottom)
    pub descent: f32,
    
    /// Default line spacing
    pub line_gap: f32,
    
    /// Units per em
    pub units_per_em: f32,
}

/// Information about a glyph in a font
#[derive(Debug, Clone)]
pub struct GlyphInfo {
    /// Unicode code point
    pub code_point: char,
    
    /// Width of the glyph
    pub width: f32,
    
    /// Height of the glyph
    pub height: f32,
    
    /// Horizontal bearing X
    pub bearing_x: f32,
    
    /// Horizontal bearing Y
    pub bearing_y: f32,
    
    /// Horizontal advance
    pub advance: f32,
    
    /// Atlas texture rectangle
    pub atlas_rect: Rect,
}

/// A font resource for text rendering
#[derive(Debug)]
pub struct FontResource {
    /// Font name
    pub name: String,
    
    /// Font metrics
    pub metrics: FontMetrics,
    
    /// Font size in pixels
    pub size: f32,
    
    /// Glyph atlas texture
    pub atlas: TextureHandle,
    
    /// Mapping from code points to glyph info
    pub glyphs: Vec<GlyphInfo>,
    
    /// Font data
    pub data: Vec<u8>,
}

impl FontResource {
    /// Create a new font resource
    pub fn new(name: String, data: Vec<u8>, size: f32, atlas: TextureHandle) -> Self {
        // In a real implementation, we would parse the font data and extract metrics
        // For now we just use placeholder values
        let metrics = FontMetrics {
            line_height: size * 1.2,
            ascent: size * 0.8,
            descent: size * 0.2,
            line_gap: size * 0.1,
            units_per_em: 1000.0,
        };
        
        Self {
            name,
            metrics,
            size,
            atlas,
            glyphs: Vec::new(),
            data,
        }
    }
    
    /// Get glyph info for a character
    pub fn get_glyph(&self, c: char) -> Option<&GlyphInfo> {
        self.glyphs.iter().find(|g| g.code_point == c)
    }
    
    /// Calculate the width of a string when rendered with this font
    pub fn calculate_text_width(&self, text: &str) -> f32 {
        let mut width = 0.0;
        for c in text.chars() {
            if let Some(glyph) = self.get_glyph(c) {
                width += glyph.advance;
            }
        }
        width
    }
} 