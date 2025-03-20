use bitflags::bitflags;

/// Texture format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // Color formats
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Float,
    RG8Unorm,
    RG8Snorm,
    RG8Uint,
    RG8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    RG16Uint,
    RG16Sint,
    RG16Float,
    RGBA8Unorm,
    RGBA8UnormSrgb,
    RGBA8Snorm,
    RGBA8Uint,
    RGBA8Sint,
    BGRA8Unorm,
    BGRA8UnormSrgb,
    RGB10A2Unorm,
    RG32Uint,
    RG32Sint,
    RG32Float,
    RGBA16Uint,
    RGBA16Sint,
    RGBA16Float,
    RGBA32Uint,
    RGBA32Sint,
    RGBA32Float,
    
    // Depth/stencil formats
    Depth16Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
    Depth32Float,
    
    // Compressed formats
    BC1RGBAUnorm,
    BC1RGBAUnormSrgb,
    BC2RGBAUnorm,
    BC2RGBAUnormSrgb,
    BC3RGBAUnorm,
    BC3RGBAUnormSrgb,
    BC4RUnorm,
    BC4RSnorm,
    BC5RGUnorm,
    BC5RGSnorm,
    BC6HRGBUfloat,
    BC6HRGBFloat,
    BC7RGBAUnorm,
    BC7RGBAUnormSrgb,
}

impl TextureFormat {
    /// Get the byte size of a single pixel in this format
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            TextureFormat::R8Unorm | TextureFormat::R8Snorm | TextureFormat::R8Uint | TextureFormat::R8Sint => 1,
            TextureFormat::R16Uint | TextureFormat::R16Sint | TextureFormat::R16Float |
            TextureFormat::RG8Unorm | TextureFormat::RG8Snorm | TextureFormat::RG8Uint | TextureFormat::RG8Sint => 2,
            TextureFormat::R32Uint | TextureFormat::R32Sint | TextureFormat::R32Float |
            TextureFormat::RG16Uint | TextureFormat::RG16Sint | TextureFormat::RG16Float |
            TextureFormat::RGBA8Unorm | TextureFormat::RGBA8UnormSrgb | TextureFormat::RGBA8Snorm |
            TextureFormat::RGBA8Uint | TextureFormat::RGBA8Sint |
            TextureFormat::BGRA8Unorm | TextureFormat::BGRA8UnormSrgb |
            TextureFormat::RGB10A2Unorm | TextureFormat::Depth24PlusStencil8 => 4,
            TextureFormat::RG32Uint | TextureFormat::RG32Sint | TextureFormat::RG32Float |
            TextureFormat::RGBA16Uint | TextureFormat::RGBA16Sint | TextureFormat::RGBA16Float => 8,
            TextureFormat::RGBA32Uint | TextureFormat::RGBA32Sint | TextureFormat::RGBA32Float => 16,
            // Special cases
            TextureFormat::Depth16Unorm => 2,
            TextureFormat::Depth24Plus | TextureFormat::Depth32Float => 4,
            // Compressed formats depend on block size
            _ => 0, // For compressed formats, this is not a straightforward calculation
        }
    }
    
    /// Check if this is a depth or stencil format
    pub fn is_depth_stencil(&self) -> bool {
        matches!(self, 
            TextureFormat::Depth16Unorm |
            TextureFormat::Depth24Plus |
            TextureFormat::Depth24PlusStencil8 |
            TextureFormat::Depth32Float
        )
    }
    
    /// Check if this is a compressed format
    pub fn is_compressed(&self) -> bool {
        matches!(self,
            TextureFormat::BC1RGBAUnorm |
            TextureFormat::BC1RGBAUnormSrgb |
            TextureFormat::BC2RGBAUnorm |
            TextureFormat::BC2RGBAUnormSrgb |
            TextureFormat::BC3RGBAUnorm |
            TextureFormat::BC3RGBAUnormSrgb |
            TextureFormat::BC4RUnorm |
            TextureFormat::BC4RSnorm |
            TextureFormat::BC5RGUnorm |
            TextureFormat::BC5RGSnorm |
            TextureFormat::BC6HRGBUfloat |
            TextureFormat::BC6HRGBFloat |
            TextureFormat::BC7RGBAUnorm |
            TextureFormat::BC7RGBAUnormSrgb
        )
    }
}

/// Texture dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
    Cube,
}

bitflags! {
    /// Texture usage flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TextureUsageFlags: u32 {
        const SAMPLED = 0b00000001;
        const STORAGE = 0b00000010;
        const RENDER_TARGET = 0b00000100;
        const DEPTH_STENCIL = 0b00001000;
        const COPY_SRC = 0b00010000;
        const COPY_DST = 0b00100000;
    }
}

/// Texture descriptor
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub size: TextureSize,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsageFlags,
    pub initial_data: Option<Vec<u8>>,
}

/// Texture size
#[derive(Debug, Clone, Copy)]
pub struct TextureSize {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

impl TextureSize {
    /// Create a new 1D texture size
    pub fn d1(width: u32) -> Self {
        Self {
            width,
            height: 1,
            depth_or_array_layers: 1,
        }
    }
    
    /// Create a new 2D texture size
    pub fn d2(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            depth_or_array_layers: 1,
        }
    }
    
    /// Create a new 3D texture size
    pub fn d3(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth_or_array_layers: depth,
        }
    }
    
    /// Create a new 2D array texture size
    pub fn d2_array(width: u32, height: u32, array_layers: u32) -> Self {
        Self {
            width,
            height,
            depth_or_array_layers: array_layers,
        }
    }
    
    /// Create a new cube texture size
    pub fn cube(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            depth_or_array_layers: 6, // Cube textures have 6 faces
        }
    }
}

/// Texture update descriptor
#[derive(Debug, Clone)]
pub struct TextureUpdateDescriptor {
    pub mip_level: u32,
    pub array_layer: u32,
    pub origin: [u32; 3],
    pub size: [u32; 3],
}

/// Texture sampler addressing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
    ClampToBorder,
}

/// Texture sampler filtering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

/// Texture sampler descriptor
#[derive(Debug, Clone)]
pub struct SamplerDescriptor {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<u16>,
    pub border_color: Option<BorderColor>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

/// Comparison function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

/// Border color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
}
