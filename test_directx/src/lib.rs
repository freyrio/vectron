
pub mod dx11;

pub use dx11::*;

// Helper function to convert a slice to bytes
pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for [T] {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                std::mem::size_of::<T>() * self.len()
            )
        }
    }
}