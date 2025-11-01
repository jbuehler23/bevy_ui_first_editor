//! Visibility toggle system for entities in the hierarchy
//!
//! Provides eye icon buttons to show/hide entities and updates visual feedback.

use bevy::prelude::*;
use crate::EditorIcons;

/// Marker component for visibility toggle buttons in the tree
#[derive(Component)]
pub struct VisibilityToggleButton {
    /// The entity this button controls visibility for
    pub target_entity: Entity,
}

/// Marker component for entity name text in tree rows
#[derive(Component)]
pub struct EntityNameText {
    /// The entity this text represents
    pub target_entity: Entity,
}

/// Handle clicks on visibility toggle buttons (eye icons)
pub fn handle_visibility_toggle_clicks(
    interaction_query: Query<(&Interaction, &VisibilityToggleButton), (Changed<Interaction>, With<Button>)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (interaction, toggle_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Toggle visibility of the target entity
            if let Ok(mut visibility) = visibility_query.get_mut(toggle_button.target_entity) {
                *visibility = match *visibility {
                    Visibility::Visible => Visibility::Hidden,
                    Visibility::Hidden => Visibility::Visible,
                    Visibility::Inherited => Visibility::Hidden,
                };
            }
        }
    }
}

/// Update the visual appearance of tree rows based on visibility changes
pub fn update_tree_row_visibility_appearance(
    visibility_changed: Query<Entity, Changed<Visibility>>,
    visibility_query: Query<&Visibility>,
    icons: Res<EditorIcons>,
    // Update eye icons
    mut toggle_buttons: Query<(&VisibilityToggleButton, &Children)>,
    mut button_images: Query<&mut ImageNode>,
    // Update entity name colors
    mut name_text: Query<(&EntityNameText, &mut TextColor)>,
) {
    // Check if any visibility changed
    if visibility_changed.is_empty() {
        return;
    }

    // Update eye icons for visibility toggle buttons
    for (toggle_button, children) in &mut toggle_buttons {
        if let Ok(visibility) = visibility_query.get(toggle_button.target_entity) {
            let is_visible = matches!(visibility, Visibility::Visible | Visibility::Inherited);
            let eye_icon = if is_visible {
                icons.eye.clone()
            } else {
                icons.eye_off.clone()
            };

            // Find the ImageNode child and update it
            for child in children.iter() {
                if let Ok(mut image_node) = button_images.get_mut(child) {
                    image_node.image = eye_icon.clone();
                }
            }
        }
    }

    // Update entity name text colors
    for (name_text_marker, mut text_color) in &mut name_text {
        if let Ok(visibility) = visibility_query.get(name_text_marker.target_entity) {
            let is_visible = matches!(visibility, Visibility::Visible | Visibility::Inherited);
            text_color.0 = if is_visible {
                Color::srgb(0.9, 0.9, 0.9)
            } else {
                Color::srgb(0.5, 0.5, 0.5) // Gray for hidden
            };
        }
    }
}
