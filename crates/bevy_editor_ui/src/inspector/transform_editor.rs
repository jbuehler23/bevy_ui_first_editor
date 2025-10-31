//! Transform property editor components and systems

use bevy::prelude::*;

/// Marker component for Transform property fields
#[derive(Component, Clone, Copy)]
pub enum TransformField {
    PositionX,
    PositionY,
    PositionZ,
    RotationX,
    RotationY,
    RotationZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

/// Tracks which entity's transform is being edited
#[derive(Component)]
pub struct TransformEditor {
    pub target_entity: Entity,
    pub field: TransformField,
}

/// Resource tracking the currently focused transform field for editing
#[derive(Resource, Default)]
pub struct TransformEditState {
    pub editing_field: Option<(Entity, TransformField)>,
    pub input_buffer: String,
}
