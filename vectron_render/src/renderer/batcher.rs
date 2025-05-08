// Operation batcher implementation for Vectron Render
//
// This module handles batching of render operations to minimize GPU state changes.

use std::collections::HashMap;
use crate::core::error::RenderError;
use crate::core::operation::render::RenderOperation;
use crate::core::operation::batch::{BatchKey, BatchableOperation};
use crate::core::traits::renderer::Renderer;

/// The OperationBatcher groups render operations by common state to minimize
/// GPU state changes and optimize rendering performance.
pub struct OperationBatcher {
    /// Batches grouped by batch key
    batches: HashMap<BatchKey, Vec<RenderOperation>>,
    
    /// Unbatchable operations that must be rendered individually
    unbatchable: Vec<RenderOperation>,
}

impl OperationBatcher {
    /// Create a new operation batcher
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
            unbatchable: Vec::new(),
        }
    }
    
    /// Clear all batches
    pub fn clear(&mut self) {
        self.batches.clear();
        self.unbatchable.clear();
    }
    
    /// Add an operation to the appropriate batch
    pub fn add(&mut self, operation: RenderOperation) {
        // Check if this operation can be batched
        if let Some(batchable) = BatchableOperation::from_operation(&operation) {
            // Get the batch key
            let key = batchable.batch_key();
            
            // Add to the appropriate batch
            self.batches.entry(key).or_insert_with(Vec::new).push(operation);
        } else {
            // Operation can't be batched, add to unbatchable list
            self.unbatchable.push(operation);
        }
    }
    
    /// Number of operations that will be rendered
    pub fn operation_count(&self) -> usize {
        let batched_count = self.batches.values().map(|batch| batch.len()).sum::<usize>();
        batched_count + self.unbatchable.len()
    }
    
    /// Number of draw calls that will be issued
    pub fn draw_call_count(&self) -> usize {
        // Each batch is one draw call, plus one for each unbatchable operation
        self.batches.len() + self.unbatchable.len()
    }
    
    /// Execute all batched operations
    pub fn execute(&self, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // First, sort batch keys for consistent rendering order
        let mut sorted_keys: Vec<&BatchKey> = self.batches.keys().collect();
        sorted_keys.sort_by(|a, b| {
            // Sort by layer first (front to back for opaque, back to front for transparent)
            match (a.is_transparent(), b.is_transparent()) {
                (true, true) => b.layer().cmp(&a.layer()), // Transparent back-to-front
                (false, false) => a.layer().cmp(&b.layer()), // Opaque front-to-back
                (true, false) => std::cmp::Ordering::Greater, // Transparent after opaque
                (false, true) => std::cmp::Ordering::Less,    // Opaque before transparent
            }
        });
        
        // First render the batched operations in sorted order
        for key in sorted_keys {
            if let Some(batch) = self.batches.get(key) {
                // Set up common state for the batch
                self.setup_batch_state(key, renderer)?;
                
                // Render each operation in the batch
                for operation in batch {
                    renderer.execute_operation(operation)?;
                }
            }
        }
        
        // Then render the unbatchable operations in layer order
        let mut sorted_unbatchable = self.unbatchable.clone();
        sorted_unbatchable.sort_by(|a, b| a.layer.cmp(&b.layer));
        
        for operation in sorted_unbatchable {
            renderer.execute_operation(&operation)?;
        }
        
        Ok(())
    }
    
    /// Set up the common state for a batch
    fn setup_batch_state(&self, key: &BatchKey, renderer: &mut dyn Renderer) -> Result<(), RenderError> {
        // This would set up common state like pipeline, textures, etc.
        // The actual implementation depends on what state is shared in the BatchKey
        
        // Example:
        // if let Some(pipeline) = key.pipeline() {
        //     renderer.set_pipeline(pipeline)?;
        // }
        
        Ok(())
    }
} 