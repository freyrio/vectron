// Simple debug logging macros specific to dx11 backend for now.

use std::sync::atomic::{AtomicBool, Ordering};

// Flag to globally enable/disable dx11 debug output, controlled at runtime.
// TODO: Integrate with a proper logging framework (log, tracing)
static DX11_DEBUG_ENABLED: AtomicBool = AtomicBool::new(false); // Default to false

/// Sets the runtime state for DX11 debug logging.
pub fn set_dx11_debug_enabled(enabled: bool) {
    DX11_DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Checks if DX11 debug logging is currently enabled.
pub fn is_dx11_debug_enabled() -> bool {
    DX11_DEBUG_ENABLED.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! dx11_debug {
    ($($arg:tt)*) => {
        // Use the atomic bool check
        if $crate::dx11::debug::is_dx11_debug_enabled() {
            print!("[DX11 DEBUG] ");
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! dx11_warn {
    ($($arg:tt)*) => {
        // Warnings might still be useful even if full debug is off
        // Decide whether to keep warnings always on or tie them to the flag
        print!("[DX11 WARN] ");
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! dx11_error {
    ($($arg:tt)*) => {
        // Errors should likely always be enabled
        print!("[DX11 ERROR] ");
        eprintln!($($arg)*);
    };
}

// Re-export functions and macros
pub use dx11_debug;
pub use dx11_warn;
pub use dx11_error;
// No need to re-export the functions usually, accessed via dx11::debug::* 