// Bitmap format definitions and conversions
//
// This module defines bitmap formats and provides conversions between them

/// Pixel format definitions for bitmap data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 8-bit grayscale (1 byte per pixel)
    Grayscale8,
    /// 8-bit grayscale with alpha (2 bytes per pixel)
    GrayscaleAlpha8,
    /// 8-bit RGB (3 bytes per pixel)
    RGB8,
    /// 8-bit RGBA (4 bytes per pixel)
    RGBA8,
    /// 16-bit RGB (6 bytes per pixel)
    RGB16,
    /// 16-bit RGBA (8 bytes per pixel)
    RGBA16,
    /// 32-bit float RGB (12 bytes per pixel)
    RGBF32,
    /// 32-bit float RGBA (16 bytes per pixel)
    RGBAF32,
}

impl PixelFormat {
    /// Get the bytes per pixel for this format
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::Grayscale8 => 1,
            Self::GrayscaleAlpha8 => 2,
            Self::RGB8 => 3,
            Self::RGBA8 => 4,
            Self::RGB16 => 6,
            Self::RGBA16 => 8,
            Self::RGBF32 => 12,
            Self::RGBAF32 => 16,
        }
    }
    
    /// Check if this format has an alpha channel
    pub fn has_alpha(&self) -> bool {
        matches!(self, 
            Self::GrayscaleAlpha8 | 
            Self::RGBA8 | 
            Self::RGBA16 | 
            Self::RGBAF32
        )
    }
    
    /// Get the number of color channels
    pub fn channels(&self) -> usize {
        match self {
            Self::Grayscale8 => 1,
            Self::GrayscaleAlpha8 => 2,
            Self::RGB8 | Self::RGB16 | Self::RGBF32 => 3,
            Self::RGBA8 | Self::RGBA16 | Self::RGBAF32 => 4,
        }
    }
}

/// Compression types for bitmap storage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    /// No compression
    None,
    /// Run-length encoding
    RLE,
    /// DEFLATE compression
    Deflate,
    /// LZ77 compression
    LZ77,
    /// DXT1/BC1 (no alpha)
    DXT1,
    /// DXT3/BC2 (explicit alpha)
    DXT3,
    /// DXT5/BC3 (interpolated alpha)
    DXT5,
}

/// Bitmap data format descriptor
#[derive(Debug, Clone)]
pub struct BitmapFormat {
    /// Pixel format
    pub pixel_format: PixelFormat,
    /// Compression type
    pub compression: CompressionType,
    /// Row alignment in bytes (typical values: 1, 4, 8)
    pub row_alignment: usize,
    /// Whether the data is stored from bottom to top
    pub flip_y: bool,
    /// Whether the RGB components are swapped (e.g., BGR)
    pub swapped_rgb: bool,
}

impl Default for BitmapFormat {
    fn default() -> Self {
        Self {
            pixel_format: PixelFormat::RGBA8,
            compression: CompressionType::None,
            row_alignment: 4, // Common default (e.g., OpenGL)
            flip_y: false,
            swapped_rgb: false,
        }
    }
}

impl BitmapFormat {
    /// Create a new format descriptor with default values
    pub fn new(pixel_format: PixelFormat) -> Self {
        Self {
            pixel_format,
            ..Default::default()
        }
    }
    
    /// Set the compression type
    pub fn with_compression(mut self, compression: CompressionType) -> Self {
        self.compression = compression;
        self
    }
    
    /// Set the row alignment
    pub fn with_row_alignment(mut self, alignment: usize) -> Self {
        self.row_alignment = alignment;
        self
    }
    
    /// Set whether the data is stored from bottom to top
    pub fn with_flip_y(mut self, flip: bool) -> Self {
        self.flip_y = flip;
        self
    }
    
    /// Set whether RGB components are swapped
    pub fn with_swapped_rgb(mut self, swapped: bool) -> Self {
        self.swapped_rgb = swapped;
        self
    }
    
    /// Calculate the size in bytes for an image with these dimensions
    pub fn calculate_size(&self, width: u32, height: u32) -> usize {
        if matches!(self.compression, 
            CompressionType::DXT1 | 
            CompressionType::DXT3 | 
            CompressionType::DXT5) {
            // Handle block compression formats separately
            return self.calculate_compressed_size(width, height);
        }
        
        let bytes_per_pixel = self.pixel_format.bytes_per_pixel();
        let row_bytes = width as usize * bytes_per_pixel;
        let aligned_row_bytes = if row_bytes % self.row_alignment == 0 {
            row_bytes
        } else {
            (row_bytes / self.row_alignment + 1) * self.row_alignment
        };
        
        aligned_row_bytes * height as usize
    }
    
    /// Calculate size for compressed formats
    fn calculate_compressed_size(&self, width: u32, height: u32) -> usize {
        // Block compression formats work with 4x4 pixel blocks
        let block_size = match self.compression {
            CompressionType::DXT1 => 8, // 8 bytes per 4x4 block
            CompressionType::DXT3 | CompressionType::DXT5 => 16, // 16 bytes per 4x4 block
            _ => unreachable!(),
        };
        
        // Round up to nearest block
        let blocks_width = (width + 3) / 4;
        let blocks_height = (height + 3) / 4;
        
        (blocks_width * blocks_height) as usize * block_size
    }
    
