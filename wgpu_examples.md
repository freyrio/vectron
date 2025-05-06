use std::borrow::Cow;
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, CreateSurfaceError, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, InstanceFlags, Limits, PresentMode, Queue, RequestAdapterOptions,
    Surface, SurfaceCapabilities, SurfaceConfiguration, SurfaceError, SurfaceTarget, TextureFormat,
    TextureUsages, GlesRobustness,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

/// Represents the WGPU rendering backend.
/// The lifetime 'window ensures that the Surface does not outlive the window it's attached to.
pub struct WgpuBackend<'window> {
    instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'window>>, // Surface now correctly uses the 'window lifetime
    surface_config: Option<SurfaceConfiguration>,
    surface_format: Option<TextureFormat>,
    window_width: u32,
    window_height: u32,
}

impl<'window> WgpuBackend<'window> {
    /// Creates a new WgpuBackend.
    /// This function is asynchronous as it needs to request the adapter and device from the GPU.
    ///
    /// # Arguments
    ///
    /// * `window_handle_owner`: A type that can provide raw window and display handles.
    ///                          This is typically your window object (e.g., `winit::window::Window`).
    ///                          It must implement `HasWindowHandle` and `HasDisplayHandle`.
    /// * `width`: The initial width of the surface.
    /// * `height`: The initial height of the surface.
    ///
    /// # Panics
    ///
    /// Panics if a suitable adapter or device cannot be found, or if surface creation fails.
    /// In a production application, you would likely return a `Result` instead of panicking.
    pub async fn new<W>(
        window_handle_owner: &'window W,
        width: u32,
        height: u32,
    ) -> Self
    where
        W: HasWindowHandle + HasDisplayHandle + Send + Sync, // Window handle owner must be Send + Sync
    {
        log::info!("Initializing WGPU backend (wgpu 0.20.x)...");

        // InstanceDescriptor specifies how wgpu will find and use the GPU.
        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(), // Use all available backends (Vulkan, Metal, DX12, OpenGL/WebGL)
            flags: if cfg!(debug_assertions) {
                InstanceFlags::VALIDATION | InstanceFlags::DEBUG // Enable validation layers and debug markers in debug builds
            } else {
                InstanceFlags::empty() // No extra flags in release builds
            },
            dx12_shader_compiler: wgpu::Dx12Compiler::default(), // Use default DXC or FXC for DirectX 12
            gles_khr_robustness: GlesRobustness::default(), // Default GLES robustness settings
        };

        let instance = Instance::new(instance_descriptor);
        log::info!("WGPU Instance created.");

        // Create a surface.
        // The surface is the part of the window that we draw to.
        // It needs to be created with a window handle that is valid for the lifetime 'window.
        // `create_surface` takes `impl Into<SurfaceTarget<'window>>`.
        // `&'window W` (where W: HasWindowHandle + HasDisplayHandle) can be converted into SurfaceTarget.
        let surface = match instance.create_surface(window_handle_owner) {
            Ok(s) => {
                log::info!("WGPU Surface created.");
                s
            }
            Err(e) => {
                log::error!("Failed to create surface: {:?}", e);
                // Depending on the application, you might want to panic or return an error.
                panic!("Surface creation failed: {:?}", e);
            }
        };

