// Pipeline cache for Vectron Render
//
// This module provides caching for graphics pipeline resources.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::core::error::RenderError;
use crate::core::Uuid;
use crate::core::types::PipelineHandle;
use crate::backend::device::BackendDevice;
use crate::backend::resource::PipelineDesc;

/// Key used for pipeline caching
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineKey {
    /// Vertex shader ID
    pub vertex_shader: Uuid,
    /// Fragment shader ID
    pub fragment_shader: Option<Uuid>,
    /// Blend mode
    pub blend_mode: u32,
    /// Descriptor hash
    pub descriptor_hash: u64,
}

impl Hash for PipelineKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vertex_shader.hash(state);
        self.fragment_shader.hash(state);
        self.blend_mode.hash(state);
        self.descriptor_hash.hash(state);
    }
}

/// Cache for pipeline resources
#[derive(Debug)]
pub struct PipelineCache {
    /// Stored pipeline descs mapped by handle
    pipelines: HashMap<PipelineHandle, PipelineDesc>,
    
    /// Mapping from pipeline keys to handles
    pipeline_keys: HashMap<PipelineKey, PipelineHandle>,
    
    /// Mapping from our pipeline handles to backend GPU pipeline handles
    gpu_pipelines: HashMap<PipelineHandle, Uuid>,
}

impl Default for PipelineCache {
    fn default() -> Self {
        Self {
            pipelines: HashMap::new(),
            pipeline_keys: HashMap::new(),
            gpu_pipelines: HashMap::new(),
        }
    }
}

impl PipelineCache {
    /// Create a new pipeline cache
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a pipeline and get a handle to it
    pub fn register_pipeline(&mut self, key: PipelineKey, desc: PipelineDesc) -> PipelineHandle {
        // Check if we already have a pipeline with this key
        if let Some(handle) = self.pipeline_keys.get(&key) {
            return *handle;
        }
        
        // Create a new handle
        let handle = PipelineHandle::new();
        self.pipelines.insert(handle, desc);
        self.pipeline_keys.insert(key, handle);
        handle
    }
    
    /// Get a reference to a pipeline descriptor
    pub fn get_pipeline(&self, handle: PipelineHandle) -> Option<&PipelineDesc> {
        self.pipelines.get(&handle)
    }
    
    /// Get a pipeline by key
    pub fn get_pipeline_by_key(&self, key: &PipelineKey) -> Option<PipelineHandle> {
        self.pipeline_keys.get(key).copied()
    }
    
    /// Check if a pipeline handle is valid
    pub fn contains_pipeline(&self, handle: PipelineHandle) -> bool {
        self.pipelines.contains_key(&handle)
    }
    
    /// Remove a pipeline from the cache
    pub fn remove_pipeline(&mut self, handle: PipelineHandle) -> Option<PipelineDesc> {
        // Find and remove the key associated with this handle
        if let Some((key, _)) = self.pipeline_keys.iter()
            .find(|(_, h)| **h == handle)
            .map(|(k, _)| (k.clone(), ())) {
            self.pipeline_keys.remove(&key);
        }
        
        self.gpu_pipelines.remove(&handle);
        self.pipelines.remove(&handle)
    }
    
    /// Associate a GPU pipeline with a pipeline handle
    pub fn associate_gpu_pipeline(&mut self, handle: PipelineHandle, gpu_handle: PipelineHandle) -> Result<(), RenderError> {
        if !self.pipelines.contains_key(&handle) {
            return Err(RenderError::ResourceNotFound(format!("Pipeline handle {:?} not found", handle)));
        }
        
        self.gpu_pipelines.insert(handle, gpu_handle.uuid());
        Ok(())
    }
    
    /// Get the GPU pipeline associated with a pipeline handle
    pub fn get_gpu_pipeline(&self, handle: PipelineHandle) -> Option<Uuid> {
        self.gpu_pipelines.get(&handle).copied()
    }
    
    /// Get or create a pipeline based on key
    pub fn get_or_create_pipeline(
        &mut self,
        key: PipelineKey,
        desc: PipelineDesc,
        device: &mut dyn BackendDevice,
    ) -> Result<PipelineHandle, RenderError> {
        // Check if we already have a pipeline with this key
        if let Some(handle) = self.pipeline_keys.get(&key) {
            return Ok(*handle);
        }
        
        // Create backend pipeline
        let gpu_pipeline = device.create_pipeline(&desc)?;
        
        // Register pipeline
        let handle = self.register_pipeline(key, desc);
        
        // Associate GPU handle
        self.gpu_pipelines.insert(handle, gpu_pipeline.uuid());
        
        Ok(handle)
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.pipelines.clear();
        self.pipeline_keys.clear();
        self.gpu_pipelines.clear();
    }
} 