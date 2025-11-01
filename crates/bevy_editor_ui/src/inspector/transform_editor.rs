//! Transform property editor components and systems

use bevy::prelude::*;
use bevy::math::EulerRot;
use bevy::input::keyboard::{KeyCode, KeyboardInput};

/// Marker component for Transform property fields
#[derive(Component, Clone, Copy, PartialEq, Debug)]
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
#[derive(Component, Clone)]
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

/// Handle clicks on Transform editor buttons
pub fn handle_transform_editor_click(
    interactions: Query<(&Interaction, &TransformEditor), Changed<Interaction>>,
    mut edit_state: ResMut<TransformEditState>,
    transforms: Query<&Transform>,
) {
    for (interaction, editor) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            // Get current value to populate input buffer
            if let Ok(transform) = transforms.get(editor.target_entity) {
                let value = get_transform_field_value(transform, editor.field);
                edit_state.editing_field = Some((editor.target_entity, editor.field));
                edit_state.input_buffer = format!("{:.2}", value);
            }
        }
    }
}

/// Handle keyboard input for transform editing
pub fn handle_transform_edit_input(
    mut edit_state: ResMut<TransformEditState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: MessageReader<KeyboardInput>,
    mut transforms: Query<&mut Transform>,
) {
    if edit_state.editing_field.is_none() {
        return;
    }

    // Handle Enter to commit
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some((entity, field)) = edit_state.editing_field {
            if let Ok(value) = edit_state.input_buffer.parse::<f32>() {
                if let Ok(mut transform) = transforms.get_mut(entity) {
                    apply_transform_field_value(&mut transform, field, value);
                }
            }
        }
        edit_state.editing_field = None;
        edit_state.input_buffer.clear();
        return;
    }

    // Handle Escape to cancel
    if keyboard.just_pressed(KeyCode::Escape) {
        edit_state.editing_field = None;
        edit_state.input_buffer.clear();
        return;
    }

    // Handle Backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        edit_state.input_buffer.pop();
        return;
    }

    // Handle character input
    for event in char_events.read() {
        if let bevy::input::keyboard::Key::Character(ref s) = event.logical_key {
            // Only accept numbers, decimal point, and minus sign
            for ch in s.chars() {
                if ch.is_numeric() || ch == '.' || ch == '-' {
                    edit_state.input_buffer.push(ch);
                }
            }
        }
    }
}

/// Update button text to show current value or edit buffer
pub fn update_transform_editor_display(
    edit_state: Res<TransformEditState>,
    transforms: Query<&Transform>,
    mut editor_query: Query<(&TransformEditor, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    // Only update if edit state changed or if we're editing (to show typing)
    if !edit_state.is_changed() && edit_state.editing_field.is_none() {
        return;
    }

    for (editor, children) in editor_query.iter_mut() {
        // Find the Text child of this button
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                // Check if this is the field being edited
                if let Some((editing_entity, editing_field)) = edit_state.editing_field {
                    if editing_entity == editor.target_entity && editing_field == editor.field {
                        // Show input buffer while editing
                        let field_label = get_field_label(editor.field);
                        text.0 = format!("{}: {}_", field_label, edit_state.input_buffer);
                        continue;
                    }
                }

                // Otherwise show current transform value
                if let Ok(transform) = transforms.get(editor.target_entity) {
                    let value = get_transform_field_value(transform, editor.field);
                    let field_label = get_field_label(editor.field);
                    let precision = if matches!(editor.field, TransformField::RotationX | TransformField::RotationY | TransformField::RotationZ) {
                        1  // 1 decimal for rotation
                    } else {
                        2  // 2 decimals for position/scale
                    };
                    text.0 = format!("{}: {:.precision$}", field_label, value, precision = precision);
                }
            }
        }
    }
}

/// Get the current value of a transform field
fn get_transform_field_value(transform: &Transform, field: TransformField) -> f32 {
    match field {
        TransformField::PositionX => transform.translation.x,
        TransformField::PositionY => transform.translation.y,
        TransformField::PositionZ => transform.translation.z,
        TransformField::RotationX => {
            let (x, _, _) = transform.rotation.to_euler(EulerRot::XYZ);
            x.to_degrees()
        }
        TransformField::RotationY => {
            let (_, y, _) = transform.rotation.to_euler(EulerRot::XYZ);
            y.to_degrees()
        }
        TransformField::RotationZ => {
            let (_, _, z) = transform.rotation.to_euler(EulerRot::XYZ);
            z.to_degrees()
        }
        TransformField::ScaleX => transform.scale.x,
        TransformField::ScaleY => transform.scale.y,
        TransformField::ScaleZ => transform.scale.z,
    }
}

/// Apply a new value to a transform field
fn apply_transform_field_value(transform: &mut Transform, field: TransformField, value: f32) {
    match field {
        TransformField::PositionX => transform.translation.x = value,
        TransformField::PositionY => transform.translation.y = value,
        TransformField::PositionZ => transform.translation.z = value,
        TransformField::RotationX => {
            let (_, y, z) = transform.rotation.to_euler(EulerRot::XYZ);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, value.to_radians(), y, z);
        }
        TransformField::RotationY => {
            let (x, _, z) = transform.rotation.to_euler(EulerRot::XYZ);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, x, value.to_radians(), z);
        }
        TransformField::RotationZ => {
            let (x, y, _) = transform.rotation.to_euler(EulerRot::XYZ);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, value.to_radians());
        }
        TransformField::ScaleX => transform.scale.x = value,
        TransformField::ScaleY => transform.scale.y = value,
        TransformField::ScaleZ => transform.scale.z = value,
    }
}

/// Get the display label for a field
fn get_field_label(field: TransformField) -> &'static str {
    match field {
        TransformField::PositionX => "X",
        TransformField::PositionY => "Y",
        TransformField::PositionZ => "Z",
        TransformField::RotationX => "X",
        TransformField::RotationY => "Y",
        TransformField::RotationZ => "Z",
        TransformField::ScaleX => "X",
        TransformField::ScaleY => "Y",
        TransformField::ScaleZ => "Z",
    }
}
