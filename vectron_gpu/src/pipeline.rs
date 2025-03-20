use crate::texture::TextureFormat;
use crate::common::ShaderId;
use crate::vertex::VertexLayoutDescriptor;

/// Pipeline type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineType {
    Graphics,
    Compute,
}

/// Pipeline descriptor
#[derive(Debug, Clone)]
pub struct PipelineDescriptor {
    pub type_: PipelineType,
    pub vertex_shader: Option<ShaderId>,
    pub fragment_shader: Option<ShaderId>,
    pub compute_shader: Option<ShaderId>,
    pub vertex_layout: Option<VertexLayoutDescriptor>,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
    pub rasterizer_state: RasterizerState,
    pub primitive_topology: PrimitiveTopology,
    pub render_target_formats: Vec<TextureFormat>,
    pub depth_stencil_format: Option<TextureFormat>,
    pub sample_count: u32,
}

impl PipelineDescriptor {
    /// Create a new graphics pipeline descriptor
    pub fn graphics(
        vertex_shader: ShaderId,
        fragment_shader: ShaderId,
        vertex_layout: VertexLayoutDescriptor,
        render_target_formats: Vec<TextureFormat>,
    ) -> Self {
        Self {
            type_: PipelineType::Graphics,
            vertex_shader: Some(vertex_shader),
            fragment_shader: Some(fragment_shader),
            compute_shader: None,
            vertex_layout: Some(vertex_layout),
            blend_state: BlendState::default(),
            depth_stencil_state: DepthStencilState::default(),
            rasterizer_state: RasterizerState::default(),
            primitive_topology: PrimitiveTopology::TriangleList,
            render_target_formats,
            depth_stencil_format: None,
            sample_count: 1,
        }
    }
    
    /// Create a new compute pipeline descriptor
    pub fn compute(compute_shader: ShaderId) -> Self {
        Self {
            type_: PipelineType::Compute,
            vertex_shader: None,
            fragment_shader: None,
            compute_shader: Some(compute_shader),
            vertex_layout: None,
            blend_state: BlendState::default(),
            depth_stencil_state: DepthStencilState::default(),
            rasterizer_state: RasterizerState::default(),
            primitive_topology: PrimitiveTopology::TriangleList,
            render_target_formats: Vec::new(),
            depth_stencil_format: None,
            sample_count: 1,
        }
    }
}

/// Primitive topology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

/// Blend state
#[derive(Debug, Clone)]
pub struct BlendState {
    pub alpha_to_coverage_enabled: bool,
    pub targets: Vec<ColorTargetState>,
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
            alpha_to_coverage_enabled: false,
            targets: vec![ColorTargetState::default()],
        }
    }
}

/// Color target state
#[derive(Debug, Clone)]
pub struct ColorTargetState {
    pub format: Option<TextureFormat>,
    pub blend: Option<BlendComponent>,
    pub write_mask: ColorWriteMask,
}

impl Default for ColorTargetState {
    fn default() -> Self {
        Self {
            format: None,
            blend: None,
            write_mask: ColorWriteMask::ALL,
        }
    }
}

/// Color write mask
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorWriteMask {
    RED,
    GREEN,
    BLUE,
    ALPHA,
    ALL,
    NONE,
}

/// Blend component
#[derive(Debug, Clone)]
pub struct BlendComponent {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOperation,
}

impl Default for BlendComponent {
    fn default() -> Self {
        Self {
            src_factor: BlendFactor::One,
            dst_factor: BlendFactor::Zero,
            operation: BlendOperation::Add,
        }
    }
}

/// Blend factor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
}

/// Blend operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

/// Depth stencil state
#[derive(Debug, Clone)]
pub struct DepthStencilState {
    pub format: Option<TextureFormat>,
    pub depth_write_enabled: bool,
    pub depth_compare: crate::texture::CompareFunction,
    pub stencil_front: StencilFaceState,
    pub stencil_back: StencilFaceState,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        Self {
            format: None,
            depth_write_enabled: true,
            depth_compare: crate::texture::CompareFunction::Less,
            stencil_front: StencilFaceState::default(),
            stencil_back: StencilFaceState::default(),
            stencil_read_mask: 0xFFFFFFFF,
            stencil_write_mask: 0xFFFFFFFF,
        }
    }
}

/// Stencil face state
#[derive(Debug, Clone)]
pub struct StencilFaceState {
    pub compare: crate::texture::CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

impl Default for StencilFaceState {
    fn default() -> Self {
        Self {
            compare: crate::texture::CompareFunction::Always,
            fail_op: StencilOperation::Keep,
            depth_fail_op: StencilOperation::Keep,
            pass_op: StencilOperation::Keep,
        }
    }
}

/// Stencil operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap,
}

/// Rasterizer state
#[derive(Debug, Clone)]
pub struct RasterizerState {
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub depth_bias: i32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

impl Default for RasterizerState {
    fn default() -> Self {
        Self {
            front_face: FrontFace::CounterClockwise,
            cull_mode: CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }
    }
}

/// Front face
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    Clockwise,
    CounterClockwise,
}

/// Cull mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

/// Logic operation for blend state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOperation {
    Clear,
    Set,
    Copy,
    CopyInverted,
    NoOp,
    Invert,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Equivalent,
    AndReverse,
    AndInverted,
    OrReverse,
    OrInverted,
}
