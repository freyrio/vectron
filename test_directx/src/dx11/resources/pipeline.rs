//! Pipeline module for managing DirectX rendering pipelines.

use crate::dx11::debug::*; // Import debug macros

use windows::Win32::Graphics::Direct3D::{D3D_PRIMITIVE_TOPOLOGY, D3D_FEATURE_LEVEL};
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, ID3D11VertexShader, ID3D11PixelShader,
    ID3D11InputLayout, ID3D11BlendState, ID3D11DepthStencilState, ID3D11RasterizerState,
    ID3D11Buffer, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE_DEFAULT,
    D3D11_BIND_VERTEX_BUFFER, D3D11_BIND_INDEX_BUFFER, D3D11_BIND_CONSTANT_BUFFER,
    D3D11_RASTERIZER_DESC, D3D11_DEPTH_STENCIL_DESC, D3D11_BLEND_DESC, D3D11_COMPARISON_LESS,
    D3D11_DEPTH_WRITE_MASK_ALL, D3D11_CULL_BACK, D3D11_FILL_SOLID,
    D3D11_INPUT_ELEMENT_DESC, D3D11_DEPTH_WRITE_MASK_ZERO,
    D3D11_BLEND_ONE, D3D11_BLEND_ZERO, D3D11_BLEND_OP_ADD
};
use windows::core::Result;
use std::ffi::CString;

/// Pipeline creation descriptor
pub struct PipelineDesc {
    /// Vertex shader bytecode
    pub vertex_shader: Option<Vec<u8>>,
    /// Pixel shader bytecode
    pub pixel_shader: Option<Vec<u8>>,
    /// Vertex input layout elements
    pub input_layout: Option<Vec<InputLayoutElement>>,
    /// Whether to enable depth testing
    pub depth_test: bool,
    /// Whether to enable depth writing
    pub depth_write: bool,
    /// Whether to enable blending
    pub blending: bool,
    /// Cull mode
    pub cull_mode: CullMode,
}

impl Default for PipelineDesc {
    fn default() -> Self {
        Self {
            vertex_shader: None,
            pixel_shader: None,
            input_layout: None,
            depth_test: true,
            depth_write: true,
            blending: false,
            cull_mode: CullMode::Back,
        }
    }
}

/// Input layout element description
pub struct InputLayoutElement {
    /// Semantic name
    pub semantic_name: String,
    /// Semantic index
    pub semantic_index: u32,
    /// Format
    pub format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT,
    /// Input slot
    pub input_slot: u32,
    /// Aligned byte offset
    pub aligned_byte_offset: u32,
    /// Input slot class
    pub input_slot_class: windows::Win32::Graphics::Direct3D11::D3D11_INPUT_CLASSIFICATION,
    /// Instance data step rate
    pub instance_data_step_rate: u32,
}

/// Cull mode for the rasterizer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    /// Cull back faces
    Back,
    /// Cull front faces
    Front,
    /// Cull both front and back faces
    None,
}

/// DirectX pipeline resource
pub struct Pipeline {
    /// Vertex shader
    pub vertex_shader: Option<ID3D11VertexShader>,
    /// Pixel shader
    pub pixel_shader: Option<ID3D11PixelShader>,
    /// Input layout
    pub input_layout: Option<ID3D11InputLayout>,
    /// Blend state
    pub blend_state: Option<ID3D11BlendState>,
    /// Depth stencil state
    pub depth_stencil_state: Option<ID3D11DepthStencilState>,
    /// Rasterizer state
    pub rasterizer_state: Option<ID3D11RasterizerState>,
}

impl Pipeline {
    /// Creates a new pipeline
    pub fn new(device: &ID3D11Device, desc: &PipelineDesc) -> Result<Self> {
        dx11_debug!("Creating new Pipeline...");
        // Create vertex shader if provided
        let vertex_shader = if let Some(shader_code) = &desc.vertex_shader {
            dx11_debug!("  Creating Vertex Shader...");
            let mut shader = None;
            unsafe {
                device.CreateVertexShader(shader_code, None, Some(&mut shader)).map_err(|e|{
                    dx11_error!("CreateVertexShader failed: {:?}", e);
                    e
                })?;
            }
            shader
        } else {
            dx11_debug!("  No Vertex Shader provided.");
            None
        };

        // Create pixel shader if provided
        let pixel_shader = if let Some(shader_code) = &desc.pixel_shader {
            dx11_debug!("  Creating Pixel Shader...");
            let mut shader = None;
            unsafe {
                device.CreatePixelShader(shader_code, None, Some(&mut shader)).map_err(|e|{
                     dx11_error!("CreatePixelShader failed: {:?}", e);
                    e
                })?;
            }
            shader
        } else {
            dx11_debug!("  No Pixel Shader provided.");
            None
        };

        // Create input layout if provided
        let input_layout = if let (Some(layout_elements), Some(vs_code)) = (&desc.input_layout, &desc.vertex_shader) {
            dx11_debug!("  Creating Input Layout ({} elements)...", layout_elements.len());
            let mut layout = None;
            let c_semantic_names: Vec<CString> = layout_elements.iter()
                .map(|el| CString::new(el.semantic_name.as_str()).expect("CString::new failed"))
                .collect();
            let mut elements = Vec::with_capacity(layout_elements.len());
            for (i, element) in layout_elements.iter().enumerate() {
                let element_desc = D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::PCSTR(c_semantic_names[i].as_ptr() as *const u8),
                    SemanticIndex: element.semantic_index,
                    Format: element.format,
                    InputSlot: element.input_slot,
                    AlignedByteOffset: element.aligned_byte_offset,
                    InputSlotClass: element.input_slot_class,
                    InstanceDataStepRate: element.instance_data_step_rate,
                };
                elements.push(element_desc);
            }
            unsafe {
                device.CreateInputLayout(&elements, vs_code, Some(&mut layout)).map_err(|e|{
                    dx11_error!("CreateInputLayout failed: {:?}", e);
                    e
                })?;
            }
            layout
        } else {
            dx11_debug!("  No Input Layout provided or Vertex Shader missing.");
            None
        };

