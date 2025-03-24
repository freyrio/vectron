use crate::core::drawable::Drawable;
use crate::core::operation::{Operation, CommandOperation};
use crate::core::style::Renderer;
use crate::error::RenderError;

/// Simple command queue for sequential rendering
pub struct CommandQueue {
    /// List of operations to execute in order
    operations: Vec<Box<dyn CommandOperation>>,
}

impl CommandQueue {
    /// Create a new empty command queue
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    
    /// Add an operation to the queue
    pub fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        self.operations.push(Box::new(operation));
        self
    }
    
    /// Add a boxed CommandOperation to the queue
    pub fn add_boxed(&mut self, operation: Box<dyn CommandOperation>) -> &mut Self {
        self.operations.push(operation);
        self
    }
    
    /// Clear all operations from the queue
    pub fn clear(&mut self) {
        self.operations.clear();
    }
    
    /// Get the number of operations in the queue
    pub fn len(&self) -> usize {
        self.operations.len()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
    
    /// Execute all operations in the queue in order
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        for operation in &self.operations {
            operation.execute(renderer)?;
        }
        Ok(())
    }
}

/// Optimizing command queue that automatically chooses between batch and sequential rendering
pub struct OptimizingQueue {
    /// Whether to try batching operations
    enable_batching: bool,
    /// The underlying queue implementation
    queue: Box<dyn RenderQueue>,
}

/// Trait for render queues
pub trait RenderQueue {
    /// Add an operation to the queue
    fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self;
    
    /// Clear all operations from the queue
    fn clear(&mut self);
    
    /// Execute all operations in the queue
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError>;
}

impl RenderQueue for CommandQueue {
    fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        self.add(operation)
    }
    
    fn clear(&mut self) {
        self.clear();
    }
    
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        self.execute(renderer)
    }
}

// Implement RenderQueue for our BatchQueue
impl RenderQueue for crate::commands::batch::BatchQueue {
    fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        self.add(operation)
    }
    
    fn clear(&mut self) {
        self.clear();
    }
    
    fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        self.execute(renderer)
    }
}

impl OptimizingQueue {
    /// Create a new optimizing queue with batching enabled
    pub fn new(enable_batching: bool) -> Self {
        let queue: Box<dyn RenderQueue> = if enable_batching {
            Box::new(crate::commands::batch::BatchQueue::new())
        } else {
            Box::new(CommandQueue::new())
        };
        
        Self {
            enable_batching,
            queue,
        }
    }
    
    /// Add an operation to the queue
    pub fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        self.queue.add(operation);
        self
    }
    
    /// Clear all operations from the queue
    pub fn clear(&mut self) {
        self.queue.clear();
    }
    
    /// Execute all operations in the queue
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        self.queue.execute(renderer)
    }
    
    /// Set whether batching is enabled
    pub fn set_batching(&mut self, enable: bool) {
        if enable != self.enable_batching {
            // Create a new queue with the updated batching setting
            let new_queue: Box<dyn RenderQueue> = if enable {
                Box::new(crate::commands::batch::BatchQueue::new())
            } else {
                Box::new(CommandQueue::new())
            };
            
            self.enable_batching = enable;
            self.queue = new_queue;
        }
    }
} 