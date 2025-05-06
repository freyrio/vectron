use std::error::Error;
use std::fmt;

/// Represents errors that can occur during rendering operations
#[derive(Debug)]
pub enum RenderError {
    /// Generic operation error with a message
    OperationError(String),
    
    /// Error related to backend device or resource management
    BackendError(String),
    
    /// Error occurring during tessellation
    TessellationError(String),
    
    /// Error during resource creation or management
    ResourceError(String),
    
    /// Error during text rendering operations
    TextError(String),
    
    /// Invalid or incompatible state
    InvalidState(String),
    
    /// Feature not supported by current backend/configuration
    UnsupportedFeature(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::OperationError(msg) => write!(f, "Operation error: {}", msg),
            RenderError::BackendError(msg) => write!(f, "Backend error: {}", msg),
            RenderError::TessellationError(msg) => write!(f, "Tessellation error: {}", msg),
            RenderError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
            RenderError::TextError(msg) => write!(f, "Text error: {}", msg),
            RenderError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            RenderError::UnsupportedFeature(msg) => write!(f, "Unsupported feature: {}", msg),
        }
    }
}

impl Error for RenderError {} 