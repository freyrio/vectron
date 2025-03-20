# Vectron Phase 1 Implementation Plan - Embedder and GPU Foundation

## Summary

This document outlines the first phase of Vectron's implementation, focusing on the foundational embedder and GPU abstraction layers. We'll establish a platform-agnostic API for both layers, with initial implementations for Windows (Win32) and DirectX. The embedder layer will support both desktop (window-based) and mobile (view-based) paradigms through a shared base interface. All implementations will be organized within their respective crates using a modular approach rather than creating separate crates for each platform/backend.

## Project Structure

```
vectron/
├── vectron_embedder/
│   ├── src/
│   │   ├── lib.rs            // Exports public API
│   │   ├── embedder.rs       // Base embedder trait definitions
│   │   ├── window.rs         // Window-based embedder interfaces
│   │   ├── view.rs           // View-based embedder interfaces
│   │   ├── platform/         // Platform-specific implementations
│   │   │   ├── mod.rs        // Exports platform modules
│   │   │   ├── windows/      // Windows platform implementation
│   │   │   │   ├── mod.rs    // Windows implementation exports
│   │   │   │   └── win32.rs  // Win32 implementation
│   │   │   ├── macos/        // macOS stubs for future implementation
│   │   │   ├── linux/        // Linux stubs for future implementation
│   │   │   ├── android/      // Android stubs for future implementation
│   │   │   └── ios/          // iOS stubs for future implementation
│   │   └── common/           // Common utilities and shared code
│   └── Cargo.toml
└── vectron_gpu/
    ├── src/
    │   ├── lib.rs            // Exports public API
    │   ├── backend.rs        // Base GPU backend traits
    │   ├── pipeline.rs       // Pipeline abstractions
    │   ├── buffer.rs         // Buffer abstractions
    │   ├── texture.rs        // Texture abstractions
    │   ├── shader.rs         // Shader abstractions
    │   ├── backends/         // Backend implementations
    │   │   ├── mod.rs        // Exports backend modules
    │   │   ├── directx/      // DirectX implementation
    │   │   │   ├── mod.rs    // DirectX exports
    │   │   │   └── dx12.rs   // DirectX 12 implementation
    │   │   ├── vulkan/       // Vulkan stubs for future implementation
    │   │   ├── metal/        // Metal stubs for future implementation
    │   │   └── software/     // Software renderer stubs
    │   └── common/           // Common utilities and shared code
    └── Cargo.toml
```

## Key Elements - Embedder Layer

### Base Embedder Trait

```rust
// In vectron_embedder/src/embedder.rs

/// Surface handle representing a drawable surface
pub struct Surface {
    pub handle: *mut std::ffi::c_void,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
}

/// Common configuration for all embedder types
#[derive(Debug, Clone)]
pub struct EmbedderConfig {
    pub application_name: String,
    pub enable_high_dpi: bool,
    pub vsync: bool,
}

/// Common events for all platforms
#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    Resized { width: u32, height: u32 },
    Moved { x: i32, y: i32 },
    Input(InputEvent),
    // Other common events...
}

/// Base embedder trait shared by all platform implementations
pub trait Embedder {
    type Handle: Copy + Clone + PartialEq + Eq + std::hash::Hash;
    
    /// Initialize the embedder with the given configuration
    fn init(&mut self, config: EmbedderConfig) -> Result<(), EmbedderError>;
    
    /// Process pending events
    fn process_events(&mut self) -> Vec<Event>;
    
    /// Check if the embedder is running
    fn is_running(&self) -> bool;
    
    /// Get a surface for rendering
    fn get_surface(&self, handle: Self::Handle) -> Result<Surface, EmbedderError>;
    
    /// Request a redraw
    fn request_redraw(&mut self, handle: Self::Handle);
    
    /// Shutdown the embedder
    fn shutdown(&mut self);
}
```

### Window Embedder Trait (Desktop)

```rust
// In vectron_embedder/src/window.rs

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub decorated: bool,
    pub visible: bool,
    pub position: Option<(i32, i32)>,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub parent: Option<WindowHandle>,
    // Other window-specific options...
}

/// Window handle type alias
pub type WindowHandle = u64;

/// Window embedder trait for desktop platforms
pub trait WindowEmbedder: Embedder<Handle = WindowHandle> {
    /// Create a new window with the given configuration
    fn create_window(&mut self, config: WindowConfig) -> Result<WindowHandle, EmbedderError>;
    
    /// Destroy a window
    fn destroy_window(&mut self, handle: WindowHandle);
    
    /// Show a window
    fn show_window(&mut self, handle: WindowHandle);
    
    /// Hide a window
    fn hide_window(&mut self, handle: WindowHandle);
    
    /// Set window title
    fn set_window_title(&mut self, handle: WindowHandle, title: &str);
    
    /// Set window size
    fn set_window_size(&mut self, handle: WindowHandle, width: u32, height: u32);
    
    /// Get window size
    fn get_window_size(&self, handle: WindowHandle) -> (u32, u32);
    
    /// Set window position
    fn set_window_position(&mut self, handle: WindowHandle, x: i32, y: i32);
    
    /// Get window position
    fn get_window_position(&self, handle: WindowHandle) -> (i32, i32);
    
    /// Set window as child of another window
    fn set_window_parent(&mut self, handle: WindowHandle, parent: WindowHandle);
}
```

