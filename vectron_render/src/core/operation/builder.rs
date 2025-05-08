use crate::core::{Dimensionality, ClipRegion, Uuid, Effect, Transform, Style, Renderable, RenderOperation};


/// A builder for creating render operations with a fluent API.
///
/// This provides a more ergonomic way to construct RenderOperation instances
/// with various configurations.
pub struct RenderOperationBuilder {
    /// The operation being built
    operation: RenderOperation,
}

impl RenderOperationBuilder {
    /// Create a new builder for the given renderable element
    pub fn new<R: Renderable + 'static>(element: R) -> Self {
        Self {
            operation: RenderOperation::new(element),
        }
    }
    
    /// Add a style to the operation
    pub fn style<S: Style + 'static>(&mut self, style: S) -> &mut Self {
        self.operation.styles.push(Box::new(style));
        self
    }
    
    /// Add multiple styles to the operation
    pub fn styles<I, S>(&mut self, styles: I) -> &mut Self 
    where 
        I: IntoIterator<Item = S>,
        S: Style + 'static,
    {
        for style in styles {
            self.operation.styles.push(Box::new(style));
        }
        self
    }
    
    /// Add an effect to the operation
    pub fn effect<E: Effect + 'static>(&mut self, effect: E) -> &mut Self {
        self.operation.effects.push(Box::new(effect));
        self
    }
    
    /// Apply a transform to the operation
    pub fn transform(&mut self, transform: Transform) -> &mut Self {
        self.operation.transform = transform;
        self
    }
    
    /// Apply a translation transform
    pub fn translate(&mut self, x: f32, y: f32, z: Option<f32>) -> &mut Self {
        let transform = match self.operation.dimensionality() {
            Dimensionality::D2 => {
                let point = crate::core::geometry::point::Point2D::new(x, y);
                crate::core::geometry::transform::translation_transform(
                    &crate::core::geometry::point::Point::D2(point)
                )
            },
            Dimensionality::D3 => {
                let point = crate::core::geometry::point::Point3D::new(x, y, z.unwrap_or(0.0));
                crate::core::geometry::transform::translation_transform(
                    &crate::core::geometry::point::Point::D3(point)
                )
            },
        };
        
        // Combine with existing transform
        if let Some(combined) = crate::core::geometry::transform::combine_transforms(
            &transform, &self.operation.transform
        ) {
            self.operation.transform = combined;
        }
        
        self
    }
    
    /// Apply a scaling transform
    pub fn scale(&mut self, sx: f32, sy: f32, sz: Option<f32>) -> &mut Self {
        let transform = match self.operation.dimensionality() {
            Dimensionality::D2 => {
                let scale = crate::core::geometry::point::Point2D::new(sx, sy);
                crate::core::geometry::transform::scaling_transform(
                    &crate::core::geometry::point::Point::D2(scale)
                )
            },
            Dimensionality::D3 => {
                let scale = crate::core::geometry::point::Point3D::new(sx, sy, sz.unwrap_or(1.0));
                crate::core::geometry::transform::scaling_transform(
                    &crate::core::geometry::point::Point::D3(scale)
                )
            },
        };
        
        // Combine with existing transform
        if let Some(combined) = crate::core::geometry::transform::combine_transforms(
            &transform, &self.operation.transform
        ) {
            self.operation.transform = combined;
        }
        
        self
    }
    
    /// Apply a uniform scaling transform
    pub fn scale_uniform(&mut self, s: f32) -> &mut Self {
        let transform = crate::core::geometry::transform::uniform_scaling_transform(
            s, self.operation.dimensionality()
        );
        
        // Combine with existing transform
        if let Some(combined) = crate::core::geometry::transform::combine_transforms(
            &transform, &self.operation.transform
        ) {
            self.operation.transform = combined;
        }
        
        self
    }
    
    /// Apply a rotation transform (2D or around Z axis in 3D)
    pub fn rotate(&mut self, angle_radians: f32) -> &mut Self {
        let transform = match self.operation.dimensionality() {
            Dimensionality::D2 => {
                crate::core::geometry::transform::rotation_transform_2d(angle_radians)
            },
            Dimensionality::D3 => {
                crate::core::geometry::transform::rotation_transform_z(angle_radians)
            },
        };
        
        // Combine with existing transform
        if let Some(combined) = crate::core::geometry::transform::combine_transforms(
            &transform, &self.operation.transform
        ) {
            self.operation.transform = combined;
        }
        
        self
    }
    
    /// Set a clipping region
    pub fn clip(&mut self, clip: ClipRegion) -> &mut Self {
        self.operation.clip = Some(clip);
        self
    }
    
    /// Set a rectangular clipping region
    pub fn clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self {
        self.operation.clip = Some(ClipRegion::Rect { 
            x, y, width, height 
        });
        self
    }
    
    /// Set an ID for the operation
    pub fn id(&mut self, id: Uuid) -> &mut Self {
        self.operation.id = Some(id);
        self
    }
    
    /// Set an auto-generated sequential ID for the operation
    pub fn auto_id(&mut self) -> &mut Self {
        self.operation.id = Some(Uuid::sequential());
        self
    }
    
    /// Set the layer for the operation
    pub fn layer(&mut self, layer: i32) -> &mut Self {
        self.operation.layer = layer;
        self
    }
    
    /// Build the final RenderOperation
    pub fn build(&self) -> RenderOperation {
        // Return a clone of the internal operation
        RenderOperation {
            element: self.operation.element.clone_renderable(),
            transform: self.operation.transform.clone(),
            styles: self.operation.styles.iter().map(|s| s.clone_style()).collect(),
            effects: self.operation.effects.iter().map(|e| e.clone_effect()).collect(),
            clip: self.operation.clip.clone(),
            id: self.operation.id,
            layer: self.operation.layer,
        }
    }
}

// Allow creating a builder directly from a renderable element
impl<R: Renderable + 'static> From<R> for RenderOperationBuilder {
    fn from(element: R) -> Self {
        Self::new(element)
    }
} 