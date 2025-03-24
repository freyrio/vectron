use crate::core::drawable::Drawable;
use crate::core::operation::{Operation, CommandOperation};
use crate::core::style::Renderer;
use crate::error::RenderError;
use std::any::TypeId;
use std::collections::HashMap;

/// Trait for operations that can be batched together
pub trait Batchable: CommandOperation {
    /// Get the type ID of the operation, used for batching similar operations
    fn type_id(&self) -> TypeId;
    
    /// Check if this operation can be batched with another
    fn can_batch_with(&self, other: &dyn Batchable) -> bool;
    
    /// Attempt to merge this operation with another
    /// Returns true if the merge was successful
    fn try_merge_with(&mut self, other: &dyn Batchable) -> bool;
}

/// Batch for a specific type of drawable and style
pub struct Batch {
    // Type identifier for the batch
    type_id: TypeId,
    // Batched operations
    operations: Vec<Box<dyn Batchable>>,
}

impl Batch {
    /// Create a new batch with the first operation
    pub fn new(type_id: TypeId, operation: Box<dyn Batchable>) -> Self {
        Self {
            type_id,
            operations: vec![operation],
        }
    }
    
    /// Try to add an operation to this batch
    pub fn try_add(&mut self, operation: Box<dyn Batchable>) -> bool {
        if operation.type_id() != self.type_id {
            return false;
        }
        
        // Try to merge with an existing operation
        for existing in &mut self.operations {
            if existing.can_batch_with(operation.as_ref()) {
                if existing.try_merge_with(operation.as_ref()) {
                    return true;
                }
            }
        }
        
        // If no merge was possible, add as a new operation
        self.operations.push(operation);
        true
    }
    
    /// Execute all operations in this batch
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        for operation in &self.operations {
            operation.execute(renderer)?;
        }
        Ok(())
    }
}

/// Batching command queue for optimized rendering
pub struct BatchQueue {
    // Batches grouped by type ID
    batches: HashMap<TypeId, Batch>,
    // Unbatchable operations that must be executed in order
    unbatchable: Vec<Box<dyn CommandOperation>>,
}

impl BatchQueue {
    /// Create a new empty batch queue
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
            unbatchable: Vec::new(),
        }
    }
    
    /// Add an operation to the batch queue
    pub fn add<D: Drawable + 'static>(&mut self, operation: Operation<D>) -> &mut Self {
        // Check if the operation is batchable
        if operation.is_batchable() {
            if let Some(batchable) = operation.as_batchable() {
                let type_id = batchable.type_id();
                
                // Try to add to an existing batch
                if let Some(batch) = self.batches.get_mut(&type_id) {
                    if batch.try_add(Box::new(operation)) {
                        return self;
                    }
                }
                
                // Create a new batch
                let batch = Batch::new(type_id, Box::new(operation));
                self.batches.insert(type_id, batch);
                return self;
            }
        }
        
        // Add as unbatchable operation
        self.unbatchable.push(Box::new(operation));
        self
    }
    
    /// Clear all operations from the queue
    pub fn clear(&mut self) {
        self.batches.clear();
        self.unbatchable.clear();
    }
    
    /// Execute all operations in optimal order
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // Execute batched operations first (these are typically more efficient)
        for batch in self.batches.values() {
            batch.execute(renderer)?;
        }
        
        // Then execute unbatchable operations
        for operation in &self.unbatchable {
            operation.execute(renderer)?;
        }
        
        Ok(())
    }
} 