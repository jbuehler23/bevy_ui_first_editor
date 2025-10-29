//! Custom UI widgets for the editor

use bevy::prelude::*;

/// Helper functions for building common UI patterns

/// Create a labeled input field
pub fn labeled_input(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    value: &str,
) -> Entity {
    // Placeholder - will be implemented with bevy_ui components
    commands.spawn(Node::default()).set_parent_in_place(parent).id()
}

/// Create a button with text
pub fn text_button(
    commands: &mut Commands,
    parent: Entity,
    text: &str,
) -> Entity {
    // Placeholder - will be implemented with bevy_ui components
    commands.spawn(Node::default()).set_parent_in_place(parent).id()
}

/// Common button style
pub fn button_style() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Px(32.0),
        padding: UiRect::all(Val::Px(8.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
