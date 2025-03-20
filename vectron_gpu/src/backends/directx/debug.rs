use crate::{debug::resource_tracker::ResourceTracker, GpuError};
use windows::Win32::Graphics::Direct3D12::*;
use windows::core::Interface;

pub struct DirectX12Debug {
    info_queue: Option<ID3D12InfoQueue>,
    debug_device: Option<ID3D12DebugDevice>,
    resource_tracker: ResourceTracker<D3D12_RESOURCE_STATES>,
}

impl DirectX12Debug {
    pub fn new(device: &ID3D12Device) -> Result<Self, GpuError> {
        let info_queue = match device.cast::<ID3D12InfoQueue>() {
            Ok(q) => {
                // Configure queue
                unsafe {
                    // Don't break on warnings in release mode
                    let break_severity = if cfg!(debug_assertions) {
                        D3D12_MESSAGE_SEVERITY_WARNING
                    } else {
                        D3D12_MESSAGE_SEVERITY_ERROR
                    };
                    
                    let _ = q.SetBreakOnSeverity(break_severity, true);
                    let _ = q.SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_CORRUPTION, true);
                }
                Some(q)
            },
            Err(_) => None
        };
        
        let debug_device = match device.cast::<ID3D12DebugDevice>() {
            Ok(d) => Some(d),
            Err(_) => None,
        };
        
        Ok(Self {
            info_queue,
            debug_device,
            resource_tracker: ResourceTracker::new(),
        })
    }
    
    pub fn track_resource(&mut self, id: u64, name: &str, initial_state: D3D12_RESOURCE_STATES) {
        self.resource_tracker.track(id, name.to_string(), initial_state);
    }
    
    pub fn update_resource_state(&mut self, id: u64, new_state: D3D12_RESOURCE_STATES) -> Result<(), GpuError> {
        self.resource_tracker.update_state(id, new_state)
    }
    
    pub fn validate_resource_state(&self, id: u64, expected_state: D3D12_RESOURCE_STATES) -> Result<(), GpuError> {
        // Validate that the resource is in the expected state
        // Implementation depends on how resource_tracker is structured
        Ok(())
    }
    
    pub fn check_and_log_messages(&self) {
        if let Some(info_queue) = &self.info_queue {
            unsafe {
                let count = info_queue.GetNumStoredMessages();
                for i in 0..count {
                    // Get the size needed for the message
                    let mut size = 0;
                    let _ = info_queue.GetMessage(i, None, &mut size);
                    
                    // Only process if we got a valid size
                    if size > 0 {
                        // Allocate buffer and get the message
                        let mut buffer = vec![0u8; size as usize];
                        let message_ptr = buffer.as_mut_ptr() as *mut _;
                        
                        if let Ok(_) = info_queue.GetMessage(i, Some(message_ptr), &mut size) {
                            // Process message here
                            log::debug!("DirectX12 Debug: Message {}", i);
                        }
                        
                    }
                    
                    // Parse message and log according to severity
                    // Simplified for brevity
                }
                
                // Clear messages after logging
                info_queue.ClearStoredMessages();
            }
        }
    }
    
    pub fn report_live_objects(&self) {
        if let Some(debug_device) = &self.debug_device {
            log::info!("Reporting live DirectX 12 objects...");
            unsafe {
                let _ = debug_device.ReportLiveDeviceObjects(D3D12_RLDO_DETAIL);
            }
        }
    }
    
    // More DirectX-specific debugging utilities
}