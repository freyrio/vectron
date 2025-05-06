# Vectron GPU: Strategy for Diverse Targets and Feature Sets

A crucial challenge in graphics abstraction layers is balancing a modern API with the need to support diverse targets (desktop, mobile, embedded, software) efficiently, particularly avoiding runtime emulation overhead on simpler backends (e.g., OpenGL ES, DX11, Software).

**Vectron GPU's Primary Strategy: Compile-Time API Profiles via Static Interface Selection**

Instead of a single unified API surface relying heavily on runtime feature queries and potentially costly emulation layers (like wgpu), Vectron GPU will offer distinct API profiles implemented as separate Device types. Applications choose the profile that best matches their minimum target requirements at compile time, ensuring they only link against and use APIs suitable for that target class, minimizing overhead.

---

## 1. Static API Profiles (Primary Strategy)

Offer multiple API levels via distinct Device structs, selected at compile time. This avoids forcing emulation of modern concepts (like Bind Groups, PSOs) on backends where they don't map naturally.

```rust
// In Cargo.toml feature flags
// Choose *one* primary API profile:
[features]
default = ["api-modern"] # Default to the full-featured API

# --- API Profiles ---
api-modern = [] # Full modern API (PSOs, Bind Groups, Barriers). Assumes Vulkan/Metal/DX12 targets.
api-compat = [] # Compatibility API (Simpler state/binding). Targets DX11/GLES, avoids heavy emulation.
api-software = [] # Minimal API tailored for the software backend (e.g., 2D/UI).

# --- Backend Selection (Orthogonal to API Profile) ---
# Select one or more backends to compile:
backend-vulkan = []
backend-dx12 = []
backend-metal = []
backend-dx11 = [] # Likely only fully usable with api-compat
backend-gles = [] # Likely only fully usable with api-compat
backend-software = [] # Requires api-software profile? Or maybe compat? TBD.

# --- Optional Capabilities (May depend on API Profile) ---
# These can refine the features *within* a profile.
opt-compute = [] # Might only be available in api-modern and api-compat (if GLES 3.1+)
opt-tessellation = [] # Likely api-modern only
opt-ray_tracing = [] # Likely api-modern only
# ... etc ...
```

Define distinct Device types for each profile:

```rust
// Represents the full-featured modern API surface (PSOs, Bind Groups, etc.)
// Requires backend-vulkan, backend-dx12, or backend-metal features.
#[cfg(feature = "api-modern")]
pub struct VectronModernDevice {
    // Implementation using modern backend concepts
    // Provides methods like create_graphics_pipeline, create_bind_group, etc.
}

// Represents a compatibility-focused API surface.
// Avoids concepts requiring heavy emulation on older APIs.
// Requires backend-dx11 or backend-gles features.
#[cfg(feature = "api-compat")]
pub struct VectronCompatDevice {
    // Implementation using DX11/GLES concepts
    // Provides alternative methods, e.g.,:
    // - cmd.set_shader_program(...)
    // - cmd.set_rasterizer_state(...)
    // - cmd.set_blend_state(...)
    // - cmd.set_vertex_shader_texture(...)
    // - cmd.set_fragment_shader_uniform_buffer(...)
    // Lacks methods like create_graphics_pipeline, create_bind_group.
}

// Represents a minimal API tailored for the software renderer.
// Focuses on 2D/UI rendering needs.
#[cfg(feature = "api-software")]
pub struct VectronSoftwareDevice {
    // Implementation using software rendering logic
    // Provides a very limited subset of commands/resource types.
}
```

Applications select the appropriate device type based on their target profile:

```rust
// Select the device type based on the chosen API profile feature
#[cfg(feature = "api-modern")]
use vectron_gpu::VectronModernDevice as Device;

#[cfg(feature = "api-compat")]
use vectron_gpu::VectronCompatDevice as Device;

#[cfg(feature = "api-software")]
use vectron_gpu::VectronSoftwareDevice as Device;

// Usage remains consistent thanks to the type alias
// let device = Device::new(...)?;
// device.create_buffer(...); // Method available in all profiles (likely)
// device.create_bind_group(...); // Compile error if 'api-compat' or 'api-software' is selected
```

**Rationale:** This provides the strongest guarantee against unwanted emulation overhead by selecting the API surface itself at compile time. The exact API for `VectronCompatDevice` and `VectronSoftwareDevice` needs detailed definition.

---

## 2. Supporting Mechanisms

The primary strategy is supported by the following mechanisms:

### 2.1 Compile-Time Feature Selection (Cargo Features)

As shown above, Cargo features are used extensively to select:
1.  The **API Profile** (`api-modern`, `api-compat`, `api-software`).
2.  The **Backend Implementation(s)** (`backend-vulkan`, `backend-dx12`, etc.).
3.  **Optional Capabilities** (`opt-compute`, `opt-ray_tracing`) applicable *within* a profile.

```rust
// ... Cargo.toml example from section 1 ...
```

This allows applications to create lean builds tailored to their specific needs.

### 2.2 Conditional Compilation (`#[cfg]`)

The underlying mechanism for implementing profiles and optional features is standard Rust conditional compilation.

```rust
// Example within library code:
#[cfg(feature = "api-modern")]
impl VectronModernDevice {
    #[cfg(feature = "opt-ray_tracing")]
    pub fn trace_rays(&self, /* ... */) -> Result<(), GpuError> {
        // Implementation specific to modern APIs + RT feature
    }
}
```

