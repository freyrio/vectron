# Vectron Render v4 Architecture Overview

Based on the blueprint document `Vectron_Render_v4.md`.

## 1. Architecture Overview

Vectron Render v4 is a redesign aiming for a modern, flexible rendering system built upon the Vectron GPU abstraction. It supports both 2D and 3D graphics with a focus on performance, composition, and platform independence.

**Goals:**

*   Modern and intuitive API.
*   Flexible support for various units, backends, and techniques.
*   High performance via tessellation, batching, and GPU acceleration.
*   Easy composition of elements, styles, and effects.
*   Extensible and modular design.

**Conceptual Layers:**

1.  **Core**: Fundamental traits, coordinate/unit systems, color, transforms.
2.  **Elements**: Concrete renderable items (shapes, text, meshes) organized by dimension.
3.  **Tessellation**: Geometry generation logic.
4.  **Operations**: Combining elements with styles/effects (`RenderOperation`).
5.  **Resource Layer**: GPU resource management (buffers, textures, fonts).
6.  **Pipeline Layer**: Specialized rendering pipelines (vector, text, 3D, effects).
7.  **Backend Abstraction**: Interface with the GPU crate.

## 2. Project Structure

```
vectron_render/
├── src/
│   ├── core/
│   │   ├── renderable.rs       # Renderable trait definitions
│   │   ├── style.rs            # Style traits and implementations
│   │   ├── effect.rs           # Visual effects
│   │   ├── render_operation.rs # Rendering operations (renamed from Operation)
│   │   ├── transform.rs        # Transform utilities
│   │   ├── color.rs            # Color system
│   │   ├── units.rs            # Unit system
│   │   └── tessellation.rs     # Tessellation traits and utilities
│   │
│   ├── elements/               # Core renderable elements
│   │   ├── mod.rs              # Re-exports and common functionality
│   │   ├── d2/                 # 2D elements
│   │   │   ├── mod.rs
│   │   │   ├── camera.rs
│   │   │   ├── layer.rs
│   │   │   ├── text.rs
│   │   │   └── geometry/
│   │   │       ├── mod.rs
│   │   │       ├── rectangle.rs
│   │   │       ├── circle.rs
│   │   │       ├── path.rs
│   │   │       └── ...
│   │   └── d3/                 # 3D elements
│   │       ├── mod.rs
│   │       ├── camera.rs
│   │       ├── light.rs
│   │       ├── scene.rs
│   │       └── geometry/
│   │           ├── mod.rs
│   │           ├── box.rs
│   │           ├── sphere.rs
│   │           ├── mesh.rs
│   │           └── ...
│   │
│   ├── tessellation/           # Tessellation module
│   │   ├── mod.rs
│   │   ├── options.rs
│   │   ├── result.rs
│   │   ├── d2/                 # 2D tessellation algorithms
│   │   └── d3/                 # 3D tessellation algorithms
│   │
│   ├── renderer/               # Renderer implementations
│   ├── pipeline/               # Rendering pipelines
│   ├── backend/                # Backend abstractions
│   ├── utils/                  # Utility modules
│   └── lib.rs                  # Public API exports
```

## 3. Key Elements & Concepts

*   **`Renderable` Trait (`core::renderable`)**: The fundamental trait for any object that can be drawn. It defines methods for retrieving and setting a local origin (`origin()`, `set_origin()`), applying transformations relative to this origin (`transform_local()`), and querying bounding volumes in both local (`local_bounds()`) and world space (`world_bounds()`). Specialized versions `Renderable2D` and `Renderable3D` provide dimension-specific methods (e.g., `local_rect()`, `world_box()`).

*   **Unit System (`core::units`)**: A highly flexible system for defining sizes and positions. Supports absolute units (Pixels, Points, Inches, Millimeters) and relative units (Percent of parent, Viewport Width/Height/Min/Max). Uses an enum `Unit<T>` (where `T` is typically `f32`) and a `UnitContext` struct (containing DPI, viewport size, parent size) to resolve units into concrete pixel values during rendering. Includes convenience constructors (`px`, `pt`, `pct`, `vw`, `vh`) and can handle computed expressions.

