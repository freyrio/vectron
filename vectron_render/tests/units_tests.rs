// Unit tests for the Vectron Render units module

use vectron_render::units::*;
use vectron_render::units::physical::*;
use vectron_render::units::logical::*;
use vectron_render::units::context::*;

#[test]
fn test_unit_creation() {
    // Physical units
    let pixels = px(100.0);
    assert_eq!(pixels.kind, UnitKind::Pixel);
    assert_eq!(pixels.value, 100.0);
    
    let points = pt(12.0);
    assert_eq!(points.kind, UnitKind::Point);
    assert_eq!(points.value, 12.0);
    
    // Logical units
    let ems = em(2.5);
    assert_eq!(ems.kind, UnitKind::Em);
    assert_eq!(ems.value, 2.5);
    
    let percent = percent(50.0);
    assert_eq!(percent.kind, UnitKind::Percent);
    assert_eq!(percent.value, 50.0);
    
    let viewport_width = vw(100.0);
    assert_eq!(viewport_width.kind, UnitKind::ViewportWidth);
    assert_eq!(viewport_width.value, 100.0);
}

#[test]
fn test_unit_arithmetic() {
    let a = px(10.0);
    let b = px(20.0);
    
    let sum = a + b;
    assert_eq!(sum.value, 30.0);
    assert_eq!(sum.kind, UnitKind::Pixel);
    
    let diff = b - a;
    assert_eq!(diff.value, 10.0);
    assert_eq!(diff.kind, UnitKind::Pixel);
    
    let scaled = a * 2.0;
    assert_eq!(scaled.value, 20.0);
    assert_eq!(scaled.kind, UnitKind::Pixel);
    
    let divided = b / 2.0;
    assert_eq!(divided.value, 10.0);
    assert_eq!(divided.kind, UnitKind::Pixel);
    
    let negated = -a;
    assert_eq!(negated.value, -10.0);
    assert_eq!(negated.kind, UnitKind::Pixel);
}

#[test]
#[should_panic(expected = "Cannot add units of different kinds")]
fn test_incompatible_unit_addition() {
    let _sum = px(10.0) + pt(10.0);
}

#[test]
fn test_unit_context_resolution() {
    let context = UnitContext::new(
        1.0,                // device_pixel_ratio
        96.0,               // ppi
        (1000.0, 500.0),    // viewport_size
        (200.0, 100.0),     // parent_size
        16.0,               // font_size
        14.0,               // root_font_size
    );
    
    // Test physical units
    assert_eq!(context.resolve(px(100.0)), 100.0);
    assert_eq!(context.resolve(pt(72.0)), 96.0); // 72pt = 1 inch = 96px at 96ppi
    
    // Test logical units
    assert_eq!(context.resolve(em(1.0)), 16.0);
    assert_eq!(context.resolve(rem(1.0)), 14.0);
    assert_eq!(context.resolve(vw(10.0)), 100.0); // 10% of 1000px
    assert_eq!(context.resolve(vh(10.0)), 50.0);  // 10% of 500px
    
    // Test percent (average of parent dimensions)
    let expected_percent = 0.01 * 50.0 * ((200.0 + 100.0) / 2.0);
    assert_eq!(context.resolve(percent(50.0)), expected_percent);
}

#[test]
fn test_size_resolution() {
    let context = UnitContext::default();
    
    let size = Size::new(px(100.0), px(200.0));
    let (width, height) = context.resolve_size(size);
    
    assert_eq!(width, 100.0);
    assert_eq!(height, 200.0);
}

#[test]
fn test_unit_resolution_trait() {
    let context = UnitContext::with_viewport_size(1000.0, 800.0);
    
    let vw_unit = vw(10.0);
    assert_eq!(vw_unit.resolve(&context), 100.0);
    
    // Default context has 800x600 viewport
    assert_eq!(vw_unit.to_pixels(), 80.0);
} 