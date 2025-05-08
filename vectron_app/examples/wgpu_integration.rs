use std::f32::consts::PI;
use vectron_app::{App, AppDelegate};
use vectron_embedder::{Event, InputEvent, Key, MouseButton};
use vectron_render::backend::{
    resource::*,
    state::*,
    commands::*,
    device::*,
    error::BackendError,
};
use vectron_wgpu::{
    RenderDevice, RenderQueue, TextureView, Buffer, RenderPipeline, ShaderModule,
    TextureFormat, BufferUsage, VertexBufferLayout, VertexStepMode, VertexAttribute,
    VertexFormat, VertexAttributeBuilder, create_shader_module, create_buffer_with_data,
    write_buffer, RenderContext, WgpuBackendDevice
};

struct TriangleApp {
    rotation: f32,
    rotation_speed: f32,
    vertices: Vec<Vertex>,
    vertex_buffer: Option<BufferHandle>,
    pipeline: Option<PipelineHandle>,
    shader: Option<ShaderHandle>,
    mouse_position: (f32, f32),
}

// Define a simple vertex structure
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> VertexBufferLayout {
        let attributes = vec![
            VertexAttributeBuilder::new(VertexFormat::Float32x2, 0).build(),
            VertexAttributeBuilder::new(VertexFormat::Float32x3, 1)
                .offset(std::mem::size_of::<[f32; 2]>() as u64)
                .build(),
        ];
        
        VertexBufferLayout::new(
            std::mem::size_of::<Vertex>() as u64,
            VertexStepMode::Vertex,
            attributes
        )
    }
}

impl TriangleApp {
    fn new() -> Self {
        Self {
            rotation: 0.0,
            rotation_speed: 1.0,
            vertices: vec![
                Vertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },
            ],
            vertex_buffer: None,
            pipeline: None,
            shader: None,
            mouse_position: (0.0, 0.0),
        }
    }

    fn create_pipeline(&mut self, device: &mut dyn BackendDevice, format: TextureFormat) -> Result<(), BackendError> {
        // Load the shader
        let shader_code = include_str!("../shaders/triangle.wgsl");
        let shader_desc = ShaderDesc {
            code: ShaderCode::Wgsl(shader_code.to_string()),
            entry_point: "main".to_string(),
            label: Some("Triangle Shader".to_string()),
        };
        
        let shader = device.create_shader(&shader_desc)?;
        self.shader = Some(shader);
        
        // Create vertex layout
        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                },
            ],
        };
        
        // Create the render pipeline descriptor
        let pipeline_desc = PipelineDesc {
            vertex_shader: shader,
            fragment_shader: Some(shader),
            vertex_layouts: vec![vertex_layout],
            color_formats: vec![Some(format)],
            depth_stencil: None,
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
            },
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("Triangle Pipeline".to_string()),
        };
        
        let pipeline = device.create_pipeline(&pipeline_desc)?;
        self.pipeline = Some(pipeline);
        
        Ok(())
    }

    fn update_vertex_data(&self, rotation: f32) -> Vec<Vertex> {
        // Rotate the triangle vertices
        let cos_rot = rotation.cos();
        let sin_rot = rotation.sin();
        
        vec![
            Vertex {
                position: [
                    self.vertices[0].position[0] * cos_rot - self.vertices[0].position[1] * sin_rot,
                    self.vertices[0].position[0] * sin_rot + self.vertices[0].position[1] * cos_rot,
                ],
                color: self.vertices[0].color,
            },
            Vertex {
                position: [
                    self.vertices[1].position[0] * cos_rot - self.vertices[1].position[1] * sin_rot,
                    self.vertices[1].position[0] * sin_rot + self.vertices[1].position[1] * cos_rot,
                ],
                color: self.vertices[1].color,
            },
            Vertex {
                position: [
                    self.vertices[2].position[0] * cos_rot - self.vertices[2].position[1] * sin_rot,
                    self.vertices[2].position[0] * sin_rot + self.vertices[2].position[1] * cos_rot,
                ],
                color: self.vertices[2].color,
            },
        ]
    }
}

