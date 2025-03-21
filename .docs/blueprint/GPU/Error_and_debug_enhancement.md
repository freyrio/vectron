# Vectron GPU Error and Debug System Blueprint

## Overview

This document outlines an enhanced error handling and debugging system for the Vectron GPU library. These improvements build on the existing architecture while introducing more robust error management and a unified debugging interface across backends.

## Error System

### Core Components

#### 1. Structured Error Metadata

```rust
/// Standardized error codes across all backends
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Device errors
    DeviceLost,
    DeviceOutOfMemory,
    DeviceTimeout,
    
    // Resource errors
    InvalidResource,
    ResourceCreationFailed,
    ResourceMappingFailed,
    ResourceDestructionFailed,
    
    // Shader errors
    ShaderCompilationFailed,
    ShaderValidationFailed,
    ShaderReflectionFailed,
    
    // Pipeline errors
    PipelineCreationFailed,
    PipelineIncompatible,
    
    // Command errors
    CommandBufferFull,
    CommandInvalidArgument,
    CommandInvalidState,
    CommandUnsupported,
    
    // Synchronization errors
    SynchronizationFailed,
    DeadlockDetected,
    
    // API errors
    ApiVersionMismatch,
    FeatureNotSupported,
    InvalidOperation,
    
    // Generic errors
    ValidationError,
    InternalError,
    Unknown,
}

/// Severity level of an error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Fatal errors that cannot be recovered from
    Fatal,
    
    /// Serious errors that might be recoverable
    Error,
    
    /// Issues that should be addressed but don't prevent operation
    Warning,
    
    /// Informational messages about potential issues
    Info,
}

/// Category of an error for filtering and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Errors related to memory management
    Memory,
    
    /// Errors related to resource creation or usage
    Resource,
    
    /// Errors related to shaders and pipelines
    Pipeline,
    
    /// Errors related to command recording and submission
    Command,
    
    /// Errors related to synchronization
    Synchronization,
    
    /// Errors related to validation
    Validation,
    
    /// Errors internal to the implementation
    Internal,
}

/// Suggested actions to recover from an error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestedAction {
    /// Retry the operation
    Retry,
    
    /// Use a fallback approach
    UseFallback,
    
    /// Reset the device
    ResetDevice,
    
    /// Free memory or resources
    FreeResources,
    
    /// Report the issue
    ReportIssue,
    
    /// Custom action with description
    Custom(String),
}

/// Metadata associated with GPU errors
#[derive(Debug, Clone)]
pub struct ErrorMetadata {
    /// Standardized error code
    pub code: ErrorCode,
    
    /// Severity level
    pub severity: ErrorSeverity,
    
    /// Error category
    pub category: ErrorCategory,
    
    /// Whether the error is potentially recoverable
    pub recoverable: bool,
    
    /// Suggested action to recover, if applicable
    pub suggested_action: Option<SuggestedAction>,
    
    /// Backend-specific error information
    pub backend_info: Option<String>,
}

impl ErrorMetadata {
    /// Create new error metadata with default severity of Error
    pub fn new(code: ErrorCode, category: ErrorCategory) -> Self {
        Self {
            code,
            severity: ErrorSeverity::Error,
            category,
            recoverable: false,
            suggested_action: None,
            backend_info: None,
        }
    }
    
    /// Builder method to set severity
    pub fn severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    /// Builder method to set recoverability
    pub fn recoverable(mut self, recoverable: bool) -> Self {
        self.recoverable = recoverable;
        self
    }
    
    /// Builder method to add a suggested action
    pub fn with_action(mut self, action: SuggestedAction) -> Self {
        self.suggested_action = Some(action);
        self
    }
    
    /// Builder method to add backend-specific information
    pub fn with_backend_info(mut self, info: impl Into<String>) -> Self {
        self.backend_info = Some(info.into());
        self
    }
}
```

#### 2. Enhanced Error Type

