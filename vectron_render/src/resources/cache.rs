/*!
 * Resource cache for efficiently managing rendering resources
 */

use std::collections::HashMap;
use crate::api::bare::resources::{ResourceId, GeometryHandle, TextureHandle, BufferHandle};
use crate::error::RenderError;

#[cfg(feature = "caching")]
use lru::LruCache;

/// Resource pool for a specific resource type
pub struct ResourcePool<T> {
    /// Resources stored by ID
    resources: HashMap<ResourceId, T>,
    
    /// Next available resource ID
    next_id: ResourceId,
}

impl<T> ResourcePool<T> {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            next_id: 1, // Start from 1, 0 is reserved for invalid/null resources
        }
    }
    
    pub fn insert(&mut self, resource: T) -> ResourceId {
        let id = self.next_id;
        self.resources.insert(id, resource);
        self.next_id += 1;
        id
    }
    
    pub fn get(&self, id: ResourceId) -> Option<&T> {
        self.resources.get(&id)
    }
    
    pub fn get_mut(&mut self, id: ResourceId) -> Option<&mut T> {
        self.resources.get_mut(&id)
    }
    
    pub fn remove(&mut self, id: ResourceId) -> Option<T> {
        self.resources.remove(&id)
    }
}

/// Resource cache for managing rendering resources
pub struct ResourceCache {
    /// Geometry resources
    geometries: ResourcePool<GeometryResource>,
    
    /// Texture resources
    textures: ResourcePool<TextureResource>,
    
    /// Font resources
    fonts: ResourcePool<FontResource>,
    
    /// GPU resource mapping - geometry buffers
    geometry_buffers: HashMap<GeometryHandle, (BufferHandle, BufferHandle)>,
    
    /// GPU resource mapping - texture handles
    texture_handles: HashMap<TextureHandle, BufferHandle>,
    
    /// Path tessellation cache
    #[cfg(feature = "caching")]
    path_cache: LruCache<PathCacheKey, GeometryHandle>,
}

/// Key for the path tessellation cache
#[cfg(feature = "caching")]
#[derive(Clone, PartialEq, Eq, Hash)]
struct PathCacheKey {
    path_hash: u64,
    style_hash: u64,
    transform_hash: u64,
}

impl ResourceCache {
    /// Create a new resource cache
    pub fn new() -> Self {
        let mut cache = Self {
            geometries: ResourcePool::new(),
            textures: ResourcePool::new(),
            fonts: ResourcePool::new(),
            geometry_buffers: HashMap::new(),
            texture_handles: HashMap::new(),
            
            #[cfg(feature = "caching")]
            path_cache: LruCache::new(100), // Cache up to 100 path tessellations
        };
        
        cache
    }
    
    /// Create a geometry resource and return a handle to it
    pub fn create_geometry(&mut self, desc: GeometryDesc) -> GeometryHandle {
        let resource = GeometryResource::new(desc);
        let id = self.geometries.insert(resource);
        GeometryHandle(id)
    }
    
    /// Create a texture resource and return a handle to it
    pub fn create_texture(&mut self, width: u32, height: u32, format: PixelFormat, data: &[u8]) -> TextureHandle {
        let resource = TextureResource::new(width, height, format, data.to_vec());
        let id = self.textures.insert(resource);
        TextureHandle(id)
    }
    
    /// Get a geometry resource by handle
    pub fn get_geometry(&self, handle: GeometryHandle) -> Option<&GeometryResource> {
        self.geometries.get(handle.0)
    }
    
    /// Get a texture resource by handle
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&TextureResource> {
        self.textures.get(handle.0)
    }
    
    /// Get a mutable reference to a geometry resource
    pub fn get_geometry_mut(&mut self, handle: GeometryHandle) -> Option<&mut GeometryResource> {
        self.geometries.get_mut(handle.0)
    }
    
    /// Get GPU buffer handles for a geometry resource
    pub fn get_geometry_buffers(&self, handle: GeometryHandle) -> Option<(BufferHandle, BufferHandle)> {
        self.geometry_buffers.get(&handle).cloned()
    }
}

// Import necessary types from other modules
use super::geometry::{GeometryResource, GeometryDesc};
use super::texture::{TextureResource, PixelFormat};
use super::font::FontResource; 