use crate::embedder::{Embedder, EmbedderError, Surface};
use std::error::Error;
use std::fmt;

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub decorated: bool,
    pub visible: bool,
    pub position: Option<(i32, i32)>,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub parent: Option<WindowHandle>,
}

/// Window handle type alias
pub type WindowHandle = u64;

/// Window-specific errors
#[derive(Debug)]
pub enum WindowError {
    CreationFailed(String),
    InvalidHandle,
    OperationFailed(String),
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowError::CreationFailed(msg) => write!(f, "Window creation failed: {}", msg),
            WindowError::InvalidHandle => write!(f, "Invalid window handle"),
            WindowError::OperationFailed(msg) => write!(f, "Window operation failed: {}", msg),
        }
    }
}

impl Error for WindowError {}

/// Window embedder trait for desktop platforms
pub trait WindowEmbedder: Embedder<Handle = WindowHandle> {
    /// Create a new window with the given configuration
    fn create_window(&mut self, config: WindowConfig) -> Result<WindowHandle, WindowError>;
    
    /// Destroy a window
    fn destroy_window(&mut self, handle: WindowHandle);
    
    /// Show a window
    fn show_window(&mut self, handle: WindowHandle);
    
    /// Hide a window
    fn hide_window(&mut self, handle: WindowHandle);
    
    /// Set window title
    fn set_window_title(&mut self, handle: WindowHandle, title: &str);
    
    /// Set window size
    fn set_window_size(&mut self, handle: WindowHandle, width: u32, height: u32);
    
    /// Get window size
    fn get_window_size(&self, handle: WindowHandle) -> (u32, u32);
    
    /// Set window position
    fn set_window_position(&mut self, handle: WindowHandle, x: i32, y: i32);
    
    /// Get window position
    fn get_window_position(&self, handle: WindowHandle) -> (i32, i32);
    
    /// Set window as child of another window
    fn set_window_parent(&mut self, handle: WindowHandle, parent: WindowHandle);
}
