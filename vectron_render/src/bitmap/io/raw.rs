// Raw bitmap I/O operations
//
// Provides low-level functions for reading and writing raw bitmap data

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::bitmap::buffer::Bitmap;
use crate::bitmap::formats::{PixelFormat, BitmapFormat};
use crate::bitmap::formats::conversion;
use crate::color::Color;

/// Header structure for raw bitmap files
#[repr(C, packed)]
struct RawBitmapHeader {
    /// Magic number to identify raw bitmap files (VRAW)
    magic: [u8; 4],
    /// Width of the bitmap
    width: u32,
    /// Height of the bitmap
    height: u32,
    /// Pixel format code
    format: u32,
    /// Reserved for future use
    reserved: [u8; 16],
}

impl RawBitmapHeader {
    /// Create a new header for the given dimensions and format
    fn new(width: u32, height: u32, format: PixelFormat) -> Self {
        Self {
            magic: *b"VRAW",
            width,
            height,
            format: format as u32,
            reserved: [0; 16],
        }
    }
    
    /// Validate the magic number
    fn is_valid(&self) -> bool {
        self.magic == *b"VRAW"
    }
    
    /// Get the pixel format
    fn pixel_format(&self) -> Option<PixelFormat> {
        match self.format {
            0 => Some(PixelFormat::Grayscale8),
            1 => Some(PixelFormat::GrayscaleAlpha8),
            2 => Some(PixelFormat::RGB8),
            3 => Some(PixelFormat::RGBA8),
            4 => Some(PixelFormat::RGB16),
            5 => Some(PixelFormat::RGBA16),
            6 => Some(PixelFormat::RGBF32),
            7 => Some(PixelFormat::RGBAF32),
            _ => None,
        }
    }
}

/// Read a bitmap from raw binary data
///
/// # Parameters
/// * `path` - Path to the raw bitmap file
///
/// # Returns
/// * `Ok(Bitmap)` - Successfully read bitmap
/// * `Err(io::Error)` - I/O error or invalid format
pub fn read_raw_bitmap<P: AsRef<Path>>(path: P) -> io::Result<Bitmap> {
    let mut file = File::open(path)?;
    
    // Read header
    let mut header_bytes = [0u8; std::mem::size_of::<RawBitmapHeader>()];
    file.read_exact(&mut header_bytes)?;
    
    // Convert bytes to header struct
    let header = unsafe {
        std::ptr::read_unaligned(header_bytes.as_ptr() as *const RawBitmapHeader)
    };
    
    // Validate header
    if !header.is_valid() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid raw bitmap file format"
        ));
    }
    
    let format = header.pixel_format().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported pixel format"
        )
    })?;
    
    // Read pixel data
    let bytes_per_pixel = format.bytes_per_pixel();
    let data_size = (header.width * header.height * bytes_per_pixel as u32) as usize;
    let mut data = vec![0u8; data_size];
    file.read_exact(&mut data)?;
    
    // Convert to bitmap
    conversion::format_to_bitmap(&data, format, header.width, header.height)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to convert pixel data to bitmap"
            )
        })
}

/// Write a bitmap to a raw binary file
///
/// # Parameters
/// * `path` - Path to write the raw bitmap file
/// * `bitmap` - The bitmap to write
/// * `format` - The pixel format to use for storage
///
/// # Returns
/// * `Ok(())` - Successfully written
/// * `Err(io::Error)` - I/O error
pub fn write_raw_bitmap<P: AsRef<Path>>(
    path: P, 
    bitmap: &Bitmap, 
    format: PixelFormat
) -> io::Result<()> {
    let mut file = File::create(path)?;
    
    // Create header
    let header = RawBitmapHeader::new(bitmap.width(), bitmap.height(), format);
    
    // Write header
    let header_bytes = unsafe {
        std::slice::from_raw_parts(
            &header as *const _ as *const u8,
            std::mem::size_of::<RawBitmapHeader>()
        )
    };
    file.write_all(header_bytes)?;
    
    // Convert bitmap to specified format
    let pixel_data = conversion::bitmap_to_format(bitmap, format);
    
    // Write pixel data
    file.write_all(&pixel_data)?;
    
    Ok(())
}

/// Read a bitmap from raw bytes
///
/// # Parameters
/// * `data` - Raw bitmap data including header
///
/// # Returns
/// * `Some(Bitmap)` - Successfully decoded bitmap
/// * `None` - Invalid data
pub fn read_raw_bitmap_from_bytes(data: &[u8]) -> Option<Bitmap> {
    if data.len() < std::mem::size_of::<RawBitmapHeader>() {
        return None;
    }
    
    // Extract header
    let header = unsafe {
        std::ptr::read_unaligned(data.as_ptr() as *const RawBitmapHeader)
    };
    
    // Validate header
    if !header.is_valid() {
        return None;
    }
    
    let format = header.pixel_format()?;
    let bytes_per_pixel = format.bytes_per_pixel();
    let data_size = (header.width * header.height * bytes_per_pixel as u32) as usize;
    let header_size = std::mem::size_of::<RawBitmapHeader>();
    
    if data.len() < header_size + data_size {
        return None;
    }
    
    // Extract pixel data
    let pixel_data = &data[header_size..header_size + data_size];
    
    // Convert to bitmap
    conversion::format_to_bitmap(pixel_data, format, header.width, header.height)
}

/// Convert a bitmap to raw bytes
///
/// # Parameters
/// * `bitmap` - The bitmap to convert
/// * `format` - The pixel format to use for storage
///
/// # Returns
/// * `Vec<u8>` - Raw bitmap data including header
pub fn write_raw_bitmap_to_bytes(bitmap: &Bitmap, format: PixelFormat) -> Vec<u8> {
    // Create header
    let header = RawBitmapHeader::new(bitmap.width(), bitmap.height(), format);
    let header_size = std::mem::size_of::<RawBitmapHeader>();
    
    // Convert bitmap to specified format
    let pixel_data = conversion::bitmap_to_format(bitmap, format);
    
    // Combine header and pixel data
    let mut result = Vec::with_capacity(header_size + pixel_data.len());
    unsafe {
        let header_bytes = std::slice::from_raw_parts(
            &header as *const _ as *const u8,
            header_size
        );
        result.extend_from_slice(header_bytes);
    }
    result.extend_from_slice(&pixel_data);
    
    result
} 