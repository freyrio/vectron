/*!
 * Resource binding for shaders
 */

use crate::api::bare::resources::{ResourceId, BufferHandle, PipelineHandle};
use crate::error::RenderError;
use super::context::CommandBuffer;

/// Types of resource bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceBindingType {
    /// Buffer resource
    Buffer,
    
    /// Texture resource
    Texture,
    
    /// Sampler resource
    Sampler,
}

/// Resource binding information
#[derive(Debug, Clone)]
pub struct ResourceBinding {
    /// Type of resource
    pub resource_type: ResourceBindingType,
    
    /// Resource ID
    pub handle: ResourceId,
    
    /// Binding point in the shader
    pub binding: u32,
    
    /// Descriptor set
    pub set: u32,
}

/// Bind resources to a pipeline
pub fn bind_resources(
    cmd: &mut CommandBuffer,
    pipeline: PipelineHandle,
    bindings: &[ResourceBinding]
) -> Result<(), RenderError> {
    // In a real implementation, we'd set the pipeline and bind resources
    // For now, we just log it
    log::debug!("Binding {} resources to pipeline {:?}", bindings.len(), pipeline);
    
    Ok(())
}

/// A group of related resource bindings
#[derive(Debug, Default)]
pub struct BindingGroup {
    /// Bindings in this group
    bindings: Vec<ResourceBinding>,
    
    /// Set index for this group
    set: u32,
}

impl BindingGroup {
    /// Create a new binding group
    pub fn new(set: u32) -> Self {
        Self {
            bindings: Vec::new(),
            set,
        }
    }
    
    /// Add a buffer binding
    pub fn add_buffer(&mut self, handle: ResourceId, binding: u32) -> &mut Self {
        self.bindings.push(ResourceBinding {
            resource_type: ResourceBindingType::Buffer,
            handle,
            binding,
            set: self.set,
        });
        self
    }
    
    /// Add a texture binding
    pub fn add_texture(&mut self, handle: ResourceId, binding: u32) -> &mut Self {
        self.bindings.push(ResourceBinding {
            resource_type: ResourceBindingType::Texture,
            handle,
            binding,
            set: self.set,
        });
        self
    }
    
    /// Add a sampler binding
    pub fn add_sampler(&mut self, handle: ResourceId, binding: u32) -> &mut Self {
        self.bindings.push(ResourceBinding {
            resource_type: ResourceBindingType::Sampler,
            handle,
            binding,
            set: self.set,
        });
        self
    }
    
    /// Get all bindings in this group
    pub fn bindings(&self) -> &[ResourceBinding] {
        &self.bindings
    }
} 