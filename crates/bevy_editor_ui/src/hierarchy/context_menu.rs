//! Reusable right-click context menu system
//!
//! Provides a context menu that can be used for any UI element.
//! Currently used for the hierarchy tree but designed to be reusable.

use bevy::prelude::*;
use bevy::picking::Pickable;
use bevy_editor_hierarchy::EntityTreeRow;
use bevy_editor_core::EditorEntity;

/// Marker component for the context menu container
#[derive(Component)]
pub struct ContextMenu {
    /// The entity this context menu is for
    pub target_entity: Entity,
}

/// Context menu action types
#[derive(Component, Clone, Copy)]
pub enum ContextMenuAction {
    Delete,
    Duplicate,
    AddChild,
    Rename,
}

/// Handle right-clicks on tree rows to open context menu
pub fn handle_tree_row_right_clicks(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    tree_row_query: Query<(&Interaction, &EntityTreeRow), With<Button>>,
    existing_menu_query: Query<Entity, With<ContextMenu>>,
    windows: Query<&Window>,
) {
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    // Close any existing context menu first
    for menu_entity in &existing_menu_query {
        commands.entity(menu_entity).despawn();
    }

    // Check if we right-clicked on a tree row
    for (interaction, tree_row) in &tree_row_query {
        if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
            // Get cursor position
            if let Ok(window) = windows.single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    // Spawn context menu at cursor position
                    spawn_context_menu(&mut commands, tree_row.entity, cursor_pos);
                }
            }
            break;
        }
    }
}

/// Spawn a context menu for an entity
fn spawn_context_menu(commands: &mut Commands, target_entity: Entity, position: Vec2) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                width: Val::Px(150.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
            ContextMenu { target_entity },
            EditorEntity,
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            },
        ))
        .with_children(|menu| {
            // Delete action
            menu.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ContextMenuAction::Delete,
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Delete"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

            // Duplicate action
            menu.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ContextMenuAction::Duplicate,
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Duplicate"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

            // Add Child action
            menu.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ContextMenuAction::AddChild,
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Add Child"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

            // Rename action
            menu.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ContextMenuAction::Rename,
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Rename"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });
        });
}

/// Handle context menu action clicks
pub fn handle_context_menu_actions(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ContextMenuAction, &ChildOf), (Changed<Interaction>, With<Button>)>,
    menu_query: Query<&ContextMenu>,
) {
    for (interaction, action, child_of) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Get the menu to find target entity (parent of the button is the menu)
            let parent_entity = child_of.parent();
            if let Ok(menu) = menu_query.get(parent_entity) {
                let target_entity = menu.target_entity;

                // Execute action based on type
                match action {
                    ContextMenuAction::Delete => {
                        // Despawn the target entity
                        commands.entity(target_entity).despawn();
                    }
                    ContextMenuAction::Duplicate => {
                        // TODO: Implement duplication
                        println!("Duplicate entity {:?}", target_entity);
                    }
                    ContextMenuAction::AddChild => {
                        // TODO: Implement add child
                        println!("Add child to entity {:?}", target_entity);
                    }
                    ContextMenuAction::Rename => {
                        // TODO: Implement rename
                        println!("Rename entity {:?}", target_entity);
                    }
                }

                // Close the context menu after action
                if menu_query.get(parent_entity).is_ok() {
                    // Find the menu entity itself (parent of this button)
                    commands.entity(parent_entity).despawn();
                }
            }
        }
    }
}

/// Close context menu when clicking outside of it
pub fn close_context_menu_on_click_outside(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    menu_query: Query<(Entity, &Interaction), With<ContextMenu>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // Check if we clicked outside the menu
        for (menu_entity, interaction) in &menu_query {
            if !matches!(interaction, Interaction::Hovered) {
                commands.entity(menu_entity).despawn();
            }
        }
    }
}
