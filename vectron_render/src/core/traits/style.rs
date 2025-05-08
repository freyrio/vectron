use std::any::Any;
use crate::core::error::RenderError;
use crate::core::types::GeometryHandle;
use crate::core::traits::renderer::Renderer;

/// A trait for any kind of stylistic properties that can be applied to renderables.
/// 
/// Styles define the visual appearance of renderable elements, such as fill colors,
/// stroke properties, gradients, textures, etc. This abstraction allows for
/// different rendering backends and implementations to handle styling.
pub trait Style: Any + Send + Sync {
    /// Apply this style to the specified geometry during rendering.
    ///
    /// This is called by the renderer when applying visual styles to tessellated geometry.
    /// It should configure the rendering pipeline appropriately for this style.
    fn apply(&self, renderer: &mut dyn Renderer, geometry_handle: GeometryHandle) -> Result<(), RenderError>;
    
    /// Return true if this style can be batched with others of the same type
    /// for more efficient rendering.
    ///
    /// Batching compatible styles allows for better performance through reduced
    /// pipeline state changes and draw calls.
    fn is_batchable(&self) -> bool {
        false
    }
    
    /// Get the z-index for this style (determines rendering order)
    fn z_index(&self) -> i32 {
        0
    }
    
    /// Returns self as Any for downcast operations
    fn as_any(&self) -> &dyn Any;
    
    /// Returns mutable self as Any for downcast operations
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    /// Create a clone of this style
    fn clone_style(&self) -> Box<dyn Style>;
}