impl AppDelegate for TriangleApp {
    fn setup(&mut self, app: &mut App) -> Result<(), String> {
        // Create a window
        app.create_main_window("Vectron WGPU Integration Example", 800, 600)?;
        println!("Window created successfully");

        // Access the device to create resources
        if let Some(device) = app.backend_device() {
            // Get the format from the surface
            let format = app.surface_format()
                .unwrap_or(TextureFormat::Bgra8UnormSrgb);
            
            // Create the pipeline
            self.create_pipeline(device, format)
                .map_err(|e| format!("Failed to create pipeline: {:?}", e))?;
            
            // Create the vertex buffer
            let vertices = self.update_vertex_data(self.rotation);
            let vertex_data = bytemuck::cast_slice(&vertices);
            
            // Create a buffer description
            let buffer_desc = BufferDesc {
                label: Some("Vertex Buffer".to_string()),
                size: vertex_data.len() as u64,
                usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
                mapped_at_creation: false,
            };
            
            // Create the buffer
            let buffer = device.create_buffer(&buffer_desc)
                .map_err(|e| format!("Failed to create buffer: {:?}", e))?;
            
            // Update the buffer with our vertex data
            device.update_buffer(buffer, vertex_data, 0)
                .map_err(|e| format!("Failed to update buffer: {:?}", e))?;
            
            self.vertex_buffer = Some(buffer);
        }

        Ok(())
    }

    fn update(&mut self, app: &mut App, delta_time: f32) -> Result<(), String> {
        // Update rotation
        self.rotation += self.rotation_speed * delta_time;
        if self.rotation > 2.0 * PI {
            self.rotation -= 2.0 * PI;
        }

        // Calculate vertices with the updated rotation
        let rotation = self.rotation;
        let rotated_vertices = self.update_vertex_data(rotation);
        let vertex_data = bytemuck::cast_slice(&rotated_vertices);
        
        // Update the vertex buffer
        if let (Some(device), Some(buffer)) = (app.backend_device(), self.vertex_buffer) {
            device.update_buffer(buffer, vertex_data, 0)
                .map_err(|e| format!("Failed to update buffer: {:?}", e))?;
        }

        Ok(())
    }

    fn render(&mut self, app: &mut App) -> Result<(), String> {
        app.render_with_encoder(|encoder| {
            if let (Some(pipeline), Some(vertex_buffer)) = (self.pipeline, self.vertex_buffer) {
                if let Some(surface_texture) = app.current_surface_texture {
                    // Begin a render pass with a clear color
                    let render_pass_desc = RenderPassDesc {
                        color_attachments: vec![
                            RenderPassColorAttachment {
                                view: surface_texture,
                                resolve_target: None,
                                load_op: LoadOp::Clear,
                                store_op: StoreOp::Store,
                                clear_value: [0.1, 0.2, 0.3, 1.0],
                            }
                        ],
                        depth_stencil_attachment: None,
                        label: Some("Triangle Render Pass".to_string()),
                    };
                    
                    // Begin render pass
                    if let Err(e) = encoder.begin_render_pass(&render_pass_desc) {
                        eprintln!("Failed to begin render pass: {:?}", e);
                        return;
                    }
                    
                    // Set pipeline and vertex buffer
                    if let Err(e) = encoder.set_pipeline(pipeline) {
                        eprintln!("Failed to set pipeline: {:?}", e);
                        return;
                    }
                    
                    if let Err(e) = encoder.set_vertex_buffer(0, vertex_buffer, 0) {
                        eprintln!("Failed to set vertex buffer: {:?}", e);
                        return;
                    }
                    
                    // Draw the triangle
                    if let Err(e) = encoder.draw(3, 1, 0, 0) {
                        eprintln!("Failed to draw: {:?}", e);
                        return;
                    }
                    
                    // End render pass
                    if let Err(e) = encoder.end_render_pass() {
                        eprintln!("Failed to end render pass: {:?}", e);
                        return;
                    }
                }
            }
        })
    }

    fn handle_event(&mut self, app: &mut App, event: &Event) -> Result<(), String> {
        match event {
            Event::Input(InputEvent::Key { key, pressed }) => {
                if *pressed {
                    match key {
                        Key::Escape => {
                            println!("Escape key pressed, exiting...");
                            app.stop();
                        },
                        Key::Up => self.rotation_speed += 0.5,
                        Key::Down => self.rotation_speed = (self.rotation_speed - 0.5).max(0.0),
                        _ => {}
                    }
                }
            },
            Event::Input(InputEvent::MouseMove { x, y }) => {
                self.mouse_position = (*x as f32, *y as f32);
            },
            Event::Input(InputEvent::MouseButton { button: MouseButton::Left, pressed, .. }) => {
                if *pressed {
                    // On click, reverse rotation direction
                    self.rotation_speed = -self.rotation_speed;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn shutdown(&mut self, _app: &mut App) -> Result<(), String> {
        println!("Shutting down Triangle App");
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut app = App::new();
    let triangle_app = TriangleApp::new();
    app.run(triangle_app)
} 