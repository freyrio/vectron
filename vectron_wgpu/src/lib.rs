use std::sync::Arc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, CreateSurfaceError, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub struct WgpuBackend<'window> {
    instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'window>>,
    surface_config: Option<SurfaceConfiguration>,
    surface_format: Option<wgpu::TextureFormat>,
}

impl<'window> WgpuBackend<'window> {
    pub fn new() -> Self {
        let backends = if cfg!(windows) {
            Backends::VULKAN
        } else {
            Backends::all()
        };
        
        let instance = Instance::new(&InstanceDescriptor {
            backends,
            ..Default::default()
        });

        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
            surface: None,
            surface_config: None,
            surface_format: None,
        }
    }

    pub fn init_surface<W>(&mut self, window: &'window W, width: u32, height: u32) -> Result<(), String>
    where
        W: HasWindowHandle + HasDisplayHandle + Send + Sync,
    {
        let surface = self.instance.create_surface(window)
            .map_err(|e: CreateSurfaceError| e.to_string())?;

        let adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter: Adapter = pollster::block_on(
            self.instance.request_adapter(&adapter_options)
        )
        .map_err(|e| format!("Failed to request adapter: {}", e))?;
        
        println!("Using adapter: {} (backend: {:?})", adapter.get_info().name, adapter.get_info().backend);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Vectron Device"),
                required_features: wgpu::Features::empty(),
                ..Default::default()
            },
        ))
        .map_err(|e| e.to_string())?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            view_formats: vec![surface_format.add_srgb_suffix()],
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.surface_format = Some(surface_format);

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let (Some(surface_config), Some(device), Some(surface)) = 
            (&mut self.surface_config, &self.device, &self.surface) {
            surface_config.width = width;
            surface_config.height = height;
            surface.configure(device, surface_config);
        }
    }

    pub fn render(&self, render_fn: impl FnOnce(&Device, &Queue, &wgpu::TextureView)) -> Result<(), String> {
        let (device, queue, surface, surface_format) = match (&self.device, &self.queue, &self.surface, &self.surface_format) {
            (Some(device), Some(queue), Some(surface), Some(format)) => (device, queue, surface, format),
            _ => return Err("Rendering resources not initialized".to_string()),
        };

        let surface_texture = surface
            .get_current_texture()
            .map_err(|e| format!("Failed to acquire next frame: {:?}", e))?;
        
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(surface_format.add_srgb_suffix()),
                ..Default::default()
            });
        
        render_fn(device, queue, &view);
        
        surface_texture.present();
        
        Ok(())
    }

    pub fn device(&self) -> Option<&Device> {
        self.device.as_ref()
    }

    pub fn queue(&self) -> Option<&Queue> {
        self.queue.as_ref()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
