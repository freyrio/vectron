use crate::math::vector::{Vec2, Vec3};
use crate::math::utils::functions;

/// A cubic Bezier curve with 4 control points.
#[derive(Debug, Clone, Copy)]
pub struct CubicBezier<T> {
    pub p0: T,
    pub p1: T,
    pub p2: T,
    pub p3: T,
}

impl<T> CubicBezier<T> 
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>
{
    /// Creates a new cubic Bezier curve with the given control points.
    #[inline]
    pub fn new(p0: T, p1: T, p2: T, p3: T) -> Self {
        Self { p0, p1, p2, p3 }
    }
    
    /// Evaluates the Bezier curve at the given parameter t in [0, 1].
    #[inline]
    pub fn evaluate(&self, t: f32) -> T {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        // Bernstein basis polynomials
        let b0 = mt3;
        let b1 = 3.0 * mt2 * t;
        let b2 = 3.0 * mt * t2;
        let b3 = t3;
        
        self.p0 * b0 + self.p1 * b1 + self.p2 * b2 + self.p3 * b3
    }
    
    /// Evaluates the derivative of the Bezier curve at the given parameter t in [0, 1].
    #[inline]
    pub fn derivative(&self, t: f32) -> T {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        
        // Derivatives of Bernstein basis polynomials
        let d0 = -3.0 * mt2;
        let d1 = 3.0 * mt2 - 6.0 * mt * t;
        let d2 = 6.0 * mt * t - 3.0 * t2;
        let d3 = 3.0 * t2;
        
        self.p0 * d0 + self.p1 * d1 + self.p2 * d2 + self.p3 * d3
    }
}

/// A quadratic Bezier curve with 3 control points.
#[derive(Debug, Clone, Copy)]
pub struct QuadraticBezier<T> {
    pub p0: T,
    pub p1: T,
    pub p2: T,
}

impl<T> QuadraticBezier<T> 
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T> + std::ops::Sub<Output = T>
{
    /// Creates a new quadratic Bezier curve with the given control points.
    #[inline]
    pub fn new(p0: T, p1: T, p2: T) -> Self {
        Self { p0, p1, p2 }
    }
    
    /// Evaluates the Bezier curve at the given parameter t in [0, 1].
    #[inline]
    pub fn evaluate(&self, t: f32) -> T {
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let t2 = t * t;
        
        // Bernstein basis polynomials
        let b0 = mt2;
        let b1 = 2.0 * mt * t;
        let b2 = t2;
        
        self.p0 * b0 + self.p1 * b1 + self.p2 * b2
    }
    
    /// Evaluates the derivative of the Bezier curve at the given parameter t in [0, 1].
    #[inline]
    pub fn derivative(&self, t: f32) -> T {
        let mt = 1.0 - t;
        
        // Derivatives of Bernstein basis polynomials
        let d0 = -2.0 * mt;
        let d1 = 2.0 * mt - 2.0 * t;
        let d2 = 2.0 * t;
        
        self.p0 * d0 + self.p1 * d1 + self.p2 * d2
    }
    
    /// Converts this quadratic Bezier curve to a cubic Bezier curve.
    #[inline]
    pub fn to_cubic(&self) -> CubicBezier<T> {
        let p0 = self.p0;
        let p1 = self.p0 + (self.p1 - self.p0) * (2.0 / 3.0);
        let p2 = self.p2 + (self.p1 - self.p2) * (2.0 / 3.0);
        let p3 = self.p2;
        
        CubicBezier::new(p0, p1, p2, p3)
    }
}

/// A Bezier spline composed of multiple connected Bezier curves.
#[derive(Debug, Clone)]
pub struct BezierSpline<T: Copy> {
    /// Control points defining the Bezier curves.
    /// Each segment uses 4 control points (cubic Bezier).
    /// Adjacent segments share their end/start control points.
    control_points: Vec<T>,
    
    /// Whether the spline is closed (last point connects to first point).
    closed: bool,
}

