use test_directx::{AsBytes, BufferId, Dx11Backend, RenderCommand, SurfaceDescriptor};
use test_directx::resources::{BufferDesc, BufferUsage, PipelineDesc, InputLayoutElement, CullMode, ShaderType};
use test_directx::types::IndexFormat;
use vectron_embedder::{create_embedder, WindowConfig, EmbedderConfig, Embedder, WindowEmbedder};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D11::D3D11_INPUT_PER_VERTEX_DATA;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32_FLOAT;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D::Fxc::{D3DCompile, D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION};
use std::ffi::c_void;
use std::f32::consts::PI;
use test_directx::dx11::debug::*;

// Declare modules we will create
mod math;
mod camera;
mod renderable;
mod material;
mod math_2d;
mod vertex_2d;
mod drawable_2d;
mod shapes;

// Use items from the new modules
use math::{TransformMatrix, Vec3};
use camera::Camera;
use renderable::{Object3D, Renderable};
use material::Material;
use math_2d::Transform2D;
use vertex_2d::Vertex2D;
use drawable_2d::Drawable2D;
use shapes::{Rectangle, Tessellatable, Text};
use drawable_2d::DrawableShape2D;

// --- Constant Buffer Structs ---

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct TransformData {
    model: TransformMatrix,
    view: TransformMatrix,
    projection: TransformMatrix,
    // Optional: Add inverse transpose model matrix for correct normal transformation if needed
    // model_inverse_transpose: TransformMatrix,
}

