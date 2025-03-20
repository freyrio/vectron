/// Shader stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

/// Shader language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderLanguage {
    HLSL,
    GLSL,
    SpirV,
    MSL,
    WGSL,
}

/// Shader source
#[derive(Debug, Clone)]
pub enum ShaderSource {
    Source(String, ShaderLanguage),
    Binary(Vec<u8>, ShaderLanguage),
}

/// Shader descriptor
#[derive(Debug, Clone)]
pub struct ShaderDescriptor {
    pub stage: ShaderStage,
    pub source: ShaderSource,
    pub entry_point: String,
}

impl ShaderDescriptor {
    /// Create a new vertex shader from HLSL source
    pub fn vertex_hlsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Vertex,
            source: ShaderSource::Source(source, ShaderLanguage::HLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new fragment shader from HLSL source
    pub fn fragment_hlsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Fragment,
            source: ShaderSource::Source(source, ShaderLanguage::HLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new compute shader from HLSL source
    pub fn compute_hlsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Compute,
            source: ShaderSource::Source(source, ShaderLanguage::HLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new vertex shader from GLSL source
    pub fn vertex_glsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Vertex,
            source: ShaderSource::Source(source, ShaderLanguage::GLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new fragment shader from GLSL source
    pub fn fragment_glsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Fragment,
            source: ShaderSource::Source(source, ShaderLanguage::GLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new compute shader from GLSL source
    pub fn compute_glsl(source: String, entry_point: impl Into<String>) -> Self {
        Self {
            stage: ShaderStage::Compute,
            source: ShaderSource::Source(source, ShaderLanguage::GLSL),
            entry_point: entry_point.into(),
        }
    }
    
    /// Create a new shader from SPIR-V binary
    pub fn spirv(data: Vec<u8>, stage: ShaderStage, entry_point: impl Into<String>) -> Self {
        Self {
            stage,
            source: ShaderSource::Binary(data, ShaderLanguage::SpirV),
            entry_point: entry_point.into(),
        }
    }
}

/// Shader binding type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderBindingType {
    UniformBuffer,
    StorageBuffer,
    SampledTexture,
    StorageTexture,
    Sampler,
}

/// Shader binding visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderBindingVisibility {
    Vertex,
    Fragment,
    Compute,
    All,
}

/// Shader binding descriptor
#[derive(Debug, Clone)]
pub struct ShaderBindingDescriptor {
    pub binding: u32,
    pub binding_type: ShaderBindingType,
    pub visibility: ShaderBindingVisibility,
    pub count: u32,
}

/// Shader binding group descriptor
#[derive(Debug, Clone)]
pub struct ShaderBindingGroupDescriptor {
    pub group: u32,
    pub bindings: Vec<ShaderBindingDescriptor>,
}
