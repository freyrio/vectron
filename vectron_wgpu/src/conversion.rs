use vectron_render::backend::resource::*;
use vectron_render::backend::state::*;
use vectron_render::types::CompareFunction;
use vectron_render::types::Face;
use vectron_render::types::FilterMode;
use vectron_render::types::FrontFace;
use vectron_render::types::IndexFormat;
use vectron_render::types::PolygonMode;
use vectron_render::types::PrimitiveTopology;

/// Convert vectron_render::TextureFormat to wgpu::TextureFormat
pub fn to_wgpu_texture_format(format: TextureFormat) -> wgpu::TextureFormat {
    match format {
        TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
        TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
    }
}

/// Convert wgpu::TextureFormat to vectron_render::TextureFormat
pub fn from_wgpu_texture_format(format: wgpu::TextureFormat) -> Option<TextureFormat> {
    match format {
        wgpu::TextureFormat::Rgba8Unorm => Some(TextureFormat::Rgba8Unorm),
        wgpu::TextureFormat::Rgba8UnormSrgb => Some(TextureFormat::Rgba8UnormSrgb),
        wgpu::TextureFormat::Bgra8Unorm => Some(TextureFormat::Bgra8Unorm),
        wgpu::TextureFormat::Bgra8UnormSrgb => Some(TextureFormat::Bgra8UnormSrgb),
        _ => None,
    }
}

/// Convert vectron_render::BufferUsage to wgpu::BufferUsages
pub fn to_wgpu_buffer_usage(usage: BufferUsage) -> wgpu::BufferUsages {
    let mut result = wgpu::BufferUsages::empty();
    
    if usage.contains(BufferUsage::VERTEX) {
        result |= wgpu::BufferUsages::VERTEX;
    }
    if usage.contains(BufferUsage::INDEX) {
        result |= wgpu::BufferUsages::INDEX;
    }
    if usage.contains(BufferUsage::UNIFORM) {
        result |= wgpu::BufferUsages::UNIFORM;
    }
    if usage.contains(BufferUsage::STORAGE) {
        result |= wgpu::BufferUsages::STORAGE;
    }
    if usage.contains(BufferUsage::COPY_SRC) {
        result |= wgpu::BufferUsages::COPY_SRC;
    }
    if usage.contains(BufferUsage::COPY_DST) {
        result |= wgpu::BufferUsages::COPY_DST;
    }
    if usage.contains(BufferUsage::MAP_READ) {
        result |= wgpu::BufferUsages::MAP_READ;
    }
    if usage.contains(BufferUsage::MAP_WRITE) {
        result |= wgpu::BufferUsages::MAP_WRITE;
    }
    if usage.contains(BufferUsage::INDIRECT) {
        result |= wgpu::BufferUsages::INDIRECT;
    }
    
    result
}

