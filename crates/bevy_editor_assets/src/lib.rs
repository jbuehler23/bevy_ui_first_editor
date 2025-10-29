//! Asset browser and management

use bevy::prelude::*;

pub mod browser;
pub mod thumbnails;
pub mod watcher;

pub use browser::*;
pub use thumbnails::*;
pub use watcher::*;

/// Plugin for asset management
pub struct EditorAssetsPlugin;

impl Plugin for EditorAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetBrowserState>()
            .add_systems(Update, (update_asset_browser, watch_file_system));
    }
}

fn update_asset_browser() {
    // Placeholder
}

fn watch_file_system() {
    // Placeholder
}
