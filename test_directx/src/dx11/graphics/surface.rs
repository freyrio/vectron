//! Manages the DXGI Swap Chain, Render Target View, and presentation to a window surface (HWND).

use crate::dx11::debug::*; // Import debug macros
use super::device::DeviceContext; // Import the DeviceContext struct
use super::Viewport; // Import the Viewport struct
use windows::{
    core::*, // Interface, Result, Error
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D11::{ID3D11DeviceContext, ID3D11RenderTargetView, ID3D11Texture2D, D3D11_VIEWPORT, ID3D11DepthStencilView, D3D11_TEXTURE2D_DESC, D3D11_BIND_DEPTH_STENCIL, D3D11_USAGE_DEFAULT, D3D11_DEPTH_STENCIL_VIEW_DESC, D3D11_DSV_DIMENSION_TEXTURE2D, D3D11_CLEAR_DEPTH, D3D11_CLEAR_STENCIL},
            Dxgi::Common::*, // DXGI_FORMAT, DXGI_SAMPLE_DESC, DXGI_ALPHA_MODE etc.
            Dxgi::*, // IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1, DXGI_USAGE_RENDER_TARGET_OUTPUT etc.
        },
    },
};

/// Manages the swap chain and render target for a specific window surface.
pub struct Surface {
    hwnd: HWND,
    device_context_ref: DeviceContext, // Keep a reference/clone to access device/context/factory
    swap_chain: IDXGISwapChain1,
    render_target_view: Option<ID3D11RenderTargetView>, // Option because it's recreated on resize
    depth_stencil_view: Option<ID3D11DepthStencilView>, // Option because it's recreated on resize
    viewport: Viewport, // Add the Viewport struct
}

impl Surface {
    /// Creates a new Surface, including the Swap Chain and initial Render Target View.
    pub fn new(
        hwnd: HWND,
        device_context: &DeviceContext, // Borrow the DeviceContext
        width: u32,
        height: u32,
    ) -> Result<Self> {
        dx11_debug!("Creating new Surface ({}x{}) for HWND {:?}", width, height, hwnd);
        let factory = device_context.factory();
        let device = device_context.device();

        // Describe the swap chain.
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: false.into(),
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2, // Double buffering
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: DXGI_ALPHA_MODE_IGNORE,
            Flags: 0,
        };
        dx11_debug!("Swap chain desc: Format={:?}, Buffers={}, SwapEffect={:?}", 
                     swap_chain_desc.Format, swap_chain_desc.BufferCount, swap_chain_desc.SwapEffect);

        // Create the swap chain for the HWND.
        let swap_chain: IDXGISwapChain1 = unsafe {
            factory.CreateSwapChainForHwnd(
                device, // Use the D3D11 device
                hwnd,
                &swap_chain_desc,
                None, // Optional: Fullscreen description
                None, // Optional: Output restrictions
            )?
        };
        dx11_debug!("Swap chain created.");

