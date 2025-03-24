/*!
 * Texture resource management
 */

use crate::api::bare::resources::BufferHandle;

/// Pixel formats for textures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    /// 8-bit per channel RGBA
    RGBA8,
    
    /// 8-bit per channel RGBA with sRGB encoding
    SRGBA8,
    
    /// 8-bit per channel RGB
    RGB8,
    
    /// 8-bit per channel RGB with sRGB encoding
    SRGB8,
    
    /// 8-bit grayscale
    R8,
    
    /// 16-bit per channel RGBA
    RGBA16F,
    
    /// 32-bit per channel RGBA
    RGBA32F,
    
    /// Depth 24-bit, stencil 8-bit
    Depth24Stencil8,
    
    /// Depth 32-bit float
    Depth32F,
}

impl PixelFormat {
    /// Get the byte size of one pixel in this format
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::RGBA8 | PixelFormat::SRGBA8 => 4,
            PixelFormat::RGB8 | PixelFormat::SRGB8 => 3,
            PixelFormat::R8 => 1,
            PixelFormat::RGBA16F => 8,
            PixelFormat::RGBA32F => 16,
            PixelFormat::Depth24Stencil8 => 4,
            PixelFormat::Depth32F => 4,
        }
    }
}

/// Sampling options for textures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFilter {
    /// Nearest neighbor filtering
    Nearest,
    
    /// Linear filtering
    Linear,
    
    /// Nearest neighbor with mipmaps
    NearestMipmapNearest,
    
    /// Linear filtering with mipmaps
    LinearMipmapLinear,
}

/// Texture wrapping modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureWrap {
    /// Repeat the texture
    Repeat,
    
    /// Mirror the texture
    MirrorRepeat,
    
    /// Clamp to edge
    ClampToEdge,
    
    /// Clamp to border
    ClampToBorder,
}

/// Texture resource for rendering
#[derive(Debug)]
pub struct TextureResource {
    /// Width of the texture
    pub width: u32,
    
    /// Height of the texture
    pub height: u32,
    
    /// Pixel format
    pub format: PixelFormat,
    
    /// Texture data
    pub data: Vec<u8>,
    
    /// Texture filter mode
    pub filter: TextureFilter,
    
    /// Texture wrap mode
    pub wrap: TextureWrap,
    
    /// Whether mipmaps should be generated
    pub generate_mipmaps: bool,
    
    /// GPU texture handle
    pub(crate) gpu_handle: Option<BufferHandle>,
    
    /// Whether the texture needs to be updated on the GPU
    pub(crate) dirty: bool,
}

impl TextureResource {
    /// Create a new texture resource
    pub fn new(width: u32, height: u32, format: PixelFormat, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            data,
            filter: TextureFilter::Linear,
            wrap: TextureWrap::ClampToEdge,
            generate_mipmaps: true,
            gpu_handle: None,
            dirty: true,
        }
    }
    
    /// Create a new texture resource with specific filter and wrap settings
    pub fn with_settings(
        width: u32,
        height: u32,
        format: PixelFormat,
        data: Vec<u8>,
        filter: TextureFilter,
        wrap: TextureWrap,
        generate_mipmaps: bool,
    ) -> Self {
        Self {
            width,
            height,
            format,
            data,
            filter,
            wrap,
            generate_mipmaps,
            gpu_handle: None,
            dirty: true,
        }
    }
    
    /// Update the texture data
    pub fn update_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
        self.dirty = true;
    }
    
    /// Get the expected size of the texture data in bytes
    pub fn expected_data_size(&self) -> usize {
        self.width as usize * self.height as usize * self.format.bytes_per_pixel()
    }
} 