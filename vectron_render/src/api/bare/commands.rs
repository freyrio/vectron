/*!
 * Basic drawing commands for the bare API
 */

use super::resources::ResourceId;
use super::state::DrawState;

/// A draw command for the low-level drawing API
#[derive(Debug, Clone)]
pub struct DrawCommand {
    /// Command type
    pub command_type: DrawCommandType,
    /// State at the time of command creation
    pub state: DrawState,
}

/// Types of drawing commands for the low-level API
#[derive(Debug, Clone)]
pub enum DrawCommandType {
    /// Clear the screen with a color
    Clear {
        color: [f32; 4],
        depth: Option<f32>,
        stencil: Option<u8>,
    },
    
    /// Set scissor rectangle
    SetScissor {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    },
    
    /// Draw a piece of geometry
    DrawGeometry {
        geometry: ResourceId,
        material: ResourceId,
        instance_count: u32,
    },
    
    /// Draw a piece of geometry with instances
    DrawGeometryInstanced {
        geometry: ResourceId,
        material: ResourceId,
        instance_buffer: ResourceId,
        instance_count: u32,
    },
    
    /// Execute a compute shader
    Compute {
        pipeline: ResourceId,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    },
    
    /// Copy data between resources
    CopyResource {
        source: ResourceId,
        destination: ResourceId,
    },
    
    /// Custom draw command
    Custom {
        data: Box<dyn std::any::Any>,
    },
} 