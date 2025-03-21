// In src/backends/directx/device.rs
use crate::backend::BackendConfig;
use crate::common::GpuError;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Foundation::*;
use windows::core::Interface;

use super::error::DirectXErrorExt;
use super::dx12::DirectX12Backend;
use super::debug::DirectX12Debug;

// Extension trait for device operations
pub(super) trait DeviceExt {
    fn init_impl(&mut self, config: BackendConfig) -> Result<(), GpuError>;
    fn query_capabilities(&mut self) -> Result<(), GpuError>;
    fn wait_for_gpu(&self) -> Result<(), GpuError>;
    fn shutdown_impl(&mut self);
}

impl DeviceExt for DirectX12Backend {
    fn init_impl(&mut self, config: BackendConfig) -> Result<(), GpuError> {
        // Enable debug layer in debug mode or when explicitly requested
        let debug_layer = config.enable_debug || cfg!(debug_assertions);
        
        // Create the DXGI factory
        let dxgi_factory_flags = if debug_layer {
            DXGI_CREATE_FACTORY_DEBUG
        } else {
            windows::Win32::Graphics::Dxgi::DXGI_CREATE_FACTORY_FLAGS(0)
        };
        
        if debug_layer {
            unsafe {
                let mut debug: Option<ID3D12Debug> = None;
                if let Ok(_) = D3D12GetDebugInterface(&mut debug) {
                    if let Some(debug_interface) = debug {
                        debug_interface.EnableDebugLayer();
                        println!("DirectX 12 debug layer enabled");
                    }
                }
            }
        }
        
        let dxgi_factory: IDXGIFactory4 = unsafe {
            CreateDXGIFactory2(dxgi_factory_flags).map_err(|e| 
                e.to_gpu_error("Failed to create DXGI factory"))?
        };
        
        // Find a suitable adapter (graphics card)
        let adapter = unsafe {
            let mut adapter: Option<IDXGIAdapter1> = None;
            
            for i in 0.. {
                let tmp = dxgi_factory.EnumAdapters1(i);
                if tmp.is_err() {
                    break;
                }
                
                let tmp = tmp.unwrap();
                let desc = tmp.GetDesc1()?;
                
                // Skip software adapters
                let software_flag = DXGI_ADAPTER_FLAG_SOFTWARE.0 as i32;
                let none_flag = DXGI_ADAPTER_FLAG_NONE.0 as i32;
                if ((desc.Flags as i32) & software_flag) != none_flag {
                    continue;
                }
                
                // Check if this adapter supports Direct3D 12
                let mut device = None;
                if D3D12CreateDevice(&tmp, D3D_FEATURE_LEVEL_11_0, &mut device).is_ok() {
                    // Print adapter info
                    let name = String::from_utf16_lossy(
                        &desc.Description[0..desc.Description.iter().position(|&c| c == 0).unwrap_or(desc.Description.len())]
                    );
                    println!("Selected GPU: {}", name);
                    
                    adapter = Some(tmp);
                    break;
                }
            }
            
            adapter.ok_or_else(|| GpuError::InitializationFailed("No DirectX 12 compatible adapter found".to_string()))?
        };
        
        // Create the Direct3D 12 device
        let mut device: Option<ID3D12Device> = None;
        unsafe {
            D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)
                .map_err(|e| e.to_gpu_error("Failed to create D3D12 device"))?;
        }
        let device = device.unwrap();