impl<T: Copy + std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>> BezierSpline<T> {
    /// Creates a new Bezier spline from control points.
    ///
    /// For open splines, the number of control points must be 3n+1 (where n is the number of segments).
    /// For closed splines, the number of control points must be 3n (where n is the number of segments).
    ///
    /// # Panics
    ///
    /// Panics if the number of control points is invalid for the specified closed state.
    #[inline]
    pub fn new(control_points: Vec<T>, closed: bool) -> Self {
        if closed {
            assert!(control_points.len() >= 3 && control_points.len() % 3 == 0,
                   "Closed spline must have 3n control points");
        } else {
            assert!(control_points.len() >= 4 && (control_points.len() - 1) % 3 == 0,
                   "Open spline must have 3n+1 control points");
        }
        
        Self { control_points, closed }
    }
    
    /// Returns the number of segments in the spline.
    #[inline]
    pub fn segment_count(&self) -> usize {
        if self.closed {
            self.control_points.len() / 3
        } else {
            (self.control_points.len() - 1) / 3
        }
    }
    
    /// Evaluates the spline at the given parameter t in [0, 1],
    /// where t=0 is the start of the spline and t=1 is the end.
    #[inline]
    pub fn evaluate(&self, t: f32) -> T {
        let segment_count = self.segment_count();
        
        // Handle edge cases
        if segment_count == 0 {
            return self.control_points[0];
        }
        
        // Clamp t to [0, 1]
        let t = functions::clamp(t, 0.0, 1.0);
        
        // For t=1, return the end point
        if t == 1.0 {
            if self.closed {
                return self.control_points[0];
            } else {
                return self.control_points[self.control_points.len() - 1];
            }
        }
        
        // Determine which segment t falls in
        let segment_t = t * segment_count as f32;
        let segment_index = segment_t.floor() as usize;
        let local_t = segment_t - segment_index as f32;
        
        self.evaluate_segment(segment_index, local_t)
    }
    
    /// Evaluates a specific segment at the given local parameter t in [0, 1].
    #[inline]
    fn evaluate_segment(&self, segment_index: usize, local_t: f32) -> T {
        assert!(segment_index < self.segment_count(), "Segment index out of bounds");
        
        // Get the control points for this segment
        let (p0, p1, p2, p3) = self.get_segment_control_points(segment_index);
        
        // Evaluate the cubic Bezier curve
        let bezier = CubicBezier::new(p0, p1, p2, p3);
        bezier.evaluate(local_t)
    }
    
    /// Gets the control points for a specific segment.
    #[inline]
    fn get_segment_control_points(&self, segment_index: usize) -> (T, T, T, T) {
        if self.closed {
            let n = self.control_points.len();
            let i = (segment_index * 3) % n;
            
            (
                self.control_points[i],
                self.control_points[(i + 1) % n],
                self.control_points[(i + 2) % n],
                self.control_points[(i + 3) % n],
            )
        } else {
            let i = segment_index * 3;
            
            (
                self.control_points[i],
                self.control_points[i + 1],
                self.control_points[i + 2],
                self.control_points[i + 3],
            )
        }
    }
    
    /// Returns whether the spline is closed.
    #[inline]
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    
    /// Returns a reference to the control points.
    #[inline]
    pub fn control_points(&self) -> &[T] {
        &self.control_points
    }
}

/// Implementation for 2D Bezier curves.
pub type CubicBezier2D = CubicBezier<Vec2>;
pub type QuadraticBezier2D = QuadraticBezier<Vec2>;

/// Implementation for 3D Bezier curves.
pub type CubicBezier3D = CubicBezier<Vec3>;
pub type QuadraticBezier3D = QuadraticBezier<Vec3>;

/// Implementation for 2D Bezier splines.
pub type BezierSpline2D = BezierSpline<Vec2>;

/// Implementation for 3D Bezier splines.
pub type BezierSpline3D = BezierSpline<Vec3>; 