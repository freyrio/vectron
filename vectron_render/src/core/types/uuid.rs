use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt::{Debug, Formatter};

/// A Universally Unique Identifier for resources
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Uuid {
    high: u64,
    low: u64,
}

impl Uuid {
    /// Create a new UUID with specific high and low bits
    pub fn new(high: u64, low: u64) -> Self {
        Self { high, low }
    }
    
    /// Create a new sequential UUID 
    pub fn sequential() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self { high: 0, low: id }
    }
    
    /// Create a UUID from a single u64 value (useful for simple IDs)
    pub fn from_u64(value: u64) -> Self {
        Self { high: 0, low: value }
    }
    
    /// Get the low 64 bits of the UUID
    pub fn low(&self) -> u64 {
        self.low
    }
    
    /// Get the high 64 bits of the UUID
    pub fn high(&self) -> u64 {
        self.high
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uuid({:016x}{:016x})", self.high, self.low)
    }
}