//! Main application backend logic, orchestrating graphics components.

use crate::dx11::debug::*; // Import debug macros

use super::graphics::{DeviceContext, Surface}; // Use graphics module items
use super::types::{RenderCommand, SurfaceDescriptor, BufferId, PipelineId};
use super::resources::{ResourceManager, Buffer, Pipeline, BufferDesc, PipelineDesc};
use windows::{core::Result, Win32::Foundation::HWND};
use windows::Win32::Graphics::Direct3D11::{D3D11_CLEAR_DEPTH, D3D11_CLEAR_STENCIL};
use std::collections::HashMap;

/// Main application struct holding graphics state.
pub struct Dx11Backend {
    _device_context: DeviceContext, // Keep ownership of the device context
    surface: Surface, // Keep ownership of the surface
    resource_manager: ResourceManager,
    pending_commands: Vec<RenderCommand>,
    // Debug flag added
    #[allow(dead_code)] // Temporarily allow unused if not immediately used elsewhere
    debug_enabled: bool, 
}

impl Dx11Backend {
    /// Creates a new application instance, initializing the graphics backend.
    pub fn init(descriptor: SurfaceDescriptor, enable_debug: bool) -> Result<Self> {
        // Set the debug state first
        crate::dx11::debug::set_dx11_debug_enabled(enable_debug);
        
        dx11_debug!("Initializing Dx11Backend... Debug Enabled: {}", enable_debug);
        
        // Create the core device and context first
        let device_context = DeviceContext::new(enable_debug)?; // Pass flag to DeviceContext potentially
        dx11_debug!("DeviceContext created successfully. Feature Level: {:?}", device_context.feature_level());

        // Create the surface (swap chain, RTV) linked to the window and device
        let hwnd = HWND(descriptor.handle as isize);
        dx11_debug!("Creating Surface for HWND: {:?} Size: {}x{}", hwnd, descriptor.width, descriptor.height);
        let surface = Surface::new(hwnd, &device_context, descriptor.width, descriptor.height )?;
        dx11_debug!("Surface created successfully.");

        Ok(Self {
            _device_context: device_context, // Store the device context
            surface,                         // Store the surface
            resource_manager: ResourceManager::new(),
            pending_commands: Vec::new(),
            debug_enabled: enable_debug, // Store the flag
        })
    }

    /// Begins a new frame, clearing any pending commands
    pub fn begin_frame(&mut self) {
        // dx11_debug!("Beginning frame..."); // Can be noisy
        self.pending_commands.clear();
    }

