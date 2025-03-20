#![cfg(all(target_os = "windows", feature = "dx12"))]

pub mod dx12;

pub use dx12::DirectX12Backend;
