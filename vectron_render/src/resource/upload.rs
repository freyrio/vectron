// Resource upload management for Vectron Render
//
// This module provides functionality for managing uploads of resources to the GPU.

use std::collections::VecDeque;
use crate::core::error::RenderError;
use crate::core::types::{ResourceType, Uuid};

/// A pending resource upload to the GPU
#[derive(Debug)]
pub struct PendingUpload {
    /// The type of resource being uploaded
    pub resource_type: ResourceType,
    /// The handle to the resource
    pub handle: Uuid,
    /// The data to upload
    pub data: Vec<u8>,
    /// Offset into the resource where the data should be uploaded
    pub offset: u64,
    /// Size of data to upload
    pub size: u64,
    /// For 2D/3D resources, the offset in each dimension
    pub offset_3d: Option<[u32; 3]>,
    /// For 2D/3D resources, the size in each dimension
    pub size_3d: Option<[u32; 3]>,
}

impl PendingUpload {
    /// Create a new pending buffer upload
    pub fn new_buffer(handle: Uuid, data: Vec<u8>, offset: u64) -> Self {
        let size = data.len() as u64;
        Self {
            resource_type: ResourceType::Buffer,
            handle,
            data,
            offset,
            size,
            offset_3d: None,
            size_3d: None,
        }
    }
    
    /// Create a new pending texture upload
    pub fn new_texture(handle: Uuid, data: Vec<u8>, offset_3d: [u32; 3], size_3d: [u32; 3]) -> Self {
        let size = data.len() as u64;
        Self {
            resource_type: ResourceType::Texture,
            handle,
            data,
            offset: 0,
            size,
            offset_3d: Some(offset_3d),
            size_3d: Some(size_3d),
        }
    }
}

/// Queue for managing pending uploads to the GPU
#[derive(Debug, Default)]
pub struct UploadQueue {
    pending_uploads: VecDeque<PendingUpload>,
}

impl UploadQueue {
    /// Create a new upload queue
    pub fn new() -> Self {
        Self {
            pending_uploads: VecDeque::new(),
        }
    }
    
    /// Add a pending upload to the queue
    pub fn enqueue(&mut self, upload: PendingUpload) {
        self.pending_uploads.push_back(upload);
    }
    
    /// Get the next pending upload from the queue
    pub fn dequeue(&mut self) -> Option<PendingUpload> {
        self.pending_uploads.pop_front()
    }
    
    /// Check if the queue has any pending uploads
    pub fn has_pending_uploads(&self) -> bool {
        !self.pending_uploads.is_empty()
    }
    
    /// Get the number of pending uploads
    pub fn pending_count(&self) -> usize {
        self.pending_uploads.len()
    }
    
    /// Clear all pending uploads
    pub fn clear(&mut self) {
        self.pending_uploads.clear();
    }
} 