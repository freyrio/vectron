use std::time::{Duration, Instant};
use vectron_embedder::{self, Embedder, Event, WindowConfig, WindowEmbedder, WindowId, Window};
use vectron_render::backend::{
    device::BackendDevice,
    commands::{CommandBuffer, CommandEncoder},
    resource::*,
    state::*,
    error::BackendError,
};
use vectron_wgpu::{self, WgpuBackendDevice, TextureView, RenderDevice, RenderQueue, 
                   BufferUsage, RenderContext};

pub struct App {
    embedder: vectron_embedder::PlatformEmbedder,
    backend_device: Option<Box<dyn BackendDevice>>,
    pub current_surface_texture: Option<TextureHandle>,
    main_window: Option<WindowId>,
    running: bool,
    last_update: Instant,
}

pub trait AppDelegate {
    fn setup(&mut self, app: &mut App) -> Result<(), String>;
    fn update(&mut self, app: &mut App, delta_time: f32) -> Result<(), String>;
    fn render(&mut self, app: &mut App) -> Result<(), String>;
    fn handle_event(&mut self, app: &mut App, event: &Event) -> Result<(), String>;
    fn shutdown(&mut self, app: &mut App) -> Result<(), String>;
}

impl App {
    pub fn new() -> Self {
        Self {
            embedder: vectron_embedder::create_embedder(),
            backend_device: None,
            current_surface_texture: None,
            main_window: None,
            running: false,
            last_update: Instant::now(),
        }
    }

    pub fn create_main_window(&mut self, title: &str, width: u32, height: u32) -> Result<WindowId, String> {
        // Initialize embedder if not already done
        if !self.embedder.is_running() {
            self.embedder.init(vectron_embedder::EmbedderConfig {
                application_name: title.to_string(),
                enable_high_dpi: true,
                vsync: true,
            }).map_err(|e| format!("Failed to initialize embedder: {:?}", e))?;
        }
        
        // Create window
        let window_config = WindowConfig::new()
            .with_title(title)
            .with_size(width, height);
            
        let window_id = self.embedder.create_window(&window_config)
            .map_err(|e| format!("Failed to create window: {:?}", e))?;
            
        // Store the window ID
        self.main_window = Some(window_id);
        
        // Get the window reference from embedder
        let window = self.embedder.get_window(&window_id)
            .ok_or_else(|| "Failed to get window from handle".to_string())?;
        
        // Create and initialize wgpu backend
        let (width, height) = window.size();
        
        // Initialize backend with the window
        let backend = vectron_wgpu::create_wgpu_backend_for_window(&window, width, height)
            .map_err(|e| format!("Failed to initialize WGPU backend: {:?}", e))?;
        
        // Store the backend device
        self.backend_device = Some(Box::new(backend));
        
        Ok(window_id)
    }
    
