Okay, here is the comprehensive plan for the Graphite rendering engine.

**Graphite Rendering Engine Plan**

**1. Introduction & Goals**

Graphite is a modern, high-performance 2D and 3D rendering engine built from scratch in Rust. It prioritizes a flexible and intuitive API, platform independence through abstraction, and efficient rendering techniques.

**Goals:**

* Provide a clean, idiomatic Rust API for immediate and retained mode rendering (primarily 2D initially).
* Support advanced 2D shape and text rendering capabilities.
* Include foundational support for 3D rendering.
* Achieve high performance via efficient GPU utilization, batching, and optimized rendering paths.
* Maintain a highly modular and extensible design.
* Abstract the underlying graphics API (initially WGPU) to allow for future backend additions.
* Minimize external dependencies.

**2. Core Philosophy**

* **Rust Idioms:** Leverage Rust's strengths: ownership, borrowing, traits, enums, `Result`/`Option` for robust and safe code.
* **Abstraction:** Decouple core rendering logic from the specific GPU backend via a well-defined trait interface (`BackendDevice`).
* **Performance:** Employ techniques like geometry tessellation, glyph caching (MSDFs), draw call batching, and specialized render pipelines.
* **Flexibility:** Support diverse units (CSS-inspired logical vs. compute), composable styles and effects, and configurable features.
* **Modularity:** Organize code into logical modules within a single crate for clarity and maintainability.

**3. Project Structure (Single Crate `graphite`)**

