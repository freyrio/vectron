// Format-specific bitmap I/O operations
//
// Provides functions for reading and writing bitmap data in various file formats

use std::io;
use std::path::Path;

use crate::bitmap::buffer::Bitmap;
use crate::bitmap::formats::PixelFormat;
use crate::bitmap::io::{BitmapFileFormat, raw};
use crate::color::Color;

#[cfg(feature = "image-io")]
use image::{GenericImageView, ImageFormat, DynamicImage};

/// Returns a list of supported bitmap file formats
///
/// # Returns
/// * `Vec<BitmapFileFormat>` - List of supported formats
pub fn supported_formats() -> Vec<BitmapFileFormat> {
    let mut formats = vec![BitmapFileFormat::RAW];
    
    #[cfg(feature = "image-io")]
    {
        formats.push(BitmapFileFormat::PNG);
        formats.push(BitmapFileFormat::JPEG);
        formats.push(BitmapFileFormat::BMP);
        formats.push(BitmapFileFormat::TGA);
    }
    
    formats
}

/// Read a bitmap from a file
///
/// Attempts to detect the format from the file extension.
///
/// # Parameters
/// * `path` - Path to the image file
///
/// # Returns
/// * `Ok(Bitmap)` - Successfully read bitmap
/// * `Err(io::Error)` - I/O error or unsupported format
pub fn read_bitmap<P: AsRef<Path>>(path: P) -> io::Result<Bitmap> {
    let path_ref = path.as_ref();
    let extension = path_ref.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    let format = extension
        .and_then(|ext| BitmapFileFormat::from_extension(&ext))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unrecognized file extension or format"
            )
        })?;
    
    read_bitmap_with_format(path, format)
}

/// Read a bitmap from a file with a specific format
///
/// # Parameters
/// * `path` - Path to the image file
/// * `format` - The file format to use for reading
///
/// # Returns
/// * `Ok(Bitmap)` - Successfully read bitmap
/// * `Err(io::Error)` - I/O error or unsupported format
pub fn read_bitmap_with_format<P: AsRef<Path>>(
    path: P, 
    format: BitmapFileFormat
) -> io::Result<Bitmap> {
    match format {
        BitmapFileFormat::RAW => raw::read_raw_bitmap(path),
        
        #[cfg(feature = "image-io")]
        BitmapFileFormat::PNG | 
        BitmapFileFormat::JPEG | 
        BitmapFileFormat::BMP | 
        BitmapFileFormat::TGA => {
            read_bitmap_with_image_crate(path, format)
        },
        
        #[cfg(not(feature = "image-io"))]
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Format requires the image-io feature to be enabled"
        )),
    }
}

/// Write a bitmap to a file
///
/// Attempts to detect the format from the file extension.
///
/// # Parameters
/// * `path` - Path to write the image file
/// * `bitmap` - The bitmap to write
///
/// # Returns
/// * `Ok(())` - Successfully written
/// * `Err(io::Error)` - I/O error or unsupported format
pub fn write_bitmap<P: AsRef<Path>>(path: P, bitmap: &Bitmap) -> io::Result<()> {
    let path_ref = path.as_ref();
    let extension = path_ref.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    let format = extension
        .and_then(|ext| BitmapFileFormat::from_extension(&ext))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unrecognized file extension or format"
            )
        })?;
    
    write_bitmap_with_format(path, bitmap, format)
}

/// Write a bitmap to a file with a specific format
///
/// # Parameters
/// * `path` - Path to write the image file
/// * `bitmap` - The bitmap to write
/// * `format` - The file format to use for writing
///
/// # Returns
/// * `Ok(())` - Successfully written
/// * `Err(io::Error)` - I/O error or unsupported format
pub fn write_bitmap_with_format<P: AsRef<Path>>(
    path: P, 
    bitmap: &Bitmap,
    format: BitmapFileFormat
) -> io::Result<()> {
    match format {
        BitmapFileFormat::RAW => raw::write_raw_bitmap(path, bitmap, PixelFormat::RGBA8),
        
        #[cfg(feature = "image-io")]
        BitmapFileFormat::PNG | 
        BitmapFileFormat::JPEG | 
        BitmapFileFormat::BMP | 
        BitmapFileFormat::TGA => {
            write_bitmap_with_image_crate(path, bitmap, format)
        },
        
        #[cfg(not(feature = "image-io"))]
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Format requires the image-io feature to be enabled"
        )),
    }
}

// Implementation of image crate based I/O when the feature is enabled
#[cfg(feature = "image-io")]
fn read_bitmap_with_image_crate<P: AsRef<Path>>(
    path: P, 
    format: BitmapFileFormat
) -> io::Result<Bitmap> {
    // Load image using the image crate
    let img = match format {
        BitmapFileFormat::PNG => image::open(path)?,
        BitmapFileFormat::JPEG => image::open(path)?,
        BitmapFileFormat::BMP => image::open(path)?,
        BitmapFileFormat::TGA => image::open(path)?,
        _ => unreachable!(),
    };
    
    // Convert to RGBA8
    let rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();
    
    // Create bitmap
    let mut bitmap = Bitmap::new(width, height);
    
    // Fill bitmap with pixel data
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x, y);
            let color = Color::from_rgba8(pixel[0], pixel[1], pixel[2], pixel[3]);
            bitmap.set_pixel(x, y, color);
        }
    }
    
    Ok(bitmap)
}

#[cfg(feature = "image-io")]
fn write_bitmap_with_image_crate<P: AsRef<Path>>(
    path: P, 
    bitmap: &Bitmap,
    format: BitmapFileFormat
) -> io::Result<()> {
    let width = bitmap.width();
    let height = bitmap.height();
    
    // Create a buffer for the image
    let mut img_buffer = image::RgbaImage::new(width, height);
    
    // Fill the buffer with bitmap data
    for y in 0..height {
        for x in 0..width {
            if let Some(color) = bitmap.get_pixel(x, y) {
                let (r, g, b, a) = color.to_rgba8();
                img_buffer.put_pixel(x, y, image::Rgba([r, g, b, a]));
            }
        }
    }
    
    // Convert to DynamicImage
    let dynamic_img = DynamicImage::ImageRgba8(img_buffer);
    
    // Determine image format
    let img_format = match format {
        BitmapFileFormat::PNG => ImageFormat::Png,
        BitmapFileFormat::JPEG => ImageFormat::Jpeg,
        BitmapFileFormat::BMP => ImageFormat::Bmp,
        BitmapFileFormat::TGA => ImageFormat::Tga,
        _ => unreachable!(),
    };
    
    // Save the image
    dynamic_img.save_with_format(path, img_format)?;
    
    Ok(())
}

// Mock implementation when image-io feature is disabled
#[cfg(not(feature = "image-io"))]
mod mock {
    use super::*;
    
    pub fn read_bitmap_with_image_crate<P: AsRef<Path>>(
        _path: P, 
        _format: BitmapFileFormat
    ) -> io::Result<Bitmap> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Format requires the image-io feature to be enabled"
        ))
    }
    
    pub fn write_bitmap_with_image_crate<P: AsRef<Path>>(
        _path: P, 
        _bitmap: &Bitmap,
        _format: BitmapFileFormat
    ) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Format requires the image-io feature to be enabled"
        ))
    }
} 