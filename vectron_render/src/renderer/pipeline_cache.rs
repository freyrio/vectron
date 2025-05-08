// Pipeline cache implementation for Vectron Render
//
// This module manages caching of pipeline objects to minimize creation overhead.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::core::error::RenderError;
use crate::core::types::{PipelineHandle, ShaderHandle, TextureFormat};
use crate::backend::resource::{VertexBufferLayout, DepthStencilState, PrimitiveState, MultisampleState};
use crate::backend::device::BackendDevice;

/// A key identifying a unique pipeline configuration
#[derive(Clone, Debug, Eq)]
pub struct PipelineKey {
    /// Vertex shader
    vertex_shader: ShaderHandle,
    
    /// Fragment shader (optional)
    fragment_shader: Option<ShaderHandle>,
    
    /// Vertex buffer layouts
    vertex_layouts: Vec<VertexBufferLayout>,
    
    /// Color attachment formats
    color_formats: Vec<Option<TextureFormat>>,
    
    /// Depth stencil state
    depth_stencil: Option<DepthStencilState>,
    
    /// Primitive state
    primitive: PrimitiveState,
    
    /// Multisample state
    multisample: MultisampleState,
}

impl PartialEq for PipelineKey {
    fn eq(&self, other: &Self) -> bool {
        self.vertex_shader == other.vertex_shader &&
        self.fragment_shader == other.fragment_shader &&
        self.vertex_layouts == other.vertex_layouts &&
        self.color_formats == other.color_formats &&
        self.depth_stencil == other.depth_stencil &&
        self.primitive == other.primitive &&
        self.multisample == other.multisample
    }
}

impl Hash for PipelineKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vertex_shader.hash(state);
        self.fragment_shader.hash(state);
        // Hash individual fields of vertex_layouts
        for layout in &self.vertex_layouts {
            layout.array_stride.hash(state);
            layout.step_mode.hash(state);
            // Hash attributes
            for attr in &layout.attributes {
                attr.format.hash(state);
                attr.offset.hash(state);
                attr.shader_location.hash(state);
            }
        }
        // Hash color formats
        for format in &self.color_formats {
            format.hash(state);
        }
        // Hash depth stencil
        if let Some(ds) = &self.depth_stencil {
            ds.format.hash(state);
            ds.depth_write_enabled.hash(state);
            ds.depth_compare.hash(state);
            ds.stencil_front.hash(state);
            ds.stencil_back.hash(state);
            ds.stencil_read_mask.hash(state);
            ds.stencil_write_mask.hash(state);
            ds.depth_bias.hash(state);
            ds.depth_bias_slope_scale.hash(state);
            ds.depth_bias_clamp.hash(state);
        }
        // Hash primitive state
        self.primitive.topology.hash(state);
        self.primitive.strip_index_format.hash(state);
        self.primitive.front_face.hash(state);
        self.primitive.cull_mode.hash(state);
        self.primitive.polygon_mode.hash(state);
        self.primitive.conservative.hash(state);
        // Hash multisample state
        self.multisample.count.hash(state);
        self.multisample.mask.hash(state);
        self.multisample.alpha_to_coverage_enabled.hash(state);
    }
}

impl PipelineKey {
    /// Create a new pipeline key
    pub fn new(
        vertex_shader: ShaderHandle,
        fragment_shader: Option<ShaderHandle>,
        vertex_layouts: Vec<VertexBufferLayout>,
        color_formats: Vec<Option<TextureFormat>>,
        depth_stencil: Option<DepthStencilState>,
        primitive: PrimitiveState,
        multisample: MultisampleState,
    ) -> Self {
        Self {
            vertex_shader,
            fragment_shader,
            vertex_layouts,
            color_formats,
            depth_stencil,
            primitive,
            multisample,
        }
    }
    
    /// Get the vertex shader
    pub fn vertex_shader(&self) -> ShaderHandle {
        self.vertex_shader
    }
    
    /// Get the fragment shader
    pub fn fragment_shader(&self) -> Option<ShaderHandle> {
        self.fragment_shader
    }
    
    /// Get the vertex buffer layouts
    pub fn vertex_layouts(&self) -> &[VertexBufferLayout] {
        &self.vertex_layouts
    }
    
    /// Get the color attachment formats
    pub fn color_formats(&self) -> &[Option<TextureFormat>] {
        &self.color_formats
    }
    
    /// Get the depth stencil state
    pub fn depth_stencil(&self) -> Option<&DepthStencilState> {
        self.depth_stencil.as_ref()
    }
    
    /// Get the primitive state
    pub fn primitive(&self) -> &PrimitiveState {
        &self.primitive
    }
    
    /// Get the multisample state
    pub fn multisample(&self) -> &MultisampleState {
        &self.multisample
    }
}

/// The PipelineCache manages pipeline objects to minimize creation overhead.
pub struct PipelineCache {
    /// Map of pipeline keys to pipeline handles
    pipelines: HashMap<PipelineKey, PipelineHandle>,
}

impl PipelineCache {
    /// Create a new pipeline cache
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
        }
    }
    
    /// Get or create a pipeline for the given key
    pub fn get_or_create(&mut self, 
                        key: PipelineKey, 
                        device: &mut dyn BackendDevice) -> Result<PipelineHandle, RenderError> {
        // Check if we already have this pipeline
        if let Some(handle) = self.pipelines.get(&key) {
            return Ok(*handle);
        }
        
        // Create new pipeline
        let pipeline = device.create_pipeline(
            key.vertex_shader(), 
            key.fragment_shader(), 
            key.vertex_layouts(), 
            key.color_formats(), 
            key.depth_stencil().cloned(), 
            key.primitive().clone(), 
            key.multisample().clone()
        )?;
        
        // Cache and return
        self.pipelines.insert(key, pipeline);
        Ok(pipeline)
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.pipelines.clear();
    }
    
    /// Number of cached pipelines
    pub fn count(&self) -> usize {
        self.pipelines.len()
    }
} 