use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::collections::hash_map::DefaultHasher;
use crate::core::{Dimensionality, Uuid, RenderOperation};

/// Key used for batching similar operations together
#[derive(Debug, Clone, Eq)]
pub struct BatchKey {
    /// Dimensionality of the operations
    pub dimensionality: Dimensionality,
    
    /// Type ID of the primary style
    pub style_type_id: u64,
    
    /// Resource ID for textures, shaders, etc.
    pub resource_id: Option<Uuid>,
    
    /// Z-index for controlling render order
    pub z_index: i32,
    
    /// Layer for grouping operations
    pub layer: i32,
    
    /// Additional custom data for fine-grained batching control
    pub custom_data: Option<u64>,
}

impl PartialEq for BatchKey {
    fn eq(&self, other: &Self) -> bool {
        self.dimensionality == other.dimensionality &&
        self.style_type_id == other.style_type_id &&
        self.resource_id == other.resource_id &&
        self.z_index == other.z_index &&
        self.layer == other.layer &&
        self.custom_data == other.custom_data
    }
}

impl Hash for BatchKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Dimensionality now implements Hash
        self.dimensionality.hash(state);
        self.style_type_id.hash(state);
        self.resource_id.hash(state);
        self.z_index.hash(state);
        self.layer.hash(state);
        self.custom_data.hash(state);
    }
}

/// A wrapper around RenderOperation that contains information for batching
pub struct BatchableOperation {
    /// The underlying render operation
    pub operation: RenderOperation,
    
    /// The key for batching
    pub batch_key: BatchKey,
}

impl fmt::Debug for BatchableOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BatchableOperation")
            .field("operation", &self.operation)
            .field("batch_key", &self.batch_key)
            .finish()
    }
}

impl BatchableOperation {
    /// Create a new batchable operation from a render operation
    pub fn new(operation: RenderOperation) -> Self {
        // Create a default batch key
        let batch_key = Self::generate_batch_key(&operation);
        
        Self {
            operation,
            batch_key,
        }
    }
    
    /// Generate a batch key from a render operation
    pub fn generate_batch_key(operation: &RenderOperation) -> BatchKey {
        let dimensionality = operation.dimensionality();
        
        // Get the type ID of the primary style (if any)
        let style_type_id = if !operation.styles.is_empty() {
            // Get a hash of the TypeId as u64
            let type_id = operation.styles[0].as_any().type_id();
            let mut hasher = DefaultHasher::new();
            type_id.hash(&mut hasher);
            hasher.finish() // Get a stable u64 hash
        } else {
            0
        };
        
        // Get the z-index from the primary style (if any)
        let z_index = if !operation.styles.is_empty() {
            operation.styles[0].z_index()
        } else {
            0
        };
        
        BatchKey {
            dimensionality,
            style_type_id,
            resource_id: None, // This would be populated from style-specific data
            z_index,
            layer: operation.layer,
            custom_data: None,
        }
    }
    
    /// Update the batch key with style-specific information
    pub fn with_style_info(mut self, resource_id: Option<Uuid>, custom_data: Option<u64>) -> Self {
        self.batch_key.resource_id = resource_id;
        self.batch_key.custom_data = custom_data;
        self
    }
    
    /// Get a reference to the underlying operation
    pub fn operation(&self) -> &RenderOperation {
        &self.operation
    }
    
    /// Get a mutable reference to the underlying operation
    pub fn operation_mut(&mut self) -> &mut RenderOperation {
        &mut self.operation
    }
    
    /// Get a reference to the batch key
    pub fn batch_key(&self) -> &BatchKey {
        &self.batch_key
    }
}

/// A system for batching compatible operations together
pub struct Batcher {
    /// Batches organized by batch key
    batches: HashMap<BatchKey, Vec<BatchableOperation>>,
}

impl Batcher {
    /// Create a new empty batcher
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }
    
    /// Add an operation to the batcher
    pub fn add(&mut self, operation: BatchableOperation) {
        let key = operation.batch_key.clone();
        self.batches.entry(key).or_default().push(operation);
    }
    
    /// Add a render operation to the batcher with an automatically generated batch key
    pub fn add_operation(&mut self, operation: RenderOperation) {
        let batchable = BatchableOperation::new(operation);
        self.add(batchable);
    }
    
    /// Get all batches in optimal rendering order
    pub fn get_sorted_batches(&self) -> Vec<(&BatchKey, &Vec<BatchableOperation>)> {
        let mut batches: Vec<_> = self.batches.iter().collect();
        
        // Sort batches for optimal rendering:
        // 1. By layer (lower layers first)
        // 2. By z-index (lower z-index first)
        // 3. By resource ID to minimize texture/shader changes
        batches.sort_by(|(key_a, _), (key_b, _)| {
            key_a.layer.cmp(&key_b.layer)
                .then(key_a.z_index.cmp(&key_b.z_index))
                .then_with(|| {
                    // Sort by resource ID if both exist
                    if let (Some(id_a), Some(id_b)) = (&key_a.resource_id, &key_b.resource_id) {
                        // Sort by high bits, then low bits
                        id_a.high().cmp(&id_b.high()).then(id_a.low().cmp(&id_b.low()))
                    } else {
                        // Sort None values last
                        key_a.resource_id.is_some().cmp(&key_b.resource_id.is_some())
                    }
                })
        });
        
        batches
    }
    
    /// Clear all batches
    pub fn clear(&mut self) {
        self.batches.clear();
    }
    
    /// Get the number of operations in all batches
    pub fn operation_count(&self) -> usize {
        self.batches.values().map(|batch| batch.len()).sum()
    }
    
    /// Get the number of batches
    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }
} 