use bitflags::bitflags;

/// Buffer usage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
    Indirect,
}

bitflags! {
    /// Buffer usage flags - multiple usages can be combined
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BufferUsageFlags: u32 {
        const VERTEX = 0b00000001;
        const INDEX = 0b00000010;
        const UNIFORM = 0b00000100;
        const STORAGE = 0b00001000;
        const INDIRECT = 0b00010000;
    }
}

impl From<BufferUsage> for BufferUsageFlags {
    fn from(usage: BufferUsage) -> Self {
        match usage {
            BufferUsage::Vertex => BufferUsageFlags::VERTEX,
            BufferUsage::Index => BufferUsageFlags::INDEX,
            BufferUsage::Uniform => BufferUsageFlags::UNIFORM,
            BufferUsage::Storage => BufferUsageFlags::STORAGE,
            BufferUsage::Indirect => BufferUsageFlags::INDIRECT,
        }
    }
}

/// CPU access mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuAccessMode {
    None,
    Write,
    Read,
    ReadWrite,
}

/// Buffer descriptor
#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    pub size: usize,
    pub usage: BufferUsageFlags,
    pub cpu_access: CpuAccessMode,
    pub initial_data: Option<Vec<u8>>,
}

impl BufferDescriptor {
    /// Create a new buffer descriptor
    pub fn new(size: usize, usage: BufferUsageFlags, cpu_access: CpuAccessMode) -> Self {
        Self {
            size,
            usage,
            cpu_access,
            initial_data: None,
        }
    }
    
    /// Create a new buffer descriptor with initial data
    pub fn with_data(size: usize, usage: BufferUsageFlags, cpu_access: CpuAccessMode, data: Vec<u8>) -> Self {
        Self {
            size,
            usage,
            cpu_access,
            initial_data: Some(data),
        }
    }
    
    /// Create a vertex buffer descriptor
    pub fn vertex(size: usize, cpu_access: CpuAccessMode) -> Self {
        Self::new(size, BufferUsageFlags::VERTEX, cpu_access)
    }
    
    /// Create an index buffer descriptor
    pub fn index(size: usize, cpu_access: CpuAccessMode) -> Self {
        Self::new(size, BufferUsageFlags::INDEX, cpu_access)
    }
    
    /// Create a uniform buffer descriptor
    pub fn uniform(size: usize, cpu_access: CpuAccessMode) -> Self {
        Self::new(size, BufferUsageFlags::UNIFORM, cpu_access)
    }
}
