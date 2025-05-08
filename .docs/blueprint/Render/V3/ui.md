## UI Core Project Structure

```
render_core/
├── ...existing structure...
└── ui/
    ├── mod.rs                  # UI module exports
    ├── core/
    │   ├── mod.rs              # Core UI module exports
    │   ├── element.rs          # UIElement trait and UIObject2D
    │   ├── container.rs        # Container trait and implementations
    │   ├── layout/
    │   │   ├── mod.rs
    │   │   ├── box.rs          # FlexBox layout implementation
    │   │   ├── grid.rs         # Grid layout implementation 
    │   │   └── stack.rs        # Stack layout implementation
    │   ├── styling/
    │   │   ├── mod.rs          # Style module
    │   │   ├── cascade.rs      # Style cascade system
    │   │   ├── selector.rs     # Style selectors
    │   │   └── properties.rs   # UI-specific style properties
    │   └── transform.rs        # UI transform hierarchy
    │
    ├── event/
    │   ├── mod.rs
    │   ├── types.rs            # Event types (Click, Hover, etc.)
    │   ├── listener.rs         # Event listener trait
    │   ├── emitter.rs          # Event emitter trait
    │   ├── propagation.rs      # Event capture/bubble logic
    │   └── dispatcher.rs       # Main event dispatcher
    │
    ├── interaction/
    │   ├── mod.rs
    │   ├── pointer.rs          # Pointer interactions
    │   ├── keyboard.rs         # Keyboard interactions
    │   ├── touch.rs            # Touch interactions
    │   ├── focus.rs            # Focus management
    │   └── draggable.rs        # Drag & drop system
    │
    ├── text/
    │   ├── mod.rs
    │   ├── cursor.rs           # Text cursor handling
    │   ├── selection.rs        # Text selection
    │   └── edit.rs             # Text editing operations
    │
    ├── animation/
    │   ├── mod.rs
    │   ├── tween.rs            # Value tweening
    │   ├── timeline.rs         # Animation timeline
    │   └── transitions.rs      # State transitions
    │
    ├── accessibility/
    │   ├── mod.rs
    │   ├── roles.rs            # ARIA roles
    │   └── reader.rs           # Screen reader support
    │
    └── primitives/
        ├── mod.rs
        ├── pane.rs             # Basic container
        ├── stack.rs            # Stack panel
        ├── scroll.rs           # Scroll container
        ├── clip.rs             # Clipping container
        └── canvas.rs           # Free-form drawing surface
```

## Core UI Elements and Integration with Render Engine

### UIElement and UIObject2D Traits

```rust
pub trait UIElement: Renderable {
    /// Get the parent element if any
    fn parent(&self) -> Option<&dyn UIElement>;
    
    /// Get the children elements
    fn children(&self) -> &[Box<dyn UIElement>];
    
    /// Get mutable access to children
    fn children_mut(&mut self) -> &mut Vec<Box<dyn UIElement>>;
    
    /// Add a child element
    fn add_child(&mut self, child: Box<dyn UIElement>);
    
    /// Remove a child element
    fn remove_child(&mut self, index: usize) -> Option<Box<dyn UIElement>>;
    
    /// Get this element's local bounds
    fn local_bounds(&self) -> BoundingVolume;
    
    /// Calculate bounds in parent space
    fn parent_bounds(&self) -> BoundingVolume;
    
    /// Get the accumulated transform to this element
    fn accumulated_transform(&self) -> TransformInSpace;
    
    /// Get the content transform (for scrollable elements)
    fn content_transform(&self) -> TransformInSpace;
    
    /// Check if point is inside this element
    fn hit_test(&self, point: &PointInSpace) -> bool;
    
    /// Get the effective style for this element
    fn effective_style(&self) -> &UIStyle;
    
    /// Set a local style override
    fn set_style(&mut self, style: UIStyle);
    
    /// Handle an input event
    fn handle_event(&mut self, event: &UIEvent) -> EventResult;
    
    /// Update layout if needed
    fn update_layout(&mut self, constraints: &LayoutConstraints);
    
    /// Mark this element as needing layout
    fn invalidate_layout(&mut self);
    
    /// Mark this element as needing rendering
    fn invalidate_render(&mut self);
    
    /// Get element's accessibility properties
    fn accessibility(&self) -> &AccessibilityProps;
    
    /// Get element's unique ID (for event targeting)
    fn id(&self) -> UIElementId;
    
    /// Clone this element
    fn clone_element(&self) -> Box<dyn UIElement>;
}

pub trait UIObject2D: UIElement + Object2D {
    /// Get the 2D local rect including children
    fn content_rect(&self) -> Rect;
    
    /// Check if this element or its children contain a point
    fn hit_test_recursive(&self, point: &Point2D, transform: &Transform2D) -> Option<UIElementId>;
    
    /// Convert a point from screen space to local space
    fn screen_to_local(&self, point: &Point2D) -> Point2D;
    
    /// Convert a point from local space to screen space
    fn local_to_screen(&self, point: &Point2D) -> Point2D;
}
```

