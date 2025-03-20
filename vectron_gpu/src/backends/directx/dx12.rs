use std::collections::HashMap;
use std::os::raw::c_void;

use windows::core::{Interface, PCSTR};
use windows::Win32::Foundation::{ HANDLE, HWND, RECT, CloseHandle, FALSE};
use windows::Win32::Graphics::Direct3D::{D3D_FEATURE_LEVEL_11_0, ID3DBlob};
use windows::Win32::Graphics::Direct3D::Fxc::{D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION, D3DCompile };
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Direct3D::Fxc::D3DCreateBlob;
use windows::Win32::System::Threading::{ CreateEventW, WaitForSingleObject, INFINITE};
use windows::core::{s, BOOL};
use std::mem::ManuallyDrop;

use crate::common::{GpuError, SurfaceId, BufferId, TextureId, ShaderId, PipelineId};
use crate::buffer::{BufferDescriptor, CpuAccessMode};
use crate::texture::{TextureDescriptor, TextureFormat, TextureUpdateDescriptor};
use crate::pipeline::PipelineDescriptor;
use crate::shader::{ShaderDescriptor, ShaderSource};
use crate::backend::{BackendConfig, BackendCapabilities, GpuBackend, RenderCommand, SurfaceDescriptor, IndexFormat};
use crate::vertex::{ VertexLayoutDescriptor, VertexFormat };
// Import the Direct3D module properly at the top of the file
use windows::Win32::Graphics::Direct3D;

// Helper function to convert TextureFormat to DXGI_FORMAT
fn to_dxgi_format(format: TextureFormat) -> DXGI_FORMAT {
    match format {
        TextureFormat::R8Unorm => DXGI_FORMAT_R8_UNORM,
        TextureFormat::RG8Unorm => DXGI_FORMAT_R8G8_UNORM,
        TextureFormat::RGBA8Unorm => DXGI_FORMAT_R8G8B8A8_UNORM,
        TextureFormat::RGBA8UnormSrgb => DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        TextureFormat::BGRA8Unorm => DXGI_FORMAT_B8G8R8A8_UNORM,
        TextureFormat::BGRA8UnormSrgb => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        TextureFormat::R16Uint => DXGI_FORMAT_R16_UINT,
        TextureFormat::RG16Uint => DXGI_FORMAT_R16G16_UINT,
        TextureFormat::R32Uint => DXGI_FORMAT_R32_UINT,
        TextureFormat::R32Float => DXGI_FORMAT_R32_FLOAT,
        TextureFormat::RG32Float => DXGI_FORMAT_R32G32_FLOAT,
        TextureFormat::RGBA16Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
        TextureFormat::RGBA32Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
        TextureFormat::Depth16Unorm => DXGI_FORMAT_D16_UNORM,
        TextureFormat::Depth24PlusStencil8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
        TextureFormat::Depth32Float => DXGI_FORMAT_D32_FLOAT,
        TextureFormat::BC1RGBAUnorm => DXGI_FORMAT_BC1_UNORM,
        TextureFormat::BC1RGBAUnormSrgb => DXGI_FORMAT_BC1_UNORM_SRGB,
        TextureFormat::BC2RGBAUnorm => DXGI_FORMAT_BC2_UNORM,
        TextureFormat::BC2RGBAUnormSrgb => DXGI_FORMAT_BC2_UNORM_SRGB,
        TextureFormat::BC3RGBAUnorm => DXGI_FORMAT_BC3_UNORM,
        TextureFormat::BC3RGBAUnormSrgb => DXGI_FORMAT_BC3_UNORM_SRGB,
        TextureFormat::BC4RUnorm => DXGI_FORMAT_BC4_UNORM,
        TextureFormat::BC5RGUnorm => DXGI_FORMAT_BC5_UNORM,
        TextureFormat::BC7RGBAUnorm => DXGI_FORMAT_BC7_UNORM,
        TextureFormat::BC7RGBAUnormSrgb => DXGI_FORMAT_BC7_UNORM_SRGB,
        // Handle other formats or use a default
        _ => DXGI_FORMAT_UNKNOWN,
    }
}

fn to_index_format(format: IndexFormat) -> DXGI_FORMAT {
    match format {
        IndexFormat::Uint16 => DXGI_FORMAT_R16_UINT,
        IndexFormat::Uint32 => DXGI_FORMAT_R32_UINT,
    }
}

// Convert GpuError from Windows HRESULT
impl From<windows::core::Error> for GpuError {
    fn from(error: windows::core::Error) -> Self {
        GpuError::BackendError(format!("DirectX error: {}", error))
    }
}

// Resource wrappers to track additional metadata
struct DirectXBuffer {
    resource: ID3D12Resource,
    size: usize,
    state: D3D12_RESOURCE_STATES,
}

struct DirectXTexture {
    resource: ID3D12Resource,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
    state: D3D12_RESOURCE_STATES,
}

struct DirectXShader {
    blob: ID3DBlob,
    stage: ShaderStage,
}

#[derive(Debug)]
enum ShaderStage {
    Vertex,
    Pixel,
    Compute,
}

struct DirectXPipeline {
    pipeline_state: ID3D12PipelineState,
    root_signature: ID3D12RootSignature,
    vertex_layout: Option<VertexLayoutDescriptor>,
}

struct SwapChainResources {
    swap_chain: IDXGISwapChain3,
    render_targets: Vec<ID3D12Resource>,
    rtv_heap: ID3D12DescriptorHeap,
    rtv_descriptor_size: usize,
}

// Frame resources - one set per frame in flight
struct FrameContext {
    command_allocator: ID3D12CommandAllocator,
    fence_value: u64,
}

pub struct DirectX12Backend {
    device: Option<ID3D12Device>,
    command_queue: Option<ID3D12CommandQueue>,
    command_list: Option<ID3D12GraphicsCommandList>,
    dxgi_factory: Option<IDXGIFactory4>,
    
    // Resource maps
    surfaces: HashMap<SurfaceId, SwapChainResources>,
    buffers: HashMap<BufferId, DirectXBuffer>,
    textures: HashMap<TextureId, DirectXTexture>,
    shaders: HashMap<ShaderId, DirectXShader>,
    pipelines: HashMap<PipelineId, DirectXPipeline>,
    
    // Descriptor heaps
    rtv_heap: Option<ID3D12DescriptorHeap>,
    dsv_heap: Option<ID3D12DescriptorHeap>,
    cbv_srv_uav_heap: Option<ID3D12DescriptorHeap>,
    sampler_heap: Option<ID3D12DescriptorHeap>,
    
    // Heap sizes
    rtv_descriptor_size: usize,
    dsv_descriptor_size: usize,
    cbv_srv_uav_descriptor_size: usize,
    sampler_descriptor_size: usize,
    
    // Synchronization
    fence: Option<ID3D12Fence>,
    fence_event: HANDLE,
    fence_value: u64,
    
    // Frame management
    frame_index: u32,
    frame_count: u32,
    frames: Vec<FrameContext>,
    
    // Current state
    current_surface_id: Option<SurfaceId>,
    current_pipeline_id: Option<PipelineId>,
    vsync: bool,
    
    // ID generation
    next_id: u64,
    
    // Capabilities
    capabilities: BackendCapabilities,
}

impl DirectX12Backend {
    pub fn new() -> Self {
        // Create a minimal instance - will be fully initialized in init()
        Self {
            device: None,
            command_queue: None,
            command_list: None,
            dxgi_factory: None,
            
            surfaces: HashMap::new(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            pipelines: HashMap::new(),
            
            rtv_heap: None,
            dsv_heap: None,
            cbv_srv_uav_heap: None,
            sampler_heap: None,
            
            rtv_descriptor_size: 0,
            dsv_descriptor_size: 0,
            cbv_srv_uav_descriptor_size: 0,
            sampler_descriptor_size: 0,
            
            fence: None,
            fence_event: HANDLE(std::ptr::null_mut()),
            fence_value: 1,
            
            frame_index: 0,
            frame_count: 2, // Double buffering by default
            frames: Vec::new(),
            
            current_surface_id: None,
            current_pipeline_id: None,
            vsync: true,
            
            next_id: 1,
            
            capabilities: BackendCapabilities {
                max_texture_size: 16384,
                supports_compute: false,
                supports_storage_buffers: false,
                supports_float_textures: false,
                max_uniform_buffer_size: 65536,
                max_color_attachments: 8,
            },
        }
    }
    
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    fn wait_for_gpu(&self) -> Result<(), GpuError> {
        // Signal and wait for the GPU to complete all work
        unsafe {
            // Schedule a signal command to the command queue
            self.command_queue.as_ref().unwrap().Signal(self.fence.as_ref().unwrap(), self.fence_value)?;
            
            // Wait until the fence has been processed
            if self.fence.as_ref().unwrap().GetCompletedValue() < self.fence_value {
                self.fence.as_ref().unwrap().SetEventOnCompletion(self.fence_value, self.fence_event)?;
                WaitForSingleObject(self.fence_event, INFINITE);
            }
        }
        
        Ok(())
    }
    
    fn wait_for_frame_fence(&self, frame_index: usize) -> Result<(), GpuError> {
        let frame_fence_value = self.frames[frame_index].fence_value;
        
        // If the frame's fence value is higher than the completed value,
        // we need to wait for the GPU to catch up
        unsafe {
            if self.fence.as_ref().unwrap().GetCompletedValue() < frame_fence_value {
                self.fence.as_ref().unwrap().SetEventOnCompletion(frame_fence_value, self.fence_event)?;
                WaitForSingleObject(self.fence_event, INFINITE);
            }
        }
        
        Ok(())
    }
    
    fn transition_resource(
        &self,
        resource: &ID3D12Resource,
        state_before: D3D12_RESOURCE_STATES,
        state_after: D3D12_RESOURCE_STATES,
    ) -> Result<(), GpuError> {
        // Skip if the states are the same
        if state_before == state_after {
            return Ok(());
        }
        
        // Create a transition barrier
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(resource.clone())),
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    StateBefore: state_before,
                    StateAfter: state_after,
                }),
            },
        };
        
        // Record the barrier
        unsafe {
            self.command_list.as_ref().unwrap().ResourceBarrier(&[barrier]);
        }
        
        Ok(())
    }
    
    fn create_descriptor_heaps(&mut self) -> Result<(), GpuError> {
        // Create render target view (RTV) heap
        let rtv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: 8, // Allocate space for multiple render targets
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        
        unsafe {
            self.rtv_heap = Some(self.device.as_ref().unwrap().CreateDescriptorHeap(&rtv_heap_desc)?);
            self.rtv_descriptor_size = self.device.as_ref().unwrap().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) as usize;
        }
        
        // Create depth stencil view (DSV) heap
        let dsv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            NumDescriptors: 1,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        
        unsafe {
            self.dsv_heap = Some(self.device.as_ref().unwrap().CreateDescriptorHeap(&dsv_heap_desc)?);
            self.dsv_descriptor_size = self.device.as_ref().unwrap().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_DSV) as usize;
        }
        
        // Create a descriptor heap for CBV/SRV/UAV
        let cbv_srv_uav_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: 64, // Allow for many resources
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };
        
        unsafe {
            self.cbv_srv_uav_heap = Some(self.device.as_ref().unwrap().CreateDescriptorHeap(&cbv_srv_uav_heap_desc)?);
            self.cbv_srv_uav_descriptor_size = self.device.as_ref().unwrap().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) as usize;
        }
        
        // Create a descriptor heap for samplers
        let sampler_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: 16,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };
        
        unsafe {
            self.sampler_heap = Some(self.device.as_ref().unwrap().CreateDescriptorHeap(&sampler_heap_desc)?);
            self.sampler_descriptor_size = self.device.as_ref().unwrap().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) as usize;
        }
        
        Ok(())
    }
}

