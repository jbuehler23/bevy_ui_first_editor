//! Panel trait and management

use bevy::prelude::*;

/// Trait for editor panels
pub trait EditorPanel: Send + Sync + 'static {
    /// Unique identifier for this panel
    fn id(&self) -> &str;

    /// Display title
    fn title(&self) -> &str;

    /// Build the panel UI
    fn ui(&mut self, world: &mut World, parent: Entity);

    /// Whether this panel is open by default
    fn default_open(&self) -> bool {
        false
    }

    /// Optional keyboard shortcut to toggle this panel
    fn shortcut(&self) -> Option<KeyCode> {
        None
    }
}

/// Component to mark a panel UI root entity
#[derive(Component)]
pub struct PanelRoot {
    pub panel_id: String,
}