### Integration with Core Render Engine

```rust
impl<T: UIElement + Tessellable> Renderable for T {
    // Bridge implementation from UIElement to Renderable
    fn dimensionality(&self) -> Dimensionality {
        Dimensionality::D2 // UI elements are 2D
    }
    
    fn origin(&self) -> PointInSpace {
        self.local_bounds().center()
    }
    
    // ... other Renderable implementations
}

// Create RenderOperations for UI elements
impl UIElement {
    pub fn create_render_operation(&self) -> RenderOperation {
        // Create base operation with this element
        let mut op = RenderOperation::new(self.clone_renderable());
        
        // Apply accumulated transform
        op = op.with_transform(self.accumulated_transform());
        
        // Apply styles
        for style in self.effective_style().styles() {
            op = op.with_style(style.clone());
        }
        
        // Apply effects
        for effect in self.effective_style().effects() {
            op = op.with_effect(effect.clone());
        }
        
        // Apply clipping
        if self.effective_style().clip() {
            op = op.with_clip(self.local_bounds());
        }
        
        op
    }
}
```

## Hierarchical Styling and Cascading

```rust
pub struct UIStyle {
    // Direct style properties
    properties: HashMap<StyleProperty, StyleValue>,
    
    // Selector-based styles
    selectors: Vec<(StyleSelector, HashMap<StyleProperty, StyleValue>)>,
    
    // State-specific styles
    states: HashMap<UIElementState, HashMap<StyleProperty, StyleValue>>,
    
    // Animation transitions
    transitions: HashMap<StyleProperty, TransitionSettings>,
}

impl UIStyle {
    // Calculate effective style value by cascading through hierarchy
    pub fn resolve_property(&self, prop: StyleProperty, element: &dyn UIElement) -> StyleValue {
        // Check direct properties
        if let Some(value) = self.properties.get(&prop) {
            return value.clone();
        }
        
        // Check state-specific styles
        if let Some(state_styles) = self.states.get(&element.state()) {
            if let Some(value) = state_styles.get(&prop) {
                return value.clone();
            }
        }
        
        // Check selector-based styles
        for (selector, properties) in &self.selectors {
            if selector.matches(element) {
                if let Some(value) = properties.get(&prop) {
                    return value.clone();
                }
            }
        }
        
        // Check parent styles
        if let Some(parent) = element.parent() {
            return parent.effective_style().resolve_property(prop, element);
        }
        
        // Default value
        StyleValue::default_for_property(prop)
    }
}
```

## Event System

