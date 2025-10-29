//! Project and scene management

use bevy::prelude::*;

pub mod project;
pub mod scene_format;

pub use project::*;
pub use scene_format::*;

/// Plugin for project management
pub struct EditorProjectPlugin;

impl Plugin for EditorProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentProject>()
            .add_systems(Update, auto_save_project);
    }
}

fn auto_save_project() {
    // Placeholder for auto-save functionality
}
