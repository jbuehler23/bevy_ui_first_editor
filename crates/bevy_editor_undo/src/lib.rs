//! Undo/redo system using command pattern

use bevy::prelude::*;

pub mod commands;

pub use commands::*;

/// Plugin for undo/redo functionality
pub struct EditorUndoPlugin;

impl Plugin for EditorUndoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandHistory>()
            .add_message::<UndoEvent>()
            .add_message::<RedoEvent>()
            .add_systems(Update, (handle_undo_events, handle_redo_events));
    }
}

/// Message to trigger an undo
#[derive(Message)]
pub struct UndoEvent;

/// Message to trigger a redo
#[derive(Message)]
pub struct RedoEvent;

/// Trait for undoable commands
pub trait Command: Send + Sync {
    /// Execute the command
    fn execute(&mut self, world: &mut World);

    /// Undo the command
    fn undo(&mut self, world: &mut World);

    /// Redo the command (default: calls execute)
    fn redo(&mut self, world: &mut World) {
        self.execute(world);
    }

    /// Get command name for UI display
    fn name(&self) -> &str;

    /// Check if this command can be merged with another
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }

    /// Merge this command with another
    fn merge(&mut self, _other: Box<dyn Command>) {}
}

/// Manages command history for undo/redo
#[derive(Resource)]
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_history: usize,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 100,
        }
    }
}

impl CommandHistory {
    pub fn execute(&mut self, mut command: Box<dyn Command>, world: &mut World) {
        command.execute(world);

        // Try to merge with previous command
        if let Some(last) = self.undo_stack.last_mut() {
            if last.can_merge(&*command) {
                last.merge(command);
                self.redo_stack.clear();
                return;
            }
        }

        self.undo_stack.push(command);
        self.redo_stack.clear();

        // Limit history size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self, world: &mut World) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(world);
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self, world: &mut World) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            command.redo(world);
            self.undo_stack.push(command);
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

fn handle_undo_events(
    mut events: MessageReader<UndoEvent>,
    history: Res<CommandHistory>,
) {
    for _ in events.read() {
        // TODO: Implement undo with commands
        // We'll need to refactor this to use Bevy's command system
        // or deferred world access
    }
}

fn handle_redo_events(
    mut events: MessageReader<RedoEvent>,
    history: Res<CommandHistory>,
) {
    for _ in events.read() {
        // TODO: Implement redo with commands
        // We'll need to refactor this to use Bevy's command system
        // or deferred world access
    }
}