impl GpuBackend for DirectX12Backend {
    fn init(&mut self, config: BackendConfig) -> Result<(), GpuError> {
        // Always enable debug layer in debug builds
        let debug_layer = config.enable_debug || cfg!(debug_assertions);
        
        // Create factory
        let dxgi_factory = unsafe {
            let factory_flags = if debug_layer {
                DXGI_CREATE_FACTORY_DEBUG
            } else {
                windows::Win32::Graphics::Dxgi::DXGI_CREATE_FACTORY_FLAGS(0)
            };
            
            if debug_layer {
                // Enable debug layer
                println!("Enabling Direct3D 12 debug layer");
                let mut debug: Option<ID3D12Debug> = None;
                D3D12GetDebugInterface(&mut debug)?;
                debug.unwrap().EnableDebugLayer();
            }
            
            let factory: IDXGIFactory4 = CreateDXGIFactory2(factory_flags)?;
            factory
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
                
                // Check if the adapter supports Direct3D 12
                if unsafe { D3D12CreateDevice(&tmp, D3D_FEATURE_LEVEL_11_0, &mut None::<ID3D12Device>) }.is_ok() {
                    adapter = Some(tmp);
                    
                    // Print adapter info
                    let name = String::from_utf16_lossy(
                        &desc.Description[0..desc.Description.iter().position(|&c| c == 0).unwrap_or(desc.Description.len())]
                    );
                    println!("Selected GPU: {}", name);
                    
                    break;
                }
            }
            
            adapter.ok_or_else(|| GpuError::BackendError("No DirectX 12 compatible adapter found".to_string()))?
        };
        