```rust
/// Source location information for better error context
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

/// GPU error type with enhanced metadata and context
#[derive(Error, Debug)]
pub enum GpuError {
    /// Generic error with metadata
    #[error("{metadata.severity:?}: [{metadata.category:?}] {message}")]
    Generic {
        /// Human-readable error message
        message: String,
        
        /// Structured error metadata
        metadata: ErrorMetadata,
        
        /// Source location where the error occurred
        location: SourceLocation,
        
        /// Underlying cause
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// Out of device memory error
    #[error("Out of device memory: {message}")]
    OutOfMemory {
        message: String,
        allocated: u64,
        requested: u64,
        available: u64,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// Error when a device is lost
    #[error("Device lost: {message}")]
    DeviceLost {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError {
        message: String,
        object_type: String,
        object_handle: u64,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    // Additional specialized error variants...
}

impl GpuError {
    /// Create a new generic GPU error
    pub fn new(message: impl Into<String>, metadata: ErrorMetadata, location: SourceLocation) -> Self {
        Self::Generic {
            message: message.into(),
            metadata,
            location,
            source: None,
        }
    }
    
    /// Check if this error is potentially recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Generic { metadata, .. } => metadata.recoverable,
            Self::OutOfMemory { .. } => true, // Memory errors might be recoverable
            Self::DeviceLost { .. } => false, // Device lost is not recoverable
            Self::ValidationError { .. } => false, // Validation errors are not recoverable
            // Handle other variants...
            _ => false,
        }
    }
    
    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Generic { metadata, .. } => metadata.code,
            Self::OutOfMemory { .. } => ErrorCode::DeviceOutOfMemory,
            Self::DeviceLost { .. } => ErrorCode::DeviceLost,
            Self::ValidationError { .. } => ErrorCode::ValidationError,
            // Handle other variants...
            _ => ErrorCode::Unknown,
        }
    }
    
    /// Get suggested action for recovery
    pub fn suggested_action(&self) -> Option<SuggestedAction> {
        match self {
            Self::Generic { metadata, .. } => metadata.suggested_action.clone(),
            Self::OutOfMemory { .. } => Some(SuggestedAction::FreeResources),
            Self::DeviceLost { .. } => Some(SuggestedAction::ResetDevice),
            Self::ValidationError { .. } => Some(SuggestedAction::ReportIssue),
            // Handle other variants...
            _ => None,
        }
    }
}
```

#### 3. Error Recovery Mechanisms

```rust
/// Trait for objects that can provide fallback behavior
pub trait WithFallback<T> {
    /// Attempt to create with fallback if primary method fails
    fn create_with_fallback(&self, primary: impl FnOnce() -> Result<T, GpuError>, fallback: impl FnOnce() -> Result<T, GpuError>) -> Result<T, GpuError>;
}

impl<D> WithFallback<Pipeline> for D 
where
    D: AsRef<GpuDevice>,
{
    fn create_with_fallback(
        &self, 
        primary: impl FnOnce() -> Result<Pipeline, GpuError>, 
        fallback: impl FnOnce() -> Result<Pipeline, GpuError>
    ) -> Result<Pipeline, GpuError> {
        match primary() {
            Ok(pipeline) => Ok(pipeline),
            Err(e) if e.is_recoverable() => {
                log::warn!("Primary pipeline creation failed, using fallback: {}", e);
                fallback()
            },
            Err(e) => Err(e),
        }
    }
}

// Example usage:
impl GpuDevice {
    pub fn create_pipeline_with_fallback(&self, primary: PipelineDesc, fallback: PipelineDesc) -> Result<Pipeline, GpuError> {
        WithFallback::create_with_fallback(
            self,
            || self.create_pipeline(primary.clone()),
            || self.create_pipeline(fallback.clone())
        )
    }
}
```

#### 4. Enhanced Error Macros

