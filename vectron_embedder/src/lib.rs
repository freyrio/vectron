mod embedder;
mod window;
mod view;
mod platform;

pub use embedder::{Embedder, EmbedderConfig, EmbedderError, Event, Surface, InputEvent, Key, MouseButton};
pub use window::{WindowConfig, WindowEmbedder, WindowId, WindowError, Window};
pub use view::{ViewConfig, ViewEmbedder, ViewHandle, ViewError};
pub use platform::*;

// Re-export the platform-specific embedder implementation
#[cfg(target_os = "windows")]
pub use platform::Win32Embedder;

// Create a type alias for the platform-specific embedder
#[cfg(target_os = "windows")]
pub type PlatformEmbedder = Win32Embedder;

// Export a function to create the appropriate embedder for the current platform
pub fn create_embedder() -> PlatformEmbedder {
    #[cfg(target_os = "windows")]
    {
        Win32Embedder::new()
    }
    #[cfg(not(target_os = "windows"))]
    {
        compile_error!("Unsupported platform");
    }
}