/// Convert vectron_render::TextureUsage to wgpu::TextureUsages
pub fn to_wgpu_texture_usage(usage: TextureUsage) -> wgpu::TextureUsages {
    let mut result = wgpu::TextureUsages::empty();
    
    if usage.contains(TextureUsage::COPY_SRC) {
        result |= wgpu::TextureUsages::COPY_SRC;
    }
    if usage.contains(TextureUsage::COPY_DST) {
        result |= wgpu::TextureUsages::COPY_DST;
    }
    if usage.contains(TextureUsage::TEXTURE_BINDING) {
        result |= wgpu::TextureUsages::TEXTURE_BINDING;
    }
    if usage.contains(TextureUsage::STORAGE_BINDING) {
        result |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    if usage.contains(TextureUsage::RENDER_ATTACHMENT) {
        result |= wgpu::TextureUsages::RENDER_ATTACHMENT;
    }
    
    result
}

/// Convert vectron_render::TextureDimension to wgpu::TextureDimension
pub fn to_wgpu_texture_dimension(dimension: TextureDimension) -> wgpu::TextureDimension {
    match dimension {
        TextureDimension::D1 => wgpu::TextureDimension::D1,
        TextureDimension::D2 => wgpu::TextureDimension::D2,
        TextureDimension::D3 => wgpu::TextureDimension::D3,
    }
}

/// Convert vectron_render::TextureViewDimension to wgpu::TextureViewDimension
pub fn to_wgpu_texture_view_dimension(dimension: TextureViewDimension) -> wgpu::TextureViewDimension {
    match dimension {
        TextureViewDimension::D1 => wgpu::TextureViewDimension::D1,
        TextureViewDimension::D2 => wgpu::TextureViewDimension::D2,
        TextureViewDimension::D2Array => wgpu::TextureViewDimension::D2Array,
        TextureViewDimension::Cube => wgpu::TextureViewDimension::Cube,
        TextureViewDimension::CubeArray => wgpu::TextureViewDimension::CubeArray,
        TextureViewDimension::D3 => wgpu::TextureViewDimension::D3,
    }
}

/// Convert vectron_render::TextureAspect to wgpu::TextureAspect
pub fn to_wgpu_texture_aspect(aspect: TextureAspect) -> wgpu::TextureAspect {
    match aspect {
        TextureAspect::All => wgpu::TextureAspect::All,
        TextureAspect::DepthOnly => wgpu::TextureAspect::DepthOnly,
        TextureAspect::StencilOnly => wgpu::TextureAspect::StencilOnly,
    }
}

/// Convert vectron_render::AddressMode to wgpu::AddressMode
pub fn to_wgpu_address_mode(mode: AddressMode) -> wgpu::AddressMode {
    match mode {
        AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
        AddressMode::Repeat => wgpu::AddressMode::Repeat,
        AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        AddressMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
    }
}

/// Convert vectron_render::FilterMode to wgpu::FilterMode
pub fn to_wgpu_filter_mode(mode: FilterMode) -> wgpu::FilterMode {
    match mode {
        FilterMode::Nearest => wgpu::FilterMode::Nearest,
        FilterMode::Linear => wgpu::FilterMode::Linear,
    }
}

/// Convert vectron_render::CompareFunction to wgpu::CompareFunction
pub fn to_wgpu_compare_function(function: CompareFunction) -> wgpu::CompareFunction {
    match function {
        CompareFunction::Never => wgpu::CompareFunction::Never,
        CompareFunction::Less => wgpu::CompareFunction::Less,
        CompareFunction::Equal => wgpu::CompareFunction::Equal,
        CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
        CompareFunction::Greater => wgpu::CompareFunction::Greater,
        CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
        CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
        CompareFunction::Always => wgpu::CompareFunction::Always,
    }
}

/// Convert vectron_render::IndexFormat to wgpu::IndexFormat
pub fn to_wgpu_index_format(format: IndexFormat) -> wgpu::IndexFormat {
    match format {
        IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
        IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
    }
}

/// Convert vectron_render::PrimitiveTopology to wgpu::PrimitiveTopology
pub fn to_wgpu_primitive_topology(topology: PrimitiveTopology) -> wgpu::PrimitiveTopology {
    match topology {
        PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
        PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
        PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
        PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
        PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
    }
}

/// Convert vectron_render::FrontFace to wgpu::FrontFace
pub fn to_wgpu_front_face(face: FrontFace) -> wgpu::FrontFace {
    match face {
        FrontFace::Ccw => wgpu::FrontFace::Ccw,
        FrontFace::Cw => wgpu::FrontFace::Cw,
    }
}

/// Convert vectron_render::Face to wgpu::Face
pub fn to_wgpu_face(face: Face) -> wgpu::Face {
    match face {
        Face::Front => wgpu::Face::Front,
        Face::Back => wgpu::Face::Back,
    }
}

/// Convert vectron_render::PolygonMode to wgpu::PolygonMode
pub fn to_wgpu_polygon_mode(mode: PolygonMode) -> wgpu::PolygonMode {
    match mode {
        PolygonMode::Fill => wgpu::PolygonMode::Fill,
        PolygonMode::Line => wgpu::PolygonMode::Line,
        PolygonMode::Point => wgpu::PolygonMode::Point,
    }
}

/// Convert vectron_render::BlendFactor to wgpu::BlendFactor
pub fn to_wgpu_blend_factor(factor: BlendFactor) -> wgpu::BlendFactor {
    match factor {
        BlendFactor::Zero => wgpu::BlendFactor::Zero,
        BlendFactor::One => wgpu::BlendFactor::One,
        BlendFactor::Src => wgpu::BlendFactor::Src,
        BlendFactor::OneMinusSrc => wgpu::BlendFactor::OneMinusSrc,
        BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
        BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
        BlendFactor::Dst => wgpu::BlendFactor::Dst,
        BlendFactor::OneMinusDst => wgpu::BlendFactor::OneMinusDst,
        BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
        BlendFactor::OneMinusDstAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
        BlendFactor::SrcAlphaSaturated => wgpu::BlendFactor::SrcAlphaSaturated,
        BlendFactor::Constant => wgpu::BlendFactor::Constant,
        BlendFactor::OneMinusConstant => wgpu::BlendFactor::OneMinusConstant,
    }
}

/// Convert vectron_render::BlendOperation to wgpu::BlendOperation
pub fn to_wgpu_blend_operation(operation: BlendOperation) -> wgpu::BlendOperation {
    match operation {
        BlendOperation::Add => wgpu::BlendOperation::Add,
        BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
        BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
        BlendOperation::Min => wgpu::BlendOperation::Min,
        BlendOperation::Max => wgpu::BlendOperation::Max,
    }
}

/// Convert vectron_render::ColorWrites to wgpu::ColorWrites
pub fn to_wgpu_color_writes(writes: ColorWrites) -> wgpu::ColorWrites {
    let mut result = wgpu::ColorWrites::empty();
    
    if writes.contains(ColorWrites::RED) {
        result |= wgpu::ColorWrites::RED;
    }
    if writes.contains(ColorWrites::GREEN) {
        result |= wgpu::ColorWrites::GREEN;
    }
    if writes.contains(ColorWrites::BLUE) {
        result |= wgpu::ColorWrites::BLUE;
    }
    if writes.contains(ColorWrites::ALPHA) {
        result |= wgpu::ColorWrites::ALPHA;
    }
    
    result
}

/// Convert vectron_render::LoadOp to wgpu::LoadOp
pub fn to_wgpu_load_op(op: LoadOp) -> wgpu::LoadOp<wgpu::Color> {
    match op {
        LoadOp::Load => wgpu::LoadOp::Load,
        LoadOp::Clear => wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
        LoadOp::DontCare => wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
    }
}

/// Convert vectron_render::StoreOp to wgpu::StoreOp
pub fn to_wgpu_store_op(op: StoreOp) -> wgpu::StoreOp {
    match op {
        StoreOp::Store => wgpu::StoreOp::Store,
        StoreOp::DontCare => wgpu::StoreOp::Discard,
    }
}

/// Convert vectron_render::VertexStepMode to wgpu::VertexStepMode
pub fn to_wgpu_vertex_step_mode(mode: VertexStepMode) -> wgpu::VertexStepMode {
    match mode {
        VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
        VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
    }
}

/// Convert vectron_render::VertexFormat to wgpu::VertexFormat
pub fn to_wgpu_vertex_format(format: VertexFormat) -> wgpu::VertexFormat {
    match format {
        VertexFormat::Uint8x2 => wgpu::VertexFormat::Uint8x2,
        VertexFormat::Uint8x4 => wgpu::VertexFormat::Uint8x4,
        VertexFormat::Sint8x2 => wgpu::VertexFormat::Sint8x2,
        VertexFormat::Sint8x4 => wgpu::VertexFormat::Sint8x4,
        VertexFormat::Unorm8x2 => wgpu::VertexFormat::Unorm8x2,
        VertexFormat::Unorm8x4 => wgpu::VertexFormat::Unorm8x4,
        VertexFormat::Snorm8x2 => wgpu::VertexFormat::Snorm8x2,
        VertexFormat::Snorm8x4 => wgpu::VertexFormat::Snorm8x4,
        VertexFormat::Uint16x2 => wgpu::VertexFormat::Uint16x2,
        VertexFormat::Uint16x4 => wgpu::VertexFormat::Uint16x4,
        VertexFormat::Sint16x2 => wgpu::VertexFormat::Sint16x2,
        VertexFormat::Sint16x4 => wgpu::VertexFormat::Sint16x4,
        VertexFormat::Unorm16x2 => wgpu::VertexFormat::Unorm16x2,
        VertexFormat::Unorm16x4 => wgpu::VertexFormat::Unorm16x4,
        VertexFormat::Snorm16x2 => wgpu::VertexFormat::Snorm16x2,
        VertexFormat::Snorm16x4 => wgpu::VertexFormat::Snorm16x4,
        VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
        VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
        VertexFormat::Float32 => wgpu::VertexFormat::Float32,
        VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
        VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
        VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
        VertexFormat::Uint32 => wgpu::VertexFormat::Uint32,
        VertexFormat::Uint32x2 => wgpu::VertexFormat::Uint32x2,
        VertexFormat::Uint32x3 => wgpu::VertexFormat::Uint32x3,
        VertexFormat::Uint32x4 => wgpu::VertexFormat::Uint32x4,
        VertexFormat::Sint32 => wgpu::VertexFormat::Sint32,
        VertexFormat::Sint32x2 => wgpu::VertexFormat::Sint32x2,
        VertexFormat::Sint32x3 => wgpu::VertexFormat::Sint32x3,
        VertexFormat::Sint32x4 => wgpu::VertexFormat::Sint32x4,
    }
} 