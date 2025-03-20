use std::time::Duration;
use std::thread::sleep;
use vectron_embedder::{Embedder, EmbedderConfig, Event, WindowConfig, WindowEmbedder, PlatformEmbedder};

fn main() {
    println!("Starting Vectron Window Example");

    // Create platform-specific embedder
    let mut embedder = vectron_embedder::create_embedder();
    
    // Initialize embedder with basic configuration
    embedder.init(EmbedderConfig {
        application_name: "Vectron Window Example".to_string(),
        enable_high_dpi: true,
        vsync: true,
    }).expect("Failed to initialize embedder");
    
    println!("Embedder initialized");
    
    // Create a window with standard configuration
    let window = embedder.create_window(WindowConfig {
        title: "Vectron Basic Window".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        decorated: true,
        visible: true,
        position: None,
        min_size: None,
        max_size: None,
        parent: None,
    }).expect("Failed to create window");
    
    println!("Window created with handle: {:?}", window);
    
    // Basic event loop
    let mut frame_count = 0;
    while embedder.is_running() && frame_count < 1000 {
        // Process window events
        let events = embedder.process_events();
        
        // Handle events
        for event in events {
            match event {
                Event::Quit => {
                    println!("Quit event received");
                    break;
                },
                Event::Resized { width, height } => {
                    println!("Window resized to {}x{}", width, height);
                },
                Event::Moved { x, y } => {
                    println!("Window moved to ({}, {})", x, y);
                },
                _ => {}
            }
        }
        
        // Request window redraw
        if frame_count % 60 == 0 {
            println!("Frame count: {}", frame_count);
            
            // Get current window size and position for demonstration
            let size = embedder.get_window_size(window);
            let position = embedder.get_window_position(window);
            
            println!("Window size: {}x{}", size.0, size.1);
            println!("Window position: ({}, {})", position.0, position.1);
            
            // Request a redraw
            embedder.request_redraw(window);
        }
        
        // Small sleep to avoid maxing CPU
        sleep(Duration::from_millis(16)); // ~60 FPS
        
        frame_count += 1;
    }
    
    // Clean up
    embedder.destroy_window(window);
    embedder.shutdown();
    
    println!("Example completed successfully");
} 