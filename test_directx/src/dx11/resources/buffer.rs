//! Buffer module for managing DirectX buffer resources.

use crate::dx11::debug::*; // Import debug macros

use windows::Win32::Graphics::Direct3D11::{
    ID3D11Buffer, ID3D11Device, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, 
    D3D11_BIND_VERTEX_BUFFER, D3D11_BIND_INDEX_BUFFER, D3D11_BIND_CONSTANT_BUFFER,
    D3D11_USAGE_DEFAULT, D3D11_USAGE_DYNAMIC, D3D11_CPU_ACCESS_WRITE,
    D3D11_MAP_WRITE_DISCARD,
    D3D11_MAPPED_SUBRESOURCE,
};
use windows::core::Result;

/// Buffer usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    /// Vertex buffer
    Vertex,
    /// Index buffer
    Index,
    /// Constant buffer
    Constant,
}

/// Buffer creation descriptor
pub struct BufferDesc {
    /// Size of the buffer in bytes
    pub size: usize,
    /// Stride of the buffer elements (for vertex buffers), in bytes
    pub stride: usize,
    /// Usage flags for the buffer
    pub usage: BufferUsage,
    /// Whether the buffer can be updated frequently
    pub dynamic: bool,
}

impl Default for BufferDesc {
    fn default() -> Self {
        Self {
            size: 0,
            stride: 0,
            usage: BufferUsage::Vertex,
            dynamic: false,
        }
    }
}

/// DirectX buffer resource
pub struct Buffer {
    /// The DirectX buffer resource
    pub buffer: ID3D11Buffer,
    /// Size of the buffer in bytes
    pub size: usize,
    /// Stride of the buffer elements (for vertex buffers), in bytes
    pub stride: usize,
    /// Usage flags for the buffer
    pub usage: BufferUsage,
    /// Whether the buffer can be updated frequently
    pub dynamic: bool,
}

impl Buffer {
    /// Creates a new buffer
    pub fn new(device: &ID3D11Device, desc: &BufferDesc, data: Option<&[u8]>) -> Result<Self> {
        // Set up buffer description
        let mut buffer_desc = D3D11_BUFFER_DESC::default();
        buffer_desc.ByteWidth = desc.size as u32;
        
        let mut bind_flags = 0;
        match desc.usage {
            BufferUsage::Vertex => bind_flags |= D3D11_BIND_VERTEX_BUFFER.0,
            BufferUsage::Index => bind_flags |= D3D11_BIND_INDEX_BUFFER.0,
            BufferUsage::Constant => {
                bind_flags |= D3D11_BIND_CONSTANT_BUFFER.0;
                // Constant buffers must be a multiple of 16 bytes
                if buffer_desc.ByteWidth % 16 != 0 {
                    dx11_warn!("Constant buffer size {} is not a multiple of 16, padding to {}.", 
                             buffer_desc.ByteWidth, (buffer_desc.ByteWidth + 15) & !15);
                    buffer_desc.ByteWidth = (buffer_desc.ByteWidth + 15) & !15;
                }
            },
        }
        buffer_desc.BindFlags = bind_flags as u32;
        
        if desc.dynamic {
            buffer_desc.Usage = D3D11_USAGE_DYNAMIC;
            buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE.0 as u32;
        } else {
            buffer_desc.Usage = D3D11_USAGE_DEFAULT;
            buffer_desc.CPUAccessFlags = 0;
        }

        dx11_debug!("Creating D3D11 Buffer: Usage={:?}, Size={}, Dynamic={}, BindFlags={}", 
                    desc.usage, buffer_desc.ByteWidth, desc.dynamic, buffer_desc.BindFlags);
        
        // Create the buffer
        let d3d_buffer = if let Some(init_data) = data {
            if init_data.len() > buffer_desc.ByteWidth as usize {
                 dx11_error!("Initial buffer data size ({}) exceeds padded buffer size ({}).",
                           init_data.len(), buffer_desc.ByteWidth);
                 return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG));
            }
            let mut subresource_data = D3D11_SUBRESOURCE_DATA::default();
            subresource_data.pSysMem = init_data.as_ptr() as *const _;
            unsafe {
                let mut buffer_ptr = None;
                device.CreateBuffer(&buffer_desc, Some(&subresource_data), Some(&mut buffer_ptr)).map_err(|e|{
                    dx11_error!("CreateBuffer (with data) failed: {:?}", e);
                    e
                })?;
                buffer_ptr.unwrap() // Should be safe due to Result check
            }
        } else {
             if desc.usage == BufferUsage::Constant && !desc.dynamic {
                 dx11_warn!("Creating non-dynamic Constant Buffer without initial data.");
             }
            unsafe {
                let mut buffer_ptr = None;
                device.CreateBuffer(&buffer_desc, None, Some(&mut buffer_ptr)).map_err(|e|{
                    dx11_error!("CreateBuffer (no data) failed: {:?}", e);
                    e
                })?;
                buffer_ptr.unwrap()
            }
        };
        dx11_debug!("D3D11 Buffer created successfully.");
        
        Ok(Self {
            buffer: d3d_buffer,
            size: buffer_desc.ByteWidth as usize, // Use the potentially padded size
            stride: desc.stride,
            usage: desc.usage,
            dynamic: desc.dynamic,
        })
    }
    
    /// Updates the buffer with new data
    pub fn update(&self, context: &windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext, data: &[u8]) -> Result<()> {
        if data.len() > self.size {
            dx11_error!("Update data size ({}) exceeds buffer size ({}).", data.len(), self.size);
            return Err(windows::core::Error::from(windows::Win32::Foundation::E_INVALIDARG));
        }
        
        if self.dynamic {
            // dx11_debug!("Updating dynamic buffer (Map/Unmap)... Size: {}", data.len()); // Noisy
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            unsafe {
                context.Map(&self.buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped_resource)).map_err(|e|{
                    dx11_error!("Map (D3D11_MAP_WRITE_DISCARD) failed: {:?}", e);
                    e
                })?;
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    mapped_resource.pData as *mut u8,
                    data.len()
                );
                context.Unmap(&self.buffer, 0);
            }
        } else {
            // UpdateSubresource for non-dynamic buffers
             dx11_debug!("Updating non-dynamic buffer (UpdateSubresource)... Size: {}", data.len());
             if data.len() != self.size {
                 dx11_warn!("Updating non-dynamic buffer with partial data ({} bytes, expected {}).", data.len(), self.size);
                 // Depending on usage, this might be intended or an error.
             }
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: data.as_ptr() as *const _,
                SysMemPitch: 0, // Not used for buffers
                SysMemSlicePitch: 0, // Not used for buffers
            };
            unsafe {
                context.UpdateSubresource(&self.buffer, 0, None, subresource_data.pSysMem, 0, 0);
            }
            // Note: UpdateSubresource doesn't return HRESULT directly here.
            // Debug layer would report errors.
        }
        
        Ok(())
    }
} 