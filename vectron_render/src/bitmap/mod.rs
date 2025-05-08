// Bitmap module for pixel-level manipulation and texture generation
//
// This module provides capabilities for direct bitmap manipulation,
// procedural texture generation, and image processing operations.

pub mod buffer;
pub mod formats;
pub mod operations;
pub mod generation;
pub mod io;

use crate::core::error::RenderError;
use crate::core::TextureHandle;
use crate::resource::cache::texture::{TextureCache, TextureData};
use crate::backend::resource::{TextureDesc, TextureUsage, TextureDimension};
use crate::TextureFormat;
use buffer::Bitmap;

/// Helper trait for converting bitmaps to textures
pub trait BitmapTextureConverter {
    /// Convert a bitmap to a texture and register it with the resource cache
    fn bitmap_to_texture(&mut self, bitmap: &Bitmap) -> Result<TextureHandle, RenderError>;
    
    /// Update an existing texture with new bitmap data
    fn update_texture_from_bitmap(
        &mut self, 
        handle: TextureHandle, 
        bitmap: &Bitmap
    ) -> Result<(), RenderError>;
}

impl BitmapTextureConverter for TextureCache {
    fn bitmap_to_texture(&mut self, bitmap: &Bitmap) -> Result<TextureHandle, RenderError> {
        let width = bitmap.width();
        let height = bitmap.height();
        let texture_data = TextureData::new_2d(
            bitmap.to_rgba_bytes(),
            width,
            height,
            TextureFormat::Rgba8Unorm
        );
        
        // Register the texture with the provided data
        Ok(self.register_texture(texture_data))
    }
    
    fn update_texture_from_bitmap(
        &mut self, 
        handle: TextureHandle, 
        bitmap: &Bitmap
    ) -> Result<(), RenderError> {
        let width = bitmap.width();
        let height = bitmap.height();
        let texture_data = TextureData::new_2d(
            bitmap.to_rgba_bytes(),
            width,
            height,
            TextureFormat::Rgba8Unorm
        );
        
        // Update existing texture with new bitmap data
        self.update_texture(handle, texture_data)
    }
} 