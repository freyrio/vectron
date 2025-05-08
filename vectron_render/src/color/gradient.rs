// Gradient implementations for Vectron Render
//
// This module provides structures for defining color gradients.

use crate::color::Color;
use std::fmt;

/// Represents a color stop in a gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    /// The color at this stop
    pub color: Color,
    /// The position of this stop [0.0, 1.0]
    pub position: f32,
}

impl ColorStop {
    /// Creates a new color stop.
    ///
    /// * `color` - The color at this stop
    /// * `position` - The position of this stop [0.0, 1.0]
    pub fn new(color: Color, position: f32) -> Self {
        Self {
            color,
            position: position.clamp(0.0, 1.0),
        }
    }
}

/// The spread method used when a gradient's normal area doesn't cover the
/// area to be painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GradientSpreadMethod {
    /// The gradient is repeated (position loops back to 0)
    Repeat,
    /// The gradient is reflected (position goes back and forth)
    Reflect,
    /// The gradient is padded (position clamped to [0,1])
    Pad,
}

impl Default for GradientSpreadMethod {
    fn default() -> Self {
        Self::Pad
    }
}

/// A linear color gradient.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    /// The color stops in this gradient
    pub stops: Vec<ColorStop>,
    /// The spread method used
    pub spread_method: GradientSpreadMethod,
    /// Start point of the gradient (normalized [0,1] space)
    pub start: (f32, f32),
    /// End point of the gradient (normalized [0,1] space)
    pub end: (f32, f32),
}

impl LinearGradient {
    /// Creates a new linear gradient with the specified color stops.
    ///
    /// The gradient goes from left to right by default (0,0.5) to (1,0.5).
    pub fn new(stops: Vec<ColorStop>) -> Self {
        Self {
            stops,
            spread_method: GradientSpreadMethod::default(),
            start: (0.0, 0.5),
            end: (1.0, 0.5),
        }
    }
    
    /// Creates a horizontal linear gradient (left to right).
    pub fn horizontal(start_color: Color, end_color: Color) -> Self {
        Self {
            stops: vec![
                ColorStop::new(start_color, 0.0),
                ColorStop::new(end_color, 1.0),
            ],
            spread_method: GradientSpreadMethod::default(),
            start: (0.0, 0.5),
            end: (1.0, 0.5),
        }
    }
    
    /// Creates a vertical linear gradient (top to bottom).
    pub fn vertical(start_color: Color, end_color: Color) -> Self {
        Self {
            stops: vec![
                ColorStop::new(start_color, 0.0),
                ColorStop::new(end_color, 1.0),
            ],
            spread_method: GradientSpreadMethod::default(),
            start: (0.5, 0.0),
            end: (0.5, 1.0),
        }
    }
    
    /// Creates a linear gradient from one point to another.
    pub fn with_points(start: (f32, f32), end: (f32, f32), stops: Vec<ColorStop>) -> Self {
        Self {
            stops,
            spread_method: GradientSpreadMethod::default(),
            start,
            end,
        }
    }
    
    /// Sets the spread method for this gradient.
    pub fn with_spread_method(mut self, method: GradientSpreadMethod) -> Self {
        self.spread_method = method;
        self
    }
    
    /// Adds a color stop to this gradient.
    pub fn add_stop(&mut self, color: Color, position: f32) {
        self.stops.push(ColorStop::new(color, position));
        // Sort stops by position
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
    }
    
