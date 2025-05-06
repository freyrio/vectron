//! Common types used throughout the DirectX application.

/// Unique identifier for a buffer resource
pub type BufferId = u32;

/// Unique identifier for a pipeline resource
pub type PipelineId = u32;

/// Describes a surface for rendering
#[derive(Debug, Clone, Copy)]
pub struct SurfaceDescriptor {
    pub handle: *mut std::ffi::c_void,
    pub width: u32,
    pub height: u32,
}

/// Index format for index buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

/// Commands that can be submitted to the renderer
#[derive(Debug)]
pub enum RenderCommand {
    SetPipeline(PipelineId),
    SetVertexBuffer {
        slot: u32,
        buffer: BufferId,
        offset: usize,
    },
    SetIndexBuffer {
        buffer: BufferId,
        offset: usize,
        index_format: IndexFormat,
    },
    SetConstantBuffer {
        slot: u32,
        buffer: BufferId,
    },
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    },
    SetViewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    },
    SetScissor {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    ClearColor {
        attachment_index: u32,
        color: [f32; 4],
    },
    ClearDepthStencil {
        depth: Option<f32>,
        stencil: Option<u32>,
    },
} 