```rust
/// Create a GpuError with current source location
#[macro_export]
macro_rules! gpu_error {
    ($code:expr, $category:expr, $message:expr) => {
        $crate::GpuError::new(
            $message,
            $crate::ErrorMetadata::new($code, $category),
            $crate::SourceLocation {
                file: file!(),
                line: line!(),
                column: column!(),
            }
        )
    };
    ($code:expr, $category:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::GpuError::new(
            format!($fmt, $($arg)*),
            $crate::ErrorMetadata::new($code, $category),
            $crate::SourceLocation {
                file: file!(),
                line: line!(),
                column: column!(),
            }
        )
    };
}

/// Try an operation, return error with context if it fails
#[macro_export]
macro_rules! gpu_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err($crate::GpuError::Generic {
                    message: format!("Operation failed: {}", err),
                    metadata: $crate::ErrorMetadata::new(
                        $crate::ErrorCode::Unknown,
                        $crate::ErrorCategory::Internal
                    ),
                    location: $crate::SourceLocation {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    source: Some(Box::new(err)),
                });
            }
        }
    };
    ($expr:expr, $code:expr, $category:expr, $message:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err($crate::GpuError::Generic {
                    message: $message.to_string(),
                    metadata: $crate::ErrorMetadata::new($code, $category),
                    location: $crate::SourceLocation {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    source: Some(Box::new(err)),
                });
            }
        }
    };
}
```

## Debug System

### Core Components

#### 1. Unified Debug Interface

```rust
/// Color for debug annotations
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
}

/// Unified debug interface for GPU debugging
pub trait DebugInterface: Send + Sync {
    /// Set a debug name for a GPU object
    fn set_object_name(&self, handle: u64, type_name: &str, name: &str) -> Result<(), GpuError>;
    
    /// Begin a debug event/group in a command buffer
    fn begin_event(&self, cmd: &CommandBuffer, name: &str, color: Color) -> Result<(), GpuError>;
    
    /// End the most recent debug event/group in a command buffer
    fn end_event(&self, cmd: &CommandBuffer) -> Result<(), GpuError>;
    
    /// Mark the beginning of a new frame with a name
    fn mark_frame(&self, name: &str) -> Result<(), GpuError>;
    
    /// Capture the next frame to the specified path
    fn capture_next_frame(&self, path: &std::path::Path) -> Result<(), GpuError>;
    
    /// Report all live objects (for leak detection)
    fn report_live_objects(&self) -> Result<(), GpuError>;
    
    /// Validate that a resource is in the expected state
    fn validate_resource_state(&self, id: u64, expected_state: u32) -> Result<(), GpuError>;
    
    /// Enable GPU-based validation
    fn enable_validation(&self, enabled: bool) -> Result<(), GpuError>;
    
    /// Check if debug interface is enabled
    fn is_enabled(&self) -> bool;
}

/// No-op implementation for release builds
#[cfg(not(debug_assertions))]
pub struct NoopDebugInterface;

#[cfg(not(debug_assertions))]
impl DebugInterface for NoopDebugInterface {
    fn set_object_name(&self, _handle: u64, _type_name: &str, _name: &str) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn begin_event(&self, _cmd: &CommandBuffer, _name: &str, _color: Color) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn end_event(&self, _cmd: &CommandBuffer) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn mark_frame(&self, _name: &str) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn capture_next_frame(&self, _path: &std::path::Path) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn report_live_objects(&self) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn validate_resource_state(&self, _id: u64, _expected_state: u32) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn enable_validation(&self, _enabled: bool) -> Result<(), GpuError> {
        Ok(())
    }
    
    fn is_enabled(&self) -> bool {
        false
    }
}
```

#### 2. Backend-Specific Debug Implementations

