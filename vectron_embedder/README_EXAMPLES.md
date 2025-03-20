# Vectron Embedder Examples

This directory contains examples demonstrating how to use the Vectron Embedder library to create and manipulate windows on different platforms.

## Available Examples

1. **Basic Window Example** (`basic_window.rs`):
   A simple example that creates a single window and demonstrates basic event handling.

2. **Multiple Windows Example** (`multiple_windows.rs`):
   A more complex example that demonstrates parent-child window relationships, window manipulation, and processing events from multiple windows.

## Running the Examples

### On Windows

You can use the provided batch script to run the examples:

```
run_examples.bat basic     # Run the basic window example
run_examples.bat multiple  # Run the multiple windows example
```

Alternatively, you can use Cargo directly:

```
cargo run --example basic_window
cargo run --example multiple_windows
```

## Notes on Implementation

- The examples are platform-specific and will only work on platforms that have the necessary embedder implementations.
- Currently, Windows implementation using Win32 API is available and fully functional.
- Each example demonstrates different aspects of window management and event handling.
- The examples include proper window cleanup to ensure resources are released correctly.

## Features Demonstrated

- Window creation and destruction
- Window showing and hiding
- Window resizing and moving
- Window title updating
- Parent-child window relationships
- Event handling (resize, move, quit)
- Window redrawing and refresh
- Window properties (size, position) 