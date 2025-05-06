// This module is intended for managing graphics resources like
// shaders, buffers, textures, pipelines, etc.

// pub mod buffer;
// pub mod pipeline;
// pub mod shader;
// pub mod texture;

// Initially, it might just contain common resource-related types or constants.

//! Resources module for managing DirectX resources like buffers, textures, and shaders.

mod buffer;
mod pipeline;
mod shader;

pub use buffer::*;
pub use pipeline::*;
pub use shader::*;

use super::types::{BufferId, PipelineId};
use std::collections::HashMap;

/// Resource manager for DirectX resources
pub struct ResourceManager {
    buffers: HashMap<BufferId, Buffer>,
    pipelines: HashMap<PipelineId, Pipeline>,
    next_buffer_id: BufferId,
    next_pipeline_id: PipelineId,
}

impl ResourceManager {
    /// Creates a new resource manager
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            pipelines: HashMap::new(),
            next_buffer_id: 1,
            next_pipeline_id: 1,
        }
    }

    /// Gets a buffer by ID
    pub fn get_buffer(&self, id: BufferId) -> Option<&Buffer> {
        self.buffers.get(&id)
    }

    /// Gets a mutable reference to a buffer by ID
    pub fn get_buffer_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
        self.buffers.get_mut(&id)
    }

    /// Gets a pipeline by ID
    pub fn get_pipeline(&self, id: PipelineId) -> Option<&Pipeline> {
        self.pipelines.get(&id)
    }

    /// Gets a mutable reference to a pipeline by ID
    pub fn get_pipeline_mut(&mut self, id: PipelineId) -> Option<&mut Pipeline> {
        self.pipelines.get_mut(&id)
    }

    /// Allocates a new buffer ID
    pub fn allocate_buffer_id(&mut self) -> BufferId {
        let id = self.next_buffer_id;
        self.next_buffer_id += 1;
        id
    }

    /// Allocates a new pipeline ID
    pub fn allocate_pipeline_id(&mut self) -> PipelineId {
        let id = self.next_pipeline_id;
        self.next_pipeline_id += 1;
        id
    }
    
    /// Stores a buffer with the given ID
    pub fn store_buffer(&mut self, id: BufferId, buffer: Buffer) {
        self.buffers.insert(id, buffer);
    }
    
    /// Stores a pipeline with the given ID
    pub fn store_pipeline(&mut self, id: PipelineId, pipeline: Pipeline) {
        self.pipelines.insert(id, pipeline);
    }
    
    /// Removes a buffer by ID
    pub fn remove_buffer(&mut self, id: BufferId) -> Option<Buffer> {
        self.buffers.remove(&id)
    }
    
    /// Removes a pipeline by ID
    pub fn remove_pipeline(&mut self, id: PipelineId) -> Option<Pipeline> {
        self.pipelines.remove(&id)
    }
}
