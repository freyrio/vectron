//! Shader module for managing DirectX shader resources.

use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11VertexShader, ID3D11PixelShader, ID3D11ComputeShader,
    ID3D11GeometryShader, ID3D11HullShader, ID3D11DomainShader,
};
use windows::core::Result;

/// Shader type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    /// Vertex shader
    Vertex,
    /// Pixel shader
    Pixel,
    /// Compute shader
    Compute,
    /// Geometry shader
    Geometry,
    /// Hull shader
    Hull,
    /// Domain shader
    Domain,
}

/// Shader resource
pub enum Shader {
    /// Vertex shader
    Vertex(ID3D11VertexShader),
    /// Pixel shader
    Pixel(ID3D11PixelShader),
    /// Compute shader
    Compute(ID3D11ComputeShader),
    /// Geometry shader
    Geometry(ID3D11GeometryShader),
    /// Hull shader
    Hull(ID3D11HullShader),
    /// Domain shader
    Domain(ID3D11DomainShader),
}

impl Shader {
    /// Creates a new shader from bytecode
    pub fn new(device: &ID3D11Device, shader_type: ShaderType, bytecode: &[u8]) -> Result<Self> {
        match shader_type {
            ShaderType::Vertex => {
                let mut shader = None;
                unsafe {
                    device.CreateVertexShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Vertex(shader.unwrap()))
            },
            ShaderType::Pixel => {
                let mut shader = None;
                unsafe {
                    device.CreatePixelShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Pixel(shader.unwrap()))
            },
            ShaderType::Compute => {
                let mut shader = None;
                unsafe {
                    device.CreateComputeShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Compute(shader.unwrap()))
            },
            ShaderType::Geometry => {
                let mut shader = None;
                unsafe {
                    device.CreateGeometryShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Geometry(shader.unwrap()))
            },
            ShaderType::Hull => {
                let mut shader = None;
                unsafe {
                    device.CreateHullShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Hull(shader.unwrap()))
            },
            ShaderType::Domain => {
                let mut shader = None;
                unsafe {
                    device.CreateDomainShader(
                        bytecode,
                        None,
                        Some(&mut shader)
                    )?;
                }
                Ok(Shader::Domain(shader.unwrap()))
            },
        }
    }

    /// Gets the shader type
    pub fn shader_type(&self) -> ShaderType {
        match self {
            Shader::Vertex(_) => ShaderType::Vertex,
            Shader::Pixel(_) => ShaderType::Pixel,
            Shader::Compute(_) => ShaderType::Compute,
            Shader::Geometry(_) => ShaderType::Geometry,
            Shader::Hull(_) => ShaderType::Hull,
            Shader::Domain(_) => ShaderType::Domain,
        }
    }
} 