        // Create command queue
        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0 as i32,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        };
        
        let command_queue = unsafe {
            device.CreateCommandQueue(&queue_desc)
                .map_err(|e| e.to_gpu_error("Failed to create command queue"))?
        };
        
        // Create fence for synchronization
        let fence = unsafe {
            device.CreateFence(0, D3D12_FENCE_FLAG_NONE)
                .map_err(|e| e.to_gpu_error("Failed to create fence"))?
        };
        
        // Create an event handle for fence completion
        let fence_event = unsafe {
            CreateEventW(None, false, false, None)
                .map_err(|e| e.to_gpu_error("Failed to create fence event"))?
        };
        
        // Initialize debug helpers if enabled
        #[cfg(feature = "validation")]
        let debug = if debug_layer {
            match DirectX12Debug::new(&device) {
                Ok(debug) => Some(debug),
                Err(e) => {
                    println!("Warning: Failed to initialize DX12 debug utilities: {}", e);
                    None
                }
            }
        } else {
            None
        };
        
        // Store the created objects
        self.device = Some(device);
        self.command_queue = Some(command_queue);
        self.dxgi_factory = Some(dxgi_factory);
        self.fence = Some(fence);
        self.fence_event = fence_event;
        self.fence_value = 1;
        
        #[cfg(feature = "validation")]
        {
            self.debug = debug;
        }
        
        // Query device capabilities
        self.query_capabilities()?;
        
        Ok(())
    }
    
    fn query_capabilities(&mut self) -> Result<(), GpuError> {
        let device = self.device.as_ref().ok_or_else(|| 
            GpuError::InvalidOperation("Device not initialized".to_string()))?;
        
        // Query texture dimension limits
        let mut feature_data_options = D3D12_FEATURE_DATA_D3D12_OPTIONS::default();
        let hr = unsafe {
            device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS,
                &mut feature_data_options as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS>() as u32,
            )
        };
        
        if hr.is_ok() {
            self.capabilities.max_texture_size = 16384;
            self.capabilities.max_color_attachments = 8;
        }
        
        // Check for compute shader support (always true in DX12)
        self.capabilities.supports_compute = true;
        
        // Check for storage buffer support (UAVs)
        self.capabilities.supports_storage_buffers = true;
        
        // Check for float texture support
        let mut format_support = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            Support1: D3D12_FORMAT_SUPPORT1_NONE,
            Support2: D3D12_FORMAT_SUPPORT2_NONE,
        };
        
        let hr = unsafe {
            device.CheckFeatureSupport(
                D3D12_FEATURE_FORMAT_SUPPORT,
                &mut format_support as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_FORMAT_SUPPORT>() as u32,
            )
        };
        
        if hr.is_ok() {
            self.capabilities.supports_float_textures = (format_support.Support1 & D3D12_FORMAT_SUPPORT1_TEXTURE2D) != D3D12_FORMAT_SUPPORT1(0);
        }
        
        // Set constant buffer size limit (64KB in DirectX 12)
        self.capabilities.max_uniform_buffer_size = 65536;
        
        Ok(())
    }
    
    fn wait_for_gpu(&self) -> Result<(), GpuError> {
        let command_queue = self.command_queue.as_ref().ok_or_else(||
            GpuError::InvalidOperation("Command queue not initialized".to_string()))?;
            
        let fence = self.fence.as_ref().ok_or_else(||
            GpuError::InvalidOperation("Fence not initialized".to_string()))?;
        
        let fence_value = self.fence_value;
        
        // Signal the fence
        unsafe {
            command_queue.Signal(fence, fence_value)
                .map_err(|e| e.to_gpu_error("Failed to signal fence"))?;
            
            // Wait until the GPU has completed commands up to this fence point
            if fence.GetCompletedValue() < fence_value {
                fence.SetEventOnCompletion(fence_value, self.fence_event)
                    .map_err(|e| e.to_gpu_error("Failed to set fence completion event"))?;
                    
                WaitForSingleObject(self.fence_event, INFINITE);
            }
        }
        
        Ok(())
    }
    
    fn shutdown_impl(&mut self) {
        // Wait for the GPU to finish all work
        let _ = self.wait_for_gpu();
        
        // Close the fence event handle
        if !self.fence_event.is_invalid() {
            unsafe { let _ = CloseHandle(self.fence_event); }
        }
        
        // Report any live objects in debug mode
        #[cfg(feature = "validation")]
        if let Some(debug) = &self.debug {
            debug.report_live_objects();
        }
        
        // Clear all resources (relying on Drop for COM objects)
        self.surfaces.clear();
        self.buffers.clear();
        self.textures.clear();
        self.shaders.clear();
        self.pipelines.clear();
        
        // Reset state
        self.command_queue = None;
        self.device = None;
        self.dxgi_factory = None;
        self.fence = None;
        self.fence_event = HANDLE(std::ptr::null_mut());
        self.current_surface_id = None;
        self.current_pipeline_id = None;
    }
}