// In user code, `#cfg` can be used for portability between profiles if needed:
fn setup_rendering(device: &Device) {
    #[cfg(feature = "api-modern")]
    let pipeline = device.create_graphics_pipeline(/* ... */);

    #[cfg(feature = "api-compat")]
    let program = device.create_shader_program(/* ... */); // Example alternative
}
```

This ensures unused code paths and entire API methods are compiled out.

### 2.3 Backend-Specific Extensions

Provide access to non-portable, backend-specific features (e.g., vendor extensions) via optional extension traits gated by backend features.

```rust
// In vectron_gpu::backends::vulkan::extensions
#[cfg(feature = "backend-vulkan")]
pub trait VulkanExtensions {
    fn create_acceleration_structure(&self, desc: &AccelerationStructureDesc) -> Result<AccelerationStructure, GpuError>;
}

// In vectron_gpu crate root or device module
#[cfg(feature = "backend-vulkan")]
impl VulkanExtensions for VectronModernDevice { // Only implement for relevant device types
    // Vulkan-specific implementation...
}

// In user code:
fn use_vulkan_features(device: &Device) {
    // Requires `use vectron_gpu::backends::vulkan::extensions::VulkanExtensions;`
    // Compile error if `backend-vulkan` is not enabled.
    #[cfg(all(feature = "api-modern", feature = "backend-vulkan"))]
    match device.create_acceleration_structure(&desc) {
        // ...
#       Ok(_) => {}
#       Err(_) => {}
    }
}
```

This keeps the core API portable while allowing opt-in access to platform specifics.

### 2.4 Native Platform Integration

For platforms where the graphics context might be managed externally (especially mobile/embedded), provide hooks for integration.

```rust
# // Dummy types for illustration
# struct VectronModernDevice {}
# struct GpuError {}
# mod ffi { pub enum c_void {} }
#
#[cfg(feature = "backend-vulkan")] // Example for Vulkan/Android
impl VectronModernDevice { // Or VectronCompatDevice if GLES used
    /// Creates a Vectron device from an existing native Vulkan instance and device handle.
    /// Takes ownership based on provided config.
    pub unsafe fn from_native_vulkan(/* ... native handles ..., config */) -> Result<Self, GpuError> {
        // Implementation...
#       todo!()
    }

    /// Allows retrieving the underlying native handles (use with caution).
    pub unsafe fn native_vulkan_handles(&self) -> (/* vkInstance, vkDevice */) {
        // Implementation...
#       todo!()
    }
}

#[cfg(feature = "backend-gles")] // Example for GLES/Android/iOS
impl VectronCompatDevice {
    /// Creates a Vectron device using an existing EGL context.
    pub unsafe fn from_native_egl(/* ... EGLDisplay, EGLContext ..., config */) -> Result<Self, GpuError> {
        // Implementation...
#       todo!()
    }

    // ... similar methods for Metal context on iOS/macOS ...
}
```

### 2.5 Build Script Platform Detection

Use `build.rs` scripts primarily for detecting **OS, architecture, or specific toolchain requirements**, not runtime GPU features.

```rust
// In build.rs
fn main() {
    let target = std::env::var("TARGET").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "android" {
        println!("cargo:rustc-cfg=target_os_android");
        // Link against Android libraries, etc.
    } else if target_os == "ios" {
        println!("cargo:rustc-cfg=target_os_ios");
    }
    // ... etc. ...
}

// In library code:
#[cfg(target_os_android)]
fn android_specific_setup() { /* ... */ }
```

---

## 3. Design Considerations & Next Steps

### Defining the `api-compat` Profile
The most critical next step is defining the exact API surface for `VectronCompatDevice`. What are the alternatives to `create_graphics_pipeline` and `create_bind_group`? Examples:
*   **State Setting:** Separate methods like `cmd.set_blend_state`, `cmd.set_depth_stencil_state`, `cmd.set_rasterizer_state`, `cmd.use_program`.
*   **Resource Binding:** Per-stage, per-slot binding like `cmd.set_vertex_shader_texture(slot, view)`, `cmd.set_fragment_shader_uniform_buffer(slot, buffer_slice)`.

This needs careful design to be ergonomic while mapping well to DX11/GLES fundamentals.

### Feature/Limit Reporting
Even with compile-time profiles, runtime querying for *specific limits* (max texture size, max UBO size) and *optional features available within a profile* (e.g., specific texture formats, compute shader support on GLES 3.1+ under `api-compat`) is still necessary. The `Adapter::features()` and `Device::limits()` mechanisms need to be defined.

### Code Complexity
Using distinct device types might lead to some code duplication or require careful use of generics/macros internally to share implementation logic where appropriate. Application code targeting multiple profiles will need `#cfg` blocks.

### Implementation Strategy
1.  **Solidify Core API:** Define common resources (`Buffer`, `Texture`, `Sampler`, `ShaderModule`) and core commands (`copy`, `dispatch`, basic `draw`) that exist across most profiles.
2.  **Define `api-compat` API:** Detail the state and binding methods for `VectronCompatDevice`.
3.  **Define `api-modern` API:** Formalize PSOs, Bind Groups/Layouts, Render Passes, explicit Barriers for `VectronModernDevice`.
4.  **Implement Backends:** Start with one modern (e.g., Vulkan for `api-modern`) and one compatibility (e.g., GLES or DX11 for `api-compat`) backend to validate the profile separation.
5.  **Develop Feature/Limit Reporting:** Define the query structures.

---

By adopting this compile-time profile strategy, Vectron GPU can provide:
- **Optimized Performance:** Avoids runtime emulation overhead for simpler targets.
- **Reduced Binary Size:** Unused API profiles and backends are compiled out.
- **Explicit Feature Sets:** Clear contracts for different target classes.
- **Flexibility:** Still allows access to modern features via `api-modern` and backend specifics via extensions.