    /// Gets the color at the specified position along the gradient.
    ///
    /// Position is normalized [0, 1] but will be interpreted based on the
    /// gradient's spread method for values outside this range.
    pub fn color_at(&self, mut position: f32) -> Color {
        // Apply spread method for positions outside [0, 1]
        if position < 0.0 || position > 1.0 {
            position = match self.spread_method {
                GradientSpreadMethod::Repeat => position - position.floor(),
                GradientSpreadMethod::Reflect => {
                    let integer = position.floor() as i32;
                    let fraction = position - position.floor();
                    if integer % 2 == 0 {
                        fraction
                    } else {
                        1.0 - fraction
                    }
                }
                GradientSpreadMethod::Pad => position.clamp(0.0, 1.0),
            };
        }
        
        // Handle edge cases
        if self.stops.is_empty() {
            return Color::BLACK;
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        
        // Find the stops bracketing this position
        if position <= self.stops[0].position {
            return self.stops[0].color;
        }
        if position >= self.stops[self.stops.len() - 1].position {
            return self.stops[self.stops.len() - 1].color;
        }
        
        // Find the two stops to interpolate between
        for i in 0..self.stops.len() - 1 {
            let curr_stop = &self.stops[i];
            let next_stop = &self.stops[i + 1];
            
            if position >= curr_stop.position && position <= next_stop.position {
                // Calculate interpolation factor
                let t = if next_stop.position == curr_stop.position {
                    0.0
                } else {
                    (position - curr_stop.position) / (next_stop.position - curr_stop.position)
                };
                
                // Interpolate between the two colors
                return curr_stop.color.lerp(&next_stop.color, t);
            }
        }
        
        // Fallback (shouldn't happen)
        Color::BLACK
    }
}

impl fmt::Display for LinearGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinearGradient({} stops, {:?} to {:?})", 
               self.stops.len(), self.start, self.end)
    }
}

/// A radial color gradient.
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    /// The color stops in this gradient
    pub stops: Vec<ColorStop>,
    /// The spread method used
    pub spread_method: GradientSpreadMethod,
    /// Center point of the gradient (normalized [0,1] space)
    pub center: (f32, f32),
    /// Radius of the gradient in normalized units
    pub radius: f32,
    /// Optional focal point for off-center gradients
    pub focal_point: Option<(f32, f32)>,
}

impl RadialGradient {
    /// Creates a new radial gradient with the specified color stops.
    ///
    /// The gradient is centered by default with radius 0.5.
    pub fn new(stops: Vec<ColorStop>) -> Self {
        Self {
            stops,
            spread_method: GradientSpreadMethod::default(),
            center: (0.5, 0.5),
            radius: 0.5,
            focal_point: None,
        }
    }
    
    /// Creates a simple radial gradient with just two colors.
    pub fn simple(center_color: Color, edge_color: Color) -> Self {
        Self {
            stops: vec![
                ColorStop::new(center_color, 0.0),
                ColorStop::new(edge_color, 1.0),
            ],
            spread_method: GradientSpreadMethod::default(),
            center: (0.5, 0.5),
            radius: 0.5,
            focal_point: None,
        }
    }
    
    /// Sets the center point of the gradient.
    pub fn with_center(mut self, center: (f32, f32)) -> Self {
        self.center = center;
        self
    }
    
    /// Sets the radius of the gradient.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }
    
    /// Sets the focal point of the gradient (for off-center lighting effects).
    pub fn with_focal_point(mut self, focal_point: (f32, f32)) -> Self {
        self.focal_point = Some(focal_point);
        self
    }
    
    /// Sets the spread method for this gradient.
    pub fn with_spread_method(mut self, method: GradientSpreadMethod) -> Self {
        self.spread_method = method;
        self
    }
    
    /// Adds a color stop to this gradient.
    pub fn add_stop(&mut self, color: Color, position: f32) {
        self.stops.push(ColorStop::new(color, position));
        // Sort stops by position
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
    }
    