        // Create the device
        let device: ID3D12Device = unsafe {
            let mut device = None;
            D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)?;
            device.unwrap()
        };
        
        // Store the device
        self.device = Some(device);
        
        // Create descriptor heaps
        self.create_descriptor_heaps()?;
        
        // Set up debug info queue if in debug mode
        if debug_layer {
            unsafe {
                let info_queue = self.device.as_ref().unwrap().cast::<ID3D12InfoQueue>();
                if let Ok(info_queue) = info_queue {
                    // Break on errors and corruptions, but not on warnings
                    println!("Setting up D3D12 debug message callbacks");
                    let _ = info_queue.SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_ERROR, true);
                    let _ = info_queue.SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_CORRUPTION, true);
                    let _ = info_queue.SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_WARNING, false);
                    
                    // Output debug messages
                    info_queue.SetMuteDebugOutput(false);
                    
                    // Filter out verbose messages
                    let mut filter = Vec::new();
                    filter.push(D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE);
                    filter.push(D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE);
                    filter.push(D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE);
                    
                    let filter_desc = D3D12_INFO_QUEUE_FILTER_DESC {
                        NumIDs: filter.len() as u32,
                        pIDList: filter.as_ptr() as *mut D3D12_MESSAGE_ID,
                        ..Default::default()
                    };
                    
                    let mut filter = D3D12_INFO_QUEUE_FILTER {
                        AllowList: D3D12_INFO_QUEUE_FILTER_DESC::default(),
                        DenyList: filter_desc,
                    };
                    
                    let _ = info_queue.AddStorageFilterEntries(&filter);
                }
            }
        }
        
        // Create the command queue
        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0 as i32,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        };
        
        unsafe {
            self.command_queue = Some(self.device.as_ref().unwrap().CreateCommandQueue(&queue_desc)?);
        }
        
        // Create synchronization objects
        unsafe {
            self.fence = Some(self.device.as_ref().unwrap().CreateFence(0, D3D12_FENCE_FLAG_NONE)?);
            self.fence_value = 1;
            
            // Create an event handle for fence signaling
            self.fence_event = CreateEventW(None, false, false, None)?;
            if self.fence_event.is_invalid() {
                return Err(GpuError::BackendError("Failed to create fence event".to_string()));
            }
        }
        
        // Initialize per-frame resources
        self.frame_count = 2; // Double buffering by default, can be changed
        self.frames.clear();
        
        for _ in 0..self.frame_count {
            let command_allocator = unsafe {
                self.device.as_ref().unwrap().CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?
            };
            
            self.frames.push(FrameContext {
                command_allocator,
                fence_value: 0,
            });
        }
        
        // Create the command list
        unsafe {
            self.command_list = Some(self.device.as_ref().unwrap().CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                &self.frames[0].command_allocator,
                None,
            )?);
            
            // Close the command list as it's opened by default
            self.command_list.as_ref().unwrap().Close()?;
        }
        
        // Query and store device capabilities
        self.query_capabilities()?;
        
        // Store vsync preference from config
        self.vsync = config.vsync;
        
        // Save the factory and device
        self.dxgi_factory = Some(dxgi_factory);
        
        Ok(())
    }
    
    fn create_surface(&mut self, desc: SurfaceDescriptor) -> Result<SurfaceId, GpuError> {
        // Cast the raw window handle to HWND
        let hwnd = HWND(desc.handle as *mut c_void);
        
        // Describe the swap chain
        let swap_desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: desc.width,
            Height: desc.height,
            Format: match self.capabilities.supports_float_textures {
                true => DXGI_FORMAT_R16G16B16A16_FLOAT,
                false => DXGI_FORMAT_R8G8B8A8_UNORM
            },
            Stereo: FALSE,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: self.frame_count,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
            Flags: 0,
        };
        
        // Create the swap chain
        let swap_chain: IDXGISwapChain1 = unsafe {
            self.dxgi_factory.as_ref().unwrap().CreateSwapChainForHwnd(
                self.command_queue.as_ref().unwrap(),
                hwnd,
                &swap_desc,
                None,
                None,
            )?
        };
        
        // Get the swap chain as IDXGISwapChain3 for more functionality
        let swap_chain: IDXGISwapChain3 = swap_chain.cast()?;
        
        // Disable Alt+Enter fullscreen toggle
        unsafe {
            self.dxgi_factory.as_ref().unwrap().MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER)?;
        }
        
        // Create render target views for each back buffer
        let mut render_targets = Vec::with_capacity(self.frame_count as usize);
        
        // Create a descriptor heap for the render target views
        let rtv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: self.frame_count,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };
        
        let rtv_heap: ID3D12DescriptorHeap = unsafe { self.device.as_ref().unwrap().CreateDescriptorHeap(&rtv_heap_desc)? };
        let rtv_descriptor_size = unsafe { 
            self.device.as_ref().unwrap().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) as usize 
        };
        
        // Create a RTV for each buffer
        unsafe {
            let rtv_handle_start = rtv_heap.GetCPUDescriptorHandleForHeapStart();
            
            for i in 0..self.frame_count {
                let buffer: ID3D12Resource = swap_chain.GetBuffer(i)?;
                
                let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                    ptr: rtv_handle_start.ptr + i as usize * rtv_descriptor_size
                };
                
                self.device.as_ref().unwrap().CreateRenderTargetView(&buffer, None, rtv_handle);
                render_targets.push(buffer);
            }
        }
        
        // Generate a surface ID
        let surface_id = self.generate_id();
        
        // Store the swap chain resources
        self.surfaces.insert(surface_id, SwapChainResources {
            swap_chain,
            render_targets,
            rtv_heap,
            rtv_descriptor_size,
        });
        
        Ok(surface_id)
    }
    
    fn destroy_surface(&mut self, id: SurfaceId) {
        // Wait for GPU to finish using the surface
        let _ = self.wait_for_gpu();
        
        // Remove the surface from our map
        if let Some(_surface) = self.surfaces.remove(&id) {
            // The DirectX resources will be automatically released when dropped
        }
        
        // If this was the current surface, clear that reference
        if let Some(current_id) = self.current_surface_id {
            if current_id == id {
                self.current_surface_id = None;
            }
        }
    }
    
    fn resize_surface(&mut self, id: SurfaceId, width: u32, height: u32) -> Result<(), GpuError> {
        // Check if the surface exists
        if !self.surfaces.contains_key(&id) {
            return Err(GpuError::InvalidResource);
        }
        
        // Make sure the GPU is not using the surface
        self.wait_for_gpu()?;
        
        // Get the surface resources
        let surface = self.surfaces.get_mut(&id).unwrap();
        
        // Release the render target views
        surface.render_targets.clear();
        
        // Resize the swap chain buffers
        unsafe {
            surface.swap_chain.ResizeBuffers(
                self.frame_count,
                width,
                height,
                DXGI_FORMAT_UNKNOWN, // Keep the existing format
                windows::Win32::Graphics::Dxgi::DXGI_SWAP_CHAIN_FLAG(0),
            )?;
        }
        
        // Recreate the render target views
        unsafe {
            let rtv_handle_start = surface.rtv_heap.GetCPUDescriptorHandleForHeapStart();
            
            for i in 0..self.frame_count {
                let buffer: ID3D12Resource = surface.swap_chain.GetBuffer(i)?;
                
                let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                    ptr: rtv_handle_start.ptr + i as usize * surface.rtv_descriptor_size
                };
                
                self.device.as_ref().unwrap().CreateRenderTargetView(&buffer, None, rtv_handle);
                surface.render_targets.push(buffer);
            }
        }
        
        Ok(())
    }
    
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<BufferId, GpuError> {
        let buffer_id = self.generate_id();
        
        // Choose the right heap type based on access mode
        let heap_type = match desc.cpu_access {
            crate::buffer::CpuAccessMode::None => D3D12_HEAP_TYPE_DEFAULT, // GPU-only access
            crate::buffer::CpuAccessMode::Read => D3D12_HEAP_TYPE_READBACK, // CPU can read
            crate::buffer::CpuAccessMode::Write => D3D12_HEAP_TYPE_UPLOAD, // CPU can write
            crate::buffer::CpuAccessMode::ReadWrite => {
                return Err(GpuError::BackendError("ReadWrite CPU access mode not supported in DirectX 12".to_string()));
            }
        };
        
        // Set resource states based on usage and access mode
        let initial_state = match (desc.usage.contains(crate::buffer::BufferUsageFlags::VERTEX), desc.cpu_access) {
            (true, crate::buffer::CpuAccessMode::None) => D3D12_RESOURCE_STATE_COMMON, // Will transition to VERTEX_AND_CONSTANT_BUFFER when used
            (true, _) => D3D12_RESOURCE_STATE_GENERIC_READ, // Upload/readback buffers are always in GENERIC_READ
            (_, crate::buffer::CpuAccessMode::Write) => D3D12_RESOURCE_STATE_GENERIC_READ,
            (_, crate::buffer::CpuAccessMode::Read) => D3D12_RESOURCE_STATE_COPY_DEST,
            _ => D3D12_RESOURCE_STATE_COMMON,
        };
        
        println!("Creating buffer with size: {}, usage: {:?}, access: {:?}, heap: {:?}, state: {:?}", 
                 desc.size, desc.usage, desc.cpu_access, heap_type, initial_state);

        // Set up heap properties
        let heap_props = D3D12_HEAP_PROPERTIES {
            Type: heap_type,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        
        // Set up resource description
        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: desc.size as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        
        // Create the resource
        let mut resource: Option<ID3D12Resource> = None;
        unsafe {
            self.device.as_ref().unwrap().CreateCommittedResource(
                &heap_props,
                D3D12_HEAP_FLAG_NONE,
                &resource_desc,
                initial_state,
                None,
                &mut resource,
            )?;
        }
        
        let resource = resource.unwrap();
        
        // If initial data is provided, copy it to the buffer
        if let Some(initial_data) = &desc.initial_data {
            if heap_type == D3D12_HEAP_TYPE_UPLOAD {
                // For upload heaps, we can map and copy directly
                let mut mapped_data: *mut std::ffi::c_void = std::ptr::null_mut();
                let range = D3D12_RANGE { Begin: 0, End: 0 }; // We're not reading, just writing
                
                unsafe {
                    resource.Map(0, Some(&range), Some(&mut mapped_data))?;
                    std::ptr::copy_nonoverlapping(
                        initial_data.as_ptr(),
                        mapped_data as *mut u8,
                        initial_data.len(),
                    );
                    // Use a null range when unmapping after writing, since we're not reading
                    resource.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
                }
            } else if heap_type == D3D12_HEAP_TYPE_DEFAULT {
                // For default heaps, we need to use an upload buffer and GPU copy
                self.upload_buffer_data(&resource, initial_data, 0, initial_data.len())?;
                
                // After copying, transition to the appropriate state based on usage
                let final_state = match desc.usage {
                    flags if flags.contains(crate::buffer::BufferUsageFlags::VERTEX) => D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER,
                    flags if flags.contains(crate::buffer::BufferUsageFlags::INDEX) => D3D12_RESOURCE_STATE_INDEX_BUFFER,
                    flags if flags.contains(crate::buffer::BufferUsageFlags::UNIFORM) => D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER,
                    flags if flags.contains(crate::buffer::BufferUsageFlags::STORAGE) => D3D12_RESOURCE_STATE_UNORDERED_ACCESS,
                    flags if flags.contains(crate::buffer::BufferUsageFlags::INDIRECT) => D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT,
                    _ => D3D12_RESOURCE_STATE_COMMON,
                };
                
                // Execute the transition
                self.transition_resource_immediate(&resource, initial_state, final_state)?;
                
                // Store the buffer with the final state
                self.buffers.insert(BufferId(buffer_id), DirectXBuffer {
                    resource,
                    size: desc.size,
                    state: final_state,
                });
                
                return Ok(BufferId(buffer_id));
            }
        }
        
        // Store the buffer with the initial state
        self.buffers.insert(BufferId(buffer_id), DirectXBuffer {
            resource,
            size: desc.size,
            state: initial_state,
        });
        
        Ok(BufferId(buffer_id))
    }
    
    fn update_buffer(&mut self, id: BufferId, data: &[u8], offset: usize) -> Result<(), GpuError> {
        // Get the resource ID and size first
        let (resource_id, buffer_size) = {
            let buffer = match self.buffers.get_mut(&id) {
                Some(buffer) => buffer,
                None => return Err(GpuError::InvalidHandle(format!("Buffer ID {:?} not found", id))),
            };
            (buffer.resource.clone(), buffer.size)
        };
        
        // Check if offset + data.len() exceeds buffer size
        if offset + data.len() > buffer_size {
            return Err(GpuError::InvalidArgument("Update size exceeds buffer size".to_string()));
        }
        
        // Get heap properties to determine how to update
        let mut heap_props = D3D12_HEAP_PROPERTIES::default();
        let mut heap_flags = D3D12_HEAP_FLAGS::default();
        unsafe {
            let _ = resource_id.GetHeapProperties(Some(&mut heap_props), Some(&mut heap_flags));
        }
        
        // Flag to track if we need to upload to default heap
        let mut need_default_upload = false;

        let _result = match heap_props.Type {
            D3D12_HEAP_TYPE_UPLOAD => {
                // For upload heaps, we can map and copy directly
                let mut mapped_data: *mut std::ffi::c_void = std::ptr::null_mut();
                let range = D3D12_RANGE { Begin: 0, End: 0 }; // We don't intend to read
                
                unsafe {
                    resource_id.Map(0, Some(&range), Some(&mut mapped_data))?;
                    std::ptr::copy_nonoverlapping(
                        data.as_ptr(),
                        (mapped_data as *mut u8).add(offset),
                        data.len(),
                    );
                    // Use a null range when unmapping after writing
                    resource_id.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
                }
                
                Ok(())
            },
            D3D12_HEAP_TYPE_DEFAULT => {
                // For default heaps, we need to use an upload buffer and GPU copy
                need_default_upload = true;
                Ok(())
            },
            D3D12_HEAP_TYPE_READBACK => {
                // Readback heaps are not meant to be written to by the CPU
                Err(GpuError::InvalidOperation("Cannot update a readback buffer from CPU".to_string()))
            },
            _ => Err(GpuError::InvalidOperation("Unsupported heap type".to_string())),
        }?;

        // Handle upload to default heap outside the match to avoid double mutable borrow
        if need_default_upload {
            self.upload_buffer_data(&resource_id, data, offset, data.len())?;
        }

        Ok(())
    }
    
    fn create_texture(&mut self, desc: TextureDescriptor) -> Result<TextureId, GpuError> {
        // Convert TextureFormat to DXGI_FORMAT
        let dxgi_format = to_dxgi_format(desc.format);
        if dxgi_format == DXGI_FORMAT_UNKNOWN {
            return Err(GpuError::InvalidArgument("Unsupported texture format".to_string()));
        }
        
        // Set up heap properties (default heap for textures)
        let heap_props = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        
        // Determine resource flags based on usage
        let mut resource_flags = D3D12_RESOURCE_FLAG_NONE;
        if desc.usage.contains(crate::texture::TextureUsageFlags::RENDER_TARGET) {
            resource_flags |= D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;
        }
        if desc.usage.contains(crate::texture::TextureUsageFlags::DEPTH_STENCIL) {
            resource_flags |= D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL;
        }
        if desc.usage.contains(crate::texture::TextureUsageFlags::STORAGE) {
            resource_flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
        }
        
        // Create resource description for texture
        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: match desc.dimension {
                crate::texture::TextureDimension::D1 => D3D12_RESOURCE_DIMENSION_TEXTURE1D,
                crate::texture::TextureDimension::D2 => D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                crate::texture::TextureDimension::D3 => D3D12_RESOURCE_DIMENSION_TEXTURE3D,
                crate::texture::TextureDimension::Cube => D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            },
            Alignment: 0,
            Width: desc.size.width as u64,
            Height: desc.size.height,
            DepthOrArraySize: match desc.dimension {
                crate::texture::TextureDimension::D3 => desc.size.depth_or_array_layers as u16,
                _ => desc.size.depth_or_array_layers as u16,
            },
            MipLevels: desc.mip_level_count as u16,
            Format: dxgi_format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: desc.sample_count,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: resource_flags,
        };
        
        // Set initial state based on usage
        let initial_state = if desc.usage.contains(crate::texture::TextureUsageFlags::DEPTH_STENCIL) {
            D3D12_RESOURCE_STATE_DEPTH_WRITE
        } else if desc.usage.contains(crate::texture::TextureUsageFlags::RENDER_TARGET) {
            D3D12_RESOURCE_STATE_RENDER_TARGET
        } else {
            D3D12_RESOURCE_STATE_COPY_DEST // For initial data upload
        };
        
        // Create optimized clear value if this is a render target or depth buffer
        let mut clear_value: Option<D3D12_CLEAR_VALUE> = None;
        if desc.usage.contains(crate::texture::TextureUsageFlags::RENDER_TARGET) {
            clear_value = Some(D3D12_CLEAR_VALUE {
                Format: dxgi_format,
                Anonymous: D3D12_CLEAR_VALUE_0 {
                    Color: [0.0, 0.0, 0.0, 1.0], // Default clear color
                },
            });
        } else if desc.usage.contains(crate::texture::TextureUsageFlags::DEPTH_STENCIL) {
            clear_value = Some(D3D12_CLEAR_VALUE {
                Format: dxgi_format,
                Anonymous: D3D12_CLEAR_VALUE_0 {
                    DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                        Depth: 1.0,
                        Stencil: 0,
                    },
                },
            });
        }
        
        // Create the resource
        let mut resource: Option<ID3D12Resource> = None;
        unsafe {
            self.device.as_ref().unwrap().CreateCommittedResource(
                &heap_props,
                D3D12_HEAP_FLAG_NONE,
                &resource_desc,
                initial_state,
                clear_value.as_ref().map(|v| v as *const _),
                &mut resource,
            )?;
        }
        
        let resource = resource.unwrap();
        
        // If initial data is provided, upload it
        if let Some(initial_data) = &desc.initial_data {
            // For textures with initial data, we need to:
            // 1. Get layout info to know how to pack the data
            // 2. Create an upload buffer
            // 3. Copy data to the upload buffer with proper layout
            // 4. Copy from upload buffer to texture using GPU
            // 5. Transition texture to its final state
            
            self.upload_texture_data(&resource, initial_data, dxgi_format, desc.size.width, desc.size.height, desc.mip_level_count)?;
            
            // Transition to shader resource state after upload if not a render target or depth buffer
            if !desc.usage.contains(crate::texture::TextureUsageFlags::RENDER_TARGET) && 
               !desc.usage.contains(crate::texture::TextureUsageFlags::DEPTH_STENCIL) {
                self.transition_resource_immediate(&resource, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE)?;
            }
        }
        
        // Generate a texture ID
        let texture_id: TextureId = self.generate_id();
        
        // Store the texture
        self.textures.insert(texture_id, DirectXTexture {
            resource,
            width: desc.size.width,
            height: desc.size.height,
            format: dxgi_format,
            state: initial_state,
        });
        
        Ok(texture_id)
    }
    
    fn update_texture(&mut self, id: TextureId, data: &[u8], desc: TextureUpdateDescriptor) -> Result<(), GpuError> {
        // Get texture and check if it exists
        let (resource, state, width, height, format) = {
            let texture = match self.textures.get_mut(&id) {
                Some(texture) => texture,
                None => return Err(GpuError::InvalidResource),
            };
            (texture.resource.clone(), texture.state, texture.width, texture.height, texture.format)
        };

        // Check if update region is valid
        if desc.origin[0] + desc.size[0] > width || desc.origin[1] + desc.size[1] > height {
            return Err(GpuError::InvalidArgument("Update region exceeds texture dimensions".to_string()));
        }

        // Calculate subresource index
        let subresource_index = desc.mip_level + (desc.array_layer * unsafe { resource.GetDesc().MipLevels } as u32);

        // Transition texture to COPY_DEST if needed
        if state != D3D12_RESOURCE_STATE_COPY_DEST {
            self.transition_resource_immediate(&resource, state, D3D12_RESOURCE_STATE_COPY_DEST)?;
            if let Some(texture) = self.textures.get_mut(&id) {
                texture.state = D3D12_RESOURCE_STATE_COPY_DEST;
            }
        }
        
        // Get the footprint for this subresource
        let mut layouts: [D3D12_PLACED_SUBRESOURCE_FOOTPRINT; 1] = unsafe { std::mem::zeroed() };
        let mut row_size_in_bytes: u32 = 0;
        let mut total_bytes: u64 = 0;
        
        unsafe {
            self.device.as_ref().unwrap().GetCopyableFootprints(
                &resource.GetDesc(),
                subresource_index,
                1,
                0,
                Some(layouts.as_mut_ptr()),
                Some(&mut row_size_in_bytes as *mut u32),
                None, // Don't need row sizes
                Some(&mut total_bytes as *mut u64),
            );
        }
        
        // Create an upload buffer
        let buffer_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: total_bytes,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        
        let upload_heap_props = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        
        let mut upload_buffer: Option<ID3D12Resource> = None;
        unsafe {
            self.device.as_ref().unwrap().CreateCommittedResource(
                &upload_heap_props,
                D3D12_HEAP_FLAG_NONE,
                &buffer_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut upload_buffer,
            )?;
        }
        
        let upload_buffer = upload_buffer.unwrap();
        
        // Map the upload buffer
        let mut mapped_data: *mut std::ffi::c_void = std::ptr::null_mut();
        let map_range = D3D12_RANGE { Begin: 0, End: 0 }; // We're not reading, just writing
        
        unsafe {
            upload_buffer.Map(0, Some(&map_range), Some(&mut mapped_data))?;
            
            let footprint = &layouts[0].Footprint;
            let src_pitch = desc.size[0] * bytes_per_pixel(format);
            let dst_pitch = footprint.RowPitch as usize;
            
            // Copy row by row, respecting pitch alignment
            for row in 0..desc.size[1] {
                let src_offset = row as usize * src_pitch as usize;
                let dst_offset = row as usize * dst_pitch;
                
                if src_offset + src_pitch as usize <= data.len() {
                    std::ptr::copy_nonoverlapping(
                        data[src_offset..].as_ptr(),
                        (mapped_data as *mut u8).add(dst_offset),
                        std::cmp::min(src_pitch as usize, dst_pitch),
                    );
                }
            }
            
            upload_buffer.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
        }
        
        // Reset the command list and record copy commands
        self.reset_command_list_for_copy()?;
        
        // Set up copy destination
        let dest_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: ManuallyDrop::new(Some(resource.clone())),
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: subresource_index,
            },
        };
        
        // Set up copy source
        let src_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: ManuallyDrop::new(Some(upload_buffer.clone())),
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: layouts[0],
            },
        };
        
        // Define the region to copy
        let box_copy = D3D12_BOX {
            left: desc.origin[0],
            top: desc.origin[1],
            front: 0,
            right: desc.origin[0] + desc.size[0],
            bottom: desc.origin[1] + desc.size[1],
            back: 1,
        };
        
        // Execute the copy
        unsafe {
            self.command_list.as_ref().unwrap().CopyTextureRegion(
                &dest_location,
                desc.origin[0],
                desc.origin[1],
                0,
                &src_location,
                Some(&box_copy),
            );
        }
        
        // Transition back to original state if needed
        let final_state = if state == D3D12_RESOURCE_STATE_COPY_DEST {
            D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE
        } else {
            state
        };
        
        if final_state != D3D12_RESOURCE_STATE_COPY_DEST {
            self.transition_resource(&resource, D3D12_RESOURCE_STATE_COPY_DEST, final_state)?;
            if let Some(texture) = self.textures.get_mut(&id) {
                texture.state = final_state;
            }
        }
        
        // Execute the command list
        self.execute_command_list()?;
        
        // Wait for completion to ensure the upload buffer stays alive until the GPU is done
        self.wait_for_gpu()?;
        
        Ok(())
    }
    
    fn create_shader(&mut self, desc: ShaderDescriptor) -> Result<ShaderId, GpuError> {
        // Convert the shader stage to our internal type
        let stage = match desc.stage {
            crate::shader::ShaderStage::Vertex => ShaderStage::Vertex,
            crate::shader::ShaderStage::Fragment => ShaderStage::Pixel,
            crate::shader::ShaderStage::Compute => ShaderStage::Compute,
        };
        
        // Clone the source to avoid partial move
        let source = desc.source.clone();
        
        // Process the shader source
        match source {
            ShaderSource::Source(_source_code, _) => {
                // Compile shader from source code
                self.compile_hlsl_shader(&desc, stage)
            },
            ShaderSource::Binary(bytecode, _) => {
                // Create a blob from the bytecode
                let mut shader_blob: Option<ID3DBlob> = None;
                unsafe {
                    let blob = D3DCreateBlob(bytecode.len())?;
                    std::ptr::copy_nonoverlapping(
                        bytecode.as_ptr(),
                        blob.GetBufferPointer() as *mut u8,
                        bytecode.len(),
                    );
                    shader_blob = Some(blob);
                }
                
                // Generate a shader ID
                let shader_id = ShaderId(self.generate_id());
                
                // Store the shader
                self.shaders.insert(shader_id, DirectXShader {
                    blob: shader_blob.unwrap(),
                    stage,
                });
                
                Ok(shader_id)
            },
            _ => Err(GpuError::InvalidArgument("Unsupported shader source type".to_string())),
        }
    }
    
    fn create_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError> {
        match desc.type_ {
            crate::pipeline::PipelineType::Graphics => self.create_graphics_pipeline(desc),
            crate::pipeline::PipelineType::Compute => self.create_compute_pipeline(desc),
        }
    }
    
    fn begin_frame(&mut self, surface_id: SurfaceId) -> Result<(), GpuError> {
        // Check if the surface exists
        let surface = match self.surfaces.get(&surface_id) {
            Some(surface) => surface,
            None => return Err(GpuError::InvalidResource),
        };
        
        // Set the current surface
        self.current_surface_id = Some(surface_id);
        
        // Get the current frame index from the swap chain
        self.frame_index = unsafe { surface.swap_chain.GetCurrentBackBufferIndex() };
        
        // Wait for the frame to be available (if we're cycling through frame resources)
        let frame_context = &self.frames[self.frame_index as usize];
        if frame_context.fence_value > 0 {
            self.wait_for_frame_fence(self.frame_index as usize)?;
        }
        
        // Reset the command allocator
        unsafe {
            frame_context.command_allocator.Reset()?;
        }
        
        // Reset the command list using the current frame's allocator
        unsafe {
            self.command_list.as_ref().unwrap().Reset(
                &frame_context.command_allocator,
                None, // No initial PSO
            )?;
        }
        
        // Get the back buffer
        let back_buffer = &surface.render_targets[self.frame_index as usize];
        
        // Transition the back buffer from PRESENT to RENDER_TARGET
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(back_buffer.clone())),
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                    StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                }),
            },
        };
        
        unsafe {
            self.command_list.as_ref().unwrap().ResourceBarrier(&[barrier]);
        }
        
        // Set the render target
        let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: unsafe { surface.rtv_heap.GetCPUDescriptorHandleForHeapStart().ptr } +
                 (self.frame_index as usize * surface.rtv_descriptor_size),
        };
        
        unsafe {
            self.command_list.as_ref().unwrap().OMSetRenderTargets(1, Some(&rtv_handle), false, None);
        }
        
        // Reset current pipeline ID
        self.current_pipeline_id = None;
        
        Ok(())
    }
    
    fn submit_commands(&mut self, commands: &[RenderCommand]) -> Result<(), GpuError> {
        // Process each command
        for command in commands {
            match command {
                RenderCommand::SetPipeline(pipeline_id) => {
                    self.set_pipeline(*pipeline_id)?;
                },
                RenderCommand::SetVertexBuffer { slot, buffer, offset } => {
                    self.set_vertex_buffer(*slot, *buffer, *offset)?;
                },
                RenderCommand::SetIndexBuffer { buffer, offset, index_format } => {
                    self.set_index_buffer(*buffer, *offset, *index_format)?;
                },
                RenderCommand::Draw { vertex_count, instance_count, first_vertex, first_instance } => {
                    println!("Executing Draw command: vertex_count={}, instance_count={}, first_vertex={}, first_instance={}", 
                        vertex_count, instance_count, first_vertex, first_instance);
                        
                    // Ensure a pipeline is set before drawing
                    if self.current_pipeline_id.is_none() {
                        return Err(GpuError::InvalidOperation("No pipeline set before Draw command".to_string()));
                    }
                    
                    // Execute the draw call
                    unsafe {
                        self.command_list.as_ref().unwrap().DrawInstanced(
                            *vertex_count as u32,
                            *instance_count as u32,
                            *first_vertex as u32,
                            *first_instance as u32,
                        );
                    }
                    
                    // Check for device removal immediately after drawing
                    let device_removed = unsafe {
                        let result = self.device.as_ref().unwrap().GetDeviceRemovedReason();
                        if let Err(e) = &result {
                            if e.code().0 != 0 {
                                Some(format!("{:?}", result))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };
                    
                    if let Some(reason) = device_removed {
                        return Err(GpuError::BackendError(
                            format!("Device removed during Draw: {}", reason)
                        ));
                    }
                },
                RenderCommand::DrawIndexed { index_count, instance_count, first_index, base_vertex, first_instance } => {
                    unsafe {
                        self.command_list.as_ref().unwrap().DrawIndexedInstanced(
                            *index_count,
                            *instance_count,
                            *first_index,
                            *base_vertex,
                            *first_instance,
                        );
                    }
                },
                RenderCommand::SetViewport { x, y, width, height, min_depth, max_depth } => {
                    let viewport = D3D12_VIEWPORT {
                        TopLeftX: *x,
                        TopLeftY: *y,
                        Width: *width,
                        Height: *height,
                        MinDepth: *min_depth,
                        MaxDepth: *max_depth,
                    };
                    
                    unsafe {
                        self.command_list.as_ref().unwrap().RSSetViewports(&[viewport]);
                    }
                },
                RenderCommand::SetScissor { x, y, width, height } => {
                    let scissor_rect = RECT {
                        left: *x as i32,
                        top: *y as i32,
                        right: (*x + *width) as i32,
                        bottom: (*y + *height) as i32,
                    };
                    
                    unsafe {
                        self.command_list.as_ref().unwrap().RSSetScissorRects(&[scissor_rect]);
                    }
                },
                RenderCommand::ClearColor { attachment_index, color } => {
                    // Only support clearing the current render target for now
                    if *attachment_index == 0 && self.current_surface_id.is_some() {
                        let surface = &self.surfaces[&self.current_surface_id.unwrap()];
                        let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                            ptr: unsafe { surface.rtv_heap.GetCPUDescriptorHandleForHeapStart().ptr } +
                                 (self.frame_index as usize * surface.rtv_descriptor_size),
                        };
                        
                        unsafe {
                            let color_array: [f32; 4] = [color[0], color[1], color[2], color[3]];
                            self.command_list.as_ref().unwrap().ClearRenderTargetView(rtv_handle, &color_array, None);
                        }
                    }
                },
                RenderCommand::ClearDepthStencil { depth: _, stencil: _ } => {
                    // This would require a depth stencil view which we haven't implemented yet
                    return Err(GpuError::Unimplemented("ClearDepthStencil not implemented yet".to_string()));
                },
            }
        }
        
        Ok(())
    }
    
    fn end_frame(&mut self) -> Result<(), GpuError> {
        // Check if we have a current surface
        let surface_id = match self.current_surface_id {
            Some(id) => id,
            None => return Err(GpuError::InvalidOperation("end_frame called without setting a current surface".to_string())),
        };
        
        let surface = match self.surfaces.get(&surface_id) {
            Some(surface) => surface,
            None => return Err(GpuError::InvalidResource),
        };
        
        println!("Ending frame: transitioning back buffer to PRESENT state");
        
        // Transition back buffer from RENDER_TARGET to PRESENT
        let back_buffer = &surface.render_targets[self.frame_index as usize];
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(back_buffer.clone())),
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                    StateAfter: D3D12_RESOURCE_STATE_PRESENT,
                }),
            },
        };
        
        unsafe {
            self.command_list.as_ref().unwrap().ResourceBarrier(&[barrier]);
        }
        
        println!("Closing command list");
        
        // Close the command list
        unsafe {
            self.command_list.as_ref().unwrap().Close()?;
        }
        
        // Execute the command list
        println!("Executing command list");
        
        unsafe {
            let command_lists: [Option<ID3D12CommandList>; 1] = [Some(self.command_list.as_ref().unwrap().cast()?)];
            self.command_queue.as_ref().unwrap().ExecuteCommandLists(&command_lists);
        }
        
        // Present
        println!("Presenting swap chain");
        
        let sync_interval = if self.vsync { 1 } else { 0 };
        unsafe {
            let present_flags = windows::Win32::Graphics::Dxgi::DXGI_PRESENT(0);
            let result = surface.swap_chain.Present(sync_interval, present_flags);
            if result.is_err() {
                println!("ERROR: Present failed with result: {:?}", result);
                return Err(GpuError::BackendError("Present failed".to_string()));
            }
        }
        
        // Update fence value for the current frame
        let fence_value = self.fence_value;
        self.frames[self.frame_index as usize].fence_value = fence_value;
        
        println!("Signaling fence with value: {}", fence_value);
        
        // Signal the fence with the new value
        unsafe {
            self.command_queue.as_ref().unwrap().Signal(self.fence.as_ref().unwrap(), fence_value)?;
        }
        
        // Increment the fence value for the next frame
        self.fence_value += 1;
        
        println!("Frame completed successfully");
        
        Ok(())
    }
    
    fn get_capabilities(&self) -> BackendCapabilities {
        self.capabilities.clone()
    }
    
    fn shutdown(&mut self) {
        // Wait for the GPU to finish all work
        let _ = self.wait_for_gpu();
        
        // Close the fence event handle
        if !self.fence_event.is_invalid() {
            unsafe { let _ = CloseHandle(self.fence_event); }
        }
        
        // Clear all resources
        self.surfaces.clear();
        self.buffers.clear();
        self.textures.clear();
        self.shaders.clear();
        self.pipelines.clear();
        self.frames.clear();
        
        // Reset state
        self.current_surface_id = None;
        self.current_pipeline_id = None;
    }
}

