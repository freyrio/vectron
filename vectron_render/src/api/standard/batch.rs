/*!
 * Batched rendering for improved performance
 */

use std::collections::HashMap;
use crate::api::bare::{DrawCommand, DrawCommandType, BlendMode};
use crate::api::bare::resources::{ResourceId, PipelineHandle, TextureHandle};
use crate::error::RenderError;

/// Key for identifying compatible batch groups
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct BatchKey {
    /// Pipeline used for rendering
    pipeline: PipelineHandle,
    
    /// Texture used for rendering (if any)
    texture: Option<TextureHandle>,
    
    /// Blend mode
    blend_mode: BlendMode,
}

/// A batch of draw commands that can be rendered together
struct DrawBatch {
    /// Commands in this batch
    commands: Vec<DrawCommand>,
    
    /// Vertex offset in the combined buffer
    vertex_offset: u32,
    
    /// Index offset in the combined buffer
    index_offset: u32,
    
    /// Vertex data for all commands in this batch
    vertex_data: Vec<Vertex>,
    
    /// Index data for all commands in this batch
    index_data: Vec<u32>,
}

/// A simple vertex structure
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// Renderer that batches similar draw calls for better performance
pub struct BatchRenderer {
    /// Current batches being built
    batches: HashMap<BatchKey, DrawBatch>,
    
    /// Maximum vertices per batch
    max_vertices: u32,
    
    /// Maximum indices per batch
    max_indices: u32,
}

impl BatchRenderer {
    /// Create a new batch renderer
    pub fn new(max_vertices: u32, max_indices: u32) -> Self {
        Self {
            batches: HashMap::new(),
            max_vertices,
            max_indices,
        }
    }
    
    /// Add a draw command to the appropriate batch
    pub fn add(&mut self, command: DrawCommand) -> Result<(), RenderError> {
        // Determine batch key from command
        let key = self.get_batch_key(&command)?;
        
        // Get or create batch
        let batch = self.batches.entry(key.clone()).or_insert_with(|| DrawBatch {
            commands: Vec::new(),
            vertex_offset: 0,
            index_offset: 0,
            vertex_data: Vec::new(),
            index_data: Vec::new(),
        });
        
        // Add command data to batch
        batch.commands.push(command);
        
        Ok(())
    }
    
    /// Get the batch key for a command
    fn get_batch_key(&self, command: &DrawCommand) -> Result<BatchKey, RenderError> {
        match &command.command_type {
            DrawCommandType::DrawGeometry { material, .. } => {
                // In a real implementation, we would lookup the material to determine
                // its pipeline and texture information. For now we just use placeholders.
                Ok(BatchKey {
                    pipeline: PipelineHandle(0),
                    texture: Some(TextureHandle(0)),
                    blend_mode: command.state.blend_mode,
                })
            },
            _ => Err(RenderError::UnsupportedOperation("Command cannot be batched".into())),
        }
    }
    
    /// Flush all batches and generate optimized draw commands
    pub fn flush(&mut self) -> Vec<DrawCommand> {
        let mut result = Vec::new();
        
        // Process each batch
        for (key, batch) in self.batches.drain() {
            // In a real implementation, we would combine geometries and emit optimized commands
            // For now we just pass through the commands
            result.extend(batch.commands);
        }
        
        result
    }
} 