### View Embedder Trait (Mobile)

```rust
// In vectron_embedder/src/view.rs

/// View configuration
#[derive(Debug, Clone)]
pub struct ViewConfig {
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub parent: Option<ViewHandle>,
    // Other view-specific options...
}

/// View handle type alias
pub type ViewHandle = u64;

/// View embedder trait for mobile platforms
pub trait ViewEmbedder: Embedder<Handle = ViewHandle> {
    /// Create a new view with the given configuration
    fn create_view(&mut self, config: ViewConfig) -> Result<ViewHandle, EmbedderError>;
    
    /// Destroy a view
    fn destroy_view(&mut self, handle: ViewHandle);
    
    /// Set view visibility
    fn set_view_visibility(&mut self, handle: ViewHandle, visible: bool);
    
    /// Set view size
    fn set_view_size(&mut self, handle: ViewHandle, width: u32, height: u32);
    
    /// Get view size
    fn get_view_size(&self, handle: ViewHandle) -> (u32, u32);
    
    /// Set view as subview of another view
    fn set_view_parent(&mut self, handle: ViewHandle, parent: ViewHandle);
    
    /// Add native view as a child
    fn add_native_subview(&mut self, handle: ViewHandle, native_view: *mut std::ffi::c_void);
}
```

### Win32 Implementation

```rust
// In vectron_embedder/src/platform/windows/win32.rs

pub struct Win32Embedder {
    instance: HINSTANCE,
    windows: HashMap<WindowHandle, Win32WindowData>,
    running: bool,
    next_handle: WindowHandle,
    event_queue: VecDeque<Event>,
    config: Option<EmbedderConfig>,
}

struct Win32WindowData {
    hwnd: HWND,
    title: String,
    width: u32,
    height: u32,
    parent: Option<WindowHandle>,
    // Other Win32-specific window data...
}

impl Embedder for Win32Embedder {
    type Handle = WindowHandle;
    
    fn init(&mut self, config: EmbedderConfig) -> Result<(), EmbedderError> {
        // Register window class
        // Set up Win32 application instance
        // Store configuration
        // ...
        self.running = true;
        Ok(())
    }
    
    fn process_events(&mut self) -> Vec<Event> {
        // Process Win32 message queue
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                
                // Process message and add to event queue
                // ...
            }
        }
        
        // Return collected events
        self.event_queue.drain(..).collect()
    }
    
    fn is_running(&self) -> bool {
        self.running
    }
    
    fn get_surface(&self, handle: WindowHandle) -> Result<Surface, EmbedderError> {
        // Get window data
        let window = self.windows.get(&handle).ok_or(EmbedderError::InvalidHandle)?;
        
        // Create surface from window handle
        Ok(Surface {
            handle: window.hwnd as *mut std::ffi::c_void,
            width: window.width,
            height: window.height,
            scale_factor: self.get_window_scale_factor(handle),
        })
    }
    
    // Additional method implementations...
}

impl WindowEmbedder for Win32Embedder {
    fn create_window(&mut self, config: WindowConfig) -> Result<WindowHandle, EmbedderError> {
        // Create Win32 window with the specified configuration
        // Store window data
        // Return handle
        // ...
        
        let handle = self.next_handle;
        self.next_handle += 1;
        
        // Create actual Win32 window...
        
        Ok(handle)
    }
    
    // Additional method implementations...
}
```

## Key Elements - GPU Layer

### Base GPU Backend Traits

