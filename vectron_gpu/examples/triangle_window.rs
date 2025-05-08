use vectron_embedder::{
    create_embedder, Embedder, EmbedderConfig, WindowConfig, WindowEmbedder
};
use vectron_gpu::{
    BackendConfig, BlendState, BufferDescriptor, BufferUsageFlags, CpuAccessMode, DepthStencilState, GpuBackend, PipelineDescriptor, PipelineType, PrimitiveTopology, RasterizerState, RenderCommand, ShaderDescriptor, ShaderLanguage, ShaderSource, ShaderStage, SurfaceDescriptor, TextureFormat, VertexAttributeDescriptor, VertexFormat, VertexLayoutDescriptor, VertexStepMode
};

fn main() -> Result<(), Box<dyn std::error::Error>> { /* 
    // Create and initialize the embedder
    let mut embedder = create_embedder();
    embedder.init(EmbedderConfig {
        application_name: "Triangle Example".to_string(),
        enable_high_dpi: true,
        vsync: true,
    })?;

    // Create a window
    let window = embedder.create_window(WindowConfig {
        title: "Triangle Example".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        decorated: true,
        visible: true,
        position: None,
        min_size: None,
        max_size: None,
        parent: None,
    })?;

    // Initialize GPU backend
    let mut backend = vectron_gpu::backends::create_backend();
    backend.init(BackendConfig {
        application_name: "Triangle Example".to_string(),
        enable_debug: true,
        preferred_format: None,
        vsync: true,
    })?;

    // Define triangle vertices
    let vertices: Vec<f32> = vec![
        // Position (x, y, z) + Color (r, g, b, a)
         0.0,  0.5, 0.0, 1.0, 0.0, 0.0, 1.0, // Top vertex (red)
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, // Bottom left vertex (green)
         0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, // Bottom right vertex (blue)
    ];

    // Create a vertex buffer
    let vertex_buffer = backend.create_buffer(BufferDescriptor::with_data(
        vertices.len() * std::mem::size_of::<f32>(),
        BufferUsageFlags::VERTEX,
        CpuAccessMode::Write,
        unsafe { std::slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * std::mem::size_of::<f32>()).to_vec() }
    ))?;

    // Define vertex and fragment shaders
    let vertex_shader_src = r#"
        struct VSInput {
            float3 Position : POSITION;
            float4 Color : COLOR;
        };

        struct VSOutput {
            float4 Position : SV_POSITION;
            float4 Color : COLOR;
        };

        VSOutput main(VSInput input) {
            VSOutput output;
            output.Position = float4(input.Position, 1.0);
            output.Color = input.Color;
            return output;
        }
    "#.to_string();

    let fragment_shader_src = r#"
        struct PSInput {
            float4 Position : SV_POSITION;
            float4 Color : COLOR;
        };

        float4 main(PSInput input) : SV_TARGET {
            return input.Color;
        }
    "#.to_string();

    // Print shader sources for debugging
    println!("Vertex Shader:\n{}", vertex_shader_src);
    println!("Fragment Shader:\n{}", fragment_shader_src);

    // Create shaders
    let vertex_shader = backend.create_shader(ShaderDescriptor {
        stage: ShaderStage::Vertex,
        source: ShaderSource::Source(vertex_shader_src, ShaderLanguage::HLSL),
        entry_point: "main".to_string(),
    })?;

    let fragment_shader = backend.create_shader(ShaderDescriptor {
        stage: ShaderStage::Fragment,
        source: ShaderSource::Source(fragment_shader_src, ShaderLanguage::HLSL),
        entry_point: "main".to_string(),
    })?;

    // Define vertex layout
    let vertex_layout = VertexLayoutDescriptor {
        stride: 7 * std::mem::size_of::<f32>(), // 3 position + 4 color
        step_mode: VertexStepMode::Vertex,
        attributes: vec![
            VertexAttributeDescriptor {
                format: VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttributeDescriptor {
                format: VertexFormat::Float4,
                offset: 3 * std::mem::size_of::<f32>(),
                shader_location: 1,
            },
        ],
    };

    // Get a surface from embedder
    let surface = embedder.get_surface(window)?;

    // Create a GPU surface
    let gpu_surface = backend.create_surface(SurfaceDescriptor {
        handle: surface.handle,
        width: surface.width,
        height: surface.height,
    })?;

    // Create a pipeline
    let pipeline = backend.create_pipeline(PipelineDescriptor {
        type_: PipelineType::Graphics,
        vertex_shader: Some(vertex_shader),
        fragment_shader: Some(fragment_shader),
        compute_shader: None,
        vertex_layout: Some(vertex_layout),
        blend_state: BlendState::default(),
        depth_stencil_state: DepthStencilState::default(),
        rasterizer_state: RasterizerState::default(),
        primitive_topology: PrimitiveTopology::TriangleList,
        render_target_formats: vec![TextureFormat::RGBA8Unorm],
        depth_stencil_format: None,
        sample_count: 1,
    })?;

    // Main loop
    while embedder.is_running() {
        // Process window events
        for event in embedder.process_events() {
            if let vectron_embedder::Event::Quit = event {
                return Ok(());
            }
        }
        
        // 1. Begin frame
        backend.begin_frame(gpu_surface)?;
        
        // 2. Submit rendering commands
        backend.submit_commands(&[
            // Clear the screen to dark gray
            RenderCommand::ClearColor {
                attachment_index: 0,
                color: [0.2, 0.2, 0.2, 1.0],
            },
            // Set pipeline for rendering
            RenderCommand::SetPipeline(pipeline),
            // Set viewport to full window
            RenderCommand::SetViewport {
                x: 0.0,
                y: 0.0,
                width: surface.width as f32,
                height: surface.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            },
            // Set vertex buffer
            RenderCommand::SetVertexBuffer {
                slot: 0,
                buffer: vertex_buffer,
                offset: 0,
            },
            // Draw the triangle
            RenderCommand::Draw {
                vertex_count: 3,
                instance_count: 1,
                first_vertex: 0,
                first_instance: 0,
            },
        ])?;
        
        // 3. End frame and present
        backend.end_frame()?;
    }

    Ok(())*/
    Ok(())
} 