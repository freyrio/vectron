#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

// Re-export platform-specific embedder implementations
#[cfg(target_os = "windows")]
pub use windows::Win32Embedder;

#[cfg(target_os = "macos")]
pub use macos::MacOSEmbedder;

#[cfg(target_os = "linux")]
pub use linux::LinuxEmbedder;

#[cfg(target_os = "android")]
pub use android::AndroidEmbedder;

#[cfg(target_os = "ios")]
pub use ios::IOSEmbedder;
