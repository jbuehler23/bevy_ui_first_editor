//! Mouse picking for entity selection

use bevy::prelude::*;

/// Result of a picking operation
#[derive(Debug, Clone)]
pub struct PickingResult {
    pub entity: Entity,
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

/// Component to mark entities as pickable in the editor
#[derive(Component)]
pub struct Pickable;