        // Request an adapter.
        // The adapter is a handle to a physical graphics device.
        let adapter_options = RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, // Request a high-performance GPU
            compatible_surface: Some(&surface), // Ensure the adapter is compatible with our surface
            force_fallback_adapter: false, // Do not use a software renderer if no hardware is found
        };

        let adapter = instance
            .request_adapter(&adapter_options)
            .await
            .unwrap_or_else(|| {
                log::error!("Failed to find a suitable adapter.");
                panic!("Failed to find a suitable adapter.");
            });
        log::info!("WGPU Adapter selected: {}", adapter.get_info().name);


        // Request a device and queue.
        // The device is a logical connection to the graphics device, used to create resources.
        // The queue is used to submit command buffers to the GPU.
        let device_descriptor = DeviceDescriptor {
            label: Some("WGPU Device"),
            required_features: Features::empty(), // Specify required features, e.g., Features::TEXTURE_COMPRESSION_BC
            required_limits: if cfg!(target_arch = "wasm32") {
                Limits::downlevel_webgl2_defaults() // WebGL2 limits for wasm targets
            } else {
                Limits::default() // Native limits for other targets
            },
            memory_hints: wgpu::MemoryHints::default(), // Default memory allocation hints
        };

        let (device, queue) = adapter
            .request_device(&device_descriptor, None /* trace_path, for debugging */)
            .await
            .unwrap_or_else(|e| {
                log::error!("Failed to create device: {:?}", e);
                panic!("Failed to create device: {:?}", e);
            });
        log::info!("WGPU Device and Queue created.");

        // Configure the surface.
        // This specifies how the surface will store and present images.
        let surface_caps = surface.get_capabilities(&adapter);

        // Choose a surface format, prefer sRGB if available for better color representation.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or_else(|| {
                log::warn!("No sRGB surface format found, using first available: {:?}", surface_caps.formats[0]);
                surface_caps.formats[0] // Fallback to the first available format
            });
        log::info!("Selected Surface Format: {:?}", surface_format);

        // Create the surface configuration.
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT, // We want to render to this surface
            format: surface_format,
            width,  // Initial width from argument
            height, // Initial height from argument
            present_mode: surface_caps.present_modes.first().copied().unwrap_or(PresentMode::Fifo), // VSync (Fifo is widely supported)
            alpha_mode: surface_caps.alpha_modes.first().copied().unwrap_or(CompositeAlphaMode::Auto), // Default alpha mode
            view_formats: vec![], // For multiview or alternative views of the surface texture
            desired_maximum_frame_latency: 2, // How many frames can be queued up by the GPU
        };

        surface.configure(&device, &surface_config);
        log::info!("WGPU Surface configured for {}x{}", width, height);


        Self {
            instance,
            adapter: Some(adapter),
            device: Some(device),
            queue: Some(queue),
            surface: Some(surface),
            surface_config: Some(surface_config),
            surface_format: Some(surface_format),
            window_width: width,
            window_height: height,
        }
    }

    /// Reconfigures the surface, usually called when the window is resized.
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 { // Ensure new dimensions are valid
            // Check if all necessary components are initialized
            if let (Some(device), Some(surface), Some(config)) =
                (self.device.as_ref(), self.surface.as_ref(), self.surface_config.as_mut())
            {
                self.window_width = new_width;
                self.window_height = new_height;
                config.width = new_width;
                config.height = new_height;
                surface.configure(device, config); // Apply the new configuration
                log::info!("WGPU Surface reconfigured to {}x{}", new_width, new_height);
            } else {
                log::warn!("Cannot resize: WGPU backend not fully initialized.");
            }
        } else {
            log::warn!("Cannot resize to zero dimensions: {}x{}", new_width, new_height);
        }
    }

    /// Acquires the next frame (SurfaceTexture) from the swap chain.
    /// This texture will be rendered to.
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, SurfaceError> {
        self.surface
            .as_ref()
            .expect("Surface not initialized. Call new() first.")
            .get_current_texture()
    }

    /// Renders a frame.
    /// This is a placeholder and should be filled with actual rendering logic
    /// (e.g., setting up render pipelines, binding data, and issuing draw calls).
    pub fn render(&self) -> Result<(), SurfaceError> {
        let device = self.device();
        let queue = self.queue();

        // Get the next frame to render to
        let output_surface_texture = match self.get_current_texture() {
            Ok(texture) => texture,
            Err(SurfaceError::Lost) => {
                log::warn!("Surface lost. It needs to be reconfigured. The caller should handle this by calling resize().");
                // The surface is lost and needs to be recreated/reconfigured.
                // The caller should typically call `resize` with the current window dimensions
                // to reconfigure the surface and then try rendering again.
                return Err(SurfaceError::Lost);
            }
            Err(SurfaceError::OutOfMemory) => {
                log::error!("Surface out of memory. This is a critical error.");
                // This is a more severe error. The application might need to terminate or
                // attempt to free up resources.
                return Err(SurfaceError::OutOfMemory);
            }
            Err(e) => { // Other errors like Outdated, Timeout
                log::error!("Error acquiring next surface texture: {:?}", e);
                return Err(e);
            }
        };

        // Create a TextureView from the SurfaceTexture. This is what we render to.
        let view_descriptor = wgpu::TextureViewDescriptor::default();
        let view = output_surface_texture.texture.create_view(&view_descriptor);

        // Create a command encoder to record rendering commands
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Start a render pass.
        // This describes where we are drawing to (the 'view') and how (e.g., clear color).
        { // Scope for the render_pass borrow
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Screen Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, // The texture view to draw to
                    resolve_target: None, // For multisampling, not used in this basic example
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { // Clear the screen with a color
                            r: 0.1, // Red component
                            g: 0.2, // Green component
                            b: 0.3, // Blue component
                            a: 1.0, // Alpha component (opaque)
                        }),
                        store: wgpu::StoreOp::Store, // Store the rendered result to the texture
                    },
                })],
                depth_stencil_attachment: None, // No depth/stencil buffer for this simple example
                timestamp_writes: None, // For GPU profiling, not used here
                occlusion_query_set: None, // For occlusion culling, not used here
            });

            // Actual drawing commands would go here:
            // render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.draw(0..num_vertices, 0..1);
        } // The _render_pass is dropped here, which finalizes it.

        // Submit the command buffer to the queue for execution
        queue.submit(std::iter::once(encoder.finish()));

        // Present the frame to the screen
        output_surface_texture.present();

        Ok(())
    }

    // --- Getters for accessing WGPU components ---

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &Adapter {
        self.adapter.as_ref().expect("Adapter not initialized. Call new() first.")
    }

    pub fn device(&self) -> &Device {
        self.device.as_ref().expect("Device not initialized. Call new() first.")
    }

    pub fn queue(&self) -> &Queue {
        self.queue.as_ref().expect("Queue not initialized. Call new() first.")
    }

    pub fn surface(&self) -> &Surface<'window> {
        self.surface.as_ref().expect("Surface not initialized. Call new() first.")
    }

    pub fn surface_config(&self) -> &SurfaceConfiguration {
        self.surface_config.as_ref().expect("Surface configuration not initialized. Call new() first.")
    }

    pub fn surface_format(&self) -> TextureFormat {
        self.surface_format.expect("Surface format not initialized. Call new() first.")
    }

    /// Returns the current logical size of the window/surface.
    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
}

