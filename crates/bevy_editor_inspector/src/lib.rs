//! Property inspector using bevy_reflect

use bevy::prelude::*;

pub mod property_editors;
pub mod component_list;

pub use property_editors::*;
pub use component_list::*;

/// Plugin for inspector functionality
pub struct EditorInspectorPlugin;

impl Plugin for EditorInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InspectorRegistry>()
            .add_systems(Update, update_inspector_ui);
    }
}

/// Registry for custom property editors
#[derive(Resource, Default)]
pub struct InspectorRegistry {
    // TODO: Store custom property drawer functions
}

fn update_inspector_ui() {
    // Placeholder
}