    pub fn run<D: AppDelegate>(&mut self, mut delegate: D) -> Result<(), String> {
        // Setup
        delegate.setup(self)?;
        
        if self.main_window.is_none() {
            return Err("No main window created".to_string());
        }
        
        self.running = true;
        self.last_update = Instant::now();
        
        // Main loop
        while self.running && self.embedder.is_running() {
            // Process events
            let events = self.embedder.process_events();
            for event in &events {
                match event {
                    Event::Resized { width, height } => {
                        // Resize backend surface if we have a device
                        if let Some(device) = &mut self.backend_device {
                            device.resize_surface(*width, *height)
                                .map_err(|e| format!("Failed to resize surface: {:?}", e))?;
                        }
                    },
                    Event::Quit => {
                        self.running = false;
                    },
                    _ => {}
                }
                
                // Let the delegate handle the event
                delegate.handle_event(self, event)?;
            }
            
            // Update
            let now = Instant::now();
            let delta_time = now.duration_since(self.last_update).as_secs_f32();
            self.last_update = now;
            
            delegate.update(self, delta_time)?;
            
            // Begin frame - get new surface texture
            if let Some(device) = &mut self.backend_device {
                let texture = device.get_current_surface_texture()
                    .map_err(|e| format!("Failed to get current surface texture: {:?}", e))?;
                
                self.current_surface_texture = Some(texture);
            }
            
            // Render
            delegate.render(self)?;
            
            // End frame
            self.current_surface_texture = None;
            
            // Request a redraw for the next frame
            if let Some(window_id) = self.main_window {
                self.embedder.request_redraw(window_id);
            }
        }
        
        // Shutdown
        delegate.shutdown(self)?;
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    // Get the backend device
    pub fn backend_device(&mut self) -> Option<&mut dyn BackendDevice> {
        self.backend_device.as_deref_mut()
    }
    
    // For compatibility with existing rendering code
    pub fn wgpu_backend_device(&mut self) -> Option<&mut WgpuBackendDevice> {
        if let Some(device) = &mut self.backend_device {
            device.as_any().downcast_mut::<WgpuBackendDevice>()
        } else {
            None
        }
    }
    
    // Helper for rendering with command encoder
    pub fn render_with_encoder<F>(&mut self, render_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut dyn CommandEncoder),
    {
        // Get backend device
        let device = self.backend_device.as_deref_mut()
            .ok_or_else(|| "Backend device not initialized".to_string())?;
        
        // Begin frame and get command encoder
        let mut encoder = device.begin_frame()
            .map_err(|e| format!("Failed to begin frame: {:?}", e))?;
        
        // Call render function with encoder
        render_fn(encoder.as_mut());
        
        // End frame and submit commands
        device.end_frame(encoder)
            .map_err(|e| format!("Failed to end frame: {:?}", e))?;
        
        Ok(())
    }
    
    // Legacy render method using the old API for compatibility
    pub fn render<F>(&mut self, render_fn: F) -> Result<(), String>
    where
        F: FnOnce(&RenderDevice, &RenderQueue, &TextureView),
    {
        let backend = self.wgpu_backend_device()
            .ok_or_else(|| "WGPU backend not initialized or wrong backend type".to_string())?;

        // Create compatibility layer for passing to the render function
        let render_wrapper = |_: &RenderDevice, _: &RenderQueue, _: &TextureView| {
            // This function intentionally left blank as we're using a different rendering approach
        };
        
        // This will be updated as needed
        Ok(())
    }
    
    // Helper for creating common buffer types
    pub fn create_vertex_buffer(&mut self, data: &[u8]) -> Result<BufferHandle, String> {
        let device = self.backend_device.as_deref_mut()
            .ok_or_else(|| "Backend device not initialized".to_string())?;
        
        let desc = BufferDesc {
            label: Some("Vertex Buffer".to_string()),
            size: data.len() as u64,
            usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
            mapped_at_creation: false,
        };
        
        let buffer = device.create_buffer(&desc)
            .map_err(|e| format!("Failed to create buffer: {:?}", e))?;
        
        device.update_buffer(buffer, data, 0)
            .map_err(|e| format!("Failed to update buffer: {:?}", e))?;
        
        Ok(buffer)
    }
    
    pub fn update_buffer(&mut self, buffer: BufferHandle, data: &[u8], offset: u64) -> Result<(), String> {
        let device = self.backend_device.as_deref_mut()
            .ok_or_else(|| "Backend device not initialized".to_string())?;
        
        device.update_buffer(buffer, data, offset)
            .map_err(|e| format!("Failed to update buffer: {:?}", e))
    }
    
    // Access embedder
    pub fn embedder(&self) -> &vectron_embedder::PlatformEmbedder {
        &self.embedder
    }
    
    pub fn embedder_mut(&mut self) -> &mut vectron_embedder::PlatformEmbedder {
        &mut self.embedder
    }
    
    pub fn get_window(&self, window_id: WindowId) -> Option<Window> {
        self.embedder.get_window(&window_id)
    }
    
    pub fn main_window(&self) -> Option<Window> {
        self.main_window.and_then(|id| self.embedder.get_window(&id))
    }
    
    // Get surface format information
    pub fn surface_format(&self) -> Option<TextureFormat> {
        if let Some(device) = &self.backend_device {
            device.get_surface_format()
        } else {
            None
        }
    }
}

// Add a simple example implementation that can be used directly
pub struct SimpleApp {
    clear_color: [f32; 4],
}

impl SimpleApp {
    pub fn new() -> Self {
        Self {
            clear_color: [0.1, 0.2, 0.3, 1.0],
        }
    }
}

impl Default for SimpleApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AppDelegate for SimpleApp {
    fn setup(&mut self, app: &mut App) -> Result<(), String> {
        app.create_main_window("Vectron App", 800, 600)?;
        Ok(())
    }
    
    fn update(&mut self, _app: &mut App, _delta_time: f32) -> Result<(), String> {
        Ok(())
    }
    
    fn render(&mut self, app: &mut App) -> Result<(), String> {
        app.render_with_encoder(|encoder| {
            // Begin a render pass
            let clear_color = [
                self.clear_color[0] as f64,
                self.clear_color[1] as f64,
                self.clear_color[2] as f64,
                self.clear_color[3] as f64,
            ];
            
            // Create a basic render pass - this is a simplified version
            // as we're transitioning to the new API
            let render_pass_desc = RenderPassDesc {
                color_attachments: vec![
                    RenderPassColorAttachment {
                        view: app.current_surface_texture.unwrap(),
                        resolve_target: None,
                        load_op: LoadOp::Clear,
                        store_op: StoreOp::Store,
                        clear_value: Some([clear_color[0], clear_color[1], clear_color[2], clear_color[3]]),
                    }
                ],
                depth_stencil_attachment: None,
                label: Some("Simple Render Pass".to_string()),
            };
            
            // Begin and end the render pass
            encoder.begin_render_pass(&render_pass_desc).unwrap();
            encoder.end_render_pass().unwrap();
        })
    }
    
    fn handle_event(&mut self, app: &mut App, event: &Event) -> Result<(), String> {
        match event {
            Event::Quit => {
                app.stop();
            },
            _ => {}
        }
        Ok(())
    }
    
    fn shutdown(&mut self, _app: &mut App) -> Result<(), String> {
        Ok(())
    }
}
