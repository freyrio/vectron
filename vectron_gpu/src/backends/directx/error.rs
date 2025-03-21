// In src/backends/directx/error.rs
use crate::common::GpuError;
use windows::core::Error as WindowsError;

pub(super) trait DirectXErrorExt {
    fn to_gpu_error(self, context: &str) -> GpuError;
}

impl DirectXErrorExt for WindowsError {
    fn to_gpu_error(self, context: &str) -> GpuError {
        let code = self.code().0;
        
        // Map specific DirectX error codes to appropriate GpuError variants
        match code {
            // Device removed/reset (0x887A0005 - DXGI_ERROR_DEVICE_REMOVED)
            0x887A0005 => GpuError::DeviceLost(format!("{}: {}", context, self)),
            
            // Out of memory (0x8007000E - E_OUTOFMEMORY)
            0x8007000E => GpuError::OutOfMemory,
            
            // Invalid arguments (0x80070057 - E_INVALIDARG)
            0x80070057 => GpuError::InvalidArgument(format!("{}: {}", context, self)),
            
            // General backend error for other codes
            _ => GpuError::BackendError {
                backend: "DirectX12".to_string(),
                message: format!("{}: {}", context, self),
                code: Some(code as i32),
                source: Some(Box::new(self)),
            },
        }
    }
}