*   **Elements Module (`elements/`)**: A dedicated module organizing concrete renderable items, separated into 2D (`elements::d2`) and 3D (`elements::d3`) submodules. Examples include `Rectangle`, `Circle`, `Path` (2D) and `Box3D`, `Sphere`, `Mesh` (3D). Each element typically holds its properties using the `Unit` system and implements the `Renderable` and `Tessellable` traits.

*   **`Tessellable` Trait (`core::tessellation`)**: Implemented by `Renderable` elements to define how they convert their abstract representation into concrete GPU-ready geometry (vertices and indices). The core method `tessellate()` takes `TessellationOptions` and a `UnitContext` and returns a `TessellationResult` containing the geometry and statistics.

*   **Tessellation Module (`tessellation/`)**: Houses the infrastructure for geometry generation. Defines `TessellationOptions` (controlling quality, tolerance, culling, generated attributes like normals/UVs), `TessellationResult` (holding vertices, indices, bounds, stats), common vertex types (`tessellation::vertex`), and potentially the tessellation algorithms themselves (organized into `d2`/`d3` submodules).

*   **`RenderOperation` (`core::render_operation`)**: The central structure representing a single draw command. It encapsulates a `Renderable` element, a world `Transform`, optional clipping (`clip`), and vectors of associated `Style` and `Effect` traits. Its `execute()` method orchestrates the rendering process for that element by interacting with a `Renderer` implementation (setting state, triggering tessellation, applying styles/effects).

*   **Resource Layer (Handles, `ResourceCache`, `GeometryDesc`)**: Responsible for managing the lifecycle of GPU resources. Uses lightweight handles (e.g., `GeometryHandle`, `TextureHandle`) associated with `ResourceId`s. A `ResourceCache` tracks CPU-side descriptions (like `GeometryDesc` which holds tessellated vertex/index data, usage hints, and bounds) and maps handles to actual GPU backend resources (buffers, textures). Uploads to the GPU are often managed lazily.

*   **Pipeline Layer (`pipeline/`)**: Contains specialized rendering pipelines optimized for specific tasks. Examples include `VectorPipeline` (for filled/stroked 2D shapes), `TextPipeline` (using MSDF or bitmap techniques), `ThreeDPipeline` (for PBR materials, lighting, shadows), and `EffectPipeline` (for post-processing like blur). These pipelines encapsulate GPU pipeline state objects (shaders, vertex layouts, blend states, etc.).

*   **Backend Abstraction (`backend/`, `GpuInterface`)**: Acts as the intermediary between the render crate and the underlying `vectron_gpu` crate. The `GpuInterface` provides methods for frame setup/teardown, resource creation (buffers, textures), pipeline creation/caching, and access to the current frame's `CommandBuffer`. It abstracts away backend-specific details.

*   **Command Translation (`CommandTranslator`)**: A key component (likely within the `renderer` or `backend` module) that implements the `Renderer` trait. When `RenderOperation::execute()` calls methods on the `Renderer` trait (like `fill_geometry`), the `CommandTranslator` translates these high-level requests into low-level GPU commands recorded into the `CommandBuffer` via the `GpuInterface`. It manages render state (transforms, clipping), interacts with the `ResourceCache` to ensure geometry is uploaded, binds resources, selects appropriate pipelines, and issues draw calls.

*   **Feature Flags**: Leverages Cargo features (`[features]` in `Cargo.toml`) to allow users to enable only the parts of the renderer they need (e.g., `d2`, `d3`, `d2-text`, `tessellation`). This helps minimize binary size and compile times.

## 4. Rendering Stack Flow

The process of rendering an object involves several stages, flowing from high-level definitions down to low-level GPU commands:

1.  **Operation Definition**: The user defines what to render by creating instances of `Renderable` elements (e.g., `Rectangle`, `Mesh`) and configuring them using the `Unit` system. These elements are then wrapped in `RenderOperation` structs, along with desired `Style`s, `Effect`s, world `Transform`s, and optional clipping rectangles.

2.  **Operation Collection**: The main application or renderer gathers a list of `RenderOperation`s to be drawn in a frame.

