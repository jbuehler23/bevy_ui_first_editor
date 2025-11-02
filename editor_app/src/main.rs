//! Bevy Native Editor
//!
//! A modular, plugin-based game editor built entirely with Bevy and bevy_ui.

use bevy::feathers::FeathersPlugins;
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;

// Import all editor plugins
use bevy_editor_assets::EditorAssetsPlugin;
use bevy_editor_core::EditorCorePlugin;
use bevy_editor_hierarchy::EditorHierarchyPlugin;
use bevy_editor_inspector::EditorInspectorPlugin;
use bevy_editor_project::EditorProjectPlugin;
use bevy_editor_ui::EditorUiPlugin;
use bevy_editor_undo::EditorUndoPlugin;
use bevy_editor_viewport::EditorViewportPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Editor".into(),
                resolution: (1600, 900).into(),
                ..default()
            }),
            ..default()
        }))
        // Add feathers UI toolkit
        .add_plugins(FeathersPlugins)
        // Add mesh picking backend for entity selection
        // Note: DefaultPlugins in Bevy main now includes DefaultPickingPlugins
        .add_plugins(MeshPickingPlugin)
        // Add all editor plugins
        // IMPORTANT: EditorViewportPlugin MUST come before EditorUiPlugin to ensure
        // proper camera initialization and render graph setup
        .add_plugins((
            EditorCorePlugin,
            EditorViewportPlugin,
            EditorUiPlugin,
            EditorHierarchyPlugin,
            EditorInspectorPlugin,
            EditorAssetsPlugin,
            EditorUndoPlugin,
            EditorProjectPlugin,
        ))
        .run()
}