```
graphite/
├── Cargo.toml            # Crate manifest, feature flags (d2, d3, text, backend_wgpu, tessellation_lyon?)
└── src/
    ├── lib.rs              # Main crate file, module definitions, public API re-exports

    // --- Core Concepts ---
    ├── core/
    │   ├── mod.rs          # Defines submodules, re-exports core types/traits
    │   ├── error.rs        # RenderError enum hierarchy (IO, Backend, Layout, Tessellation)
    │   ├── types.rs        # Fundamental primitives (Point, Size, Rect, Box, ID types...)
    │   ├── math.rs         # Vector/Matrix math (likely re-exporting `glam`)
    │   ├── units.rs        # Unit enum (Px, Percent, Em, etc.), Unit<T>, ResolutionContext
    │   ├── color.rs        # Color (RGBA, HSLA), Gradient types (Linear, Radial)
    │   ├── style.rs        # Fill (Solid, Gradient), Stroke (Width, Color, Cap, Join, Dash), Style struct
    │   ├── effect.rs       # Effect definitions/traits (e.g., Shadow, Blur)
    │   ├── transform.rs    # Transform struct (matrix), operations, local origin concept
    │   ├── backend_api.rs  # BackendDevice trait, Surface trait, CommandBuffer trait, etc.
    │   ├── resource.rs     # Resource Handles (BufferHandle, TextureHandle, PipelineHandle), ResourceId, Resource Descriptors (GeometryDesc, TextureDesc)
    │   ├── renderable.rs   # Renderable trait (local origin/transform/bounds), Renderable2D/3D variants
    │   └── tessellation.rs # Tessellable trait definition, TessellationOptions struct, TessellationResult struct/enum, common tessellation vertex types/helpers
    │
    // --- Concrete Renderable Elements ---
    ├── elements/
    │   ├── mod.rs          # Defines d2/d3 submodules, maybe common element helpers
    │   ├── d2/             # 2D Elements (enabled by 'd2' feature?)
    │   │   ├── mod.rs      # Exports 2D elements
    │   │   ├── path.rs     # Path struct, PathCommand enum, `impl Tessellable for Path`
    │   │   ├── line.rs     # Line struct, `impl Tessellable for Line` (handles stroke width)
    │   │   ├── rect/       # Module for Rectangle
    │   │   │   ├── mod.rs  # Defines Rect struct, convenience constructors
    │   │   │   └── tessellation.rs # `impl Tessellable for Rect` (may bypass for simple cases)
    │   │   ├── oval/       # Module for Oval/Circle
    │   │   │   ├── mod.rs  # Defines Oval struct
    │   │   │   └── tessellation.rs # `impl Tessellable for Oval` (approximation)
    │   │   └── text/       # Module for Text Elements (enabled by 'text' feature?)
    │   │       ├── mod.rs  # Defines TextRun, StyledGlyph, TextBlob etc. `impl Renderable for TextBlob`
    │   │       ├── font.rs # Font loading/metrics facade (requires `rusttype` dependency)
    │   │       ├── layout.rs # Text layout logic (wrapping, alignment)
    │   │       └── glyph_cache.rs # Manages texture atlas for glyphs (strategy: MSDFs)
    │   │
    │   └── d3/             # 3D Elements (enabled by 'd3' feature?)
    │       ├── mod.rs      # Exports 3D elements
    │       ├── camera.rs
    │       ├── light.rs
    │       ├── material.rs
    │       ├── mesh/       # Module for Mesh
    │       │   ├── mod.rs  # Defines Mesh struct (vertices, indices, material ref?)
    │       │   └── tessellation.rs # `impl Tessellable for Mesh` (trivial pass-through)
    │       ├── object.rs   # Object3D struct/trait holding transform + element (e.g., Mesh)
    │       └── scene.rs    # 3D Scene Graph node (Node3D)
    │
    // --- Rendering Orchestration & State ---
    ├── operation/          # Definition of the RenderOperation intermediate representation
    │   ├── mod.rs          # Defines RenderOperation struct (Renderable handle/ref, Transform, Style/Effect refs, Clip state)
    │   └── clip.rs         # Clipping state (e.g., ScissorRect)
    │
    ├── pipeline/           # Definitions and management of rendering pipelines
    │   ├── mod.rs          # Pipeline cache trait/struct, common pipeline state definitions
    │   ├── state.rs        # BlendMode, DepthStencilState, RasterizationState etc. definitions
    │   ├── cache.rs        # Pipeline cache implementation (maps descriptor -> PipelineHandle)
    │   ├── vector_pipeline.rs # Descriptors/shader info for 2D vectors (fill, stroke)
    │   ├── text_pipeline.rs   # Descriptors/shader info for MSDF text rendering
    │   └── pbr_pipeline.rs    # Descriptors/shader info for 3D PBR rendering ('d3' feature)
    │
    ├── renderer/           # The main rendering engine logic and user-facing API
    │   ├── mod.rs          # Exports Context, defines internal components
    │   ├── context.rs      # User-facing RenderingContext (holds BackendDevice instance, resource cache, pipeline cache) - provides draw APIs
    │   ├── batcher.rs      # Logic for sorting/batching RenderOperations
    │   ├── command_translator.rs # Internal: Consumes RenderOperations, interacts w/ caches & BackendDevice
    │   └── resource_cache.rs # Internal: Maps ResourceHandles/IDs to actual backend resources, manages lifetimes/uploads
    │
    // --- Backend Implementation ---
    ├── backend_wgpu/       # WGPU Backend Implementation (enabled by 'backend_wgpu' feature)
    │   ├── mod.rs          # Defines WgpuBackend struct implementing BackendDevice, Surface etc.
    │   ├── device.rs       # WGPU device/adapter/instance/queue logic
    │   ├── surface.rs      # WGPU surface/swapchain management
    │   ├── buffer.rs       # WGPU buffer creation/management
    │   ├── texture.rs      # WGPU texture/sampler creation/management
    │   ├── pipeline.rs     # WGPU render pipeline creation from descriptions in `pipeline` module
    │   └── command_encoder.rs # WGPU command buffer recording logic
    │
    // --- Utilities ---
    ├── util/               # Optional: General utilities (logging, helpers)
    │   └── mod.rs
    └── debug.rs            # Optional: Debug drawing helpers/overlays
```

**4. Key Modules & Concepts**

