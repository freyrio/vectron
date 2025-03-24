/*!
 * Low-level drawing API with minimal abstraction
 */

mod commands;
mod resources;
mod state;

// Re-export public types
pub use commands::{DrawCommand, DrawCommandType};
pub use resources::ResourceId;
pub use state::DrawState;

/// A list of drawing commands with associated state
pub struct DrawList {
    commands: Vec<DrawCommand>,
    state_stack: Vec<DrawState>,
    current_state: DrawState,
}

impl DrawList {
    /// Create a new empty draw list
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            state_stack: Vec::new(),
            current_state: DrawState::default(),
        }
    }
    
    /// Push the current state onto the stack
    pub fn push_state(&mut self) -> &mut Self {
        self.state_stack.push(self.current_state.clone());
        self
    }
    
    /// Pop a state from the stack
    pub fn pop_state(&mut self) -> &mut Self {
        if let Some(state) = self.state_stack.pop() {
            self.current_state = state;
        }
        self
    }
    
    /// Add a raw draw command
    pub fn add_command(&mut self, command: DrawCommand) -> &mut Self {
        self.commands.push(command);
        self
    }
    
    /// Get all commands in this draw list
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }
} 