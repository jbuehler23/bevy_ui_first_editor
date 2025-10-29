//! Entity hierarchy tree view panel

use bevy::prelude::*;

pub mod tree_view;
pub mod operations;

pub use tree_view::*;
pub use operations::*;

/// Plugin for hierarchy panel
pub struct EditorHierarchyPlugin;

impl Plugin for EditorHierarchyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HierarchyState>();
        // Note: The UI rendering happens in bevy_editor_ui crate
        // This plugin just provides the data structures and state
    }
}