* **`core`**: Foundation of the engine. Contains dimension-agnostic traits, structures, and fundamental systems.
    * `backend_api`: Defines the essential `BackendDevice` trait, the abstraction layer over the specific GPU API (like WGPU). Also includes traits for surfaces, command buffers, etc.
    * `resource`: Defines `ResourceHandle` types (e.g., `BufferHandle`, `TextureHandle`) returned by the backend and `ResourceDescriptor` structs (e.g., `GeometryDesc`, `TextureDesc`) describing resource properties on the CPU side.
    * `renderable`: Defines the `Renderable` trait, implemented by anything drawable. Includes methods for local origin, transformations (`Transform`), and bounds calculation (`local_bounds`, `world_bounds`).
    * `tessellation`: Defines the `Tessellable` trait for converting shapes into GPU-ready triangle meshes. Includes `TessellationOptions` (quality, tolerance) and `TessellationResult` (vertex/index data). Implementations live with the elements.
    * `units`: Implements the unit system (`Px`, `Percent`, `Em`, etc.) and the `ResolutionContext` needed to resolve logical units to compute units (pixels).
    * `color`, `style`, `effect`, `transform`, `math`, `types`, `error`: Provide fundamental building blocks for appearance, positioning, calculations, and error handling.
* **`elements`**: Contains concrete, renderable items, organized into `d2` and `d3` submodules.
    * Each element (e.g., `Rect`, `Path`, `TextBlob`, `Mesh`) defines its properties, often using `core::units`.
    * Elements that need geometry generation implement the `core::tessellation::Tessellable` trait. The implementation logic resides within the element's module (e.g., `elements/d2/rect/tessellation.rs`).
    * `d2::text`: Handles advanced text rendering, including font loading/management (`font.rs` via `rusttype`), layout (`layout.rs`), and glyph caching using MSDFs (`glyph_cache.rs`).
* **`operation`**: Defines the crucial `RenderOperation` struct. This acts as an intermediate representation that bundles a reference to a `Renderable`, its world `Transform`, applied `Style` and `Effect` information, and clipping state (`clip.rs`). It's the primary unit processed by the renderer.
* **`pipeline`**: Manages rendering pipeline states.
    * Defines GPU state components (`state.rs`: blend modes, depth/stencil settings).
    * Provides structures (`vector_pipeline.rs`, `text_pipeline.rs`, etc.) describing the requirements for specific rendering tasks (shaders, vertex layouts).
    * Includes a `cache.rs` mechanism (used internally by the renderer) to reuse pipeline objects created by the backend.
* **`renderer`**: Orchestrates the rendering process and provides the main user API.
    * `context.rs`: The primary user-facing struct (`RenderingContext`). It holds the active `BackendDevice`, manages internal caches (`ResourceCache`, `PipelineCache`), and provides drawing methods (e.g., `fill_rect`, `draw_text`, potentially scene management methods).
    * Internal Components: `batcher.rs` sorts and batches `RenderOperation`s; `resource_cache.rs` maps handles to live GPU resources and manages uploads; `command_translator.rs` consumes batched operations and issues calls to the `BackendDevice`.
* **`backend_wgpu`**: The concrete implementation of the `core::backend_api` traits using the WGPU library. Handles all WGPU-specific object creation, command encoding, and submission. Enabled via the `backend_wgpu` feature flag.

**5. Rendering Pipeline / Stack Flows**

The general flow involves translating high-level draw commands or scene descriptions into low-level GPU instructions via the `BackendDevice` abstraction.

**A. Immediate Mode Shape Drawing (e.g., `ctx.fill_rect(rect, style)`)**

1.  **API Call:** User calls `ctx.fill_rect(...)` on the `RenderingContext`.
2.  **Operation Creation:** The `Context` creates a `RenderOperation` containing:
    * Reference to the `Rect` element data.
    * Current world transform.
    * Reference to the resolved `Style` (fill color/gradient).
    * Current clipping state.
