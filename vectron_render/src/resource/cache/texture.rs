// Texture cache for Vectron Render
//
// This module provides caching for texture resources.

use std::collections::HashMap;
use crate::core::{Uuid, TextureHandle, RenderError};
use crate::backend::resource::{TextureDesc, TextureUsage, TextureDimension};
use crate::resource::upload::PendingUpload;

/// Texture data descriptor
#[derive(Debug, Clone)]
pub struct TextureData {
    /// Raw texture data
    pub data: Vec<u8>,
    /// Width of the texture
    pub width: u32,
    /// Height of the texture
    pub height: u32,
    /// Depth of the texture (1 for 2D textures)
    pub depth: u32,
    /// Format of the texture
    pub format: crate::TextureFormat,
    /// Mip level count (1 for base level only)
    pub mip_level_count: u32,
    /// Sample count
    pub sample_count: u32,
}

impl TextureData {
    /// Create a new 2D texture data
    pub fn new_2d(
        data: Vec<u8>,
        width: u32,
        height: u32,
        format: crate::TextureFormat,
    ) -> Self {
        Self {
            data,
            width,
            height,
            depth: 1,
            format,
            mip_level_count: 1,
            sample_count: 1,
        }
    }
    
    /// Create a new 3D texture data
    pub fn new_3d(
        data: Vec<u8>,
        width: u32,
        height: u32,
        depth: u32,
        format: crate::TextureFormat,
    ) -> Self {
        Self {
            data,
            width,
            height,
            depth,
            format,
            mip_level_count: 1,
            sample_count: 1,
        }
    }
    
    /// With mip level count
    pub fn with_mip_levels(mut self, mip_level_count: u32) -> Self {
        self.mip_level_count = mip_level_count;
        self
    }
    
    /// With sample count
    pub fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }
    
    /// Convert to a backend texture descriptor
    pub fn to_texture_desc(&self, usage: TextureUsage) -> TextureDesc {
        TextureDesc {
            size: [self.width, self.height, self.depth],
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: if self.depth > 1 { TextureDimension::D3 } else { TextureDimension::D2 },
            format: self.format,
            usage,
            label: None,
        }
    }
}

/// Cache for texture resources
#[derive(Debug)]
pub struct TextureCache {
    /// Stored texture data mapped by handle
    textures: HashMap<TextureHandle, TextureData>,
    
    /// Mapping from our texture handles to backend GPU texture handles
    gpu_textures: HashMap<TextureHandle, Uuid>,
}

impl Default for TextureCache {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
            gpu_textures: HashMap::new(),
        }
    }
}

impl TextureCache {
    /// Create a new texture cache
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a texture and get a handle to it
    pub fn register_texture(&mut self, texture: TextureData) -> TextureHandle {
        let handle = TextureHandle::new();
        self.textures.insert(handle, texture);
        handle
    }
    
    /// Update an existing texture
    pub fn update_texture(&mut self, handle: TextureHandle, texture: TextureData) -> Result<(), RenderError> {
        if !self.textures.contains_key(&handle) {
            return Err(RenderError::ResourceNotFound(format!("Texture handle {:?} not found", handle)));
        }
        
        self.textures.insert(handle, texture);
        Ok(())
    }
    
    /// Get a reference to a texture
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&TextureData> {
        self.textures.get(&handle)
    }
    
    /// Check if a texture handle is valid
    pub fn contains_texture(&self, handle: TextureHandle) -> bool {
        self.textures.contains_key(&handle)
    }
    
    /// Remove a texture from the cache
    pub fn remove_texture(&mut self, handle: TextureHandle) -> Option<TextureData> {
        self.gpu_textures.remove(&handle);
        self.textures.remove(&handle)
    }
    
    /// Associate a GPU texture with a texture handle
    pub fn associate_gpu_texture(&mut self, handle: TextureHandle, gpu_handle: TextureHandle) -> Result<(), RenderError> {
        if !self.textures.contains_key(&handle) {
            return Err(RenderError::ResourceNotFound(format!("Texture handle {:?} not found", handle)));
        }
        
        self.gpu_textures.insert(handle, gpu_handle.uuid());
        Ok(())
    }
    
    /// Get the GPU texture associated with a texture handle
    pub fn get_gpu_texture(&self, handle: TextureHandle) -> Option<Uuid> {
        self.gpu_textures.get(&handle).copied()
    }
    
    /// Generate a pending upload for a texture
    pub fn generate_upload(&self, handle: TextureHandle) -> Result<PendingUpload, RenderError> {
        let texture = self.get_texture(handle)
            .ok_or_else(|| RenderError::ResourceNotFound(format!("Texture handle {:?} not found", handle)))?;
        
        let gpu_handle = self.get_gpu_texture(handle)
            .ok_or_else(|| RenderError::ResourceNotFound(format!("GPU texture for handle {:?} not found", handle)))?;
        
        Ok(PendingUpload::new_texture(
            gpu_handle,
            texture.data.clone(),
            [0, 0, 0],
            [texture.width, texture.height, texture.depth],
        ))
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.textures.clear();
        self.gpu_textures.clear();
    }
} 