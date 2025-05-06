//! Manages the core D3D11 Device, Device Context, and associated DXGI Factory.

use windows::{
    core::*, // Interface, HRESULT, Result, Error, E_FAIL
    Win32::{
        Foundation::E_FAIL, Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0,
                D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
            },
            Direct3D11::*, // ID3D11Device, ID3D11DeviceContext, D3D11CreateDevice, D3D11_SDK_VERSION etc.
            Dxgi::*, // IDXGIDevice, IDXGIAdapter, IDXGIFactory2
        }
    },
};

/// Holds the essential D3D11 device, context, DXGI factory and feature level.
#[derive(Clone)] // Clone is useful if multiple parts need owned access, but be mindful of COM ref counting.
pub struct DeviceContext {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    dxgi_factory: IDXGIFactory2,
    feature_level: D3D_FEATURE_LEVEL,
}

impl DeviceContext {
    /// Creates the D3D11 Device, Context, and finds the associated DXGI Factory.
    pub fn new(enable_debug_layer: bool) -> Result<Self> {
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        let mut feature_level = D3D_FEATURE_LEVEL_11_0; // Default

        let mut create_device_flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
        if enable_debug_layer {
            // Only request debug layer if explicitly enabled
            // Note: Requires "Graphics Tools" optional feature installed in Windows
            create_device_flags |= D3D11_CREATE_DEVICE_DEBUG;
        }

        let feature_levels_to_try = [
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
        ];

        // Create the Direct3D 11 device and immediate context.
        unsafe {
            D3D11CreateDevice(
                None,                       // pAdapter: Use default adapter
                D3D_DRIVER_TYPE_HARDWARE,   // DriverType: Use hardware acceleration
                None,                       // Software: Not applicable for hardware
                create_device_flags,        // Flags
                Some(&feature_levels_to_try), // pFeatureLevels: Array to try
                D3D11_SDK_VERSION,          // SDKVersion
                Some(&mut device),          // ppDevice: Output device
                Some(&mut feature_level),   // pFeatureLevel: Output actual feature level
                Some(&mut context),         // ppImmediateContext: Output context
            )?;
        }

        let device = device.ok_or_else(|| Error::from(E_FAIL))?;
        let context = context.ok_or_else(|| Error::from(E_FAIL))?;

        // Get the DXGI Factory associated with the device.
        let dxgi_device: IDXGIDevice = device.cast()?;
        let dxgi_adapter: IDXGIAdapter = unsafe { dxgi_device.GetAdapter()? };
        let dxgi_factory: IDXGIFactory2 = unsafe { dxgi_adapter.GetParent()? };

        Ok(Self {
            device,
            context,
            dxgi_factory,
            feature_level,
        })
    }

    /// Provides access to the D3D11 Device.
    pub fn device(&self) -> &ID3D11Device {
        &self.device
    }

    /// Provides access to the D3D11 Immediate Context.
    pub fn context(&self) -> &ID3D11DeviceContext {
        &self.context
    }

    /// Provides access to the DXGI Factory.
    pub fn factory(&self) -> &IDXGIFactory2 {
        &self.dxgi_factory
    }

    /// Returns the selected D3D Feature Level.
    #[allow(dead_code)]
    pub fn feature_level(&self) -> D3D_FEATURE_LEVEL {
        self.feature_level
    }
}
