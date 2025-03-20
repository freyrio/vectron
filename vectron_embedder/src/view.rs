use crate::embedder::{Embedder, EmbedderError, Surface};
use std::error::Error;
use std::fmt;

/// View configuration
#[derive(Debug, Clone)]
pub struct ViewConfig {
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub parent: Option<ViewHandle>,
}

/// View handle type alias
pub type ViewHandle = u64;

/// View-specific errors
#[derive(Debug)]
pub enum ViewError {
    CreationFailed(String),
    InvalidHandle,
    OperationFailed(String),
}

impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewError::CreationFailed(msg) => write!(f, "View creation failed: {}", msg),
            ViewError::InvalidHandle => write!(f, "Invalid view handle"),
            ViewError::OperationFailed(msg) => write!(f, "View operation failed: {}", msg),
        }
    }
}

impl Error for ViewError {}

/// View embedder trait for mobile platforms
pub trait ViewEmbedder: Embedder<Handle = ViewHandle> {
    /// Create a new view with the given configuration
    fn create_view(&mut self, config: ViewConfig) -> Result<ViewHandle, ViewError>;
    
    /// Destroy a view
    fn destroy_view(&mut self, handle: ViewHandle);
    
    /// Set view visibility
    fn set_view_visibility(&mut self, handle: ViewHandle, visible: bool);
    
    /// Set view size
    fn set_view_size(&mut self, handle: ViewHandle, width: u32, height: u32);
    
    /// Get view size
    fn get_view_size(&self, handle: ViewHandle) -> (u32, u32);
    
    /// Set view as subview of another view
    fn set_view_parent(&mut self, handle: ViewHandle, parent: ViewHandle);
    
    /// Add native view as a child
    fn add_native_subview(&mut self, handle: ViewHandle, native_view: *mut std::ffi::c_void);
}
