#[cfg(all(target_os = "windows", feature = "dx12"))]
pub mod directx;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(target_os = "macos")]
pub mod metal;

#[cfg(feature = "software")]
pub mod software;

/// Create a GPU backend based on the current platform and features
pub fn create_backend() -> Box<dyn crate::backend::GpuBackend> {
    #[cfg(all(target_os = "windows", feature = "dx12"))]
    {
        Box::new(directx::dx12::DirectX12Backend::new())
    }
    #[cfg(all(not(target_os = "windows"), feature = "vulkan"))]
    {
        Box::new(vulkan::VulkanBackend::new())
    }
    #[cfg(all(target_os = "macos", not(feature = "vulkan")))]
    {
        Box::new(metal::MetalBackend::new())
    }
    #[cfg(not(any(
        all(target_os = "windows", feature = "dx12"),
        all(not(target_os = "windows"), feature = "vulkan"),
        all(target_os = "macos", not(feature = "vulkan"))
    )))]
    {
        panic!("No suitable GPU backend found for the current platform and features!")
    }
}
