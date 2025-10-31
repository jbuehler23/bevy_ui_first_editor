//! Layout persistence system
//!
//! Save and load docking layouts to/from JSON files.

use bevy::prelude::*;
use std::path::Path;
use super::DockingLayout;

/// Save the current docking layout to a file
pub fn save_layout(layout: &DockingLayout, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(layout)
        .map_err(|e| format!("Failed to serialize layout: {}", e))?;

    std::fs::write(path, json)
        .map_err(|e| format!("Failed to write layout file: {}", e))?;

    info!("Saved docking layout to {:?}", path);
    Ok(())
}

/// Load a docking layout from a file
pub fn load_layout(path: &Path) -> Result<DockingLayout, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read layout file: {}", e))?;

    let layout = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to deserialize layout: {}", e))?;

    info!("Loaded docking layout from {:?}", path);
    Ok(layout)
}

/// System to save layout on exit (triggered by Ctrl+S or editor close)
pub fn auto_save_layout(
    layout: Res<DockingLayout>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Save on Ctrl+Shift+S
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            if keyboard.just_pressed(KeyCode::KeyS) {
                let path = Path::new("editor_layout.json");
                if let Err(e) = save_layout(&layout, path) {
                    error!("Failed to save layout: {}", e);
                } else {
                    info!("Layout saved successfully");
                }
            }
        }
    }
}

/// System to load layout on startup
pub fn auto_load_layout(
    mut layout: ResMut<DockingLayout>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    *loaded = true;

    let path = Path::new("editor_layout.json");
    if path.exists() {
        match load_layout(path) {
            Ok(loaded_layout) => {
                *layout = loaded_layout;
                layout.set_changed();  // Force change detection for renderer
                info!("Loaded saved layout");
            }
            Err(e) => {
                warn!("Failed to load layout, using default: {}", e);
                layout.set_changed();  // Force change detection even with default layout
            }
        }
    } else {
        info!("No saved layout found, using default");
        layout.set_changed();  // Force change detection for default layout
    }
}
