//! Component add/remove UI

use bevy::prelude::*;

/// Component to display list of all registered components
#[derive(Component)]
pub struct ComponentPalette {
    pub filter: String,
}
