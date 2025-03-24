/*!
 * Main renderer interface for the standard API
 */

use crate::error::{RenderError, RecoveryAction, WithRecovery};
use crate::api::bare::DrawList;
use crate::resources::ResourceCache;
use crate::backend::RenderContext;

/// Configuration options for the renderer
#[derive(Debug, Clone)]
pub struct RendererConfig {
    /// Maximum number of commands per frame
    pub max_commands: usize,
    
    /// Whether to enable debug features
    pub debug_mode: bool,
    
    /// Whether to allow fallbacks for errors
    pub allow_fallbacks: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            max_commands: 10000,
            debug_mode: false,
            allow_fallbacks: true,
        }
    }
}

/// Main renderer interface for the standard API
pub struct Renderer {
    /// Render context for backend operations
    pub(crate) context: RenderContext,
    
    /// Resource cache for managing resources
    pub(crate) resource_cache: ResourceCache,
    
    /// Configuration options
    pub(crate) config: RendererConfig,
    
    /// Current frame index
    frame_index: u64,
    
    /// Whether a frame is in progress
    frame_active: bool,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(context: RenderContext, config: RendererConfig) -> Result<Self, RenderError> {
        Ok(Self {
            context,
            resource_cache: ResourceCache::new(),
            config,
            frame_index: 0,
            frame_active: false,
        })
    }
    
    /// Begin a new frame
    pub fn begin_frame(&mut self, width: u32, height: u32) -> Result<&mut Self, RenderError> {
        if self.frame_active {
            return Err(RenderError::InvalidState("Frame already in progress".into()));
        }
        
        // Begin frame on the backend
        self.context.begin_frame(width, height)?;
        
        self.frame_active = true;
        self.frame_index += 1;
        
        Ok(self)
    }
    
    /// End the current frame
    pub fn end_frame(&mut self) -> Result<&mut Self, RenderError> {
        if !self.frame_active {
            return Err(RenderError::InvalidState("No frame in progress".into()));
        }
        
        // End frame on the backend
        self.context.end_frame()?;
        
        self.frame_active = false;
        
        Ok(self)
    }
    
    /// Submit a draw list for rendering
    pub fn submit(&mut self, draw_list: DrawList) -> Result<&mut Self, RenderError> {
        if !self.frame_active {
            return Err(RenderError::InvalidState("No frame in progress".into()));
        }
        
        // Submit draw list to the backend
        self.context.submit_commands(draw_list.commands())?;
        
        Ok(self)
    }
    
    /// Create a new draw list
    pub fn create_draw_list(&self) -> DrawList {
        DrawList::new()
    }
}

impl WithRecovery for Renderer {
    fn handle_error<T>(
        &self,
        error: RenderError,
        operation: &str,
        recovery: impl FnOnce(RecoveryAction) -> Result<T, RenderError>
    ) -> Result<T, RenderError> {
        // Log the error
        log::error!("Render error in {}: {}", operation, error);
        
        // Determine recovery action based on error type and severity
        let action = match &error {
            RenderError::ResourceCreationFailed(_) => {
                if self.config.allow_fallbacks {
                    RecoveryAction::UseFallback
                } else {
                    RecoveryAction::Abort
                }
            },
            RenderError::InvalidResourceHandle(_) => RecoveryAction::Skip,
            // Other error types...
            _ => RecoveryAction::Abort,
        };
        
        // Execute recovery strategy
        recovery(action)
    }
} 