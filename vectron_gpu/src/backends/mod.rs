// In src/backends/mod.rs
/* 
#[cfg(all(target_os = "windows", feature = "dx12"))]
pub mod directx;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(target_os = "macos")]
pub mod metal;

#[cfg(feature = "software")]
pub mod software;

mod common;

// Re-export the backend-specific implementations
#[cfg(all(target_os = "windows", feature = "dx12"))]
pub use directx::DirectX12Backend;

#[cfg(feature = "vulkan")]
pub use vulkan::VulkanBackend;

#[cfg(target_os = "macos")]
pub use metal::MetalBackend;

#[cfg(feature = "software")]
pub use software::SoftwareBackend;

use crate::backend::{GpuBackend, BackendConfig};
use crate::common::GpuError;

/// Create a GPU backend based on the current platform and features
pub fn create_backend() -> Box<dyn GpuBackend> {
    #[cfg(all(target_os = "windows", feature = "dx12"))]
    {
        return Box::new(DirectX12Backend::new());
    }
    
    #[cfg(all(not(target_os = "windows"), feature = "vulkan"))]
    {
        return Box::new(vulkan::VulkanBackend::new());
    }
    
    #[cfg(all(target_os = "macos", not(feature = "vulkan")))]
    {
        return Box::new(metal::MetalBackend::new());
    }
    
    #[cfg(feature = "software")]
    {
        return Box::new(software::SoftwareBackend::new());
    }
    
    #[cfg(not(any(
        all(target_os = "windows", feature = "dx12"),
        all(not(target_os = "windows"), feature = "vulkan"),
        all(target_os = "macos", not(feature = "vulkan")),
        feature = "software"
    )))]
    {
        panic!("No suitable GPU backend found for the current platform and features!");
    }
}
    */