```rust
pub enum UIEventType {
    // Pointer events
    PointerDown(PointerInfo),
    PointerMove(PointerInfo),
    PointerUp(PointerInfo),
    PointerEnter(PointerInfo),
    PointerLeave(PointerInfo),
    Click(PointerInfo),
    DoubleClick(PointerInfo),
    
    // Keyboard events
    KeyDown(KeyInfo),
    KeyUp(KeyInfo),
    TextInput(String),
    
    // Focus events
    FocusIn,
    FocusOut,
    
    // UI state events
    StateChanged(UIElementState),
    
    // Gesture events
    Pan(PanInfo),
    Pinch(PinchInfo),
    Rotate(RotateInfo),
    
    // Layout events
    LayoutUpdated,
    SizeChanged(Size),
}

pub struct UIEvent {
    pub event_type: UIEventType,
    pub target: UIElementId,
    pub current_target: UIElementId,
    pub timestamp: Instant,
    pub propagation_phase: PropagationPhase,
    pub propagation_stopped: bool,
}

pub enum PropagationPhase {
    Capture,  // Top-down phase
    Target,   // At target
    Bubble,   // Bottom-up phase
}

impl UIEvent {
    // Stop event propagation
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
}

// Event dispatcher
pub struct EventDispatcher {
    element_map: HashMap<UIElementId, Weak<RefCell<dyn UIElement>>>,
    event_queue: VecDeque<UIEvent>,
}

impl EventDispatcher {
    // Process an input event through the UI hierarchy
    pub fn dispatch_event(&mut self, event: UIEvent, root: &dyn UIElement) -> EventResult {
        // For capture phase, we walk down the tree
        let path = self.build_event_path(event.target, root);
        
        // Capture phase (top-down)
        let mut event = event;
        event.propagation_phase = PropagationPhase::Capture;
        
        for element_id in path.iter().rev() {
            if event.propagation_stopped {
                break;
            }
            
            if let Some(element) = self.get_element(*element_id) {
                event.current_target = *element_id;
                element.borrow_mut().handle_event(&event);
            }
        }
        
        // Target phase
        if !event.propagation_stopped {
            event.propagation_phase = PropagationPhase::Target;
            if let Some(target) = self.get_element(event.target) {
                event.current_target = event.target;
                target.borrow_mut().handle_event(&event);
            }
        }
        
        // Bubble phase (bottom-up)
        if !event.propagation_stopped {
            event.propagation_phase = PropagationPhase::Bubble;
            
            for element_id in path.iter() {
                if event.propagation_stopped {
                    break;
                }
                
                if let Some(element) = self.get_element(*element_id) {
                    event.current_target = *element_id;
                    element.borrow_mut().handle_event(&event);
                }
            }
        }
        
        EventResult::Processed
    }
    
    // Build path from target to root
    fn build_event_path(&self, target: UIElementId, root: &dyn UIElement) -> Vec<UIElementId> {
        let mut path = Vec::new();
        let mut current = target;
        
        while current != root.id() {
            if let Some(element) = self.get_element(current) {
                if let Some(parent) = element.borrow().parent() {
                    path.push(parent.id());
                    current = parent.id();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        path
    }
}
```

## Layout System

```rust
pub struct LayoutConstraints {
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub preferred_width: Option<f32>,
    pub preferred_height: Option<f32>,
}

pub trait Layout {
    // Measure and arrange children elements
    fn layout(&self, container: &mut dyn UIElement, constraints: &LayoutConstraints) -> Size;
    
    // Clone this layout
    fn clone_layout(&self) -> Box<dyn Layout>;
}

// Example FlexBox layout
pub struct FlexBoxLayout {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap: f32,
}

impl Layout for FlexBoxLayout {
    fn layout(&self, container: &mut dyn UIElement, constraints: &LayoutConstraints) -> Size {
        // Implement flexbox layout algorithm
        // 1. Calculate flexible space
        // 2. Distribute children
        // 3. Handle wrapping if needed
        // 4. Apply justification and alignment
        
        // Simplified version:
        let mut cursor = 0.0;
        let container_width = constraints.max_width.unwrap_or(f32::INFINITY);
        
        for child in container.children_mut() {
            // Measure child
            let child_constraints = LayoutConstraints {
                max_width: Some(container_width),
                ..LayoutConstraints::default()
            };
            
            child.update_layout(&child_constraints);
            let child_size = child.local_bounds().size();
            
            // Position child based on direction
            match self.direction {
                FlexDirection::Row => {
                    let transform = Transform2D::translation(Point2D::new(cursor, 0.0));
                    child.set_transform(TransformInSpace::D2(transform));
                    cursor += child_size.width + self.gap;
                },
                FlexDirection::Column => {
                    let transform = Transform2D::translation(Point2D::new(0.0, cursor));
                    child.set_transform(TransformInSpace::D2(transform));
                    cursor += child_size.height + self.gap;
                },
                // Handle other directions
                _ => {},
            }
        }
        
        // Return container size
        Size::new(container_width, cursor)
    }
    
    fn clone_layout(&self) -> Box<dyn Layout> {
        Box::new(Self {
            direction: self.direction,
            wrap: self.wrap,
            justify_content: self.justify_content,
            align_items: self.align_items,
            align_content: self.align_content,
            gap: self.gap,
        })
    }
}
```

## Base Containers