        // Create blend state if blending is enabled
        let blend_state = if desc.blending {
            dx11_debug!("  Creating Blend State (Blending Enabled)...");
            let mut blend_desc = D3D11_BLEND_DESC::default();
            // TODO: Make blend state configurable via PipelineDesc
            blend_desc.RenderTarget[0].BlendEnable = true.into();
            blend_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_ONE; // Example: Additive (can be changed)
            blend_desc.RenderTarget[0].DestBlend = D3D11_BLEND_ONE;
            blend_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
            blend_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
            blend_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
            blend_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
            blend_desc.RenderTarget[0].RenderTargetWriteMask = 0xF;
            let mut state = None;
            unsafe {
                device.CreateBlendState(&blend_desc, Some(&mut state)).map_err(|e|{
                    dx11_error!("CreateBlendState failed: {:?}", e);
                    e
                })?;
            }
            state
        } else {
            dx11_debug!("  Skipping Blend State creation (Blending Disabled).");
            None
        };

        // Create depth stencil state
        dx11_debug!("  Creating Depth Stencil State (Depth Test: {}, Depth Write: {})...", desc.depth_test, desc.depth_write);
        let mut depth_desc = D3D11_DEPTH_STENCIL_DESC::default();
        depth_desc.DepthEnable = desc.depth_test.into();
        depth_desc.DepthWriteMask = if desc.depth_write { D3D11_DEPTH_WRITE_MASK_ALL } else { D3D11_DEPTH_WRITE_MASK_ZERO };
        depth_desc.DepthFunc = D3D11_COMPARISON_LESS;
        depth_desc.StencilEnable = false.into();
        depth_desc.StencilReadMask = 0xFF;
        depth_desc.StencilWriteMask = 0xFF;
        depth_desc.FrontFace.StencilFailOp = windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP;
        depth_desc.FrontFace.StencilDepthFailOp = windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP;
        depth_desc.FrontFace.StencilPassOp = windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP;
        depth_desc.FrontFace.StencilFunc = windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_ALWAYS;
        depth_desc.BackFace = depth_desc.FrontFace;
        
        let mut depth_stencil_state = None;
        unsafe {
            device.CreateDepthStencilState(&depth_desc, Some(&mut depth_stencil_state)).map_err(|e|{
                dx11_error!("CreateDepthStencilState failed: {:?}", e);
                e
            })?;
        }

        // Create rasterizer state
        dx11_debug!("  Creating Rasterizer State (Cull Mode: {:?})...", desc.cull_mode);
        let mut raster_desc = D3D11_RASTERIZER_DESC::default();
        raster_desc.FillMode = D3D11_FILL_SOLID;
        match desc.cull_mode {
            CullMode::Back => raster_desc.CullMode = D3D11_CULL_BACK,
            CullMode::Front => raster_desc.CullMode = windows::Win32::Graphics::Direct3D11::D3D11_CULL_FRONT,
            CullMode::None => raster_desc.CullMode = windows::Win32::Graphics::Direct3D11::D3D11_CULL_NONE,
        }
        
        let mut rasterizer_state = None;
        unsafe {
            device.CreateRasterizerState(&raster_desc, Some(&mut rasterizer_state)).map_err(|e|{
                dx11_error!("CreateRasterizerState failed: {:?}", e);
                e
            })?;
        }

        dx11_debug!("Pipeline created successfully.");
        Ok(Self {
            vertex_shader,
            pixel_shader,
            input_layout,
            blend_state,
            depth_stencil_state,
            rasterizer_state,
        })
    }

    /// Binds the pipeline to the device context
    pub fn bind(&self, context: &ID3D11DeviceContext) -> Result<()> {
        // This might be too noisy for regular use, called every time pipeline is set.
        // dx11_debug!("Binding pipeline states..."); 
        unsafe {
            if let Some(vs) = &self.vertex_shader {
                context.VSSetShader(Some(vs), None);
            } else {
                dx11_warn!("Binding pipeline with no Vertex Shader!");
            }
            
            if let Some(layout) = &self.input_layout {
                context.IASetInputLayout(Some(layout));
            } else {
                 // Allowed if VS doesn't take input, but maybe warn?
                 // dx11_warn!("Binding pipeline with no Input Layout!");
            }
            
            if let Some(ps) = &self.pixel_shader {
                context.PSSetShader(Some(ps), None);
            } else {
                 dx11_warn!("Binding pipeline with no Pixel Shader!");
            }
            
            if let Some(rasterizer) = &self.rasterizer_state {
                context.RSSetState(Some(rasterizer));
            } else {
                 dx11_error!("Binding pipeline with no Rasterizer State!"); // Should always exist
            }
            
            if let Some(depth_stencil) = &self.depth_stencil_state {
                context.OMSetDepthStencilState(Some(depth_stencil), 0); // StencilRef = 0
            } else {
                 dx11_error!("Binding pipeline with no DepthStencil State!"); // Should always exist
            }
            
            if let Some(blend) = &self.blend_state {
                let blend_factor = [1.0, 1.0, 1.0, 1.0]; // Default factor
                context.OMSetBlendState(Some(blend), Some(&blend_factor), 0xFFFFFFFF); // SampleMask = all
            } else {
                // If blending disabled, reset to default state (null)
                context.OMSetBlendState(None, None, 0xFFFFFFFF);
            }
            
            context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY(4)); // D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST
        }
        Ok(())
    }
} 