```rust
// In vectron_gpu/src/backend.rs

/// Surface descriptor for creating a render target
pub struct SurfaceDescriptor {
    pub handle: *mut std::ffi::c_void,
    pub width: u32,
    pub height: u32,
}

/// GPU backend configuration
#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub application_name: String,
    pub enable_debug: bool,
    pub preferred_format: Option<TextureFormat>,
    pub vsync: bool,
}

/// Backend capabilities
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub max_texture_size: u32,
    pub supports_compute: bool,
    pub supports_storage_buffers: bool,
    pub supports_float_textures: bool,
    // Other capability flags...
}

/// Base GPU backend trait
pub trait GpuBackend {
    /// Initialize the backend with the given configuration
    fn init(&mut self, config: BackendConfig) -> Result<(), GpuError>;
    
    /// Create a render surface from a platform surface
    fn create_surface(&mut self, desc: SurfaceDescriptor) -> Result<SurfaceId, GpuError>;
    
    /// Destroy a render surface
    fn destroy_surface(&mut self, id: SurfaceId);
    
    /// Resize a surface
    fn resize_surface(&mut self, id: SurfaceId, width: u32, height: u32) -> Result<(), GpuError>;
    
    /// Create a pipeline
    fn create_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError>;
    
    /// Create a buffer
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<BufferId, GpuError>;
    
    /// Update buffer data
    fn update_buffer(&mut self, id: BufferId, data: &[u8], offset: usize) -> Result<(), GpuError>;
    
    /// Create a texture
    fn create_texture(&mut self, desc: TextureDescriptor) -> Result<TextureId, GpuError>;
    
    /// Update texture data
    fn update_texture(&mut self, id: TextureId, data: &[u8], desc: TextureUpdateDescriptor) -> Result<(), GpuError>;
    
    /// Create a shader
    fn create_shader(&mut self, desc: ShaderDescriptor) -> Result<ShaderId, GpuError>;
    
    /// Begin rendering to a surface
    fn begin_frame(&mut self, surface_id: SurfaceId) -> Result<(), GpuError>;
    
    /// Submit render commands
    fn submit_commands(&mut self, commands: &[RenderCommand]) -> Result<(), GpuError>;
    
    /// End rendering and present
    fn end_frame(&mut self) -> Result<(), GpuError>;
    
    /// Get backend capabilities
    fn get_capabilities(&self) -> BackendCapabilities;
    
    /// Shutdown the backend
    fn shutdown(&mut self);
}
```

### DirectX 12 Implementation

```rust
// In vectron_gpu/src/backends/directx/dx12.rs

pub struct DirectX12Backend {
    device: ComPtr<ID3D12Device>,
    command_queue: ComPtr<ID3D12CommandQueue>,
    swap_chains: HashMap<SurfaceId, ComPtr<IDXGISwapChain3>>,
    rtv_heap: ComPtr<ID3D12DescriptorHeap>,
    command_allocators: Vec<ComPtr<ID3D12CommandAllocator>>,
    pipelines: HashMap<PipelineId, Dx12Pipeline>,
    buffers: HashMap<BufferId, Dx12Buffer>,
    textures: HashMap<TextureId, Dx12Texture>,
    shaders: HashMap<ShaderId, Dx12Shader>,
    frame_index: u32,
    fence_value: u64,
    fence: ComPtr<ID3D12Fence>,
    fence_event: HANDLE,
    capabilities: BackendCapabilities,
    // Other DirectX-specific state...
}

struct Dx12Pipeline {
    pipeline_state: ComPtr<ID3D12PipelineState>,
    root_signature: ComPtr<ID3D12RootSignature>,
    // Other pipeline-specific data...
}

impl GpuBackend for DirectX12Backend {
    fn init(&mut self, config: BackendConfig) -> Result<(), GpuError> {
        // Initialize DirectX 12
        // Create device, command queue, etc.
        // Set up descriptor heaps
        // Set up synchronization primitives
        // Detect capabilities
        // ...
        
        Ok(())
    }
    
    fn create_surface(&mut self, desc: SurfaceDescriptor) -> Result<SurfaceId, GpuError> {
        // Create a swap chain for the given surface
        // Store swap chain in map
        // Return surface ID
        // ...
        
        let id = generate_id();
        
        // Create swap chain using DXGI and the provided surface handle...
        
        Ok(id)
    }
    
    fn create_pipeline(&mut self, desc: PipelineDescriptor) -> Result<PipelineId, GpuError> {
        // Create root signature
        // Compile shaders
        // Create pipeline state object
        // Store pipeline in map
        // Return pipeline ID
        // ...
        
        let id = generate_id();
        
        // Create DirectX 12 pipeline...
        
        Ok(id)
    }
    
    fn begin_frame(&mut self, surface_id: SurfaceId) -> Result<(), GpuError> {
        // Wait for previous frame to complete
        // Get current back buffer
        // Reset command allocator and command list
        // Set render targets
        // ...
        
        Ok(())
    }
    
    fn submit_commands(&mut self, commands: &[RenderCommand]) -> Result<(), GpuError> {
        // Process and execute render commands
        // Set pipeline states, vertex buffers, index buffers, etc.
        // Issue draw calls
        // ...
        
        Ok(())
    }
    
    fn end_frame(&mut self) -> Result<(), GpuError> {
        // Close command list
        // Execute command list
        // Present swap chain
        // Signal fence for synchronization
        // ...
        
        Ok(())
    }
    
    // Additional method implementations...
}
```

### Pipeline and Buffer Abstractions