    /// Submits all pending commands to the renderer
    pub fn submit_commands(&mut self) -> Result<()> {
        dx11_debug!("Submitting {} commands...", self.pending_commands.len());
        // Bind the main render target and depth stencil view first
        // This also sets the viewport now
        self.surface.bind_render_targets_and_viewport(self._device_context.context());

        // Process each command in order
        for (i, command) in self.pending_commands.iter().enumerate() { // Use iter() to avoid consuming
            // Optional: Log the command being processed
            // dx11_debug!("  Processing cmd[{}]: {:?}", i, command);
            
            // Execute the command
            match command {
                RenderCommand::ClearColor { attachment_index, color } => {
                    if *attachment_index == 0 {
                        // Use the new surface method to clear
                        self.surface.clear_render_target(self._device_context.context(), color);
                    } else {
                        dx11_warn!("ClearColor for attachment index {} not supported.", attachment_index);
                    }
                },
                RenderCommand::SetViewport { x, y, width, height, min_depth, max_depth } => {
                    // Viewport is now primarily set by surface.bind_render_targets_and_viewport
                    // This command might still be useful for sub-viewports, but needs careful handling
                    dx11_debug!("Manual SetViewport command encountered (may override surface viewport).");
                    let viewport = windows::Win32::Graphics::Direct3D11::D3D11_VIEWPORT {
                        TopLeftX: *x,
                        TopLeftY: *y,
                        Width: *width, 
                        Height: *height,
                        MinDepth: *min_depth,
                        MaxDepth: *max_depth,
                    };
                    unsafe {
                        self._device_context.context().RSSetViewports(Some(&[viewport]));
                    }
                },
                RenderCommand::SetScissor { x, y, width, height } => {
                    // TODO: Implement scissor setting
                    dx11_warn!("SetScissor command not yet implemented.");
                },
                RenderCommand::SetPipeline(pipeline_id) => {
                    if let Some(pipeline) = self.resource_manager.get_pipeline(*pipeline_id) {
                        dx11_debug!("Binding pipeline ID: {}", pipeline_id);
                        pipeline.bind(self._device_context.context())?;
                        // Remove internal prints from pipeline.bind() later
                    } else {
                        dx11_error!("Pipeline ID {} not found!", pipeline_id);
                        // Return an error or handle appropriately
                        return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG)); 
                    }
                },
                RenderCommand::SetVertexBuffer { slot, buffer, offset } => {
                    if let Some(buffer_res) = self.resource_manager.get_buffer(*buffer) {
                         dx11_debug!("Binding VB ID: {} to slot {}", buffer, slot);
                        unsafe {
                            let strides = [buffer_res.stride as u32];
                            let offsets = [*offset as u32];
                            self._device_context.context().IASetVertexBuffers(
                                *slot,
                                1,
                                Some(&buffer_res.buffer as *const _ as *const Option<windows::Win32::Graphics::Direct3D11::ID3D11Buffer>),
                                Some(strides.as_ptr()),
                                Some(offsets.as_ptr()),
                            );
                        }
                    } else {
                         dx11_error!("Vertex Buffer ID {} not found!", buffer);
                         return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG));
                    }
                },
                RenderCommand::SetConstantBuffer { slot, buffer } => {
                    if let Some(buffer_res) = self.resource_manager.get_buffer(*buffer) {
                        dx11_debug!("Binding CB ID: {} to slot {}", buffer, slot);
                        unsafe {
                            let d3d_buffer = Some(buffer_res.buffer.clone());
                            // Set for vertex shader
                            self._device_context.context().VSSetConstantBuffers(*slot, Some(&[d3d_buffer.clone()]));
                            // Set for pixel shader
                            self._device_context.context().PSSetConstantBuffers(*slot, Some(&[d3d_buffer]));
                        }
                    } else {
                        dx11_error!("Constant Buffer ID {} not found!", buffer);
                        return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG));
                    }
                },
                RenderCommand::SetIndexBuffer { buffer, offset, index_format } => {
                    if let Some(buffer_res) = self.resource_manager.get_buffer(*buffer) {
                        dx11_debug!("Binding IB ID: {}", buffer);
                        let format = match index_format {
                            super::types::IndexFormat::Uint16 => windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_UINT,
                            super::types::IndexFormat::Uint32 => windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_UINT,
                        };
                        unsafe {
                            self._device_context.context().IASetIndexBuffer(
                                Some(&buffer_res.buffer),
                                format,
                                *offset as u32,
                            );
                        }
                    } else {
                        dx11_error!("Index Buffer ID {} not found!", buffer);
                        return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG));
                    }
                },
                RenderCommand::Draw { vertex_count, instance_count, first_vertex, first_instance } => {
                    dx11_debug!("Draw: vertices={}, instances={}", vertex_count, instance_count);
                    unsafe {
                        self._device_context.context().DrawInstanced(
                            *vertex_count,
                            *instance_count,
                            *first_vertex,
                            *first_instance,
                        );
                    }
                },
                RenderCommand::DrawIndexed { index_count, instance_count, first_index, base_vertex, first_instance } => {
                    dx11_debug!("DrawIndexed: indices={}, instances={}", index_count, instance_count);
                    unsafe {
                        self._device_context.context().DrawIndexedInstanced(
                            *index_count,
                            *instance_count,
                            *first_index,
                            *base_vertex,
                            *first_instance,
                        );
                    }
                },
                RenderCommand::ClearDepthStencil { depth, stencil } => {
                    let mut flags = 0;
                    if depth.is_some() {
                        flags |= D3D11_CLEAR_DEPTH.0;
                    }
                    if stencil.is_some() {
                        flags |= D3D11_CLEAR_STENCIL.0;
                    }
                    if let Some(dsv) = self.surface.depth_stencil_view() {
                         dx11_debug!("Clearing Depth/Stencil: depth={:?}, stencil={:?}", depth, stencil);
                        unsafe {
                            self._device_context.context().ClearDepthStencilView(
                                dsv,
                                flags as u32,
                                depth.unwrap_or(1.0),
                                stencil.unwrap_or(0).try_into().unwrap_or(0), // Handle potential TryIntoError
                            );
                        }
                    } else {
                        dx11_warn!("ClearDepthStencil called but no DepthStencilView available.");
                    }
                },
            }
        }
        
        // Present the final image
        dx11_debug!("Presenting frame...");
        let present_result = self.surface.present();
        if let Err(e) = &present_result {
            dx11_error!("Present failed: {:?}", e);
        }
        present_result
    }

    /// Ends the current frame and presents it
    pub fn end_frame(&mut self) -> Result<()> {
        // dx11_debug!("Ending frame..."); // Can be noisy
        self.submit_commands()
    }

    /// Handles resizing of the application window. Delegates to the Surface.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        dx11_debug!("Resizing surface to {}x{}", width, height);
        self.surface.resize(width, height)
    }

    /// Returns the D3D feature level supported by the device.
    pub fn feature_level(&self) -> windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL {
        self._device_context.feature_level()
    }

    /// Creates a new buffer with the given data
    pub fn create_buffer(&mut self, desc: &BufferDesc, data: Option<&[u8]>) -> Result<BufferId> {
        let id = self.resource_manager.allocate_buffer_id();
        dx11_debug!("Creating Buffer ID: {} Size: {} Dynamic: {}", id, desc.size, desc.dynamic);
        let buffer = Buffer::new(self._device_context.device(), desc, data)?;
        self.resource_manager.store_buffer(id, buffer);
        Ok(id)
    }

    /// Updates an existing buffer with new data
    pub fn update_buffer(&mut self, id: BufferId, data: &[u8]) -> Result<()> {
        dx11_debug!("Updating Buffer ID: {} Size: {}", id, data.len());
        if let Some(buffer) = self.resource_manager.get_buffer_mut(id) {
            buffer.update(self._device_context.context(), data)
        } else {
            dx11_error!("Update failed: Buffer ID {} not found!", id);
            Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG))
        }
    }

    /// Creates a new pipeline
    pub fn create_pipeline(&mut self, desc: &PipelineDesc) -> Result<PipelineId> {
        let id = self.resource_manager.allocate_pipeline_id();
        dx11_debug!("Creating Pipeline ID: {}", id);
        let pipeline = Pipeline::new(self._device_context.device(), desc)?;
        self.resource_manager.store_pipeline(id, pipeline);
        Ok(id)
    }

    /// Adds a command to the pending command list
    pub fn add_command(&mut self, command: RenderCommand) {
        // dx11_debug!("Adding command: {:?}", command); // Can be very noisy
        self.pending_commands.push(command);
    }
}