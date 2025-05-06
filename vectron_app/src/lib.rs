use std::time::{Duration, Instant};
use vectron_embedder::{self, Embedder, Event, WindowConfig, WindowEmbedder, WindowId, Window};
use vectron_wgpu::WgpuBackend;

pub struct App {
    embedder: vectron_embedder::PlatformEmbedder,
    wgpu_backend: Option<WgpuBackend<'static>>,
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
            wgpu_backend: None,
            main_window: None,
            running: false,
            last_update: Instant::now(),
        }
    }

    pub fn create_main_window(&mut self, title: &str, width: u32, height: u32) -> Result<WindowId, String> {
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
        
        // Use a locally scoped lifetime for WgpuBackend
        let (width, height) = window.size();
        let mut backend = WgpuBackend::new();
        backend.init_surface(&window, width, height)
            .map_err(|e| format!("Failed to initialize WGPU: {}", e))?;
        
        // Now convert it with a static lifetime - this is safe as long as the window
        // outlives the application, which is guaranteed by the embedder
        self.wgpu_backend = Some(unsafe {
            std::mem::transmute::<WgpuBackend<'_>, WgpuBackend<'static>>(backend)
        });
        
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
        while self.running {
            // Process events
            let events = self.embedder.process_events();
            for event in &events {
                match event {
                    Event::Resized { width, height } => {
                        // Resize WGPU surface if we have a window
                        if let Some(backend) = &mut self.wgpu_backend {
                            backend.resize(*width, *height);
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
            
            // Render
            delegate.render(self)?;
        }
        
        // Shutdown
        delegate.shutdown(self)?;
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    pub fn render<F>(&self, render_fn: F) -> Result<(), String>
    where
        F: FnOnce(&wgpu::Device, &wgpu::Queue, &wgpu::TextureView),
    {
        if let Some(backend) = &self.wgpu_backend {
            backend.render(render_fn)
        } else {
            Err("WGPU backend not initialized".to_string())
        }
    }
    
    pub fn wgpu_device(&self) -> Option<&wgpu::Device> {
        self.wgpu_backend.as_ref().and_then(|backend| backend.device())
    }
    
    pub fn wgpu_queue(&self) -> Option<&wgpu::Queue> {
        self.wgpu_backend.as_ref().and_then(|backend| backend.queue())
    }
    
    pub fn embedder(&self) -> &vectron_embedder::PlatformEmbedder {
        &self.embedder
    }
    
    pub fn embedder_mut(&mut self) -> &mut vectron_embedder::PlatformEmbedder {
        &mut self.embedder
    }
}

// Add a simple example implementation that can be used directly
pub struct SimpleApp;

impl AppDelegate for SimpleApp {
    fn setup(&mut self, app: &mut App) -> Result<(), String> {
        app.create_main_window("Vectron App", 800, 600)?;
        Ok(())
    }
    
    fn update(&mut self, _app: &mut App, _delta_time: f32) -> Result<(), String> {
        Ok(())
    }
    
    fn render(&mut self, app: &mut App) -> Result<(), String> {
        app.render(|device, queue, view| {
            // Clear the view with a blue color
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            }
            
            queue.submit(std::iter::once(encoder.finish()));
        })
    }
    
    fn handle_event(&mut self, _app: &mut App, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    
    fn shutdown(&mut self, _app: &mut App) -> Result<(), String> {
        Ok(())
    }
}