```rust
/// DirectX 12 debug implementation
#[cfg(all(target_os = "windows", feature = "dx12"))]
pub struct DirectX12Debug {
    info_queue: Option<ID3D12InfoQueue>,
    debug_device: Option<ID3D12DebugDevice>,
    debug_commandlist: Option<ID3D12DebugCommandList>,
    pix_runtime: Option<PixRuntime>,
}

#[cfg(all(target_os = "windows", feature = "dx12"))]
impl DebugInterface for DirectX12Debug {
    fn set_object_name(&self, handle: u64, type_name: &str, name: &str) -> Result<(), GpuError> {
        // Implementation using SetName on DirectX objects
        // ...
        Ok(())
    }
    
    fn begin_event(&self, cmd: &CommandBuffer, name: &str, color: Color) -> Result<(), GpuError> {
        if let Some(pix) = &self.pix_runtime {
            unsafe {
                pix.BeginEvent(cmd.raw_ptr() as _, name, color.r, color.g, color.b);
            }
        }
        Ok(())
    }
    
    // Other method implementations...
}

/// Vulkan debug implementation
#[cfg(feature = "vulkan")]
pub struct VulkanDebug {
    debug_utils: Option<vk::DebugUtilsMessengerEXT>,
    debug_utils_loader: Option<ash::extensions::ext::DebugUtils>,
    instance: ash::Instance,
    device: ash::Device,
}

#[cfg(feature = "vulkan")]
impl DebugInterface for VulkanDebug {
    // Implementation for Vulkan...
}

/// Metal debug implementation
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub struct MetalDebug {
    // Metal debug implementation details
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl DebugInterface for MetalDebug {
    // Implementation for Metal...
}
```

#### 3. Hierarchical Debug Groups with RAII Guards

```rust
/// RAII guard for debug groups
pub struct DebugGroupGuard<'a> {
    command_buffer: &'a CommandBuffer,
    debug: &'a dyn DebugInterface,
}

impl<'a> DebugGroupGuard<'a> {
    pub fn new(command_buffer: &'a CommandBuffer, debug: &'a dyn DebugInterface, name: &str, color: Color) -> Result<Self, GpuError> {
        debug.begin_event(command_buffer, name, color)?;
        Ok(Self { command_buffer, debug })
    }
}

impl<'a> Drop for DebugGroupGuard<'a> {
    fn drop(&mut self) {
        // Ignore errors in drop
        let _ = self.debug.end_event(self.command_buffer);
    }
}

impl CommandBuffer {
    /// Begin a debug group with automatic scope-based cleanup
    pub fn begin_debug_group<'a>(&'a self, debug: &'a dyn DebugInterface, name: &str, color: Color) -> Result<DebugGroupGuard<'a>, GpuError> {
        DebugGroupGuard::new(self, debug, name, color)
    }
}

// Example usage:
// {
//     let _guard = cmd.begin_debug_group(device.debug(), "Shadow pass", Color::RED)?;
//     // Draw shadow pass...
// } // Automatically ends the debug group when guard is dropped
```

#### 4. Debug Resource Labeling

```rust
/// Trait for GPU resources that can be labeled
pub trait Labeled {
    /// Set a debug label for this resource
    fn set_label(&self, debug: &dyn DebugInterface, label: &str) -> Result<(), GpuError>;
    
    /// Get the raw handle for this resource
    fn raw_handle(&self) -> u64;
    
    /// Get the type name for this resource
    fn type_name(&self) -> &'static str;
}

impl Labeled for Buffer {
    fn set_label(&self, debug: &dyn DebugInterface, label: &str) -> Result<(), GpuError> {
        debug.set_object_name(self.raw_handle(), self.type_name(), label)
    }
    
    fn raw_handle(&self) -> u64 {
        self.handle
    }
    
    fn type_name(&self) -> &'static str {
        "Buffer"
    }
}

// Implement for other resource types (Texture, Pipeline, etc.)

// Extension trait for builder pattern
pub trait WithLabel<T: Labeled> {
    /// Set a label and return self for chaining
    fn with_label(self, debug: &dyn DebugInterface, label: &str) -> Result<T, GpuError>;
}

impl<T: Labeled> WithLabel<T> for T {
    fn with_label(self, debug: &dyn DebugInterface, label: &str) -> Result<T, GpuError> {
        self.set_label(debug, label)?;
        Ok(self)
    }
}

// Example usage:
// let buffer = device.create_buffer(desc)?
//     .with_label(device.debug(), "Shadow map buffer")?;
```

## Integration

### Device Integration

