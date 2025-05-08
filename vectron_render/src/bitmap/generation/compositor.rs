// Bitmap layer composition
//
// Provides functionality for composing bitmaps using a layer-based approach

use crate::bitmap::buffer::Bitmap;
use crate::bitmap::operations::blend::{self, BlendMode};
use crate::color::Color;

/// A layer in a compositor
#[derive(Debug, Clone)]
pub struct Layer {
    /// The bitmap data for this layer
    bitmap: Bitmap,
    /// Position offset from origin (x, y)
    position: (i32, i32),
    /// Layer opacity (0.0 to 1.0)
    opacity: f32,
    /// Blend mode to use when compositing
    blend_mode: BlendMode,
    /// Whether the layer is visible
    visible: bool,
    /// Layer name (optional)
    name: Option<String>,
}

impl Layer {
    /// Create a new layer from a bitmap
    pub fn new(bitmap: Bitmap) -> Self {
        Self {
            bitmap,
            position: (0, 0),
            opacity: 1.0,
            blend_mode: BlendMode::Alpha,
            visible: true,
            name: None,
        }
    }
    
    /// Set the layer position
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position = (x, y);
        self
    }
    
    /// Set the layer opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.max(0.0).min(1.0);
        self
    }
    
    /// Set the layer blend mode
    pub fn with_blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = blend_mode;
        self
    }
    
    /// Set the layer visibility
    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    
    /// Set the layer name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Get the layer bitmap
    pub fn bitmap(&self) -> &Bitmap {
        &self.bitmap
    }
    
    /// Get mutable access to the layer bitmap
    pub fn bitmap_mut(&mut self) -> &mut Bitmap {
        &mut self.bitmap
    }
    
    /// Get the layer position
    pub fn position(&self) -> (i32, i32) {
        self.position
    }
    
    /// Set the layer position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
    
    /// Get the layer opacity
    pub fn opacity(&self) -> f32 {
        self.opacity
    }
    
    /// Set the layer opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.max(0.0).min(1.0);
    }
    
    /// Get the layer blend mode
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    
    /// Set the layer blend mode
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.blend_mode = blend_mode;
    }
    
    /// Get the layer visibility
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Set the layer visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Get the layer name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    
    /// Set the layer name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }
}

/// Compositor for layered bitmap creation
pub struct Compositor {
    /// The width of the composition
    width: u32,
    /// The height of the composition
    height: u32,
    /// The layers in the composition (bottom to top)
    layers: Vec<Layer>,
    /// Background color (for transparent layers)
    background: Color,
}

impl Compositor {
    /// Create a new empty compositor
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            layers: Vec::new(),
            background: Color::TRANSPARENT,
        }
    }
    
    /// Set the background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
    
    /// Add a layer to the composition (at the top)
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    
    /// Remove a layer by index
    pub fn remove_layer(&mut self, index: usize) -> Option<Layer> {
        if index < self.layers.len() {
            Some(self.layers.remove(index))
        } else {
            None
        }
    }
    
    /// Get the number of layers
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
    
    /// Get a reference to a layer by index
    pub fn layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }
    
    /// Get a mutable reference to a layer by index
    pub fn layer_mut(&mut self, index: usize) -> Option<&mut Layer> {
        self.layers.get_mut(index)
    }
    
    /// Find a layer by name
    pub fn find_layer(&self, name: &str) -> Option<usize> {
        self.layers.iter().position(|layer| {
            layer.name.as_ref().map_or(false, |n| n == name)
        })
    }
    
    /// Move a layer to a new position in the stack
    pub fn move_layer(&mut self, from_index: usize, to_index: usize) -> bool {
        if from_index >= self.layers.len() || to_index >= self.layers.len() {
            return false;
        }
        
        let layer = self.layers.remove(from_index);
        self.layers.insert(to_index, layer);
        true
    }
    
    /// Merge the specified layer down to the layer below it
    pub fn merge_down(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.layers.len() {
            return false;
        }
        
        // First, check if the upper layer is visible
        let upper_visible = self.layers[index].is_visible();
        if !upper_visible {
            // Remove invisible layer and return
            self.layers.remove(index);
            return true;
        }
        
        // Clone the necessary properties from the upper layer before borrowing lower layer mutably
        let upper_bitmap = self.layers[index].bitmap().clone();
        let upper_position = self.layers[index].position();
        let upper_opacity = self.layers[index].opacity();
        let upper_blend_mode = self.layers[index].blend_mode();
        
        // Get the lower layer
        let lower_layer = &mut self.layers[index - 1];
        let (lower_x, lower_y) = lower_layer.position();
        
        // Adjust for position differences
        let x_offset = upper_position.0 - lower_x;
        let y_offset = upper_position.1 - lower_y;
        
        // Create a temporary bitmap for blending if opacity < 1.0
        let blend_bitmap = if upper_opacity < 1.0 {
            let mut opacity_bitmap = upper_bitmap.clone();
            for y in 0..opacity_bitmap.height() {
                for x in 0..opacity_bitmap.width() {
                    if let Some(color) = opacity_bitmap.get_pixel(x, y) {
                        let mut new_color = color;
                        new_color.a *= upper_opacity;
                        opacity_bitmap.set_pixel(x, y, new_color);
                    }
                }
            }
            opacity_bitmap
        } else {
            upper_bitmap
        };
        
        // Blend the upper layer into the lower layer
        blend::blend_rect(
            lower_layer.bitmap_mut(),
            &blend_bitmap,
            x_offset.max(0) as u32,
            y_offset.max(0) as u32,
            (-x_offset).max(0) as u32,
            (-y_offset).max(0) as u32,
            blend_bitmap.width(),
            blend_bitmap.height(),
            upper_blend_mode,
        );
        
        // Remove the upper layer
        self.layers.remove(index);
        true
    }
    
    /// Flatten all visible layers into a single bitmap
    pub fn flatten(&self) -> Bitmap {
        let mut result = Bitmap::with_color(self.width, self.height, self.background);
        
        // Composite layers from bottom to top
        for layer in &self.layers {
            if !layer.is_visible() {
                continue;
            }
            
            let (x, y) = layer.position();
            let bitmap = layer.bitmap();
            
            // Create a temporary bitmap for blending if opacity < 1.0
            let blend_bitmap = if layer.opacity() < 1.0 {
                let mut opacity_bitmap = bitmap.clone();
                for py in 0..opacity_bitmap.height() {
                    for px in 0..opacity_bitmap.width() {
                        if let Some(color) = opacity_bitmap.get_pixel(px, py) {
                            let mut new_color = color;
                            new_color.a *= layer.opacity();
                            opacity_bitmap.set_pixel(px, py, new_color);
                        }
                    }
                }
                opacity_bitmap
            } else {
                bitmap.clone()
            };
            
            // Blend this layer into the result
            blend::blend_rect(
                &mut result,
                &blend_bitmap,
                x.max(0) as u32,
                y.max(0) as u32,
                (-x).max(0) as u32,
                (-y).max(0) as u32,
                blend_bitmap.width().min(self.width),
                blend_bitmap.height().min(self.height),
                layer.blend_mode(),
            );
        }
        
        result
    }
    
    /// Create a new layer filled with a solid color
    pub fn create_solid_layer(&self, color: Color) -> Layer {
        Layer::new(Bitmap::with_color(self.width, self.height, color))
    }
    
    /// Create a new empty (transparent) layer
    pub fn create_empty_layer(&self) -> Layer {
        Layer::new(Bitmap::new(self.width, self.height))
    }
} 