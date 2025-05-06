use vectron_app::{App, AppDelegate};
use vectron_embedder::Event;

struct BasicWindowApp {
    clear_color: [f32; 3],
    direction: [f32; 3],
}

impl BasicWindowApp {
    fn new() -> Self {
        Self {
            clear_color: [0.1, 0.2, 0.3],
            direction: [0.01, 0.007, 0.005],
        }
    }
}

impl AppDelegate for BasicWindowApp {
    fn setup(&mut self, app: &mut App) -> Result<(), String> {
        // Create a window
        app.create_main_window("Vectron Basic Window Example", 800, 600)?;
        println!("Window created successfully");
        Ok(())
    }

    fn update(&mut self, _app: &mut App, delta_time: f32) -> Result<(), String> {
        // Animate the clear color
        for i in 0..3 {
            self.clear_color[i] += self.direction[i] * delta_time;
            if self.clear_color[i] > 1.0 || self.clear_color[i] < 0.0 {
                self.direction[i] *= -1.0;
                self.clear_color[i] = self.clear_color[i].max(0.0).min(1.0);
            }
        }
        Ok(())
    }

    fn render(&mut self, app: &mut App) -> Result<(), String> {
        // Render a frame with our animated clear color
        app.render(|device, queue, view| {
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
                                r: self.clear_color[0] as f64,
                                g: self.clear_color[1] as f64,
                                b: self.clear_color[2] as f64,
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

    fn handle_event(&mut self, app: &mut App, event: &Event) -> Result<(), String> {
        /*match event {
            Event::Input(vectron_embedder::InputEvent::Key { key, pressed }) => {
                if *pressed && *key == vectron_embedder::Key::Escape {
                    app.stop();
                }
            },
            _ => {}
        }
        Ok(())*/
        Ok(())
    }

    fn shutdown(&mut self, _app: &mut App) -> Result<(), String> {
        println!("Shutting down...");
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut app = App::new();
    let delegate = BasicWindowApp::new();
    app.run(delegate)
} 