```rust
pub struct GpuDevice {
    // Existing fields...
    
    // Debug interface for this device
    debug_interface: Box<dyn DebugInterface>,
}

impl GpuDevice {
    /// Access the debug interface
    pub fn debug(&self) -> &dyn DebugInterface {
        &*self.debug_interface
    }
    
    /// Create a new device with appropriate debug interface based on backend
    pub fn new(backend_type: BackendType, desc: &DeviceDesc) -> Result<Self, GpuError> {
        // Create backend-specific device...
        
        // Create appropriate debug interface based on backend
        let debug_interface: Box<dyn DebugInterface> = match backend_type {
            #[cfg(all(target_os = "windows", feature = "dx12"))]
            BackendType::DirectX12 => Box::new(DirectX12Debug::new(&device)?),
            
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => Box::new(VulkanDebug::new(&instance, &device)?),
            
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            BackendType::Metal => Box::new(MetalDebug::new(&device)?),
            
            #[cfg(not(debug_assertions))]
            _ => Box::new(NoopDebugInterface),
            
            #[cfg(debug_assertions)]
            _ => return Err(gpu_error!(ErrorCode::ApiVersionMismatch, ErrorCategory::Internal, 
                          "No debug interface available for backend {:?}", backend_type)),
        };
        
        // Return device with debug interface
        Ok(Self {
            // Existing fields...
            debug_interface,
        })
    }
}
```

### Command Buffer Integration

```rust
impl CommandBuffer {
    /// Record a scope with a debug label
    pub fn scope<F, R>(&mut self, label: &str, color: Color, f: F) -> Result<R, GpuError>
    where
        F: FnOnce(&mut Self) -> Result<R, GpuError>
    {
        let debug = self.device().debug();
        debug.begin_event(self, label, color)?;
        let result = f(self);
        debug.end_event(self)?;
        result
    }
    
    /// Record a scope with a debug label that captures the scope name
    #[macro_export]
    macro_rules! gpu_scope {
        ($cmd:expr, $color:expr, $body:block) => {
            $cmd.scope(function_name!(), $color, |cmd| $body)
        };
        ($cmd:expr, $label:expr, $color:expr, $body:block) => {
            $cmd.scope($label, $color, |cmd| $body)
        };
    }
}

// Example usage:
// gpu_scope!(cmd, Color::BLUE, {
//     cmd.draw(...)?;
//     cmd.draw(...)?;
//     Ok(())
// })?;
```

## Configuration

### Global Debug Options

```rust
/// Global debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Enable GPU-based validation
    pub enable_validation: bool,
    
    /// Enable automatic resource state tracking
    pub enable_resource_tracking: bool,
    
    /// Enable performance markers
    pub enable_performance_markers: bool,
    
    /// Enable debug naming of resources
    pub enable_debug_names: bool,
    
    /// Enable error reporting with extended information
    pub enable_extended_error_reporting: bool,
    
    /// Directory for debug dumps
    pub debug_dump_dir: Option<std::path::PathBuf>,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            enable_resource_tracking: cfg!(debug_assertions),
            enable_performance_markers: true,
            enable_debug_names: cfg!(debug_assertions),
            enable_extended_error_reporting: cfg!(debug_assertions),
            debug_dump_dir: None,
        }
    }
}

/// Initialize global debug configuration
pub fn init_debug(config: DebugConfig) {
    // Set global debug configuration
    // ...
}
```

## Implementation Notes

1. **Feature Flags**:
   - Make debug systems conditional on feature flags
   - Enable fine-grained control over which debug features are included

2. **Performance Considerations**:
   - Debug features should have minimal impact in release builds
   - Use dynamic dispatch only when necessary
   - Consider object pooling for frequently allocated debug objects

3. **Compatibility**:
   - Ensure consistent behavior across backends
   - Provide sensible fallbacks when backend-specific features are not available

4. **Error Handling Philosophy**:
   - Debug functions should avoid generating new errors when handling existing ones
   - Error recovery should be progressive (try simpler approaches first)
   - Errors should provide actionable information when possible