// Bitmap I/O module
//
// Provides functionality for loading and saving bitmap data in various formats

pub mod raw;
pub mod format;

// Re-export commonly used functions for convenience
pub use raw::{read_raw_bitmap, write_raw_bitmap};
pub use format::{read_bitmap, write_bitmap, supported_formats};

/// Enumeration of supported bitmap file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapFileFormat {
    /// PNG format
    PNG,
    /// JPEG format
    JPEG,
    /// BMP format
    BMP,
    /// TGA format
    TGA,
    /// Raw bitmap data
    RAW,
}

impl BitmapFileFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::PNG => "png",
            Self::JPEG => "jpg",
            Self::BMP => "bmp",
            Self::TGA => "tga",
            Self::RAW => "raw",
        }
    }
    
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::PNG),
            "jpg" | "jpeg" => Some(Self::JPEG),
            "bmp" => Some(Self::BMP),
            "tga" => Some(Self::TGA),
            "raw" => Some(Self::RAW),
            _ => None,
        }
    }
} 