    /// Gets the color at the specified position, if given as a normalized distance from the center.
    ///
    /// Distance is normalized [0, 1] where 1.0 represents the edge of the radius,
    /// but will be interpreted based on the gradient's spread method for values outside this range.
    pub fn color_at_distance(&self, mut distance: f32) -> Color {
        // Apply spread method for positions outside [0, 1]
        if distance < 0.0 || distance > 1.0 {
            distance = match self.spread_method {
                GradientSpreadMethod::Repeat => distance - distance.floor(),
                GradientSpreadMethod::Reflect => {
                    let integer = distance.floor() as i32;
                    let fraction = distance - distance.floor();
                    if integer % 2 == 0 {
                        fraction
                    } else {
                        1.0 - fraction
                    }
                }
                GradientSpreadMethod::Pad => distance.clamp(0.0, 1.0),
            };
        }
        
        // Handle edge cases
        if self.stops.is_empty() {
            return Color::BLACK;
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        
        // Find the stops bracketing this position
        if distance <= self.stops[0].position {
            return self.stops[0].color;
        }
        if distance >= self.stops[self.stops.len() - 1].position {
            return self.stops[self.stops.len() - 1].color;
        }
        
        // Find the two stops to interpolate between
        for i in 0..self.stops.len() - 1 {
            let curr_stop = &self.stops[i];
            let next_stop = &self.stops[i + 1];
            
            if distance >= curr_stop.position && distance <= next_stop.position {
                // Calculate interpolation factor
                let t = if next_stop.position == curr_stop.position {
                    0.0
                } else {
                    (distance - curr_stop.position) / (next_stop.position - curr_stop.position)
                };
                
                // Interpolate between the two colors
                return curr_stop.color.lerp(&next_stop.color, t);
            }
        }
        
        // Fallback (shouldn't happen)
        Color::BLACK
    }
    
    /// Gets the color at the specified point in normalized coordinates.
    pub fn color_at_point(&self, point: (f32, f32)) -> Color {
        let (x, y) = point;
        let (cx, cy) = self.center;
        
        // If we have a focal point, calculate distance differently
        if let Some((fx, fy)) = self.focal_point {
            // This is a simplified approximation for focal gradients
            // A proper implementation would calculate based on angle from focal to point
            let center_to_focal_dist = ((cx - fx).powi(2) + (cy - fy).powi(2)).sqrt();
            let focal_to_point_dist = ((fx - x).powi(2) + (fy - y).powi(2)).sqrt();
            let center_to_point_dist = ((cx - x).powi(2) + (cy - y).powi(2)).sqrt();
            
            // Normalize the distance
            // We use a weighted distance based on the angle from focal to point
            let normalized_dist = if center_to_focal_dist < 0.001 {
                // If focal and center are nearly the same, use simple distance
                center_to_point_dist / self.radius
            } else {
                // Otherwise, use a weighted distance based on angle
                focal_to_point_dist / self.radius
            };
            
            return self.color_at_distance(normalized_dist);
        }
        
        // Simple case - just use distance from center
        let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        let normalized_dist = dist / self.radius;
        
        self.color_at_distance(normalized_dist)
    }
}

impl fmt::Display for RadialGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RadialGradient({} stops, center={:?}, radius={})", 
               self.stops.len(), self.center, self.radius)
    }
}

/// A conic color gradient (also known as an angular gradient).
#[derive(Debug, Clone, PartialEq)]
pub struct ConicGradient {
    /// The color stops in this gradient
    pub stops: Vec<ColorStop>,
    /// Center point of the gradient (normalized [0,1] space)
    pub center: (f32, f32),
    /// Angle offset in degrees [0, 360)
    pub angle_offset: f32,
}

impl ConicGradient {
    /// Creates a new conic gradient with the specified color stops.
    ///
    /// The gradient is centered by default.
    pub fn new(stops: Vec<ColorStop>) -> Self {
        Self {
            stops,
            center: (0.5, 0.5),
            angle_offset: 0.0,
        }
    }
    
    /// Creates a simple conic gradient with evenly spaced colors.
    pub fn from_colors(colors: &[Color]) -> Self {
        if colors.is_empty() {
            return Self::new(vec![ColorStop::new(Color::BLACK, 0.0)]);
        }
        
        let mut stops = Vec::with_capacity(colors.len());
        let step = 1.0 / colors.len() as f32;
        
        for (i, color) in colors.iter().enumerate() {
            stops.push(ColorStop::new(*color, i as f32 * step));
        }
        
        // Add the first color at position 1.0 to complete the circle
        if colors.len() > 1 {
            stops.push(ColorStop::new(colors[0], 1.0));
        }
        
        Self::new(stops)
    }
    
    /// Sets the center point of the gradient.
    pub fn with_center(mut self, center: (f32, f32)) -> Self {
        self.center = center;
        self
    }
    
    /// Sets the angle offset of the gradient in degrees.
    pub fn with_angle_offset(mut self, angle_degrees: f32) -> Self {
        self.angle_offset = angle_degrees.rem_euclid(360.0);
        self
    }
    
    /// Adds a color stop to this gradient.
    pub fn add_stop(&mut self, color: Color, position: f32) {
        self.stops.push(ColorStop::new(color, position));
        // Sort stops by position
        self.stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
    }
    