3.  **Operation Execution (`RenderOperation::execute`)**: The renderer iterates through the collected operations and calls the `execute()` method on each one. This method takes an implementation of the `Renderer` trait (typically the `CommandTranslator`).

4.  **Tessellation (`Tessellable::tessellate`)**: Inside `execute()`, the `Renderer` is asked to tessellate the `Renderable` element (if it implements `Tessellable`). This involves:
    *   The `Renderer` providing the current `UnitContext` and `TessellationOptions`.
    *   Calling the `tessellate()` method on the specific `Renderable` element.
    *   The element generating vertices and indices based on its properties and the options, returning a `TessellationResult`.

5.  **Geometry Management (`Renderer::upload_geometry`)**: The `TessellationResult` is passed to the `Renderer` (specifically, the `CommandTranslator`).
    *   The `CommandTranslator` interacts with the `ResourceCache` to manage this geometry.
    *   A `GeometryDesc` is created containing the vertex/index data, stride, format, usage hints, and bounds.
    *   The `ResourceCache` assigns a `GeometryHandle` and stores the description.
    *   The method returns `GeometryData` (potentially containing a copy of vertices/indices needed by styles/effects and the handle).

6.  **Style & Effect Application (`Style::apply`, `Effect::apply_pre/post`)**: The `execute()` method proceeds to call the `apply` methods on the operation's `Style`s and `Effect`s, passing them the `Renderer` interface and the `GeometryData`.

7.  **High-Level Draw Commands (`Renderer::fill_geometry`, etc.)**: Styles and effects translate their logic into high-level draw commands by calling methods on the `Renderer` trait implementation (e.g., `fill_geometry`, `stroke_geometry`, `set_material`).

8.  **Command Translation (`CommandTranslator`)**: These high-level draw calls are received by the `CommandTranslator`:
    *   It checks the `ResourceCache` using the `GeometryHandle` to find the associated GPU buffer handles.
    *   If the geometry hasn't been uploaded to the GPU yet (or is marked dirty), it calls `GpuInterface::create_buffer` and `GpuInterface::write_buffer` to transfer the vertex/index data from the `GeometryDesc` to the GPU. The GPU handles are stored in the `ResourceCache` mapping.
    *   It determines the correct specialized GPU pipeline (`VectorPipeline`, `ThreeDPipeline`, etc.) based on the operation/style.
    *   It configures the GPU state (setting the pipeline, binding textures, samplers, uniform buffers containing transforms/colors derived from the operation and style) using the `GpuInterface`.
    *   It binds the appropriate vertex and index buffers.

9.  **Low-Level Draw Call (`CommandBuffer::draw_indexed`)**: The `CommandTranslator` records the final, low-level, backend-agnostic draw command (e.g., `draw_indexed`, `draw_indexed_instanced`) into the current `CommandBuffer` obtained from the `GpuInterface`.

10. **GPU Execution**: At the end of the frame (`GpuInterface::end_frame`), the collected commands in the `CommandBuffer` are submitted via the `vectron_gpu` abstraction to the actual graphics API (Vulkan, Metal, DirectX) for execution on the GPU.

## 5. Features

*   **Unified 2D/3D Rendering**: Core concepts apply to both dimensions.
*   **Local Coordinate System**: Simplifies element positioning and transformation hierarchies.
*   **Flexible Unit Support**: Enables responsive and resolution-independent layouts.
*   **Advanced Tessellation**: Converts complex shapes and paths into optimized GPU geometry.
*   **Composable Operations**: `RenderOperation` allows easy combination of elements, styles, and effects.
*   **GPU Resource Management**: Efficient handling of buffers, textures, etc.
*   **Specialized Pipelines**: Optimized rendering paths for different types of content.
*   **Backend Agnostic**: Leverages the `vectron_gpu` abstraction for platform independence.
*   **Performance Optimizations**: Includes strategies for batching, culling, caching, and parallelism.
*   **Structured Error Handling**: Robust `RenderError` enum and recovery strategies.
*   **Modular Design**: Facilitates extension with new elements, styles, or effects.
*   **Configurable Features**: Feature flags allow tailoring the build to specific needs.
