//! Core editor framework
//!
//! Provides the fundamental architecture for the Bevy editor including:
//! - Dual-world architecture (EditorWorld + GameWorld)
//! - Editor state management
//! - Plugin system for editor extensions
//! - Common editor utilities and types

use bevy::prelude::*;

pub mod editor_state;
pub mod selection;

pub use editor_state::*;
pub use selection::*;

/// Core editor plugin that sets up the fundamental editor infrastructure
pub struct EditorCorePlugin;

impl Plugin for EditorCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .init_resource::<EditorSelection>()
            .add_systems(Update, update_editor_state);
    }
}

fn update_editor_state() {
    // Placeholder for editor state management
}