3.  **Submission (Implicit/Explicit):** The `RenderOperation` is added to a queue within the `Context`. At frame end (or explicit flush), the queue is processed.
4.  **Batching (`renderer::batcher`):** Operations are sorted based on pipeline compatibility, depth, texture bindings, etc., to minimize state changes.
5.  **Translation (`renderer::command_translator`):** Iterates through batches:
    * **Tessellation:** If the `Rect` requires tessellation (e.g., complex rounded corners not handled by shader), it calls the `Tessellable::tessellate` implementation for `Rect`.
    * **Resource Check/Upload (`renderer::resource_cache`):** Checks if the required geometry (tessellated data or standard quad) exists on the GPU. If not, uploads vertex/index data via `BackendDevice::create_buffer_init` and stores the resulting `BufferHandle`. Retrieves handles for existing resources.
    * **Pipeline Selection (`pipeline::cache` & `backend_api`):** Determines the required pipeline state (e.g., solid vector fill shader, blend mode) based on the `RenderOperation`. Requests the pipeline from the cache or creates it via `BackendDevice::create_render_pipeline`. Gets a `PipelineHandle`.
    * **Backend Calls:** Uses the `BackendDevice` trait (implemented by `backend_wgpu`) to:
        * `begin_render_pass(...)`
        * `set_pipeline(pipeline_handle)`
        * `set_vertex_buffer(buffer_handle)`
        * `set_index_buffer(buffer_handle)`
        * `set_bind_group(...)` (for uniforms like transform, color)
        * `draw_indexed(...)` or `draw(...)`
6.  **WGPU Execution (`backend_wgpu`):** The trait calls translate into specific WGPU commands recorded onto a `wgpu::CommandEncoder`.
7.  **Frame Submission:** The recorded command buffer is submitted to the WGPU queue.

**B. Text Rendering (e.g., `ctx.draw_text(text_blob)`)**

1.  **API Call:** User calls `ctx.draw_text(...)`.
2.  **Layout (`elements::d2::text::layout`):** The layout engine processes the `TextBlob` (text content, styling, constraints) to determine the position of each glyph.
3.  **Glyph Processing (`elements::d2::text::glyph_cache`):** For each required glyph:
    * Check if the glyph's MSDF representation exists in the glyph cache texture atlas.
    * If not: Use the font loader (`font.rs`) to get glyph metrics/outline. Generate the MSDF data. Find space in the atlas (or create a new one). Upload the MSDF data to the GPU texture atlas via `BackendDevice::write_texture`. Store glyph metadata (UV coordinates in atlas).
4.  **Operation Creation:** The `Context` generates multiple `RenderOperation`s, one for each glyph quad (or potentially batches quads into larger vertex buffers). Each operation references:
    * Quad geometry (likely stored in a shared, dynamic vertex buffer).
    * World transform (combined text block transform + glyph position).
    * Style (glyph color, effects derived from text styling).
    * Texture binding information for the glyph cache atlas.
5.  **Batching/Translation/Execution:** Follows the same steps as shape rendering (Steps 4-7 above), but:
    * **Resource Check:** Ensures the glyph quad vertex buffer and the glyph atlas `TextureHandle` are available/bound.
    * **Pipeline Selection:** Selects the specialized MSDF text rendering pipeline (`TextPipeline`).
    * **Backend Calls:** Binds the text pipeline, vertex buffers, and the glyph atlas texture bind group before issuing draw calls for the glyph quads.

**C. Retained Mode (Conceptual)**

1.  **Scene Definition:** User builds a scene graph (e.g., using `elements::d3::Node3D` or a similar 2D structure) containing elements, transforms, and styles.
2.  **Traversal:** The `RenderingContext` (or a dedicated scene manager) traverses the graph.
3.  **Culling:** Performs visibility checks (e.g., frustum culling for 3D).
4.  **Operation Generation:** For each visible node, calculates the final world transform and resolved style. Generates `RenderOperation`s corresponding to the renderable content of the node.
5.  **Batching/Translation/Execution:** The generated list of `RenderOperation`s is processed identically to the immediate mode flow (Steps 4-7 above).

**6. Dependencies**

* **Core:** `wgpu` (required by `backend_wgpu`), `raw-window-handle`.
* **Math:** `glam` (likely, for vector/matrix math).
* **Text:** `rusttype` (or a similar font loading/parsing library, required by `elements::d2::text`).
* **Optional:** `log`, `thiserror`, potentially `lyon_tessellation` (if integrated internally for path tessellation).

**7. Feature Flags**

Cargo features will be used to control optional functionality and backend selection:

* `d2`: Enable 2D elements and rendering logic.
* `d3`: Enable 3D elements and rendering logic.
* `text`: Enable advanced text rendering (`elements::d2::text`).
* `backend_wgpu`: Enable the WGPU backend implementation.
* `tessellation_lyon`: (Optional) Enable usage of the Lyon library internally for path tessellation if not implementing from scratch.