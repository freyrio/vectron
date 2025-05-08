# Vectron Rendering Architecture

This project implements a modern, flexible rendering engine architecture based on the "Unified Render Engine" design.

## Project Structure

The project consists of two main crates:

1. **vectron_render** - The core rendering abstraction layer providing:
   - Backend interfaces and traits
   - Resource handling (buffers, textures, shaders, etc.)
   - Command abstraction
   - Error handling

2. **vectron_wgpu** - The WebGPU implementation of the backend interface, providing:
   - Implementation of all backend traits for WGPU
   - Resource management for WGPU objects
   - Command translation

## Architecture Overview

The architecture follows a layered design pattern:

```
+-------------------------------------------------+
|               Public Facing APIs                |
| (SVG API, D2 API, D3 API, Graphic API, etc.)    |
+-------------------------------------------------+
                      |
+-------------------------------------------------+
|            Core Render Engine                   |
| (RenderContext, Renderer, RenderOperation...)   |
+-------------------------------------------------+
                      |
+-------------------------------------------------+
|               Backend Interface                 |
| (BackendDevice, CommandBuffer, Resources...)    |
+-------------------------------------------------+
                      |
+-------------------------------------------------+
|            Backend Implementations              |
| (WGPU, Vulkan, Metal, OpenGL...)                |
+-------------------------------------------------+
```

### vectron_render Backend Interface

The backend interface defines traits like:

- `BackendDevice` - For creating resources and managing the GPU device
- `CommandBuffer` - For recording rendering commands
- `CommandEncoder` - For creating command buffers

It also defines common types for resources like:

- `BufferHandle`, `TextureHandle`, `ShaderHandle`, etc.
- `BufferDesc`, `TextureDesc`, `ShaderDesc`, etc.

### vectron_wgpu Implementation

The WGPU implementation provides:

- `WgpuBackendDevice` - Implements `BackendDevice` using WGPU
- `WgpuCommandBuffer` - Implements `CommandBuffer` for WGPU
- Type converters between abstraction types and WGPU types

## Getting Started

To use the rendering engine with WGPU:

```rust
use vectron_wgpu::create_wgpu_backend_for_window;
use vectron_render::backend::device::BackendDevice;

fn main() {
    // Create a window (not shown)
    let window = create_window();
    
    // Create a WGPU backend for this window
    let mut backend = create_wgpu_backend_for_window(&window, width, height).unwrap();
    
    // Rendering loop
    loop {
        // Get command encoder
        let mut encoder = backend.begin_frame().unwrap();
        
        // Record commands
        // ...
        
        // Finish command encoding
        let command_buffer = encoder.finish_encoding().unwrap();
        
        // Submit commands
        backend.end_frame(command_buffer).unwrap();
    }
}
```

## Current Status

The implementation is currently in progress:

- ✅ Backend interface design
- ✅ Basic WGPU implementation structure
- ✅ Resource management
- ⚠️ Command buffer implementation (partially complete)
- ⚠️ Shader and pipeline creation (partially complete)
- ❌ Higher-level rendering abstractions

## Future Work

Future development will focus on:

1. Completing the WGPU backend implementation
2. Implementing 2D and 3D rendering APIs
3. Adding SVG support
4. Adding text rendering capabilities
5. Implementing transform hierarchy and scene graph