impl AsBytes for TransformData {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            model: TransformMatrix::identity(),
            view: TransformMatrix::identity(),
            projection: TransformMatrix::identity(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LightProperties {
    light_direction: [f32; 3],
    _padding1: f32, // Pad to align color
    light_color: [f32; 4],
    ambient_color: [f32; 4],
}

impl AsBytes for LightProperties {
     fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl Default for LightProperties {
    fn default() -> Self {
        // Define a direction slightly from the top-left-front
        let dir = [-0.5, -0.7, -0.4]; 
        // Normalize the direction vector
        let len_sq = dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2];
        let norm_dir = if len_sq > f32::EPSILON {
            let inv_len = 1.0 / len_sq.sqrt();
            [dir[0] * inv_len, dir[1] * inv_len, dir[2] * inv_len]
        } else {
            [0.0, -1.0, 0.0] // Default downwards if zero
        };

        Self {
            light_direction: norm_dir,
            _padding1: 0.0,
            light_color: [1.0, 1.0, 1.0, 1.0], // White directional light
            ambient_color: [0.25, 0.25, 0.25, 1.0], // Slightly brighter ambient
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Transform2DData { 
    // We need a 4x4 matrix for HLSL constant buffer alignment and multiplication
    // Even though our 2D transform is logically 3x3
    transform: TransformMatrix, // Use the 3D matrix type, store ortho * model
}

impl AsBytes for Transform2DData {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl Default for Transform2DData {
    fn default() -> Self {
        Self { transform: TransformMatrix::identity() }
    }
}

// Vertex structure for the cube
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3], // Added normal vector
}

impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }
}

// Cube mesh structure
struct CubeMesh {
    // Remove buffer IDs, CubeMesh only defines data now
    // vertices: Vec<Vertex>,
    // indices: Vec<u32>,
    // vertex_buffer: u32,
    // index_buffer: u32,
}

impl CubeMesh {
    // Return buffer IDs and index count instead of Self
    fn new(app: &mut Dx11Backend) -> Result<(BufferId, BufferId, u32), windows::core::Error> {
        // Define cube vertices with positions and normals
        let vertices = vec![
            // Front face (+Z)
            Vertex::new([-0.5, -0.5,  0.5], [0.0, 0.0, 1.0]),
            Vertex::new([ 0.5, -0.5,  0.5], [0.0, 0.0, 1.0]),
            Vertex::new([ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            
            // Back face (-Z)
            Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
            Vertex::new([ 0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
            Vertex::new([ 0.5,  0.5, -0.5], [0.0, 0.0, -1.0]),
            Vertex::new([-0.5,  0.5, -0.5], [0.0, 0.0, -1.0]),

            // Top face (+Y)
            Vertex::new([-0.5,  0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([ 0.5,  0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([ 0.5,  0.5,  0.5], [0.0, 1.0, 0.0]),
            Vertex::new([-0.5,  0.5,  0.5], [0.0, 1.0, 0.0]),
            
            // Bottom face (-Y)
            Vertex::new([-0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
            Vertex::new([ 0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
            Vertex::new([ 0.5, -0.5,  0.5], [0.0, -1.0, 0.0]),
            Vertex::new([-0.5, -0.5,  0.5], [0.0, -1.0, 0.0]),

            // Right face (+X)
            Vertex::new([ 0.5, -0.5, -0.5], [1.0, 0.0, 0.0]),
            Vertex::new([ 0.5,  0.5, -0.5], [1.0, 0.0, 0.0]),
            Vertex::new([ 0.5,  0.5,  0.5], [1.0, 0.0, 0.0]),
            Vertex::new([ 0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),

            // Left face (-X)
            Vertex::new([-0.5, -0.5, -0.5], [-1.0, 0.0, 0.0]),
            Vertex::new([-0.5,  0.5, -0.5], [-1.0, 0.0, 0.0]),
            Vertex::new([-0.5,  0.5,  0.5], [-1.0, 0.0, 0.0]),
            Vertex::new([-0.5, -0.5,  0.5], [-1.0, 0.0, 0.0]),
        ];
        
        // Indices remain the same (referencing the 24 vertices defined above)
        let indices = vec![
             // Front face
             0, 1, 2, 0, 2, 3,
             // Back face
             4, 5, 6, 4, 6, 7,
             // Top face
             8, 9, 10, 8, 10, 11,
             // Bottom face
             12, 13, 14, 12, 14, 15,
             // Right face
             16, 17, 18, 16, 18, 19,
             // Left face
             20, 21, 22, 20, 22, 23,
        ];
        
        // Create vertex buffer (update size and stride)
        let vertex_buffer_desc = BufferDesc {
            size: std::mem::size_of::<Vertex>() * vertices.len(), // Updated size
            stride: std::mem::size_of::<Vertex>(), // Updated stride
            usage: BufferUsage::Vertex,
            dynamic: false,
        };
        
        let vertex_buffer = app.create_buffer(&vertex_buffer_desc, Some(vertices.as_bytes()))?;
        
        // Create index buffer
        let index_buffer_desc = BufferDesc {
            size: std::mem::size_of::<u32>() * indices.len(),
            stride: 0,
            usage: BufferUsage::Index,
            dynamic: false,
        };
        
        let index_buffer = app.create_buffer(&index_buffer_desc, Some(indices.as_bytes()))?;
        
        // Return the created buffer IDs and index count
        Ok((vertex_buffer, index_buffer, indices.len() as u32))
    }
}

// Simple vertex shader for the cube
const VERTEX_SHADER: &[u8] = include_bytes!("shaders/cube_vertex.hlsl");

// Simple pixel shader for the cube
const PIXEL_SHADER: &[u8] = include_bytes!("shaders/cube_pixel.hlsl");

fn compile_shader(source: &[u8], entry_point: &str, shader_type: ShaderType) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target = match shader_type {
        ShaderType::Vertex => "vs_5_0",
        ShaderType::Pixel => "ps_5_0",
        _ => return Err("Unsupported shader type".into()),
    };

    let mut shader_blob = None;
    let mut error_blob = None;

    // Convert strings to null-terminated C strings
    let entry_point = std::ffi::CString::new(entry_point)?;
    let target = std::ffi::CString::new(target)?;

    unsafe {
        let result = D3DCompile(
            source.as_ptr() as *const c_void,
            source.len(),
            None,
            None,
            None,
            windows::core::PCSTR(entry_point.as_ptr() as *const u8),
            windows::core::PCSTR(target.as_ptr() as *const u8),
            D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION,
            0,
            &mut shader_blob,
            Some(&mut error_blob),
        );

        if let Err(e) = result {
            if let Some(error) = error_blob {
                let error_message = std::str::from_utf8(std::slice::from_raw_parts(
                    error.GetBufferPointer() as *const u8,
                    error.GetBufferSize(),
                ))?;
                return Err(format!("Shader compilation failed: {}\n{}", e, error_message).into());
            }
            return Err(e.into());
        }
    }

    let shader_blob = shader_blob.unwrap();
    let bytecode = unsafe {
        std::slice::from_raw_parts(
            shader_blob.GetBufferPointer() as *const u8,
            shader_blob.GetBufferSize(),
        ).to_vec()
    };

    Ok(bytecode)
}

// --- Helper Functions --- 

// Function to create an orthographic projection matrix for 2D rendering
// Maps coordinates from [0, width] and [0, height] to [-1, 1] clip space
// (DirectX uses a left-handed coordinate system, Y points down in screen space usually)
fn create_orthographic_matrix(width: f32, height: f32, near_plane: f32, far_plane: f32) -> TransformMatrix {
    let mut m = TransformMatrix::identity();
    
    m.matrix[0][0] = 2.0 / width;
    m.matrix[1][1] = -2.0 / height; // Y is negated to flip from screen space (Y down) to clip space (Y up)
    m.matrix[2][2] = 1.0 / (far_plane - near_plane);
    
    m.matrix[3][0] = -1.0; // Translate X
    m.matrix[3][1] = 1.0;  // Translate Y (after negation)
    m.matrix[3][2] = -near_plane / (far_plane - near_plane); // Translate Z
    
    m // Return the 4x4 matrix
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace initial println! with info log or remove
    // println!("Rust DirectX 11 Example - 3D Cube"); 
    
    // Create and initialize the embedder
    let mut embedder = create_embedder();
    if let Err(e) = embedder.init(EmbedderConfig {
        application_name: "DirectX 11 Cube Example".to_string(),
        enable_high_dpi: true,
        vsync: true,
    }) {
        eprintln!("Failed to initialize embedder: {}", e);
        return Ok(());
    }

    // Create a window
    let window_config = WindowConfig {
        title: "DirectX 11 Cube Example".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        decorated: true,
        visible: true,
        position: None,
        min_size: None,
        max_size: None,
        parent: None,
    };

    let window_handle = match embedder.create_window(window_config) {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return Ok(());
        }
    };

    // Get the window surface for DirectX
    let surface = match embedder.get_surface(window_handle) {
        Ok(surface) => surface,
        Err(e) => {
            eprintln!("Failed to get window surface: {}", e);
            return Ok(());
        }
    };

    let debug_enabled = false; // Or read from config/args

    // Initialize DirectX with the window handle and debug flag
    let hwnd = HWND(surface.handle as isize);
    let app_result = Dx11Backend::init(
        SurfaceDescriptor {
            handle: hwnd.0 as *mut c_void,
            width: 800,
            height: 600,
        },
        debug_enabled // Pass the flag
    );
    if let Err(e) = &app_result {
        eprintln!("\nFailed to initialize DirectXApp:");
        eprintln!("{}", e);
        return Ok(());
    }
    let mut app = app_result.unwrap();

    // Replace init success/feature level logs with dx11_debug!
    // println!("\nDirectXApp initialized successfully.");
    // println!("-> Feature Level: {:?}", app.feature_level());
    dx11_debug!("DirectXApp initialized successfully. Feature Level: {:?}", app.feature_level());

    // Create the cube mesh data and buffers
    let (vertex_buffer_id, index_buffer_id, index_count) = match CubeMesh::new(&mut app) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to create cube mesh buffers: {}", e);
            return Ok(());
        }
    };

    // Replace cube vertices debug logs with dx11_debug!
    // println!("\n--- Debug: Cube Vertices ---");
    // println!("Vertex buffer ID: {}", vertex_buffer_id);
    // println!("Index buffer ID: {}", index_buffer_id);
    // println!("Indices count: {}", index_count);
    // println!("Cube should render as a colored 3D cube with 8 vertices and 36 indices");
    dx11_debug!("Cube Mesh Resources: VB={}, IB={}, Indices={}", vertex_buffer_id, index_buffer_id, index_count);

    // Create transform buffer using TransformData struct
    let transform_data = TransformData::default();
    let transform_buffer_desc = BufferDesc {
        size: std::mem::size_of::<TransformData>(), // Use new struct size
        usage: BufferUsage::Constant,
        stride: 0,
        dynamic: true,
    };
    let transform_buffer = match app.create_buffer(&transform_buffer_desc, Some(transform_data.as_bytes())) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Failed to create transform buffer: {}", e);
            return Ok(());
        }
    };

    // Create light properties buffer
    let light_props = LightProperties::default();
    let light_buffer_desc = BufferDesc {
        size: std::mem::size_of::<LightProperties>(),
        usage: BufferUsage::Constant,
        stride: 0,
        dynamic: false, // Assuming light doesn't change for now
    };
    let light_buffer = match app.create_buffer(&light_buffer_desc, Some(light_props.as_bytes())) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Failed to create light buffer: {}", e);
            return Ok(());
        }
    };

    // Create a gradient material for the cube
    // let cube_material = Material::new_solid([0.8, 0.2, 0.2, 1.0]); // Reddish solid color
    let cube_material = Material::new_linear_gradient(
        [1.0, 0.0, 0.0, 1.0], // Red at start
        [0.0, 0.0, 1.0, 1.0], // Blue at end
        [0.0, 1.0, 0.0],      // Gradient along Y-axis (bottom to top)
    );

    // Create material constant buffer
    let material_buffer_desc = BufferDesc {
        size: std::mem::size_of::<Material>(),
        usage: BufferUsage::Constant,
        stride: 0,
        dynamic: true, // Or false if material doesn't change often
    };

    let material_buffer = match app.create_buffer(&material_buffer_desc, Some(cube_material.as_bytes())) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Failed to create material buffer: {}", e);
            return Ok(());
        }
    };

    // Create an Object3D instance for the cube, now with material
    let mut cube_object = Object3D::new(
        vertex_buffer_id,
        index_buffer_id,
        index_count,
        IndexFormat::Uint32, // Explicitly set format
        cube_material, // Pass the material
    );

    // Compile shaders
    let vertex_shader = compile_shader(VERTEX_SHADER, "main", ShaderType::Vertex)?;
    let pixel_shader = compile_shader(PIXEL_SHADER, "main", ShaderType::Pixel)?;

    // Create pipeline
    let pipeline_desc = PipelineDesc {
        vertex_shader: Some(vertex_shader),
        pixel_shader: Some(pixel_shader),
        input_layout: Some(vec![
            InputLayoutElement {
                semantic_name: "POSITION".to_string(),
                semantic_index: 0,
                format: DXGI_FORMAT_R32G32B32_FLOAT,
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: D3D11_INPUT_PER_VERTEX_DATA,
                instance_data_step_rate: 0,
            },
            InputLayoutElement {
                semantic_name: "NORMAL".to_string(),
                semantic_index: 0,
                format: DXGI_FORMAT_R32G32B32_FLOAT,
                input_slot: 0,
                aligned_byte_offset: 12,
                input_slot_class: D3D11_INPUT_PER_VERTEX_DATA,
                instance_data_step_rate: 0,
            },
            /* Removed COLOR input element
            InputLayoutElement {
                semantic_name: "COLOR".to_string(),
                semantic_index: 0,
                format: DXGI_FORMAT_R32G32B32_FLOAT,
                input_slot: 0,
                aligned_byte_offset: 12, // Offset is now irrelevant
                input_slot_class: D3D11_INPUT_PER_VERTEX_DATA,
                instance_data_step_rate: 0,
            },
            */
        ]),
        depth_test: true,
        depth_write: true,
        blending: false,
        cull_mode: CullMode::None, // Changed from Back to None for debugging
    };

    let pipeline = match app.create_pipeline(&pipeline_desc) {
        Ok(pipeline) => pipeline,
        Err(e) => {
            eprintln!("Failed to create pipeline: {}", e);
            return Ok(());
        }
    };

    // --- Setup 2D Rendering --- 

    // Create Rectangle shape and drawable
    let rect_shape = Rectangle::new(10.0, 10.0, 150.0, 80.0, [0.2, 0.8, 0.3, 0.9]);
    let mut quad_drawable = match DrawableShape2D::new(&rect_shape, &mut app) {
        Ok(drawable) => drawable,
        Err(e) => {
            eprintln!("Failed to create DrawableShape2D for Rectangle: {}", e);
            return Ok(());
        }
    };

    // Create Text shape and drawable
    let text_shape = Text::new("Hello, DirectX!".to_string(), 200.0, 20.0, 16.0, [1.0, 1.0, 0.0, 1.0]); // Yellow text
    let mut text_drawable = match DrawableShape2D::new(&text_shape, &mut app) {
        Ok(drawable) => drawable,
        Err(e) => {
            eprintln!("Failed to create DrawableShape2D for Text: {}", e);
            return Ok(());
        }
    };

    // Store drawables in a list
    let mut drawables_2d: Vec<Box<dyn Drawable2D>> = vec![
        Box::new(quad_drawable), // Add the rectangle
        Box::new(text_drawable),  // Add the text
    ];

    // Compile 2D Shaders
    const QUAD_VERTEX_SHADER: &[u8] = include_bytes!("shaders/quad_vertex.hlsl");
    const QUAD_PIXEL_SHADER: &[u8] = include_bytes!("shaders/quad_pixel.hlsl");

    let quad_vs_bytecode = compile_shader(QUAD_VERTEX_SHADER, "main", ShaderType::Vertex)?;
    let quad_ps_bytecode = compile_shader(QUAD_PIXEL_SHADER, "main", ShaderType::Pixel)?;

    // Create 2D Pipeline State
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
    use windows::Win32::Graphics::Direct3D11::{D3D11_BLEND_SRC_ALPHA, D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_OP_ADD};

    let pipeline_desc_2d = PipelineDesc {
        vertex_shader: Some(quad_vs_bytecode),
        pixel_shader: Some(quad_ps_bytecode),
        input_layout: Some(vec![
            InputLayoutElement {
                semantic_name: "POSITION".to_string(),
                semantic_index: 0,
                format: DXGI_FORMAT_R32G32_FLOAT, // float2 position
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: D3D11_INPUT_PER_VERTEX_DATA,
                instance_data_step_rate: 0,
            },
            InputLayoutElement {
                semantic_name: "COLOR".to_string(),
                semantic_index: 0,
                format: DXGI_FORMAT_R32G32B32A32_FLOAT, // float4 color
                input_slot: 0,
                aligned_byte_offset: 8, // Offset after float2 position (2 * 4 bytes)
                input_slot_class: D3D11_INPUT_PER_VERTEX_DATA,
                instance_data_step_rate: 0,
            },
        ]),
        depth_test: false, // Disable depth test for 2D overlay
        depth_write: false, // Disable depth write
        blending: true, // Enable alpha blending
        // Customize blend state if needed (default might be okay, but explicit is better)
        // Example: Standard alpha blending
        // blend_desc: Some(D3D11_BLEND_DESC { ... SrcAlpha, InvSrcAlpha ... })
        cull_mode: CullMode::None, // No culling for 2D quad typically
    };

    let pipeline_2d = match app.create_pipeline(&pipeline_desc_2d) {
        Ok(pipeline) => pipeline,
        Err(e) => {
            eprintln!("Failed to create 2D pipeline: {}", e);
            return Ok(());
        }
    };

    // Create 2D Transform Constant Buffer
    let transform_2d_data = Transform2DData::default(); // Start with identity
    let transform_buffer_2d_desc = BufferDesc {
        size: std::mem::size_of::<Transform2DData>(),
        usage: BufferUsage::Constant,
        stride: 0,
        dynamic: true, // We will update this per frame or on resize
    };
    let transform_buffer_2d = match app.create_buffer(&transform_buffer_2d_desc, Some(transform_2d_data.as_bytes())) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Failed to create 2D transform buffer: {}", e);
            return Ok(());
        }
    };

    // Main event loop
    // Replace render loop start message
    // println!("\nStarting render loop...");
    dx11_debug!("Starting render loop...");
    let mut frame_count = 0;
    let mut should_exit = false;
    
    // Create Camera instance with adjusted position
    let mut camera = Camera::new(
        [0.0, 0.0, -3.0], // Eye position - closer to origin
        [0.0, 0.0, 0.0],  // Target position - looking at origin
        [0.0, 1.0, 0.0],  // Up direction
        PI / 3.0,         // FOV - wider for better visibility
        800.0 / 600.0,    // Aspect Ratio
        0.1,              // Near plane
        100.0,            // Far plane
    );

    // Replace window size logs
    let mut window_width = 800; // Default
    let mut window_height = 600; // Default
    // Get actual window size if possible
    let (width, height) = embedder.get_window_size(window_handle);
    // Check if the returned size is valid (not the default 0,0)
    if width > 0 && height > 0 {
        window_width = width;
        window_height = height;
        // println!("Using window dimensions: {}x{}", window_width, window_height);
        dx11_debug!("Using window dimensions: {}x{}", window_width, window_height);
    } else {
        // println!("get_window_size returned (0,0), using defaults: {}x{}", window_width, window_height);
         dx11_warn!("get_window_size returned (0,0), using defaults: {}x{}", window_width, window_height);
    }

    while embedder.is_running() && !should_exit {
        // Process window events
        let events = embedder.process_events();
        for event in events {
            match event {
                vectron_embedder::Event::Quit => {
                    // println!("Received quit event"); // Keep this or use info level
                    should_exit = true;
                    break;
                }
                vectron_embedder::Event::Resized { width, height } => {
                    if let Err(e) = app.resize(width, height) {
                        eprintln!("Resize failed: {}", e);
                        should_exit = true;
                        break;
                    }
                    // Update camera aspect ratio on resize
                    camera.update_aspect_ratio(width as f32 / height as f32);
                }
                _ => {}
            }
        }

        // Get matrices from camera
        let projection_matrix = camera.get_projection_matrix();
        let view_matrix = camera.get_view_matrix();

        // Calculate model matrix (rotation)
        let angle = (frame_count as f32) * 0.02;
        // Update the cube object's model matrix directly
        cube_object.model_matrix = TransformMatrix::rotation_y(angle);
        
        // Get the model matrix from the Renderable trait
        let model_matrix = cube_object.get_model_matrix();

        // Calculate MVP with original order
        let model_view = model_matrix.multiply(&view_matrix);
        let mvp = model_view.multiply(&projection_matrix);

        // Update transform buffer with the separate matrices
        let current_transform_data = TransformData {
            model: model_matrix, // From cube_object.get_model_matrix()
            view: view_matrix, // From camera
            projection: projection_matrix, // From camera
        };
        if let Err(e) = app.update_buffer(transform_buffer, current_transform_data.as_bytes()) {
            eprintln!("Failed to update transform buffer: {}", e);
            should_exit = true;
            break;
        }

        // --- Update 2D Transforms ---
        let ortho_matrix = create_orthographic_matrix(window_width as f32, window_height as f32, 0.1, 100.0);
        // We will update the transform buffer PER drawable inside the loop now
        // So, remove the single update here
        /*
        let quad_model_transform_2d = quad_drawable.get_transform(); 
        let quad_model_matrix = TransformMatrix::identity(); // Placeholder
        let transform_2d_final = quad_model_matrix.multiply(&ortho_matrix);
        let current_transform_2d_data = Transform2DData { transform: transform_2d_final };
        if let Err(e) = app.update_buffer(transform_buffer_2d, current_transform_2d_data.as_bytes()) { ... }
        */

        // Begin frame
        app.begin_frame();

        // --- Clear Buffers --- 
        app.add_command(RenderCommand::ClearColor {
            attachment_index: 0,
            color: [0.2, 0.2, 0.3, 1.0], // Use the original clear color
        });
        app.add_command(RenderCommand::ClearDepthStencil {
            depth: Some(1.0),
            stencil: Some(0),
        });
        
        // --- Draw 3D Cube ---
        app.add_command(RenderCommand::SetPipeline(pipeline)); // Set 3D pipeline

        // Set 3D constant buffers
        app.add_command(RenderCommand::SetConstantBuffer {
            slot: 0, // b0 for TransformData (VS)
            buffer: transform_buffer,
        });
        app.add_command(RenderCommand::SetConstantBuffer {
            slot: 1, // b1 for Material (PS)
            buffer: material_buffer,
        });
        app.add_command(RenderCommand::SetConstantBuffer {
            slot: 2, // b2 for LightProperties (PS)
            buffer: light_buffer,
        });

        // Set 3D geometry buffers
        app.add_command(RenderCommand::SetVertexBuffer {
            slot: 0,
            buffer: cube_object.get_vertex_buffer(),
            offset: 0,
        });
        app.add_command(RenderCommand::SetIndexBuffer {
            buffer: cube_object.get_index_buffer(),
            offset: 0,
            index_format: cube_object.get_index_format(),
        });
        
        // Draw the 3D cube
        app.add_command(RenderCommand::DrawIndexed {
            index_count: cube_object.get_index_count(),
            instance_count: 1,
            first_index: 0,
            base_vertex: 0,
            first_instance: 0,
        });
        
        // --- Draw 2D Elements --- 
        // Set 2D pipeline once (assuming all use the same simple pipeline for now)
        app.add_command(RenderCommand::SetPipeline(pipeline_2d));

        // Iterate through 2D drawables
        for drawable in drawables_2d.iter() { // Iterate immutably for drawing
            // Calculate transform for this specific drawable
            let model_transform_2d = drawable.get_transform(); 
            // Convert 2D transform to 4x4 matrix
            let model_matrix = model_transform_2d.to_matrix4x4(); 
            // Combine with orthographic projection
            let final_transform_matrix = model_matrix.multiply(&ortho_matrix);
            let transform_data = Transform2DData { transform: final_transform_matrix };

            // Update the 2D transform buffer for this drawable
            // NOTE: This assumes the backend allows buffer updates *after* begin_frame
            //       and that updating the buffer frequently is acceptable.
            //       A more robust system might use multiple buffers or dynamic updates.
            if let Err(e) = app.update_buffer(transform_buffer_2d, transform_data.as_bytes()) {
                 eprintln!("Warning: Failed to update 2D transform buffer: {}", e);
                 continue; // Skip drawing this item if update fails
            }

            // Bind transform buffer (already contains data for this drawable)
            app.add_command(RenderCommand::SetConstantBuffer {
                slot: 0, // b0 for 2D transform
                buffer: transform_buffer_2d,
            });

            // Bind vertex buffer
            app.add_command(RenderCommand::SetVertexBuffer {
                slot: 0,
                buffer: drawable.get_vertex_buffer(),
                offset: 0,
            });
            
            // Bind index buffer
            app.add_command(RenderCommand::SetIndexBuffer {
                buffer: drawable.get_index_buffer(),
                offset: 0,
                index_format: drawable.get_index_format(),
            });
            
            // Draw the item
            app.add_command(RenderCommand::DrawIndexed {
                index_count: drawable.get_index_count(),
                instance_count: 1,
                first_index: 0,
                base_vertex: 0,
                first_instance: 0,
            });
        }

        // End the frame and submit all commands
        if let Err(e) = app.end_frame() {
            eprintln!("Render failed: {}", e);
            should_exit = true;
            break;
        }

        frame_count += 1;
    }

    // Replace exit message
    // println!("\nExiting application.");
    dx11_debug!("Exiting application.");
    // Clean up
    embedder.destroy_window(window_handle);
    embedder.shutdown();
    Ok(())
}

