use std::time::Duration;
use std::thread::sleep;
use vectron_embedder::{Embedder, EmbedderConfig, Event, WindowConfig, WindowEmbedder, WindowId};

fn main() {
    println!("Starting Vectron Multiple Windows Example");

    // Create platform-specific embedder
    let mut embedder = vectron_embedder::create_embedder();
    
    // Initialize embedder with configuration
    embedder.init(EmbedderConfig {
        application_name: "Vectron Multiple Windows Example".to_string(),
        enable_high_dpi: true,
        vsync: true,
    }).expect("Failed to initialize embedder");
    
    println!("Embedder initialized");
    
    // Create the main window
    let main_config = WindowConfig::new()
        .with_title("Main Window")
        .with_size(800, 600)
        .with_resizable(true)
        .with_decorated(true)
        .with_visible(true)
        .with_position(100, 100);
        
    let main_window = embedder.create_window(&main_config)
        .expect("Failed to create main window");
    
    println!("Main window created with handle: {:?}", main_window);
    
    // Create a child window
    let child_config = WindowConfig::new()
        .with_title("Child Window")
        .with_size(400, 300)
        .with_resizable(true)
        .with_decorated(true)
        .with_visible(false) // We'll show it later
        .with_position(50, 50)
        .with_parent(main_window);
        
    let child_window = embedder.create_window(&child_config)
        .expect("Failed to create child window");
    
    println!("Child window created with handle: {:?}", child_window);
    
    // Create a popup window (no parent)
    let popup_config = WindowConfig::new()
        .with_title("Popup Window")
        .with_size(300, 200)
        .with_resizable(false)
        .with_decorated(false) // Borderless popup
        .with_visible(false) // We'll show it later
        .with_position(200, 200);
        
    let popup_window = embedder.create_window(&popup_config)
        .expect("Failed to create popup window");
    
    println!("Popup window created with handle: {:?}", popup_window);
    
    // Main event loop
    let mut frame_count = 0;
    let mut demonstration_phase = 0;
    
    while embedder.is_running() && frame_count < 1000 {
        // Process window events
        let events = embedder.process_events();
        
        // Handle events
        for event in events {
            match event {
                Event::Quit => {
                    println!("Quit event received");
                    embedder.shutdown();
                    return;
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
        
        // Demonstrate different window operations in phases
        match demonstration_phase {
            // Phase 0: Show the main window for a while
            0 => {
                if frame_count == 60 {
                    println!("Showing child window");
                    embedder.show_window(&child_window);
                    demonstration_phase = 1;
                }
            },
            
            // Phase 1: Show the child window and modify its title
            1 => {
                if frame_count == 120 {
                    println!("Changing child window title");
                    embedder.set_window_title(&child_window, "Updated Child Window Title");
                    demonstration_phase = 2;
                }
            },
            
            // Phase 2: Resize the child window
            2 => {
                if frame_count == 180 {
                    println!("Resizing child window");
                    embedder.set_window_size(&child_window, 500, 350);
                    demonstration_phase = 3;
                }
            },
            
            // Phase 3: Move the child window
            3 => {
                if frame_count == 240 {
                    println!("Moving child window");
                    embedder.set_window_position(&child_window, 100, 100);
                    demonstration_phase = 4;
                }
            },
            
            // Phase 4: Show popup window
            4 => {
                if frame_count == 300 {
                    println!("Showing popup window");
                    embedder.show_window(&popup_window);
                    demonstration_phase = 5;
                }
            },
            
            // Phase 5: Move popup window
            5 => {
                if frame_count == 360 {
                    println!("Moving popup window");
                    embedder.set_window_position(&popup_window, 400, 300);
                    demonstration_phase = 6;
                }
            },
            
            // Phase 6: Hide popup window
            6 => {
                if frame_count == 420 {
                    println!("Hiding popup window");
                    embedder.hide_window(&popup_window);
                    demonstration_phase = 7;
                }
            },
            
            // Phase 7: Demonstrate getting window properties
            7 => {
                if frame_count == 480 {
                    let main_size = embedder.get_window_size(&main_window);
                    let main_pos = embedder.get_window_position(&main_window);
                    let child_size = embedder.get_window_size(&child_window);
                    let child_pos = embedder.get_window_position(&child_window);
                    
                    println!("Main window size: {}x{}, position: ({}, {})", 
                             main_size.0, main_size.1, main_pos.0, main_pos.1);
                    println!("Child window size: {}x{}, position: ({}, {})", 
                             child_size.0, child_size.1, child_pos.0, child_pos.1);
                    
                    demonstration_phase = 8;
                }
            },
            
            // Phase 8: Finish demonstration
            8 => {
                if frame_count == 540 {
                    println!("Demonstration completed");
                    break;
                }
            },
            
            _ => {}
        }
        
        // Request window redraws - these use Embedder trait which takes value
        if frame_count % 60 == 0 {
            println!("Frame count: {}", frame_count);
            embedder.request_redraw(main_window);
            embedder.request_redraw(child_window);
            
            if demonstration_phase >= 4 && demonstration_phase < 7 {
                embedder.request_redraw(popup_window);
            }
        }
        
        // Small sleep to avoid maxing CPU
        sleep(Duration::from_millis(16)); // ~60 FPS
        
        frame_count += 1;
    }
    
    // Clean up
    println!("Cleaning up windows");
    embedder.destroy_window(&popup_window);
    embedder.destroy_window(&child_window);
    embedder.destroy_window(&main_window);
    embedder.shutdown();
    
    println!("Example completed successfully");
} 