// Helper methods for the DirectX12Backend
impl DirectX12Backend {
    // Query device capabilities
    fn query_capabilities(&mut self) -> Result<(), GpuError> {
        // Query texture dimension limits
        let mut feature_data_options = D3D12_FEATURE_DATA_D3D12_OPTIONS::default();
        let hr = unsafe {
            self.device.as_ref().unwrap().CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS,
                &mut feature_data_options as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS>() as u32,
            )
        };
        
        if hr.is_ok() {
            self.capabilities.max_texture_size = 16384;
            self.capabilities.max_color_attachments = 8;
        }
        
        // Check for compute shader support (always supported in DX12)
        self.capabilities.supports_compute = true;
        
        // Check for storage buffer support (always supported in DX12 via UAVs)
        self.capabilities.supports_storage_buffers = true;
        
        // Check for float texture support
        let mut format_support = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            Support1: D3D12_FORMAT_SUPPORT1_NONE,
            Support2: D3D12_FORMAT_SUPPORT2_NONE,
        };
        
        let hr = unsafe {
            self.device.as_ref().unwrap().CheckFeatureSupport(
                D3D12_FEATURE_FORMAT_SUPPORT,
                &mut format_support as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_FORMAT_SUPPORT>() as u32,
            )
        };
        
        if hr.is_ok() {
            self.capabilities.supports_float_textures = (format_support.Support1 & D3D12_FORMAT_SUPPORT1_TEXTURE2D) != windows::Win32::Graphics::Direct3D12::D3D12_FORMAT_SUPPORT1(0);
        }
        
        // Set constant buffer size limit (64KB is the DirectX 12 limit)
        self.capabilities.max_uniform_buffer_size = 65536;
        
        Ok(())
    }
    
    // Compile an HLSL shader
    fn compile_hlsl_shader(&mut self, desc: &ShaderDescriptor, stage: ShaderStage) -> Result<ShaderId, GpuError> {
        // Determine shader model based on stage
        let target = match stage {
            ShaderStage::Vertex => s!("vs_5_1"),
            ShaderStage::Pixel => s!("ps_5_1"),
            ShaderStage::Compute => s!("cs_5_1"),
        };
        
        // Determine entry point (default to "main" if not provided)
        let entry_point_string;
        let entry_point = if desc.entry_point.is_empty() {
            println!("Using default entry point: main");
            PCSTR("main\0".as_ptr() as *const u8)
        } else {
            entry_point_string = format!("{}\0", desc.entry_point);
            println!("Using entry point: {} (bytes: {:?})", entry_point_string, entry_point_string.as_bytes());
            PCSTR(entry_point_string.as_ptr() as *const u8)
        };
        
        // Set compilation flags
        let compile_flags = if cfg!(debug_assertions) {
            D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION
        } else {
            0
        };
        
        // Compile the shader
        let mut shader_blob: Option<ID3DBlob> = None;
        let mut error_blob: Option<ID3DBlob> = None;
        
        // Get the shader source
        let source = match &desc.source {
            ShaderSource::Source(src, _) => {
                println!("Compiling {} shader with entry point: {}", 
                         match stage {
                             ShaderStage::Vertex => "Vertex",
                             ShaderStage::Pixel => "Pixel",
                             ShaderStage::Compute => "Compute",
                         },
                         desc.entry_point);
                println!("Source code:\n{}", src);
                src
            },
            ShaderSource::Binary(_, _) => return Err(GpuError::InvalidArgument("Binary shader source not supported for HLSL compilation".to_string())),
        };

        // Use D3DCompile instead of D3DCompileFromFile since we have the source code directly
        let source_name = s!("shader");
        let source_ptr = source.as_ptr() as *const std::ffi::c_void;
        let source_len = source.len();
        
        let result = unsafe {
            D3DCompile(
                source_ptr,
                source_len,
                source_name,
                None,               // Defines
                None,               // Include handler
                entry_point,
                target,
                compile_flags,
                0,                  // Effect flags
                &mut shader_blob,
                Some(&mut error_blob),
            )
        };
        
        if result.is_err() {
            // If compilation failed, return the error message
            if let Some(error_blob) = error_blob {
                let error_message = unsafe {
                    let error_ptr = error_blob.GetBufferPointer() as *const i8;
                    let error_size = error_blob.GetBufferSize();
                    let slice = std::slice::from_raw_parts(error_ptr as *const u8, error_size);
                    String::from_utf8_lossy(slice).to_string()
                };
                
                // Print detailed error info for debugging
                println!("Shader compilation failed:");
                println!("Stage: {:?}", stage);
                println!("Entry point: {:?}", entry_point);
                println!("Error: {}", error_message);
                
                return Err(GpuError::ShaderCompilationFailed(error_message));
            } else {
                return Err(GpuError::ShaderCompilationFailed("Unknown error".to_string()));
            }
        }
        
        let shader_blob = shader_blob.unwrap();
        
        // Generate a shader ID
        let shader_id = ShaderId(self.generate_id());
        
        // Store the shader
        self.shaders.insert(shader_id, DirectXShader {
            blob: shader_blob,
            stage,
        });
        
        Ok(shader_id)
    }
    
    // Create a graphics pipeline
    fn create_graphics_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError> {
        // Create a root signature
        let root_signature = self.create_root_signature(desc.vertex_layout.is_some())?;
        
        // Create pipeline state
        let mut pso_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(Some(root_signature.clone())),
            // Initialize with empty shader bytecode
            VS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            PS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            DS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            HS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            GS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            StreamOutput: D3D12_STREAM_OUTPUT_DESC::default(),
            BlendState: self.create_blend_state(&desc.blend_state),
            SampleMask: u32::MAX,
            RasterizerState: self.create_rasterizer_state(&desc.rasterizer_state),
            DepthStencilState: self.create_depth_stencil_state(&desc.depth_stencil_state),
            InputLayout: D3D12_INPUT_LAYOUT_DESC::default(), // Will fill in if vertex layout provided
            IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED,
            PrimitiveTopologyType: self.convert_primitive_topology(desc.primitive_topology),
            NumRenderTargets: desc.render_target_formats.len() as u32,
            RTVFormats: [DXGI_FORMAT_UNKNOWN; 8], // Will fill in from desc
            DSVFormat: match desc.depth_stencil_format {
                Some(format) => to_dxgi_format(format),
                None => DXGI_FORMAT_UNKNOWN,
            },
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1, // No MSAA for now
                Quality: 0,
            },
            NodeMask: 0,
            CachedPSO: D3D12_CACHED_PIPELINE_STATE::default(),
            Flags: D3D12_PIPELINE_STATE_FLAG_NONE,
        };
        
        // Set render target formats
        for (i, format) in desc.render_target_formats.iter().enumerate() {
            if i < 8 {
                pso_desc.RTVFormats[i] = to_dxgi_format(*format);
            }
        }
        
        // Set vertex shader if provided
        if let Some(shader_id) = desc.vertex_shader {
            if let Some(shader) = self.shaders.get(&shader_id) {
                if let ShaderStage::Vertex = shader.stage {
                    pso_desc.VS = D3D12_SHADER_BYTECODE {
                        pShaderBytecode: unsafe { shader.blob.GetBufferPointer() },
                        BytecodeLength: unsafe { shader.blob.GetBufferSize() },
                    };
                } else {
                    return Err(GpuError::InvalidArgument("Shader is not a vertex shader".to_string()));
                }
            } else {
                return Err(GpuError::InvalidResource);
            }
        } else {
            return Err(GpuError::InvalidArgument("Vertex shader is required".to_string()));
        }
        
        // Set pixel shader if provided
        if let Some(shader_id) = desc.fragment_shader {
            if let Some(shader) = self.shaders.get(&shader_id) {
                // Check if shader is a pixel/fragment shader
                match shader.stage {
                    ShaderStage::Pixel => {
                        pso_desc.PS = D3D12_SHADER_BYTECODE {
                            pShaderBytecode: unsafe { shader.blob.GetBufferPointer() },
                            BytecodeLength: unsafe { shader.blob.GetBufferSize() },
                        };
                    },
                    _ => {
                        return Err(GpuError::InvalidArgument("Shader is not a pixel/fragment shader".to_string()));
                    }
                }
            } else {
                return Err(GpuError::InvalidResource);
            }
        }
        
        // Set input layout if provided
        if let Some(vertex_layout) = &desc.vertex_layout {
            let input_elements = self.create_input_layout(vertex_layout);
            pso_desc.InputLayout = D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_elements.as_ptr(),
                NumElements: input_elements.len() as u32,
            };
            
            // We need to keep the input_elements alive until CreateGraphicsPipelineState returns
            let pipeline_state = unsafe {
                self.device.as_ref().unwrap().CreateGraphicsPipelineState(&pso_desc)?
            };
            
            // Generate a pipeline ID
            let pipeline_id = PipelineId(self.generate_id());
            
            // Store the pipeline
            self.pipelines.insert(pipeline_id, DirectXPipeline {
                pipeline_state,
                root_signature,
                vertex_layout: Some(vertex_layout.clone()),
            });
            
            Ok(pipeline_id)
        } else {
            // No vertex layout provided
            let pipeline_state = unsafe {
                self.device.as_ref().unwrap().CreateGraphicsPipelineState(&pso_desc)?
            };
            
            // Generate a pipeline ID
            let pipeline_id = PipelineId(self.generate_id());
            
            // Store the pipeline
            self.pipelines.insert(pipeline_id, DirectXPipeline {
                pipeline_state,
                root_signature,
                vertex_layout: None,
            });
            
            Ok(pipeline_id)
        }
    }
    
    // Create a compute pipeline
    fn create_compute_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError> {
        // Create a root signature (no input assembly for compute)
        let root_signature = self.create_root_signature(false)?;
        
        // Create compute pipeline state
        let mut pso_desc = D3D12_COMPUTE_PIPELINE_STATE_DESC {
            pRootSignature: std::mem::ManuallyDrop::new(Some(root_signature.clone())),
            CS: D3D12_SHADER_BYTECODE { pShaderBytecode: std::ptr::null(), BytecodeLength: 0 },
            NodeMask: 0,
            CachedPSO: D3D12_CACHED_PIPELINE_STATE::default(),
            Flags: D3D12_PIPELINE_STATE_FLAG_NONE,
        };
        
        // Set compute shader
        if let Some(shader_id) = desc.compute_shader {
            if let Some(shader) = self.shaders.get(&shader_id) {
                if let ShaderStage::Compute = shader.stage {
                    pso_desc.CS = D3D12_SHADER_BYTECODE {
                        pShaderBytecode: unsafe { shader.blob.GetBufferPointer() },
                        BytecodeLength: unsafe { shader.blob.GetBufferSize() },
                    };
                } else {
                    return Err(GpuError::InvalidArgument("Shader is not a compute shader".to_string()));
                }
            } else {
                return Err(GpuError::InvalidResource);
            }
        } else {
            return Err(GpuError::InvalidArgument("Compute shader is required".to_string()));
        }
        
        // Create the pipeline state
        let pipeline_state = unsafe {
            self.device.as_ref().unwrap().CreateComputePipelineState(&pso_desc)?
        };
        
        // Generate a pipeline ID
        let pipeline_id = PipelineId(self.generate_id());
        
        // Store the pipeline
        self.pipelines.insert(pipeline_id, DirectXPipeline {
            pipeline_state,
            root_signature,
            vertex_layout: None,
        });
        
        Ok(pipeline_id)
    }
    
    // Create a root signature
    fn create_root_signature(&self, allow_input_assembler: bool) -> Result<ID3D12RootSignature, GpuError> {
        // Create a simple root signature for now
        // In a real implementation, this would be customized based on shader needs
        let mut flags = D3D12_ROOT_SIGNATURE_FLAG_NONE;
        if allow_input_assembler {
            flags |= D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;
        }
        
        let root_signature_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: std::ptr::null(),
            NumStaticSamplers: 0,
            pStaticSamplers: std::ptr::null(),
            Flags: flags,
        };
        
        // Serialize the root signature
        let mut signature_blob: Option<ID3DBlob> = None;
        let mut error_blob: Option<ID3DBlob> = None;
        let result = unsafe {
            D3D12SerializeRootSignature(
                &root_signature_desc,
                D3D_ROOT_SIGNATURE_VERSION_1,
                &mut signature_blob,
                Some(&mut error_blob),
            )
        };
        
        if result.is_err() {
            if let Some(error_blob) = error_blob {
                let error_message = unsafe {
                    let error_ptr = error_blob.GetBufferPointer() as *const i8;
                    let error_size = error_blob.GetBufferSize();
                    let slice = std::slice::from_raw_parts(error_ptr as *const u8, error_size);
                    String::from_utf8_lossy(slice).to_string()
                };
                return Err(GpuError::BackendError(format!("Root signature serialization failed: {}", error_message)));
            }
            return Err(GpuError::BackendError("Root signature serialization failed".to_string()));
        }
        
        let signature_blob = signature_blob.unwrap();
        
        // Create the root signature
        let root_signature = unsafe {
            self.device.as_ref().unwrap().CreateRootSignature(
                0, // Single GPU
                std::slice::from_raw_parts(
                    signature_blob.GetBufferPointer() as *const u8,
                    signature_blob.GetBufferSize(),
                ),
            )?
        };
        
        Ok(root_signature)
    }
    
    // Create input layout from vertex layout descriptor
    fn create_input_layout(&self, vertex_layout: &VertexLayoutDescriptor) -> Vec<D3D12_INPUT_ELEMENT_DESC> {
        let mut input_elements = Vec::new();
        
        for attribute in &vertex_layout.attributes {
            // Map shader locations to standard semantic names
            let semantic_name = match attribute.shader_location {
                0 => PCSTR("POSITION\0".as_ptr() as *const u8),
                1 => PCSTR("COLOR\0".as_ptr() as *const u8),
                2 => PCSTR("TEXCOORD\0".as_ptr() as *const u8),
                3 => PCSTR("NORMAL\0".as_ptr() as *const u8),
                4 => PCSTR("TANGENT\0".as_ptr() as *const u8),
                _ => {
                    // For other attributes, use generic ATTRx naming
                    let name = format!("ATTR{}\0", attribute.shader_location);
                    // This string will be temporary, but that's OK because we create the input
                    // elements immediately and don't store the PCSTR long-term
                    PCSTR(name.as_ptr() as *const u8)
                }
            };
            
            let input_element = D3D12_INPUT_ELEMENT_DESC {
                SemanticName: semantic_name,
                SemanticIndex: 0, // Use 0 as default index
                Format: self.vertex_format_to_dxgi_format(attribute.format),
                InputSlot: 0, // Default to slot 0
                AlignedByteOffset: attribute.offset as u32,
                InputSlotClass: match vertex_layout.step_mode {
                    crate::vertex::VertexStepMode::Vertex => D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                    crate::vertex::VertexStepMode::Instance => D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA,
                },
                InstanceDataStepRate: match vertex_layout.step_mode {
                    crate::vertex::VertexStepMode::Vertex => 0,
                    crate::vertex::VertexStepMode::Instance => 1,
                },
            };
            
            input_elements.push(input_element);
        }
        
        input_elements
    }
    
    // Convert primitive topology
    fn convert_primitive_topology(&self, topology: crate::pipeline::PrimitiveTopology) -> D3D12_PRIMITIVE_TOPOLOGY_TYPE {
        match topology {
            crate::pipeline::PrimitiveTopology::PointList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT,
            crate::pipeline::PrimitiveTopology::LineList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            crate::pipeline::PrimitiveTopology::LineStrip => D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            crate::pipeline::PrimitiveTopology::TriangleList => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            crate::pipeline::PrimitiveTopology::TriangleStrip => D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
        }
    }
    
    // Create blend state from descriptor
    fn create_blend_state(&self, blend_state: &crate::pipeline::BlendState) -> D3D12_BLEND_DESC {
        let mut desc = D3D12_BLEND_DESC::default();
        desc.AlphaToCoverageEnable = if blend_state.alpha_to_coverage_enabled { BOOL(1) } else { BOOL(0) };
        desc.IndependentBlendEnable = BOOL(0); // Set to false as default
        
        for (i, target) in blend_state.targets.iter().enumerate() {
            if i >= 8 {
                break; // DX12 supports up to 8 render targets
            }
            
            desc.RenderTarget[i] = D3D12_RENDER_TARGET_BLEND_DESC {
                BlendEnable: if target.blend.is_some() { BOOL(1) } else { BOOL(0) },
                LogicOpEnable: BOOL(0), // Logic operations not used
                SrcBlend: if let Some(blend) = &target.blend {
                    self.convert_blend_factor(blend.src_factor)
                } else {
                    D3D12_BLEND_ONE
                },
                DestBlend: if let Some(blend) = &target.blend {
                    self.convert_blend_factor(blend.dst_factor)
                } else {
                    D3D12_BLEND_ZERO
                },
                BlendOp: if let Some(blend) = &target.blend {
                    self.convert_blend_op(blend.operation)
                } else {
                    D3D12_BLEND_OP_ADD
                },
                SrcBlendAlpha: if let Some(blend) = &target.blend {
                    self.convert_blend_factor(blend.src_factor)
                } else {
                    D3D12_BLEND_ONE
                },
                DestBlendAlpha: if let Some(blend) = &target.blend {
                    self.convert_blend_factor(blend.dst_factor)
                } else {
                    D3D12_BLEND_ZERO
                },
                BlendOpAlpha: if let Some(blend) = &target.blend {
                    self.convert_blend_op(blend.operation)
                } else {
                    D3D12_BLEND_OP_ADD
                },
                LogicOp: D3D12_LOGIC_OP_NOOP, // Default no-op
                RenderTargetWriteMask: self.convert_color_write_mask(target.write_mask),
            };
        }
        
        desc
    }
    
    // Convert blend factor
    fn convert_blend_factor(&self, factor: crate::pipeline::BlendFactor) -> D3D12_BLEND {
        match factor {
            crate::pipeline::BlendFactor::Zero => D3D12_BLEND_ZERO,
            crate::pipeline::BlendFactor::One => D3D12_BLEND_ONE,
            crate::pipeline::BlendFactor::SrcColor => D3D12_BLEND_SRC_COLOR,
            crate::pipeline::BlendFactor::OneMinusSrcColor => D3D12_BLEND_INV_SRC_COLOR,
            crate::pipeline::BlendFactor::DstColor => D3D12_BLEND_DEST_COLOR,
            crate::pipeline::BlendFactor::OneMinusDstColor => D3D12_BLEND_INV_DEST_COLOR,
            crate::pipeline::BlendFactor::SrcAlpha => D3D12_BLEND_SRC_ALPHA,
            crate::pipeline::BlendFactor::OneMinusSrcAlpha => D3D12_BLEND_INV_SRC_ALPHA,
            crate::pipeline::BlendFactor::DstAlpha => D3D12_BLEND_DEST_ALPHA,
            crate::pipeline::BlendFactor::OneMinusDstAlpha => D3D12_BLEND_INV_DEST_ALPHA,
            crate::pipeline::BlendFactor::BlendColor => D3D12_BLEND_BLEND_FACTOR,
            crate::pipeline::BlendFactor::OneMinusBlendColor => D3D12_BLEND_INV_BLEND_FACTOR,
            crate::pipeline::BlendFactor::SrcAlphaSaturated => D3D12_BLEND_SRC_ALPHA_SAT,
        }
    }
    
    // Convert blend operation
    fn convert_blend_op(&self, op: crate::pipeline::BlendOperation) -> D3D12_BLEND_OP {
        match op {
            crate::pipeline::BlendOperation::Add => D3D12_BLEND_OP_ADD,
            crate::pipeline::BlendOperation::Subtract => D3D12_BLEND_OP_SUBTRACT,
            crate::pipeline::BlendOperation::ReverseSubtract => D3D12_BLEND_OP_REV_SUBTRACT,
            crate::pipeline::BlendOperation::Min => D3D12_BLEND_OP_MIN,
            crate::pipeline::BlendOperation::Max => D3D12_BLEND_OP_MAX,
        }
    }
    
    // Convert logic operation
    fn convert_logic_op(&self, op: crate::pipeline::LogicOperation) -> D3D12_LOGIC_OP {
        match op {
            crate::pipeline::LogicOperation::Clear => D3D12_LOGIC_OP_CLEAR,
            crate::pipeline::LogicOperation::Set => D3D12_LOGIC_OP_SET,
            crate::pipeline::LogicOperation::Copy => D3D12_LOGIC_OP_COPY,
            crate::pipeline::LogicOperation::CopyInverted => D3D12_LOGIC_OP_COPY_INVERTED,
            crate::pipeline::LogicOperation::NoOp => D3D12_LOGIC_OP_NOOP,
            crate::pipeline::LogicOperation::Invert => D3D12_LOGIC_OP_INVERT,
            crate::pipeline::LogicOperation::And => D3D12_LOGIC_OP_AND,
            crate::pipeline::LogicOperation::Nand => D3D12_LOGIC_OP_NAND,
            crate::pipeline::LogicOperation::Or => D3D12_LOGIC_OP_OR,
            crate::pipeline::LogicOperation::Nor => D3D12_LOGIC_OP_NOR,
            crate::pipeline::LogicOperation::Xor => D3D12_LOGIC_OP_XOR,
            crate::pipeline::LogicOperation::Equivalent => D3D12_LOGIC_OP_EQUIV,
            crate::pipeline::LogicOperation::AndReverse => D3D12_LOGIC_OP_AND_REVERSE,
            crate::pipeline::LogicOperation::AndInverted => D3D12_LOGIC_OP_AND_INVERTED,
            crate::pipeline::LogicOperation::OrReverse => D3D12_LOGIC_OP_OR_REVERSE,
            crate::pipeline::LogicOperation::OrInverted => D3D12_LOGIC_OP_OR_INVERTED,
        }
    }
    
    // Convert color write mask
    fn convert_color_write_mask(&self, mask: crate::pipeline::ColorWriteMask) -> u8 {
        use crate::pipeline::ColorWriteMask;
        
        let mut result: u8 = 0;
        if mask == ColorWriteMask::ALL {
            // Enable all channels
            result |= (D3D12_COLOR_WRITE_ENABLE_RED.0 |
                     D3D12_COLOR_WRITE_ENABLE_GREEN.0 |
                     D3D12_COLOR_WRITE_ENABLE_BLUE.0 |
                     D3D12_COLOR_WRITE_ENABLE_ALPHA.0) as u8;
        } else if mask == ColorWriteMask::NONE {
            // Disable all channels
            result = 0;
        } else {
            // Individual channels based on enum variants
            match mask {
                ColorWriteMask::RED => result |= D3D12_COLOR_WRITE_ENABLE_RED.0 as u8,
                ColorWriteMask::GREEN => result |= D3D12_COLOR_WRITE_ENABLE_GREEN.0 as u8,
                ColorWriteMask::BLUE => result |= D3D12_COLOR_WRITE_ENABLE_BLUE.0 as u8,
                ColorWriteMask::ALPHA => result |= D3D12_COLOR_WRITE_ENABLE_ALPHA.0 as u8,
                // ALL and NONE are handled in the if/else above
                _ => {} // No additional flags
            }
        }
        result
    }
    
    // Create rasterizer state from descriptor
    fn create_rasterizer_state(&self, rasterizer_state: &crate::pipeline::RasterizerState) -> D3D12_RASTERIZER_DESC {
        D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID, // Default to solid
            CullMode: match rasterizer_state.cull_mode {
                crate::pipeline::CullMode::None => D3D12_CULL_MODE_NONE,
                crate::pipeline::CullMode::Front => D3D12_CULL_MODE_FRONT,
                crate::pipeline::CullMode::Back => D3D12_CULL_MODE_BACK,
            },
            FrontCounterClockwise: if rasterizer_state.front_face == crate::pipeline::FrontFace::CounterClockwise { BOOL(1) } else { BOOL(0) },
            DepthBias: rasterizer_state.depth_bias,
            DepthBiasClamp: rasterizer_state.depth_bias_clamp,
            SlopeScaledDepthBias: rasterizer_state.depth_bias_slope_scale,
            DepthClipEnable: BOOL(1), // Enable depth clipping by default
            MultisampleEnable: BOOL(0), // Disable by default
            AntialiasedLineEnable: BOOL(0), // Disable by default
            ForcedSampleCount: 0,
            ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
        }
    }
    
    // Create depth stencil state from descriptor
    fn create_depth_stencil_state(&self, depth_stencil_state: &crate::pipeline::DepthStencilState) -> D3D12_DEPTH_STENCIL_DESC {
        D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: BOOL(1), // Always enable depth testing
            DepthWriteMask: if depth_stencil_state.depth_write_enabled {
                D3D12_DEPTH_WRITE_MASK_ALL
            } else {
                D3D12_DEPTH_WRITE_MASK_ZERO
            },
            DepthFunc: match depth_stencil_state.depth_compare {
                crate::texture::CompareFunction::Never => D3D12_COMPARISON_FUNC_NEVER,
                crate::texture::CompareFunction::Less => D3D12_COMPARISON_FUNC_LESS,
                crate::texture::CompareFunction::Equal => D3D12_COMPARISON_FUNC_EQUAL,
                crate::texture::CompareFunction::LessEqual => D3D12_COMPARISON_FUNC_LESS_EQUAL,
                crate::texture::CompareFunction::Greater => D3D12_COMPARISON_FUNC_GREATER,
                crate::texture::CompareFunction::NotEqual => D3D12_COMPARISON_FUNC_NOT_EQUAL,
                crate::texture::CompareFunction::GreaterEqual => D3D12_COMPARISON_FUNC_GREATER_EQUAL,
                crate::texture::CompareFunction::Always => D3D12_COMPARISON_FUNC_ALWAYS,
            },
            StencilEnable: if depth_stencil_state.stencil_read_mask != 0 || depth_stencil_state.stencil_write_mask != 0 { BOOL(1) } else { BOOL(0) },
            StencilReadMask: depth_stencil_state.stencil_read_mask as u8,
            StencilWriteMask: depth_stencil_state.stencil_write_mask as u8,
            FrontFace: self.convert_stencil_op_desc(&depth_stencil_state.stencil_front),
            BackFace: self.convert_stencil_op_desc(&depth_stencil_state.stencil_back),
        }
    }
    
    // Convert stencil operation description
    fn convert_stencil_op_desc(&self, desc: &crate::pipeline::StencilFaceState) -> D3D12_DEPTH_STENCILOP_DESC {
        D3D12_DEPTH_STENCILOP_DESC {
            StencilFailOp: self.convert_stencil_op(desc.fail_op),
            StencilDepthFailOp: self.convert_stencil_op(desc.depth_fail_op),
            StencilPassOp: self.convert_stencil_op(desc.pass_op),
            StencilFunc: match desc.compare {
                crate::texture::CompareFunction::Never => D3D12_COMPARISON_FUNC_NEVER,
                crate::texture::CompareFunction::Less => D3D12_COMPARISON_FUNC_LESS,
                crate::texture::CompareFunction::Equal => D3D12_COMPARISON_FUNC_EQUAL,
                crate::texture::CompareFunction::LessEqual => D3D12_COMPARISON_FUNC_LESS_EQUAL,
                crate::texture::CompareFunction::Greater => D3D12_COMPARISON_FUNC_GREATER,
                crate::texture::CompareFunction::NotEqual => D3D12_COMPARISON_FUNC_NOT_EQUAL,
                crate::texture::CompareFunction::GreaterEqual => D3D12_COMPARISON_FUNC_GREATER_EQUAL,
                crate::texture::CompareFunction::Always => D3D12_COMPARISON_FUNC_ALWAYS,
            },
        }
    }
    
    // Convert stencil operation
    fn convert_stencil_op(&self, op: crate::pipeline::StencilOperation) -> D3D12_STENCIL_OP {
        match op {
            crate::pipeline::StencilOperation::Keep => D3D12_STENCIL_OP_KEEP,
            crate::pipeline::StencilOperation::Zero => D3D12_STENCIL_OP_ZERO,
            crate::pipeline::StencilOperation::Replace => D3D12_STENCIL_OP_REPLACE,
            crate::pipeline::StencilOperation::IncrementClamp => D3D12_STENCIL_OP_INCR_SAT,
            crate::pipeline::StencilOperation::DecrementClamp => D3D12_STENCIL_OP_DECR_SAT,
            crate::pipeline::StencilOperation::Invert => D3D12_STENCIL_OP_INVERT,
            crate::pipeline::StencilOperation::IncrementWrap => D3D12_STENCIL_OP_INCR,
            crate::pipeline::StencilOperation::DecrementWrap => D3D12_STENCIL_OP_DECR,
        }
    }
    
    // Helper for uploading buffer data to a GPU-only buffer
    fn upload_buffer_data(&mut self, resource: &ID3D12Resource, data: &[u8], offset: usize, size: usize) -> Result<(), GpuError> {
        // Create an upload buffer
        let upload_heap_props = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        
        let upload_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        
        let mut upload_buffer: Option<ID3D12Resource> = None;
        unsafe {
            self.device.as_ref().unwrap().CreateCommittedResource(
                &upload_heap_props,
                D3D12_HEAP_FLAG_NONE,
                &upload_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut upload_buffer,
            )?;
        }
        
        let upload_buffer = upload_buffer.unwrap();
        
        // Map the upload buffer
        let mut mapped_data: *mut std::ffi::c_void = std::ptr::null_mut();
        let map_range = D3D12_RANGE { Begin: 0, End: 0 }; // We're not reading, just writing
        
        unsafe {
            upload_buffer.Map(0, Some(&map_range), Some(&mut mapped_data))?;
            
            // Copy data to the upload buffer
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapped_data as *mut u8, size);
            
            upload_buffer.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
        }
        
        // Reset the command list and record copy commands
        self.reset_command_list_for_copy()?;
        
        // Copy from upload buffer to default buffer
        unsafe {
            self.command_list.as_ref().unwrap().CopyBufferRegion(
                resource,
                offset as u64,
                &upload_buffer,
                0,
                size as u64,
            );
        }
        
        // Execute the command list
        self.execute_command_list()?;
        
        // Wait for GPU to finish
        self.wait_for_gpu()?;
        
        Ok(())
    }
    
    // Helper for uploading texture data
    fn upload_texture_data(
        &mut self,
        resource: &ID3D12Resource,
        data: &[u8],
        format: DXGI_FORMAT,
        width: u32,
        height: u32,
        mip_levels: u32,
    ) -> Result<(), GpuError> {
        // Get footprints for all mip levels
        let mut layouts: Vec<D3D12_PLACED_SUBRESOURCE_FOOTPRINT> = vec![Default::default(); mip_levels as usize];
        let mut row_sizes: Vec<u32> = vec![0; mip_levels as usize];
        let mut total_bytes: u64 = 0;
        
        let resource_desc = unsafe { resource.GetDesc() };
        
        unsafe {
            self.device.as_ref().unwrap().GetCopyableFootprints(
                &resource_desc,
                0,
                mip_levels,
                0,
                Some(layouts.as_mut_ptr()),
                Some(row_sizes.as_mut_ptr()),
                None, // Don't need row sizes
                Some(&mut total_bytes as *mut u64),
            );
        }
        
        // Create upload buffer large enough for all mip levels
        let upload_heap_props = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        
        let upload_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: total_bytes,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        
        let mut upload_buffer: Option<ID3D12Resource> = None;
        unsafe {
            self.device.as_ref().unwrap().CreateCommittedResource(
                &upload_heap_props,
                D3D12_HEAP_FLAG_NONE,
                &upload_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut upload_buffer,
            )?;
        }
        
        let upload_buffer = upload_buffer.unwrap();
        
        // Map the upload buffer
        let mut mapped_data: *mut std::ffi::c_void = std::ptr::null_mut();
        let map_range = D3D12_RANGE { Begin: 0, End: 0 }; // We're not reading, just writing
        
        unsafe {
            upload_buffer.Map(0, Some(&map_range), Some(&mut mapped_data))?;
            
            // Copy data to the upload buffer for each mip level
            // For simplicity, we assume a single mip level for now
            let bytes_per_pixel = bytes_per_pixel(format);
            let src_pitch = width * bytes_per_pixel;
            let dst_pitch = layouts[0].Footprint.RowPitch as u32;
            
            for row in 0..height {
                let src_offset = row as usize * src_pitch as usize;
                let dst_offset = row as usize * dst_pitch as usize + layouts[0].Offset as usize;
                
                std::ptr::copy_nonoverlapping(
                    data[src_offset..].as_ptr(),
                    (mapped_data as *mut u8).add(dst_offset),
                    std::cmp::min(src_pitch as usize, dst_pitch as usize),
                );
            }
            
            upload_buffer.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
        }
        
        // Reset the command list and record copy commands
        self.reset_command_list_for_copy()?;
        
        // Copy from upload buffer to texture
        for mip in 0..mip_levels {
            let dest_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: ManuallyDrop::new(Some(resource.clone())),
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: mip,
                },
            };
            
            let src_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: ManuallyDrop::new(Some(upload_buffer.clone())),
                Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: layouts[mip as usize],
                },
            };
            
            // For simplicity, we only deal with the first mip level
            if mip == 0 {
                unsafe {
                    self.command_list.as_ref().unwrap().CopyTextureRegion(
                        &dest_location,
                        0, 0, 0,
                        &src_location,
                        None,
                    );
                }
            }
        }
        
        // Execute the command list
        self.execute_command_list()?;
        
        // Wait for GPU to finish
        self.wait_for_gpu()?;
        
        Ok(())
    }
    
    // Helper for resetting command list for copying
    fn reset_command_list_for_copy(&mut self) -> Result<(), GpuError> {
        // Reset command allocator and list
        unsafe {
            self.frames[0].command_allocator.Reset()?;
            self.command_list.as_ref().unwrap().Reset(&self.frames[0].command_allocator, None)?;
        }
        
        Ok(())
    }
    
    // Helper for executing the command list
    fn execute_command_list(&mut self) -> Result<(), GpuError> {
        // Close the command list
        unsafe {
            self.command_list.as_ref().unwrap().Close()?;
            
            // Execute the command list
            let command_lists: [Option<ID3D12CommandList>; 1] = [Some(self.command_list.as_ref().unwrap().cast()?)];
            self.command_queue.as_ref().unwrap().ExecuteCommandLists(&command_lists);
        }
        
        Ok(())
    }
    
    // Helper for transitioning a resource's state immediately
    fn transition_resource_immediate(
        &mut self,
        resource: &ID3D12Resource,
        state_before: D3D12_RESOURCE_STATES,
        state_after: D3D12_RESOURCE_STATES,
    ) -> Result<(), GpuError> {
        // Skip if states are the same
        if state_before == state_after {
            return Ok(());
        }
        
        // Reset command list
        self.reset_command_list_for_copy()?;
        
        // Create and record the barrier
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(resource.clone())),
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    StateBefore: state_before,
                    StateAfter: state_after,
                }),
            },
        };
        
        unsafe {
            self.command_list.as_ref().unwrap().ResourceBarrier(&[barrier]);
        }
        
        // Execute the command list
        self.execute_command_list()?;
        
        // Wait for GPU to finish
        self.wait_for_gpu()?;
        
        Ok(())
    }
    
    // Helper for setting pipeline state
    fn set_pipeline(&mut self, pipeline_id: PipelineId) -> Result<(), GpuError> {
        // Skip if the pipeline is already set
        if self.current_pipeline_id == Some(pipeline_id) {
            return Ok(());
        }
        
        // Get the pipeline
        let pipeline = match self.pipelines.get(&pipeline_id) {
            Some(pipeline) => pipeline,
            None => return Err(GpuError::InvalidResource),
        };
        
        // Set the pipeline state and root signature
        unsafe {
            self.command_list.as_ref().unwrap().SetPipelineState(&pipeline.pipeline_state);
            self.command_list.as_ref().unwrap().SetGraphicsRootSignature(&pipeline.root_signature);
            
            // Set primitive topology - hardcoded for triangle_window example
            // In a real implementation, this would be stored with the pipeline
            self.command_list.as_ref().unwrap().IASetPrimitiveTopology(Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        }
        
        // Set a default scissor rect that covers the entire viewport
        if let Some(surface_id) = self.current_surface_id {
            if let Some(surface) = self.surfaces.get(&surface_id) {
                unsafe {
                    let width = surface.render_targets[0].GetDesc().Width as i32;
                    let height = surface.render_targets[0].GetDesc().Height as i32;
                    
                    let scissor_rect = RECT {
                        left: 0,
                        top: 0,
                        right: width,
                        bottom: height,
                    };
                    
                    self.command_list.as_ref().unwrap().RSSetScissorRects(&[scissor_rect]);
                }
            }
        }
        
        // Update the current pipeline
        self.current_pipeline_id = Some(pipeline_id);
        
        Ok(())
    }
    
    // Helper for setting vertex buffer
    fn set_vertex_buffer(&mut self, slot: u32, buffer_id: BufferId, offset: usize) -> Result<(), GpuError> {
        // Get the current buffer state and resource before any mutable borrows
        let (buffer_state, buffer_resource) = {
            if let Some(buffer) = self.buffers.get(&buffer_id) {
                (buffer.state, buffer.resource.clone())
            } else {
                return Err(GpuError::InvalidBuffer(format!("Buffer not found: {:?}", buffer_id)));
            }
        };
        
        // Get stride from pipeline
        let stride = if let Some(pipeline_id) = self.current_pipeline_id {
            if let Some(pipeline) = self.pipelines.get(&pipeline_id) {
                // Get stride from the pipeline's vertex layout if available
                if let Some(vertex_layout) = &pipeline.vertex_layout {
                    println!("Using vertex layout stride: {}", vertex_layout.stride);
                    vertex_layout.stride
                } else {
                    // Fallback to hardcoded value 
                    println!("No vertex layout found, using default stride of 28 bytes");
                    28 // 7 floats per vertex (3 position + 4 color) * 4 bytes
                }
            } else {
                println!("WARNING: Pipeline not found: {:?}, using default stride of 28 bytes", pipeline_id);
                28
            }
        } else {
            println!("WARNING: No pipeline set before setting vertex buffer, using default stride of 28 bytes");
            28
        };
        
        // Get buffer size for the view
        let buffer_size = if let Some(buffer) = self.buffers.get(&buffer_id) {
            println!("Setting vertex buffer with SizeInBytes: {}, StrideInBytes: {}", buffer.size, stride); // Debug info
            buffer.size
        } else {
            return Err(GpuError::InvalidBuffer(format!("Buffer not found: {:?}", buffer_id)));
        };
        
        // Transition the buffer to VERTEX_AND_CONSTANT_BUFFER state if needed
        if buffer_state != D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER {
            self.transition_resource_immediate(
                &buffer_resource,
                buffer_state,
                D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER
            )?;
            
            // Update the buffer state
            if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                buffer.state = D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER;
            }
        }
        
        // Create vertex buffer view with the updated buffer
        if let Some(buffer) = self.buffers.get(&buffer_id) {
            let view = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: unsafe { buffer.resource.GetGPUVirtualAddress() } + offset as u64,
                SizeInBytes: (buffer_size - offset) as u32,
                StrideInBytes: stride as u32,
            };
            
            // Set the vertex buffer
            unsafe {
                self.command_list.as_ref().unwrap().IASetVertexBuffers(slot, Some(&[view]));
            }
            
            Ok(())
        } else {
            Err(GpuError::InvalidArgument(format!("Buffer {:?} not found", buffer_id)))
        }
    }
    
    // Helper for setting index buffer
    fn set_index_buffer(&mut self, buffer_id: BufferId, offset: usize, index_format: IndexFormat) -> Result<(), GpuError> {
        // Get the buffer
        let buffer = match self.buffers.get(&buffer_id) {
            Some(buffer) => buffer,
            None => return Err(GpuError::InvalidResource),
        };
        
        // Create index buffer view
        let ibv = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: unsafe { buffer.resource.GetGPUVirtualAddress() } + offset as u64,
            SizeInBytes: (buffer.size - offset) as u32,
            Format: to_index_format(index_format),
        };
        
        // Set the index buffer
        unsafe {
            self.command_list.as_ref().unwrap().IASetIndexBuffer(Some(&ibv));
        }
        
        Ok(())
    }
    
    // Convert vertex format to DXGI format
    fn vertex_format_to_dxgi_format(&self, format: VertexFormat) -> DXGI_FORMAT {
        use VertexFormat;
        
        match format {
            VertexFormat::Float2 => DXGI_FORMAT_R32G32_FLOAT,
            VertexFormat::Float3 => DXGI_FORMAT_R32G32B32_FLOAT,
            VertexFormat::Float4 => DXGI_FORMAT_R32G32B32A32_FLOAT,
            VertexFormat::Uint2 => DXGI_FORMAT_R32G32_UINT,
            VertexFormat::Uint3 => DXGI_FORMAT_R32G32B32_UINT,
            VertexFormat::Uint4 => DXGI_FORMAT_R32G32B32A32_UINT,
            VertexFormat::Int2 => DXGI_FORMAT_R32G32_SINT,
            VertexFormat::Int3 => DXGI_FORMAT_R32G32B32_SINT,
            VertexFormat::Int4 => DXGI_FORMAT_R32G32B32A32_SINT,
            VertexFormat::Float => DXGI_FORMAT_R32_FLOAT,
            VertexFormat::Int => DXGI_FORMAT_R32_SINT,
            VertexFormat::Uint => DXGI_FORMAT_R32_UINT,
            VertexFormat::Unorm8x4 => DXGI_FORMAT_R8G8B8A8_UNORM,
            VertexFormat::Snorm8x4 => DXGI_FORMAT_R8G8B8A8_SNORM,
            // Add more formats as needed
            _ => DXGI_FORMAT_UNKNOWN,
        }
    }
}

// Helper function to calculate bytes per pixel for a DXGI format
fn bytes_per_pixel(format: DXGI_FORMAT) -> u32 {
    match format {
        DXGI_FORMAT_R8_UNORM | DXGI_FORMAT_R8_UINT => 1,
        DXGI_FORMAT_R8G8_UNORM | DXGI_FORMAT_R8G8_UINT => 2,
        DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_R8G8B8A8_UNORM_SRGB |
        DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM_SRGB |
        DXGI_FORMAT_R16G16_UINT | DXGI_FORMAT_R32_UINT | DXGI_FORMAT_R32_FLOAT => 4,
        DXGI_FORMAT_R32G32_FLOAT => 8,
        DXGI_FORMAT_R32G32B32_FLOAT => 12,
        DXGI_FORMAT_R32G32B32A32_FLOAT => 16,
        _ => 4, // Default to 4 bytes if unknown
    }
}