    /// Create a standard RGBA8 format
    pub fn standard_rgba8() -> Self {
        Self::new(PixelFormat::RGBA8)
    }
    
    /// Create a standard RGB8 format
    pub fn standard_rgb8() -> Self {
        Self::new(PixelFormat::RGB8)
    }
    
    /// Create a format compatible with OpenGL
    pub fn opengl_compatible() -> Self {
        Self::new(PixelFormat::RGBA8)
            .with_row_alignment(4)
            .with_flip_y(true)
    }
    
    /// Create a format compatible with DirectX
    pub fn directx_compatible() -> Self {
        Self::new(PixelFormat::RGBA8)
            .with_row_alignment(4)
            .with_swapped_rgb(true)
    }
}

/// Convert between different pixel formats
pub mod conversion {
    use super::PixelFormat;
    use crate::bitmap::buffer::Bitmap;
    
    /// Convert raw pixel data from one format to another
    pub fn convert_pixel_data(
        src_data: &[u8],
        src_format: PixelFormat,
        dst_format: PixelFormat,
        width: u32,
        height: u32,
    ) -> Vec<u8> {
        let src_bpp = src_format.bytes_per_pixel();
        let dst_bpp = dst_format.bytes_per_pixel();
        let pixel_count = (width * height) as usize;
        
        let mut dst_data = Vec::with_capacity(pixel_count * dst_bpp);
        
        for i in 0..pixel_count {
            let src_offset = i * src_bpp;
            
            // Extract color components based on source format
            let (r, g, b, a) = match src_format {
                PixelFormat::Grayscale8 => {
                    let v = src_data[src_offset];
                    (v, v, v, 255)
                },
                PixelFormat::GrayscaleAlpha8 => {
                    let v = src_data[src_offset];
                    let a = src_data[src_offset + 1];
                    (v, v, v, a)
                },
                PixelFormat::RGB8 => {
                    let r = src_data[src_offset];
                    let g = src_data[src_offset + 1];
                    let b = src_data[src_offset + 2];
                    (r, g, b, 255)
                },
                PixelFormat::RGBA8 => {
                    let r = src_data[src_offset];
                    let g = src_data[src_offset + 1];
                    let b = src_data[src_offset + 2];
                    let a = src_data[src_offset + 3];
                    (r, g, b, a)
                },
                // Add more format conversions as needed
                _ => (0, 0, 0, 255), // Placeholder for unimplemented formats
            };
            
            // Write color components based on destination format
            match dst_format {
                PixelFormat::Grayscale8 => {
                    // Convert to grayscale using standard luminance formula
                    let v = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    dst_data.push(v);
                },
                PixelFormat::GrayscaleAlpha8 => {
                    let v = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    dst_data.push(v);
                    dst_data.push(a);
                },
                PixelFormat::RGB8 => {
                    dst_data.push(r);
                    dst_data.push(g);
                    dst_data.push(b);
                },
                PixelFormat::RGBA8 => {
                    dst_data.push(r);
                    dst_data.push(g);
                    dst_data.push(b);
                    dst_data.push(a);
                },
                // Add more format conversions as needed
                _ => {
                    // Placeholder for unimplemented formats
                    for _ in 0..dst_bpp {
                        dst_data.push(0);
                    }
                },
            }
        }
        
        dst_data
    }
    
    /// Convert a bitmap to raw pixel data in the specified format
    pub fn bitmap_to_format(bitmap: &Bitmap, format: PixelFormat) -> Vec<u8> {
        let width = bitmap.width();
        let height = bitmap.height();
        let bpp = format.bytes_per_pixel();
        let mut data = Vec::with_capacity((width * height) as usize * bpp);
        
        for y in 0..height {
            for x in 0..width {
                if let Some(color) = bitmap.get_pixel(x, y) {
                    match format {
                        PixelFormat::Grayscale8 => {
                            let (r, g, b, _) = color.to_rgba8();
                            let v = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                            data.push(v);
                        },
                        PixelFormat::GrayscaleAlpha8 => {
                            let (r, g, b, a) = color.to_rgba8();
                            let v = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                            data.push(v);
                            data.push(a);
                        },
                        PixelFormat::RGB8 => {
                            let (r, g, b, _) = color.to_rgba8();
                            data.push(r);
                            data.push(g);
                            data.push(b);
                        },
                        PixelFormat::RGBA8 => {
                            let (r, g, b, a) = color.to_rgba8();
                            data.push(r);
                            data.push(g);
                            data.push(b);
                            data.push(a);
                        },
                        // Add more format conversions as needed
                        _ => {
                            // Placeholder for unimplemented formats
                            for _ in 0..bpp {
                                data.push(0);
                            }
                        },
                    }
                }
            }
        }
        
        data
    }
    
    /// Create a bitmap from raw pixel data in the specified format
    pub fn format_to_bitmap(
        data: &[u8],
        format: PixelFormat,
        width: u32,
        height: u32,
    ) -> Option<Bitmap> {
        // Convert to RGBA8 first if not already in that format
        let rgba_data = if format == PixelFormat::RGBA8 {
            data.to_vec()
        } else {
            convert_pixel_data(data, format, PixelFormat::RGBA8, width, height)
        };
        
        // Then create the bitmap from RGBA8 data
        Bitmap::from_rgba_bytes(width, height, &rgba_data)
    }
} 