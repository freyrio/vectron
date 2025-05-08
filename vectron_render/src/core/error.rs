use thiserror::Error;

/// Core error types for the rendering engine
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Device creation failed: {0}")]
    DeviceCreation(String),
    
    #[error("Resource creation failed: {0}")]
    ResourceCreation(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
    
    #[error("Surface error: {0}")]
    SurfaceError(String),
    
    #[error("Tessellation error: {0}")]
    TessellationError(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Backend error: {0}")]
    Backend(#[from] BackendError),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Backend-specific errors
#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Device creation failed: {0}")]
    DeviceCreation(String),
    
    #[error("Resource creation failed: {0}")]
    ResourceCreation(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
    
    #[error("Surface error: {0}")]
    SurfaceError(String),
    
    #[error("Backend error: {0}")]
    Other(String),
} 