        // Prevent DXGI from monitoring Alt+Enter.
        unsafe {
            if let Err(e) = factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER) {
                 dx11_warn!("Failed to MakeWindowAssociation: {:?}", e);
            }
        }

        let mut surface = Self {
            hwnd,
            device_context_ref: device_context.clone(), // Clone the DeviceContext to store it
            swap_chain,
            render_target_view: None, // Will be created in resize
            depth_stencil_view: None, // Will be created in resize
            viewport: Viewport::default(), // Initialize with default
        };

        // Perform initial resize to create RTV, DSV and set viewport
        dx11_debug!("Performing initial resize...");
        surface.resize(width, height)?;

        dx11_debug!("Surface created successfully.");
        Ok(surface)
    }

    /// Resizes the swap chain buffers and recreates the Render Target View.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        dx11_debug!("Surface resize called: {}x{}", width, height);
        // Release the old RTV and DSV
        self.render_target_view = None;
        self.depth_stencil_view = None;

        if width == 0 || height == 0 {
            dx11_warn!("Surface resize to zero dimensions, skipping buffer resize.");
            self.viewport = Viewport::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
            return Ok(());
        }

        // Resize swap chain buffers
        dx11_debug!("Resizing swap chain buffers...");
        unsafe {
            self.swap_chain.ResizeBuffers(
                0, // Preserve buffer count
                width,
                height,
                DXGI_FORMAT_UNKNOWN, // Preserve format
                0, // Flags
            ).map_err(|e| { 
                dx11_error!("ResizeBuffers failed: {:?}", e);
                e
            })?;
        }
        dx11_debug!("Swap chain buffers resized.");

        // Get the new back buffer
        let back_buffer: ID3D11Texture2D = unsafe { self.swap_chain.GetBuffer(0)? };

        // Create the new Render Target View
        dx11_debug!("Creating RenderTargetView...");
        let mut rtv_option: Option<ID3D11RenderTargetView> = None;
        let device = self.device_context_ref.device(); // Get device from stored context
        unsafe {
            device.CreateRenderTargetView(
                &back_buffer,
                None, // Default desc
                Some(&mut rtv_option)
            ).map_err(|e| {
                dx11_error!("CreateRenderTargetView failed: {:?}", e);
                e
            })?;
        }
        self.render_target_view = rtv_option;
        if self.render_target_view.is_none() { 
            dx11_error!("RenderTargetView is None after creation!");
            // Return an error maybe?
        }
        dx11_debug!("RenderTargetView created.");

        // Create depth stencil texture
        dx11_debug!("Creating DepthStencil texture...");
        let depth_desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };

        let mut depth_texture: Option<ID3D11Texture2D> = None;
        unsafe {
            device.CreateTexture2D(
                &depth_desc,
                None,
                Some(&mut depth_texture),
            ).map_err(|e| {
                dx11_error!("CreateTexture2D (DepthStencil) failed: {:?}", e);
                e
            })?;
        }
        dx11_debug!("DepthStencil texture created.");
        
        // Create depth stencil view
        dx11_debug!("Creating DepthStencilView...");
        let dsv_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
            Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
            ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2D,
            Flags: 0,
            Anonymous: Default::default(),
        };

        let mut dsv_option: Option<ID3D11DepthStencilView> = None;
        if let Some(depth_texture) = depth_texture {
            unsafe {
                device.CreateDepthStencilView(
                    &depth_texture,
                    Some(&dsv_desc),
                    Some(&mut dsv_option),
                ).map_err(|e| {
                    dx11_error!("CreateDepthStencilView failed: {:?}", e);
                    e
                })?;
            }
        } else {
             dx11_error!("Depth texture is None, cannot create DSV!");
             // Return error?
        }
        self.depth_stencil_view = dsv_option;
         if self.depth_stencil_view.is_none() { 
            dx11_warn!("DepthStencilView is None after creation!");
        }
        dx11_debug!("DepthStencilView created.");

        // Update and set the viewport using the Viewport struct
        self.viewport = Viewport::new(0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
        let d3d_viewport = self.viewport.to_d3d11();
        let context = self.device_context_ref.context(); // Get context from stored context
        dx11_debug!("Setting viewport: {:?}", self.viewport);
        unsafe { context.RSSetViewports(Some(&[d3d_viewport])) };

        dx11_debug!("Surface resize finished.");
        Ok(())
    }

    /// Binds the render target and sets the viewport.
    pub fn bind_render_targets_and_viewport(&self, context: &ID3D11DeviceContext) {
        // This can be very noisy, maybe enable only with higher debug level
        // dx11_debug!("Binding render targets and setting viewport...");
        unsafe {
            // Bind the render target view and depth stencil view to the output merger stage
            context.OMSetRenderTargets(
                Some(&[self.render_target_view.clone()]),
                self.depth_stencil_view.as_ref(),
            );
            // dx11_debug!("RTV bound: {:?}", self.render_target_view.is_some());
            // dx11_debug!("DSV bound: {:?}", self.depth_stencil_view.is_some());
            
            let d3d_viewport = self.viewport.to_d3d11();
            context.RSSetViewports(Some(&[d3d_viewport]));
            // dx11_debug!("Viewport set: {:?}", self.viewport);
        }
    }

    /// Clears the render target to the specified color
    pub fn clear_render_target(&self, context: &ID3D11DeviceContext, color: &[f32; 4]) {
        // dx11_debug!("Clearing render target with color: {:?}", color); // Noisy
        unsafe {
            if let Some(rtv) = &self.render_target_view {
                context.ClearRenderTargetView(rtv, color);
                // dx11_debug!("✓ RTV cleared.");
            } else {
                dx11_warn!("Attempted to clear render target, but no RTV is available!");
            }
        }
    }

    /// Presents the frame to the window.
    pub fn present(&self) -> Result<()> {
        // dx11_debug!("Presenting frame..."); // Noisy, logged in backend
        let result = unsafe { self.swap_chain.Present(1, 0).ok() }; // SyncInterval=1 (vsync), Flags=0
        // Logging done in backend
        result
    }

    /// Gets the HWND associated with this surface.
    #[allow(dead_code)]
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    /// Gets the depth stencil view.
    pub fn depth_stencil_view(&self) -> Option<&ID3D11DepthStencilView> {
        self.depth_stencil_view.as_ref()
    }

    /// Gets the current viewport settings.
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

