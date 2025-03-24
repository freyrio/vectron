/*!
 * Error handling for the render crate
 */

/// Render error type with detailed information
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to create resource: {0}")]
    ResourceCreationFailed(String),
    
    #[error("Invalid resource handle: {0}")]
    InvalidResourceHandle(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Stack underflow: {0}")]
    StackUnderflow(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Text layout error: {0}")]
    TextLayoutError(String),
    
    #[error("Path tessellation error: {0}")]
    PathTessellationError(String),
    
    /// This is a placeholder for GPU errors - will be replaced with actual GPU error type
    #[error("GPU error: {0}")]
    GpuError(String),
    
    #[error("Unexpected error: {0}")]
    Other(String),
}

// Additional error context
pub struct ErrorContext {
    pub file: &'static str,
    pub line: u32,
    pub operation: &'static str,
}

// Macro for creating detailed errors
#[macro_export]
macro_rules! render_error {
    ($type:expr, $message:expr) => {
        $type(format!("{} (at {}:{})", $message, file!(), line!()))
    };
    
    ($type:expr, $fmt:expr, $($arg:tt)*) => {
        $type(format!("{} (at {}:{})", format!($fmt, $($arg)*), file!(), line!()))
    };
}

/// Error recovery options
pub enum RecoveryAction {
    /// Continue with default/fallback values
    UseFallback,
    
    /// Retry the operation
    Retry,
    
    /// Skip the current operation
    Skip,
    
    /// Abort the entire rendering process
    Abort,
}

/// Context-aware error handling trait
pub trait WithRecovery {
    /// Handle an error with possible recovery
    fn handle_error<T>(
        &self,
        error: RenderError,
        operation: &str,
        recovery: impl FnOnce(RecoveryAction) -> Result<T, RenderError>
    ) -> Result<T, RenderError>;
} 