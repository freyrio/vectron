// shapes module entry point

pub mod rectangle;
pub mod tessellatable;
pub mod text;

// Re-export for easier access
pub use rectangle::Rectangle;
pub use tessellatable::Tessellatable;
pub use text::Text; 