```rust
// In vectron_gpu/src/pipeline.rs

/// Pipeline type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineType {
    Graphics,
    Compute,
}

/// Pipeline descriptor
#[derive(Debug, Clone)]
pub struct PipelineDescriptor {
    pub type_: PipelineType,
    pub vertex_shader: Option<ShaderId>,
    pub fragment_shader: Option<ShaderId>,
    pub compute_shader: Option<ShaderId>,
    pub vertex_layout: Option<VertexLayoutDescriptor>,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
    pub rasterizer_state: RasterizerState,
    pub primitive_topology: PrimitiveTopology,
    pub render_target_formats: Vec<TextureFormat>,
    pub depth_stencil_format: Option<TextureFormat>,
    // Other pipeline configuration...
}

// In vectron_gpu/src/buffer.rs

/// Buffer usage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
    Indirect,
}

/// Buffer descriptor
#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    pub size: usize,
    pub usage: BufferUsage,
    pub cpu_access: CpuAccessMode,
    pub initial_data: Option<Vec<u8>>,
}

/// CPU access mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuAccessMode {
    None,
    Write,
    Read,
    ReadWrite,
}
```

## Integration Between Embedder and GPU

```rust
// Example of how the two layers will work together

// Application code
fn main() {
    // Create embedder
    let mut embedder = Win32Embedder::new();
    
    // Initialize embedder
    embedder.init(EmbedderConfig {
        application_name: "Vectron Demo".to_string(),
        enable_high_dpi: true,
        vsync: true,
    }).expect("Failed to initialize embedder");
    
    // Create window
    let window = embedder.create_window(WindowConfig {
        title: "Vectron Window".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        decorated: true,
        visible: true,
        position: None,
        min_size: None,
        max_size: None,
        parent: None,
    }).expect("Failed to create window");
    
    // Create GPU backend
    let mut gpu = DirectX12Backend::new();
    
    // Initialize GPU backend
    gpu.init(BackendConfig {
        application_name: "Vectron Demo".to_string(),
        enable_debug: true,
        preferred_format: None,
        vsync: true,
    }).expect("Failed to initialize GPU");
    
    // Get surface from embedder
    let surface = embedder.get_surface(window).expect("Failed to get surface");
    
    // Create GPU surface
    let gpu_surface = gpu.create_surface(SurfaceDescriptor {
        handle: surface.handle,
        width: surface.width,
        height: surface.height,
    }).expect("Failed to create GPU surface");
    
    // Main loop
    while embedder.is_running() {
        // Process events
        let events = embedder.process_events();
        
        // Handle events
        for event in events {
            match event {
                Event::Quit => return,
                Event::Resized { width, height } => {
                    gpu.resize_surface(gpu_surface, width, height).expect("Failed to resize surface");
                },
                // Handle other events...
                _ => {},
            }
        }
        
        // Render frame
        gpu.begin_frame(gpu_surface).expect("Failed to begin frame");
        
        // Submit render commands
        gpu.submit_commands(&[/* ... */]).expect("Failed to submit commands");
        
        // End frame
        gpu.end_frame().expect("Failed to end frame");
    }
    
    // Cleanup
    gpu.destroy_surface(gpu_surface);
    gpu.shutdown();
    embedder.shutdown();
}
```

## Implementation Plan Milestones

1. **Base Trait Definitions (Week 1)**
   - Define base embedder and GPU backend traits
   - Define handle and ID types
   - Define error types and result wrappers

2. **Win32 Embedder Implementation (Week 2-3)**
   - Implement window creation and management
   - Implement event processing
   - Implement surface handling

3. **DirectX 12 Backend Implementation (Week 3-4)**
   - Implement device and swap chain setup
   - Implement basic resource creation (buffers, textures)
   - Implement pipeline state management
   - Implement rendering and presentation

4. **Integration and Testing (Week 5)**
   - Create integration tests
   - Implement basic triangle rendering example
   - Test multi-window support
   - Optimize performance bottlenecks

5. **Documentation and Cleanup (Week 6)**
   - Write API documentation
   - Clean up code and improve error handling
   - Prepare for next phase of development

## Key Design Considerations

1. **Error Handling**
   - Use custom error types with detailed error information
   - Provide context for debugging platform-specific issues
   - Include validation checks to catch common mistakes

2. **Thread Safety**
   - Design APIs with thread safety in mind
   - Use interior mutability where appropriate
   - Provide clear documentation on thread safety guarantees

3. **Performance**
   - Minimize allocations in hot paths
   - Use efficient data structures for resource management
   - Batch operations where possible to reduce overhead

4. **Extensibility**
   - Design traits and interfaces for future extensions
   - Leave room for additional backends and platforms
   - Use feature flags for optional functionality

This Phase 1 implementation plan provides a solid foundation for the Vectron IMGUI library, focusing on the embedder and GPU abstraction layers with initial support for Windows (Win32) and DirectX. The modular design allows for easy extension to other platforms and backends in future phases.