// Example usage with winit (requires winit, env_logger, log, and futures/pollster dependencies)
// Add to Cargo.toml:
// wgpu = "0.20" # Reflects WGPU v25.0.0 API
// raw-window-handle = "0.6"
// winit = "0.29" # Or 0.30, but event loop handling changes in 0.30
// env_logger = "0.11"
// log = "0.4"
// futures-lite = "2.3" # For block_on, or use pollster

/*
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ EventLoop}, // ControlFlow removed, use EventLoop directly for 0.29
    window::WindowBuilder,
};
use std::error::Error;


async fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init(); // Initialize logger
    let event_loop = EventLoop::new()?; // For winit 0.29
    let window = WindowBuilder::new()
        .with_title("WGPU Backend Test (wgpu 0.20)")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)?;

    let initial_size = window.inner_size();
    let mut wgpu_backend = WgpuBackend::new(&window, initial_size.width, initial_size.height).await;

    // For winit 0.29, run consumes the event_loop and closure.
    // For winit 0.30+, you'd use `event_loop.run_app(&mut app_state)` or `event_loop.run_on_demand(...)`.
    event_loop.run(move |event, elwt| { // elwt is EventLoopWindowTarget for 0.29
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(), // Use elwt.exit() for 0.29
                WindowEvent::Resized(physical_size) => {
                    log::info!("Window resized to: {}x{}", physical_size.width, physical_size.height);
                    if physical_size.width > 0 && physical_size.height > 0 {
                        wgpu_backend.resize(physical_size.width, physical_size.height);
                    }
                    window.request_redraw(); // Important to request redraw after resize
                }
                WindowEvent::RedrawRequested => {
                    // Perform rendering
                    match wgpu_backend.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => {
                            log::warn!("Surface lost, attempting to reconfigure by resizing.");
                            let (w,h) = wgpu_backend.window_size();
                            wgpu_backend.resize(w,h); // Reconfigure with current size
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("OutOfMemory error during render, exiting.");
                            elwt.exit(); // Use elwt.exit() for 0.29
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => log::error!("Error during render: {:?}", e),
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Application update code can go here.
                // Request a redraw for continuous rendering (e.g., for animations).
                window.request_redraw();
            }
            _ => {}
        }
    })?;
    Ok(())
}

fn main() {
    // For native, futures_lite::future::block_on or pollster::block_on is common.
    // For wasm, you'd use wasm_bindgen_futures::spawn_local.
    if let Err(e) = futures_lite::future::block_on(run()) {
        eprintln!("Error running application: {}", e);
    }
}
*/
