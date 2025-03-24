/*!
 * Shader management for rendering
 */

// Vector rendering shaders
#[cfg(feature = "vector")]
pub mod vector;

// Text rendering shaders
#[cfg(feature = "text")]
pub mod text;

// 3D rendering shaders
#[cfg(feature = "three_d")]
pub mod three_d;

// Effect shaders
#[cfg(feature = "effect")]
pub mod effects;

/// Shader module with metadata and binary
pub struct ShaderModule {
    /// Name of the shader
    pub name: &'static str,
    
    /// Shader type
    pub shader_type: ShaderType,
    
    /// Shader stage(s)
    pub shader_stage: ShaderStage,
    
    /// Entry point (function name)
    pub entry_point: &'static str,
    
    /// Compiled shader binary
    pub binary: &'static [u8],
}

/// Types of shaders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    /// GLSL source code
    GLSL,
    
    /// SPIRV binary
    SPIRV,
    
    /// WGSL source code
    WGSL,
}

/// Shader stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    /// Vertex shader
    Vertex,
    
    /// Fragment shader
    Fragment,
    
    /// Compute shader
    Compute,
    
    /// Geometry shader
    Geometry,
    
    /// Tessellation control shader
    TessellationControl,
    
    /// Tessellation evaluation shader
    TessellationEvaluation,
} 