    /// Gets the color at the specified angle in degrees.
    ///
    /// Angle is in degrees [0, 360) and will be normalized.
    pub fn color_at_angle(&self, mut angle_degrees: f32) -> Color {
        // Normalize angle to [0, 360)
        angle_degrees = (angle_degrees - self.angle_offset).rem_euclid(360.0);
        
        // Convert to position in [0, 1]
        let position = angle_degrees / 360.0;
        
        // Handle edge cases
        if self.stops.is_empty() {
            return Color::BLACK;
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        
        // Find the stops bracketing this position, with special handling for the wraparound
        // at 1.0/0.0
        if position <= self.stops[0].position {
            // If before the first stop, wrap around (last stop to first stop)
            let last_stop = &self.stops[self.stops.len() - 1];
            let first_stop = &self.stops[0];
            
            // If the last stop is at 1.0 and the first at 0.0, interpolate between them
            if last_stop.position == 1.0 && first_stop.position == 0.0 {
                let t = position;
                return last_stop.color.lerp(&first_stop.color, t);
            }
            
            // Otherwise just return the first stop
            return first_stop.color;
        }
        
        if position >= self.stops[self.stops.len() - 1].position {
            // If after the last stop, wrap around (last stop to first stop)
            let last_stop = &self.stops[self.stops.len() - 1];
            let first_stop = &self.stops[0];
            
            // If the last stop is at 1.0 and the first at 0.0, interpolate between them
            if last_stop.position == 1.0 && first_stop.position == 0.0 {
                let t = (position - last_stop.position) / (1.0 + first_stop.position - last_stop.position);
                return last_stop.color.lerp(&first_stop.color, t);
            }
            
            // Otherwise just return the last stop
            return last_stop.color;
        }
        
        // Find the two stops to interpolate between
        for i in 0..self.stops.len() - 1 {
            let curr_stop = &self.stops[i];
            let next_stop = &self.stops[i + 1];
            
            if position >= curr_stop.position && position <= next_stop.position {
                // Calculate interpolation factor
                let t = if next_stop.position == curr_stop.position {
                    0.0
                } else {
                    (position - curr_stop.position) / (next_stop.position - curr_stop.position)
                };
                
                // Interpolate between the two colors
                return curr_stop.color.lerp(&next_stop.color, t);
            }
        }
        
        // Fallback (shouldn't happen)
        Color::BLACK
    }
    
    /// Gets the color at the specified point in normalized coordinates.
    pub fn color_at_point(&self, point: (f32, f32)) -> Color {
        let (x, y) = point;
        let (cx, cy) = self.center;
        
        // Calculate angle from center to point
        let dx = x - cx;
        let dy = cy - y; // Flip y to match standard angle orientation
        
        // Calculate angle in degrees
        let angle_degrees = dy.atan2(dx).to_degrees().rem_euclid(360.0);
        
        self.color_at_angle(angle_degrees)
    }
}

impl fmt::Display for ConicGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConicGradient({} stops, center={:?}, offset={}°)", 
               self.stops.len(), self.center, self.angle_offset)
    }
}

/// Represents any type of gradient.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    /// A linear gradient
    Linear(LinearGradient),
    /// A radial gradient
    Radial(RadialGradient),
    /// A conic gradient
    Conic(ConicGradient),
}

impl Gradient {
    /// Gets the color at the specified point in normalized coordinates.
    pub fn color_at_point(&self, point: (f32, f32)) -> Color {
        match self {
            Gradient::Linear(gradient) => {
                let (x, y) = point;
                let (x1, y1) = gradient.start;
                let (x2, y2) = gradient.end;
                
                // Calculate the projection of point onto the line from start to end
                let dx = x2 - x1;
                let dy = y2 - y1;
                let len_squared = dx * dx + dy * dy;
                
                if len_squared == 0.0 {
                    // Start and end are the same point, use the first color
                    return gradient.stops.first().map_or(Color::BLACK, |stop| stop.color);
                }
                
                // Calculate t parameter along the line
                let t = ((x - x1) * dx + (y - y1) * dy) / len_squared;
                
                gradient.color_at(t)
            },
            Gradient::Radial(gradient) => gradient.color_at_point(point),
            Gradient::Conic(gradient) => gradient.color_at_point(point),
        }
    }
} 