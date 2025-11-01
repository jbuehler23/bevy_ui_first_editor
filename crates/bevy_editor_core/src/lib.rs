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

/// Marker component for entities that are part of the editor infrastructure
/// These entities should not appear in the scene tree or be saved with the scene
#[derive(Component, Default, Clone, Copy)]
pub struct EditorEntity;

/// Resource tracking which UI element has keyboard focus
#[derive(Resource, Default)]
pub struct UiFocus {
    /// The currently focused UI element (if any)
    pub focused_entity: Option<Entity>,
}

/// Core editor plugin that sets up the fundamental editor infrastructure
pub struct EditorCorePlugin;

impl Plugin for EditorCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .init_resource::<EditorSelection>()
            .init_resource::<UiFocus>()
            .add_systems(Update, update_editor_state);
    }
}

fn update_editor_state() {
    // Placeholder for editor state management
}
