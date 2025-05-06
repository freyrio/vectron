//! Graphics module: includes device management, surface/swapchain handling, and resources.

pub mod device;
pub mod surface;
pub mod viewport;

// Re-export key structs for easier access from parent modules if desired
pub use device::DeviceContext;
pub use surface::Surface;
pub use viewport::Viewport;
