//! Icon asset management for editor UI
//!
//! Loads PNG icon images converted from Tabler SVG icons.

use bevy::prelude::*;

/// Resource holding handles to editor UI icon images
#[derive(Resource)]
pub struct EditorIcons {
    pub eye: Handle<Image>,
    pub eye_off: Handle<Image>,
    pub x: Handle<Image>,
}

/// Load editor UI icon assets at startup
pub fn load_editor_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icons = EditorIcons {
        eye: asset_server.load("icons/eye.png"),
        eye_off: asset_server.load("icons/eye-off.png"),
        x: asset_server.load("icons/x.png"),
    };

    commands.insert_resource(icons);
}
