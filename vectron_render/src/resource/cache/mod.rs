// Resource caching for Vectron Render
//
// This module provides resource caching for various rendering resources.

pub mod geometry;
pub mod texture;
pub mod pipeline;

use crate::core::{RenderError, BufferHandle, TextureHandle, PipelineHandle};
use crate::backend::device::BackendDevice;
use crate::backend::resource::{BufferDesc, BufferUsage, TextureUsage};
use crate::resource::buffer::GeometryBuffer;
use crate::resource::upload::UploadQueue;
use crate::core::types::{VertexBufferHandle, IndexBufferHandle};
use crate::GeometryHandle;

pub use self::texture::TextureData;
pub use self::pipeline::PipelineKey;

/// Combined resource cache for all resource types
#[derive(Debug)]
pub struct ResourceCache<V> {
    /// Cache for geometry resources
    pub geometry_cache: geometry::GeometryCache<V>,
    /// Cache for texture resources
    pub texture_cache: texture::TextureCache,
    /// Cache for pipeline resources
    pub pipeline_cache: pipeline::PipelineCache,
    /// Queue for pending resource uploads
    pub upload_queue: UploadQueue,
}

impl<V: Clone + 'static> Default for ResourceCache<V> {
    fn default() -> Self {
        Self {
            geometry_cache: geometry::GeometryCache::new(),
            texture_cache: texture::TextureCache::new(),
            pipeline_cache: pipeline::PipelineCache::new(),
            upload_queue: UploadQueue::new(),
        }
    }
}

impl<V: Clone + 'static> ResourceCache<V> {
    /// Create a new resource cache
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a geometry buffer and prepare it for upload
    pub fn register_geometry(
        &mut self,
        geometry: GeometryBuffer<V>,
        device: &mut dyn BackendDevice,
    ) -> Result<GeometryHandle, RenderError> {
        // Register the geometry with the cache
        let handle = self.geometry_cache.register_geometry(geometry);
        let geom = self.geometry_cache.get_geometry(handle)
            .ok_or_else(|| RenderError::ResourceNotFound("Geometry not found immediately after registration".to_string()))?;
        
        // Create vertex buffer
        let vertex_size = std::mem::size_of::<V>() as u64 * geom.vertex_count() as u64;
        let vertex_desc = BufferDesc {
            size: vertex_size,
            usage: BufferUsage::VERTEX.union(BufferUsage::COPY_DST),
            mapped_at_creation: false,
            label: Some(format!("Vertex Buffer {:?}", handle)),
        };
        
        let vertex_buffer = device.create_buffer(&vertex_desc)?;
        let vertex_handle = VertexBufferHandle::with_id(vertex_buffer.id());
        
        // Create index buffer if needed
        let index_handle = if let Some(indices) = &geom.indices {
            let index_size = std::mem::size_of::<u32>() as u64 * indices.len() as u64;
            let index_desc = BufferDesc {
                size: index_size,
                usage: BufferUsage::INDEX.union(BufferUsage::COPY_DST),
                mapped_at_creation: false,
                label: Some(format!("Index Buffer {:?}", handle)),
            };
            
            let index_buffer = device.create_buffer(&index_desc)?;
            Some(IndexBufferHandle::with_id(index_buffer.id()))
        } else {
            None
        };
        
        // Associate GPU buffers with the geometry
        self.geometry_cache.associate_gpu_buffers(handle, vertex_handle, index_handle)?;
        
        // Generate and queue uploads
        let uploads = self.geometry_cache.generate_uploads(handle)?;
        for upload in uploads {
            self.upload_queue.enqueue(upload);
        }
        
        Ok(handle)
    }
    
    /// Register a texture and prepare it for upload
    pub fn register_texture(
        &mut self,
        texture: TextureData,
        device: &mut dyn BackendDevice,
    ) -> Result<TextureHandle, RenderError> {
        // Register the texture with the cache
        let handle = self.texture_cache.register_texture(texture);
        let tex = self.texture_cache.get_texture(handle)
            .ok_or_else(|| RenderError::ResourceNotFound("Texture not found immediately after registration".to_string()))?;
        
        // Create texture descriptor
        let texture_desc = tex.to_texture_desc(
            TextureUsage::TEXTURE_BINDING.union(TextureUsage::COPY_DST)
        );
        
        // Create GPU texture
        let gpu_texture_handle = device.create_texture(&texture_desc)?;
        
        // Associate GPU texture with the handle (use our TextureHandle constructor for the GPU handle)
        self.texture_cache.associate_gpu_texture(handle, TextureHandle::with_id(gpu_texture_handle.uuid()))?;
        
        // Generate and queue upload
        let upload = self.texture_cache.generate_upload(handle)?;
        self.upload_queue.enqueue(upload);
        
        Ok(handle)
    }
    
    /// Get or create a pipeline
    pub fn get_or_create_pipeline(
        &mut self,
        key: PipelineKey,
        desc: crate::backend::resource::PipelineDesc,
        device: &mut dyn BackendDevice,
    ) -> Result<PipelineHandle, RenderError> {
        self.pipeline_cache.get_or_create_pipeline(key, desc, device)
    }
    
    /// Upload all pending resources to the GPU
    pub fn upload_pending_resources(&mut self, device: &mut dyn BackendDevice) -> Result<(), RenderError> {
        while self.upload_queue.has_pending_uploads() {
            if let Some(upload) = self.upload_queue.dequeue() {
                match upload.resource_type {
                    crate::core::types::ResourceType::Buffer => {
                        // Use the constructor from the core module that accepts a UUID
                        let buffer_handle = BufferHandle::with_id(upload.handle);
                        device.update_buffer(buffer_handle, &upload.data, upload.offset)?;
                    },
                    crate::core::types::ResourceType::Texture => {
                        if let (Some(offset), Some(size)) = (upload.offset_3d, upload.size_3d) {
                            // Use the constructor from the core module that accepts a UUID
                            let texture_handle = TextureHandle::with_id(upload.handle);
                            device.update_texture(texture_handle, &upload.data, offset, size)?;
                        } else {
                            return Err(RenderError::InvalidOperation(
                                "Texture upload missing offset_3d or size_3d".to_string()
                            ));
                        }
                    },
                    _ => {
                        return Err(RenderError::UnsupportedFeature(
                            format!("Uploading resource type {:?} not supported", upload.resource_type)
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Clear all caches
    pub fn clear(&mut self) {
        self.geometry_cache.clear();
        self.texture_cache.clear();
        self.pipeline_cache.clear();
        self.upload_queue.clear();
    }
} 