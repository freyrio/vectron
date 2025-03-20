use thiserror::Error;

/// GPU operation errors
#[derive(Error, Debug)]
pub enum GpuError {
    #[error("Backend initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Invalid handle or resource ID: {0}")]
    InvalidHandle(String),
    
    #[error("Invalid resource")]
    InvalidResource,
    
    #[error("Invalid buffer: {0}")]
    InvalidBuffer(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Resource creation failed: {0}")]
    ResourceCreationFailed(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Surface error: {0}")]
    SurfaceError(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("Unimplemented feature: {0}")]
    Unimplemented(String),
    
    #[error("Backend-specific error: {0}")]
    BackendError(String),
    
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Device lost")]
    DeviceLost,
} 