```rust
// Basic UI container
pub struct UIPane {
    id: UIElementId,
    children: Vec<Box<dyn UIElement>>,
    layout: Box<dyn Layout>,
    style: UIStyle,
    parent: Option<Weak<RefCell<dyn UIElement>>>,
    local_transform: TransformInSpace,
    content_transform: TransformInSpace,
    size: Size,
    needs_layout: bool,
    accessibility: AccessibilityProps,
    event_listeners: HashMap<UIEventType, Vec<Box<dyn Fn(&UIEvent) -> EventResult>>>,
}

impl UIElement for UIPane {
    // Implementation of the UIElement trait methods
}

impl Object2D for UIPane {
    // Implementation of the Object2D trait methods
}

impl UIObject2D for UIPane {
    // Implementation of the UIObject2D trait methods
}

// Scrollable container
pub struct ScrollPane {
    // Base UIPane properties
    base: UIPane,
    
    // Scroll-specific properties
    scroll_position: Point2D,
    scroll_bounds: Rect,
    scrollable_x: bool,
    scrollable_y: bool,
    scroll_bar_x: Option<Box<dyn UIElement>>,
    scroll_bar_y: Option<Box<dyn UIElement>>,
    scroll_behavior: ScrollBehavior,
    overscroll_effect: OverscrollEffect,
}

impl UIElement for ScrollPane {
    // Most implementations delegate to self.base
    
    // Override content_transform to include scrolling
    fn content_transform(&self) -> TransformInSpace {
        let base_transform = self.base.content_transform();
        
        // Apply scroll offset
        match base_transform {
            TransformInSpace::D2(transform) => {
                let scroll_transform = Transform2D::translation(
                    Point2D::new(-self.scroll_position.x(), -self.scroll_position.y())
                );
                let combined = combine_transforms(&transform, &scroll_transform);
                TransformInSpace::D2(combined)
            },
            _ => base_transform,
        }
    }
    
    // Custom event handling for scroll gestures
    fn handle_event(&mut self, event: &UIEvent) -> EventResult {
        match &event.event_type {
            UIEventType::PointerMove(info) if info.is_dragging() => {
                // Handle drag-to-scroll
                if self.is_dragging_scroll {
                    let delta = info.movement();
                    self.scroll_by(-delta.x(), -delta.y());
                    EventResult::Consumed
                } else {
                    self.base.handle_event(event)
                }
            },
            UIEventType::Pan(info) => {
                // Handle pan gesture for scrolling
                if self.scrollable_x {
                    self.scroll_position.set_x(
                        (self.scroll_position.x() - info.delta_x)
                            .clamp(0.0, self.max_scroll_x())
                    );
                }
                
                if self.scrollable_y {
                    self.scroll_position.set_y(
                        (self.scroll_position.y() - info.delta_y)
                            .clamp(0.0, self.max_scroll_y())
                    );
                }
                
                self.invalidate_render();
                EventResult::Consumed
            },
            _ => self.base.handle_event(event),
        }
    }
}
```

## Key Features and Benefits

### 1. Seamless Integration with Render Engine

- **Leverages Existing Traits**: Extends `Object2D` and `Renderable` to ensure UI elements work with existing rendering pipeline
- **Reuses Style System**: Uses the existing style system but adds cascading behavior
- **Operation-based Rendering**: UI elements create `RenderOperation`s like other renderables

### 2. Hierarchy and Transformation

- **Nested Elements**: Full support for nested UI elements with transform inheritance
- **Local vs. Global Coordinates**: Transforms between coordinate spaces for proper event handling
- **Scrollable Content**: Content transform separate from element transform for scrolling

### 3. Event System

- **Capture/Bubble Phases**: Standard event propagation model similar to DOM
- **Event Delegation**: Efficient event handling through delegation to parent elements
- **Gesture Support**: Built-in recognition for gestures like pan, pinch, and rotate

### 4. Layout System

- **Flexible Layout Algorithm**: Layout managers can be swapped (FlexBox, Grid, etc.)
- **Constraint-Based Layout**: Uses a constraint system for responsive layouts
- **Automatic Invalidation**: Cascading layout updates when elements change

### 5. Styling

- **Cascading Styles**: Styles cascade down the UI hierarchy
- **Selector System**: Target elements based on relationships, attributes or states
- **State-Based Styling**: Different styles for hover, pressed, focused states

### 6. Accessibility

- **Screen Reader Support**: Built-in accessibility properties for screen readers
- **Keyboard Navigation**: Focus management and keyboard event handling
- **ARIA Roles**: Maps UI elements to standard accessibility roles

## Implementation Recommendations

1. **Start Small**: Begin with basic element containers and layout before tackling complex features
2. **Focus on Core Traits**: The UIElement and UIObject2D traits are the foundation to build upon
3. **Leverage Existing Systems**: Use RenderOperation, Transform, and Style systems extensively
4. **Performance Optimizations**:
   - Only invalidate and re-render parts of the UI that change
   - Use hierarchical culling for large UI trees
   - Cache transformed bounds for hit testing
5. **Keep It Modular**: Make components optional with feature flags

This UI core provides a solid foundation for building more complex UI frameworks and widget libraries while maintaining the performance and flexibility of the core rendering engine.
