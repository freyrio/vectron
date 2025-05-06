use crate::embedder::{Embedder, EmbedderError, Surface};
use std::error::Error;
use std::fmt;
use raw_window_handle::{HasWindowHandle, HasDisplayHandle, RawWindowHandle, RawDisplayHandle, HandleError};

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
    pub parent: Option<WindowId>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self {
            title: "Vectron Window".to_string(),
            width: 800,
            height: 600,
            resizable: true,
            decorated: true,
            visible: true,
            position: None,
            min_size: None,
            max_size: None,
            parent: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_decorated(mut self, decorated: bool) -> Self {
        self.decorated = decorated;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position = Some((x, y));
        self
    }

    pub fn with_min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    pub fn with_max_size(mut self, width: u32, height: u32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    pub fn with_parent(mut self, parent: WindowId) -> Self {
        self.parent = Some(parent);
        self
    }
}

/// Window identifier type
pub type WindowId = u64;

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

/// Window struct representing a platform window
pub struct Window {
    // The embedder that created this window
    embedder: *mut std::ffi::c_void,
    // The platform-specific window ID
    id: WindowId,
    // Native window handle
    native_handle: *mut std::ffi::c_void,
    // Window size
    width: u32,
    height: u32,
}

impl Window {
    pub(crate) fn new(
        embedder: *mut std::ffi::c_void,
        id: WindowId,
        native_handle: *mut std::ffi::c_void,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            embedder,
            id,
            native_handle,
            width,
            height,
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn native_handle(&self) -> *mut std::ffi::c_void {
        self.native_handle
    }
}

// Add these lines if you are sure that window handle manipulation is thread-safe
// (e.g., all interactions with the window occur on the main thread).
unsafe impl Send for Window {}
unsafe impl Sync for Window {}

/// Window embedder trait for desktop platforms
pub trait WindowEmbedder: Embedder<Handle = WindowId> {
    /// Create a new window with the given configuration
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowId, WindowError>;
    
    /// Get a window object from a window ID
    fn get_window(&self, handle: &WindowId) -> Option<Window>;
    
    /// Destroy a window
    fn destroy_window(&mut self, handle: &WindowId);
    
    /// Show a window
    fn show_window(&mut self, handle: &WindowId);
    
    /// Hide a window
    fn hide_window(&mut self, handle: &WindowId);
    
    /// Set window title
    fn set_window_title(&mut self, handle: &WindowId, title: &str);
    
    /// Set window size
    fn set_window_size(&mut self, handle: &WindowId, width: u32, height: u32);
    
    /// Get window size
    fn get_window_size(&self, handle: &WindowId) -> (u32, u32);
    
    /// Set window position
    fn set_window_position(&mut self, handle: &WindowId, x: i32, y: i32);
    
    /// Get window position
    fn get_window_position(&self, handle: &WindowId) -> (i32, i32);
    
    /// Set window as child of another window
    fn set_window_parent(&mut self, handle: &WindowId, parent: &WindowId);
}

#[cfg(target_os = "windows")]
impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        use raw_window_handle::Win32WindowHandle;
        use std::num::NonZeroIsize;
        
        let mut handle = Win32WindowHandle::new(
            // Safely create a NonZeroIsize from the pointer
            NonZeroIsize::new(self.native_handle as isize)
                .ok_or(HandleError::NotSupported)?
        );
        
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                RawWindowHandle::Win32(handle)
            ))
        }
    }
}

#[cfg(target_os = "windows")]
impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        use raw_window_handle::WindowsDisplayHandle;
        
        let handle = WindowsDisplayHandle::new();
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                RawDisplayHandle::Windows(handle)
            ))
        }
    }
}

// Add other platform-specific implementations as needed
#[cfg(target_os = "macos")]
impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        use raw_window_handle::AppKitWindowHandle;
        
        let mut handle = AppKitWindowHandle::new(self.native_handle as _);
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                RawWindowHandle::AppKit(handle)
            ))
        }
    }
}

#[cfg(target_os = "macos")]
impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        use raw_window_handle::AppKitDisplayHandle;
        
        let handle = AppKitDisplayHandle::new();
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                RawDisplayHandle::AppKit(handle)
            ))
        }
    }
}

// Linux X11
#[cfg(all(target_os = "linux", feature = "x11"))]
impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        use raw_window_handle::XlibWindowHandle;
        
        let mut handle = XlibWindowHandle::new(self.native_handle as _);
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                RawWindowHandle::Xlib(handle)
            ))
        }
    }
}

#[cfg(all(target_os = "linux", feature = "x11"))]
impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        use raw_window_handle::XlibDisplayHandle;
        
        let handle = XlibDisplayHandle::new();
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                RawDisplayHandle::Xlib(handle)
            ))
        }
    }
}
