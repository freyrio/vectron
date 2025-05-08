// Core bitmap buffer implementation
//
// Provides the fundamental Bitmap type for pixel manipulation

use crate::color::Color;
use std::ops::{Index, IndexMut};

/// Represents a 2D bitmap with pixel data
#[derive(Clone, Debug)]
pub struct Bitmap {
    /// The width of the bitmap in pixels
    width: u32,
    /// The height of the bitmap in pixels
    height: u32,
    /// The pixel data stored as Color values
    pixels: Vec<Color>,
    /// Whether the bitmap has an alpha channel
    has_alpha: bool,
}

impl Bitmap {
    /// Create a new empty bitmap with the specified dimensions
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            pixels: vec![Color::TRANSPARENT; size],
            has_alpha: true,
        }
    }
    
    /// Create a new bitmap filled with a specific color
    pub fn with_color(width: u32, height: u32, color: Color) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            pixels: vec![color; size],
            has_alpha: color.a < 1.0,
        }
    }
    
    /// Get the width of the bitmap
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get the height of the bitmap
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Get the total number of pixels in the bitmap
    #[inline]
    pub fn len(&self) -> usize {
        self.pixels.len()
    }
    
    /// Check if the bitmap has any pixels
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
    
    /// Check if the bitmap has an alpha channel (any transparent pixels)
    #[inline]
    pub fn has_alpha(&self) -> bool {
        self.has_alpha
    }
    
    /// Get a pixel at the specified coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        
        let index = (y * self.width + x) as usize;
        Some(self.pixels[index])
    }
    
    /// Set a pixel at the specified coordinates
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        
        let index = (y * self.width + x) as usize;
        self.pixels[index] = color;
        
        if color.a < 1.0 {
            self.has_alpha = true;
        }
        
        true
    }
    
    /// Fill the entire bitmap with a specific color
    pub fn fill(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
        
        self.has_alpha = color.a < 1.0;
    }
    
    /// Fill a rectangular region with a specific color
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);
        
        for cy in y..y_end {
            for cx in x..x_end {
                self.set_pixel(cx, cy, color);
            }
        }
    }
    
    /// Get direct access to the underlying pixel data
    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }
    
    /// Get mutable access to the underlying pixel data
    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }
    
    /// Convert the bitmap to raw RGBA8 bytes for texture upload
    pub fn to_rgba_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.width as usize * self.height as usize * 4);
        
        for color in &self.pixels {
            let (r, g, b, a) = color.to_rgba8();
            bytes.push(r);
            bytes.push(g);
            bytes.push(b);
            bytes.push(a);
        }
        
        bytes
    }
    
    /// Create a bitmap from raw RGBA8 bytes
    pub fn from_rgba_bytes(width: u32, height: u32, data: &[u8]) -> Option<Self> {
        let expected_len = (width * height * 4) as usize;
        if data.len() != expected_len {
            return None;
        }
        
        let mut pixels = Vec::with_capacity((width * height) as usize);
        let mut has_alpha = false;
        
        for chunk in data.chunks_exact(4) {
            let color = Color::from_rgba8(chunk[0], chunk[1], chunk[2], chunk[3]);
            if chunk[3] < 255 {
                has_alpha = true;
            }
            pixels.push(color);
        }
        
        Some(Self {
            width,
            height,
            pixels,
            has_alpha,
        })
    }
    
    /// Resize the bitmap to new dimensions, optionally preserving content
    pub fn resize(&mut self, new_width: u32, new_height: u32, preserve_content: bool) {
        if new_width == self.width && new_height == self.height {
            return;
        }
        
        let new_size = (new_width * new_height) as usize;
        let mut new_pixels = vec![Color::TRANSPARENT; new_size];
        
        if preserve_content {
            let min_width = self.width.min(new_width);
            let min_height = self.height.min(new_height);
            
            for y in 0..min_height {
                for x in 0..min_width {
                    let old_idx = (y * self.width + x) as usize;
                    let new_idx = (y * new_width + x) as usize;
                    new_pixels[new_idx] = self.pixels[old_idx];
                }
            }
        }
        
        self.width = new_width;
        self.height = new_height;
        self.pixels = new_pixels;
        
        // Recalculate has_alpha
        self.has_alpha = self.pixels.iter().any(|color| color.a < 1.0);
    }
    
    /// Create a cropped version of this bitmap
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        if x + width > self.width || y + height > self.height {
            return None;
        }
        
        let mut result = Self::new(width, height);
        
        for cy in 0..height {
            for cx in 0..width {
                let src_pixel = self.get_pixel(x + cx, y + cy).unwrap();
                result.set_pixel(cx, cy, src_pixel);
            }
        }
        
        Some(result)
    }
    
    /// Create a sub-region view into this bitmap
    pub fn subregion(&self, x: u32, y: u32, width: u32, height: u32) -> Option<BitmapView> {
        if x + width > self.width || y + height > self.height {
            return None;
        }
        
        Some(BitmapView {
            parent: self,
            x,
            y,
            width,
            height,
        })
    }
}

/// A view into a portion of a bitmap without copying data
#[derive(Clone, Copy, Debug)]
pub struct BitmapView<'a> {
    parent: &'a Bitmap,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<'a> BitmapView<'a> {
    /// Get the width of the view
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// Get the height of the view
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// Get a pixel at the specified coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        
        self.parent.get_pixel(self.x + x, self.y + y)
    }
    
    /// Convert this view to a new bitmap by copying the data
    pub fn to_bitmap(&self) -> Bitmap {
        let mut result = Bitmap::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(color) = self.get_pixel(x, y) {
                    result.set_pixel(x, y, color);
                }
            }
        }
        
        result
    }
}

// Implement indexing for Bitmap for more ergonomic access
impl Index<(u32, u32)> for Bitmap {
    type Output = Color;
    
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let index = (y * self.width + x) as usize;
        &self.pixels[index]
    }
}

impl IndexMut<(u32, u32)> for Bitmap {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let index = (y * self.width + x) as usize;
        &mut self.pixels[index]
    }
} 