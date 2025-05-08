// Material system for Vectron Render
//
// This module provides structures for defining materials used in 3D rendering.

use std::any::Any;
use std::fmt;
use crate::color::Color;
use crate::core::traits::Style;
use crate::core::{TextureHandle, GeometryHandle, Renderer, RenderError};
use crate::style::paint::Paint;

/// The base material type for 3D rendering
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Diffuse color or texture
    pub diffuse: Paint,
    /// Specular color or texture
    pub specular: Option<Paint>,
    /// Normal map texture
    pub normal_map: Option<TextureHandle>,
    /// Roughness value [0.0, 1.0] or texture
    pub roughness: Option<Paint>,
    /// Metallic value [0.0, 1.0] or texture
    pub metallic: Option<Paint>,
    /// Ambient occlusion value [0.0, 1.0] or texture
    pub ambient_occlusion: Option<Paint>,
    /// Emissive color or texture
    pub emission: Option<Paint>,
    /// The z-index (rendering order)
    pub z_index: i32,
    /// Whether this material is transparent
    pub is_transparent: bool,
    /// Alpha cutoff value for masking
    pub alpha_cutoff: Option<f32>,
}

impl Material {
    /// Create a new material with the specified diffuse color
    pub fn new(diffuse: impl Into<Paint>) -> Self {
        Self {
            diffuse: diffuse.into(),
            specular: None,
            normal_map: None,
            roughness: None, 
            metallic: None,
            ambient_occlusion: None,
            emission: None,
            z_index: 0,
            is_transparent: false,
            alpha_cutoff: None,
        }
    }
    
    /// Create a basic physically-based rendering (PBR) material
    pub fn pbr(
        base_color: impl Into<Paint>, 
        metallic: f32, 
        roughness: f32
    ) -> Self {
        let base_color = base_color.into();
        let metallic_paint = Paint::Solid(Color::new(metallic, metallic, metallic, 1.0));
        let roughness_paint = Paint::Solid(Color::new(roughness, roughness, roughness, 1.0));
        
        Self {
            diffuse: base_color,
            specular: None,
            normal_map: None,
            roughness: Some(roughness_paint),
            metallic: Some(metallic_paint),
            ambient_occlusion: None,
            emission: None,
            z_index: 0,
            is_transparent: false,
            alpha_cutoff: None,
        }
    }
    
    /// Add a specular component to this material
    pub fn with_specular(mut self, specular: impl Into<Paint>) -> Self {
        self.specular = Some(specular.into());
        self
    }
    
    /// Add a normal map to this material
    pub fn with_normal_map(mut self, normal_map: TextureHandle) -> Self {
        self.normal_map = Some(normal_map);
        self
    }
    
    /// Set the roughness for this material
    pub fn with_roughness(mut self, roughness: impl Into<Paint>) -> Self {
        self.roughness = Some(roughness.into());
        self
    }
    
    /// Set the metallic factor for this material
    pub fn with_metallic(mut self, metallic: impl Into<Paint>) -> Self {
        self.metallic = Some(metallic.into());
        self
    }
    
    /// Set the ambient occlusion for this material
    pub fn with_ambient_occlusion(mut self, ao: impl Into<Paint>) -> Self {
        self.ambient_occlusion = Some(ao.into());
        self
    }
    
    /// Add emission to this material
    pub fn with_emission(mut self, emission: impl Into<Paint>) -> Self {
        self.emission = Some(emission.into());
        self
    }
    
    /// Set whether this material is transparent
    pub fn with_transparency(mut self, is_transparent: bool) -> Self {
        self.is_transparent = is_transparent;
        self
    }
    
    /// Set the alpha cutoff for alpha masking
    pub fn with_alpha_cutoff(mut self, cutoff: f32) -> Self {
        self.alpha_cutoff = Some(cutoff);
        self
    }
    
    /// Set the z-index for this material
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::WHITE)
    }
}

impl Style for Material {
    fn apply(&self, renderer: &mut dyn Renderer, geometry_handle: GeometryHandle) -> Result<(), RenderError> {
        renderer.apply_style(self, geometry_handle)
    }
    
    fn is_batchable(&self) -> bool {
        // PBR materials generally require more state changes
        // and are less batchable than simple 2D styles
        false
    }
    
    fn z_index(&self) -> i32 {
        self.z_index
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn clone_style(&self) -> Box<dyn Style> {
        Box::new(self.clone())
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material(diffuse: {})", self.diffuse)
    }
} 