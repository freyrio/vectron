use std::collections::HashMap;
use vectron_render::{backend::{
    resource::*,
}, core::error::BackendError};
use crate::conversion::*;

/// Resource manager for WGPU resources
pub struct WgpuResourceManager {
    // GPU resources
    buffers: HashMap<BufferHandle, wgpu::Buffer>,
    textures: HashMap<TextureHandle, wgpu::Texture>,
    texture_views: HashMap<TextureHandle, wgpu::TextureView>,
    samplers: HashMap<SamplerHandle, wgpu::Sampler>,
    bind_groups: HashMap<BindGroupHandle, wgpu::BindGroup>,
    pipelines: HashMap<PipelineHandle, wgpu::RenderPipeline>,
    shaders: HashMap<ShaderHandle, wgpu::ShaderModule>,
}

impl WgpuResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            textures: HashMap::new(),
            texture_views: HashMap::new(),
            samplers: HashMap::new(),
            bind_groups: HashMap::new(),
            pipelines: HashMap::new(),
            shaders: HashMap::new(),
        }
    }
    
    /// Register a buffer
    pub fn register_buffer(&mut self, handle: BufferHandle, buffer: wgpu::Buffer) {
        self.buffers.insert(handle, buffer);
    }
    
    /// Register a texture
    pub fn register_texture(&mut self, handle: TextureHandle, texture: wgpu::Texture) {
        self.textures.insert(handle, texture);
    }
    
    /// Register a texture view
    pub fn register_texture_view(&mut self, handle: TextureHandle, view: wgpu::TextureView) {
        self.texture_views.insert(handle, view);
    }
    
    /// Register a sampler
    pub fn register_sampler(&mut self, handle: SamplerHandle, sampler: wgpu::Sampler) {
        self.samplers.insert(handle, sampler);
    }
    
    /// Register a bind group
    pub fn register_bind_group(&mut self, handle: BindGroupHandle, bind_group: wgpu::BindGroup) {
        self.bind_groups.insert(handle, bind_group);
    }
    
    /// Register a pipeline
    pub fn register_pipeline(&mut self, handle: PipelineHandle, pipeline: wgpu::RenderPipeline) {
        self.pipelines.insert(handle, pipeline);
    }
    
    /// Register a shader
    pub fn register_shader(&mut self, handle: ShaderHandle, shader: wgpu::ShaderModule) {
        self.shaders.insert(handle, shader);
    }
    
    /// Get a buffer
    pub fn get_buffer(&self, handle: &BufferHandle) -> Option<&wgpu::Buffer> {
        self.buffers.get(handle)
    }
    
    /// Get a texture
    pub fn get_texture(&self, handle: &TextureHandle) -> Option<&wgpu::Texture> {
        self.textures.get(handle)
    }
    
    /// Get a texture view
    pub fn get_texture_view(&self, handle: &TextureHandle) -> Option<&wgpu::TextureView> {
        self.texture_views.get(handle)
    }
    
    /// Get a sampler
    pub fn get_sampler(&self, handle: &SamplerHandle) -> Option<&wgpu::Sampler> {
        self.samplers.get(handle)
    }
    
    /// Get a bind group
    pub fn get_bind_group(&self, handle: &BindGroupHandle) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(handle)
    }
    
    /// Get a pipeline
    pub fn get_pipeline(&self, handle: &PipelineHandle) -> Option<&wgpu::RenderPipeline> {
        self.pipelines.get(handle)
    }
    
    /// Get a shader
    pub fn get_shader(&self, handle: &ShaderHandle) -> Option<&wgpu::ShaderModule> {
        self.shaders.get(handle)
    }
    
    /// Remove a buffer
    pub fn remove_buffer(&mut self, handle: &BufferHandle) {
        self.buffers.remove(handle);
    }
    
    /// Remove a texture
    pub fn remove_texture(&mut self, handle: &TextureHandle) {
        self.textures.remove(handle);
        self.texture_views.remove(handle);
    }
    
    /// Remove a sampler
    pub fn remove_sampler(&mut self, handle: &SamplerHandle) {
        self.samplers.remove(handle);
    }
    
    /// Remove a bind group
    pub fn remove_bind_group(&mut self, handle: &BindGroupHandle) {
        self.bind_groups.remove(handle);
    }
    
    /// Remove a pipeline
    pub fn remove_pipeline(&mut self, handle: &PipelineHandle) {
        self.pipelines.remove(handle);
    }
    
    /// Remove a shader
    pub fn remove_shader(&mut self, handle: &ShaderHandle) {
        self.shaders.remove(handle);
    }
} 