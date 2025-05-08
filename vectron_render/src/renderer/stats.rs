// Render statistics implementation for Vectron Render
//
// This module provides performance tracking for rendering operations.

use std::time::{Instant, Duration};

/// The RenderStats struct tracks performance metrics for rendering operations.
#[derive(Clone, Debug)]
pub struct RenderStats {
    /// Time when the current frame started
    frame_start_time: Option<Instant>,
    
    /// Duration of the last completed frame
    last_frame_time: Duration,
    
    /// Number of frames rendered
    frame_count: u64,
    
    /// Running average of frame times (in seconds)
    average_frame_time: f32,
    
    /// Number of draw calls in the current frame
    draw_calls: u32,
    
    /// Number of render operations in the current frame
    operation_count: u32,
    
    /// Number of triangles rendered in the current frame
    triangle_count: u32,
    
    /// Number of vertices processed in the current frame
    vertex_count: u32,
}

impl RenderStats {
    /// Create a new RenderStats instance
    pub fn new() -> Self {
        Self {
            frame_start_time: None,
            last_frame_time: Duration::from_secs(0),
            frame_count: 0,
            average_frame_time: 0.0,
            draw_calls: 0,
            operation_count: 0,
            triangle_count: 0,
            vertex_count: 0,
        }
    }
    
    /// Begin tracking a new frame
    pub fn begin_frame(&mut self) {
        self.frame_start_time = Some(Instant::now());
        self.draw_calls = 0;
        self.operation_count = 0;
        self.triangle_count = 0;
        self.vertex_count = 0;
    }
    
    /// End tracking the current frame
    pub fn end_frame(&mut self) {
        if let Some(start_time) = self.frame_start_time {
            // Calculate frame time
            self.last_frame_time = start_time.elapsed();
            
            // Update average frame time (with exponential smoothing)
            let current_time_secs = self.last_frame_time.as_secs_f32();
            
            if self.frame_count == 0 {
                self.average_frame_time = current_time_secs;
            } else {
                // Use a smoothing factor of 0.05 (adjustable)
                const SMOOTHING: f32 = 0.05;
                self.average_frame_time = self.average_frame_time * (1.0 - SMOOTHING) + 
                                          current_time_secs * SMOOTHING;
            }
            
            // Increment frame count
            self.frame_count += 1;
            
            // Reset frame start time
            self.frame_start_time = None;
        }
    }
    
    /// Record a draw call with the given number of triangles and vertices
    pub fn record_draw_call(&mut self, triangles: u32, vertices: u32) {
        self.draw_calls += 1;
        self.triangle_count += triangles;
        self.vertex_count += vertices;
    }
    
    /// Increment the operation count
    pub fn increment_operations(&mut self) {
        self.operation_count += 1;
    }
    
    /// Get the duration of the last completed frame
    pub fn last_frame_time(&self) -> Duration {
        self.last_frame_time
    }
    
    /// Get the average frame time in seconds
    pub fn average_frame_time(&self) -> f32 {
        self.average_frame_time
    }
    
    /// Get the average frames per second
    pub fn fps(&self) -> f32 {
        if self.average_frame_time > 0.0 {
            1.0 / self.average_frame_time
        } else {
            0.0
        }
    }
    
    /// Get the number of draw calls in the current/last frame
    pub fn draw_calls(&self) -> u32 {
        self.draw_calls
    }
    
    /// Get the number of render operations in the current/last frame
    pub fn operation_count(&self) -> u32 {
        self.operation_count
    }
    
    /// Get the number of triangles rendered in the current/last frame
    pub fn triangle_count(&self) -> u32 {
        self.triangle_count
    }
    
    /// Get the number of vertices processed in the current/last frame
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    
    /// Get the total number of frames rendered
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        self.frame_start_time = None;
        self.last_frame_time = Duration::from_secs(0);
        self.frame_count = 0;
        self.average_frame_time = 0.0;
        self.draw_calls = 0;
        self.operation_count = 0;
        self.triangle_count = 0;
        self.vertex_count = 0;
    }
} 