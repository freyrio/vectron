//! Units module for Vectron Render
//!
//! This module provides a flexible unit system for defining positions and sizes,
//! enabling responsive layouts.
//!
//! # Overview
//!
//! The units system allows for defining measurements in various units:
//!
//! - **Absolute units**: `px`, `pt`, `in`, `cm`, `mm`
//! - **Relative units**: `em`, `rem`, `%`, `vw`, `vh`, `vmin`, `vmax`
//!
//! These units can be combined with a `UnitContext` to resolve to actual pixel
//! values at runtime based on the current viewport, device, and font settings.
//!
//! # Examples
//!
//! ## Creating units
//!
//! ```
//! use vectron_render::units::*;
//! use vectron_render::units::physical::*;
//! use vectron_render::units::logical::*;
//!
//! // Create various unit values
//! let width = px(100.0);       // 100 pixels
//! let height = vw(50.0);       // 50% of viewport width
//! let padding = em(1.5);       // 1.5 times the current font size
//! let margin = percent(10.0);  // 10% of parent container
//! let font_size = pt(16.0);    // 16 points
//! ```
//!
//! ## Resolving units to pixels
//!
//! ```
//! use vectron_render::units::*;
//! use vectron_render::units::physical::*;
//! use vectron_render::units::logical::*;
//! use vectron_render::units::context::*;
//!
//! // Create some units
//! let width = vw(20.0);  // 20% of viewport width
//! let height = vh(30.0); // 30% of viewport height
//!
//! // Create a context with specific properties
//! let context = UnitContext::with_viewport_size(1000.0, 800.0);
//!
//! // Resolve units to pixels
//! let width_px = width.resolve(&context);  // 200 pixels (20% of 1000)
//! let height_px = height.resolve(&context); // 240 pixels (30% of 800)
//! ```
//!
//! ## Using unit arithmetic
//!
//! ```
//! use vectron_render::units::*;
//! use vectron_render::units::physical::*;
//!
//! // Units of the same kind can be combined
//! let a = px(10.0);
//! let b = px(20.0);
//!
//! let sum = a + b;       // 30px
//! let product = a * 2.0; // 20px
//! let diff = b - a;      // 10px
//! let half = b / 2.0;    // 10px
//! ```
//!

mod types;
mod logical;
mod physical;
mod context;

pub use types::{Unit, Size};
pub use logical::*;
pub use physical::*;
pub use context::*; 