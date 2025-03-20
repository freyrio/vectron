use thiserror::Error;

/// GPU operation errors
#[derive(Error, Debug)]
pub enum GpuError {
    #[error("Backend initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Invalid handle: {0}")]
    InvalidHandle(String),
    
    #[error("Invalid resource: {0}")]
    InvalidResource(String),
    
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
    
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Device lost: {0}")]
    DeviceLost(String),
    
    #[error("Buffer update failed (size: {size}, offset: {offset}): {reason}")]
    BufferUpdateFailed {
        size: usize,
        offset: usize,
        reason: String,
    },
    
    #[error("Pipeline creation failed: {reason}")]
    PipelineCreationFailed {
        reason: String,
        shader_errors: Option<Vec<String>>,
    },
    
    // Backend error with code formatting
    #[error("Backend error [{backend}]: {message}{}", format_code(.code))]
    BackendError {
        backend: String,
        message: String,
        code: Option<i32>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl GpuError {
    // Helper method to create a backend error with formatted code
    pub fn backend_error<S: Into<String>>(
        backend: S, 
        message: S, 
        code: Option<i32>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>
    ) -> Self {
        GpuError::BackendError {
            backend: backend.into(),
            message: message.into(),
            code,
            source,
        }
    }
}

// Helper function for thiserror to format the code
fn format_code(code: &Option<i32>) -> String {
    match code {
        Some(code) => format!(" (code: 0x{:X})", code),
        None => String::new(),
    }
}