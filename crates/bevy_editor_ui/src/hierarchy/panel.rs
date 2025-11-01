//! Scene tree panel rendering system
//!
//! Builds and updates the visual hierarchy tree with entity rows,
//! including visibility toggles, expand/collapse indicators, and selection highlighting.

use bevy::prelude::*;
use bevy::picking::Pickable;
use bevy_editor_core::{EditorSelection, EditorEntity};
use bevy_editor_hierarchy::{HierarchyState, build_entity_tree_flat, EntityTreeRow};
use crate::{SceneTreePanel, VisibilityToggleButton, EntityNameText, EditorIcons};

/// Update the Scene Tree panel with the current entity hierarchy
pub fn update_scene_tree_panel(
    mut commands: Commands,
    scene_tree_query: Query<Entity, With<SceneTreePanel>>,
    hierarchy_state: Res<HierarchyState>,
    selection: Res<EditorSelection>,
    icons: Res<EditorIcons>,
    world: &World,
    all_entities: Query<(Entity, Option<&Name>)>,
    children_query: Query<&Children>,
    entity_row_query: Query<Entity, With<EntityTreeRow>>,
    mut needs_initial_update: Local<bool>,
) {
    let Ok(panel_entity) = scene_tree_query.single() else {
        return;
    };

    // Check if panel is empty (needs initial population)
    let is_empty = children_query
        .get(panel_entity)
        .map(|children| !children.iter().any(|c| entity_row_query.contains(c)))
        .unwrap_or(true);

    // Only update if hierarchy state or selection changed, OR if panel is empty
    if !is_empty && !hierarchy_state.is_changed() && !selection.is_changed() {
        return;
    }

    // Mark that we've done the initial update
    if is_empty {
        *needs_initial_update = false;
    }

    // Collect all entities with their names
    let entities_data: Vec<(Entity, Option<String>)> = all_entities
        .iter()
        .map(|(entity, name)| (entity, name.map(|n| n.as_str().to_string())))
        .collect();

    // Build the entity tree
    let tree_entities = build_entity_tree_flat(world, &hierarchy_state, &entities_data);

    // Clear existing tree rows (keep the title)
    if let Ok(children) = children_query.get(panel_entity) {
        for child in children.iter() {
            // Only despawn entity tree rows, not the title
            if entity_row_query.contains(child) {
                commands.entity(child).despawn();
            }
        }
    }

    // Spawn new tree rows
    commands.entity(panel_entity).with_children(|parent| {
        for tree_entity in tree_entities {
            let indent = tree_entity.depth as f32 * 16.0; // 16px per depth level
            let is_selected = selection.is_selected(tree_entity.entity);

            // Row background color
            let bg_color = if is_selected {
                Color::srgb(0.3, 0.5, 0.8) // Blue for selected
            } else {
                Color::srgb(0.18, 0.18, 0.18) // Slightly lighter than panel
            };

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    padding: UiRect::new(Val::Px(indent + 4.0), Val::Px(4.0), Val::Px(2.0), Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(bg_color),
                EntityTreeRow {
                    entity: tree_entity.entity,
                    depth: tree_entity.depth,
                },
                Pickable {
                    should_block_lower: true,  // Tree rows should be clickable
                    is_hoverable: true,
                },
                Button, // Make it clickable
                EditorEntity, // Mark tree row as editor entity
            ))
            .with_children(|row| {
                // Visibility toggle button (eye icon)
                let is_visible = world
                    .get::<Visibility>(tree_entity.entity)
                    .map(|v| matches!(v, Visibility::Visible | Visibility::Inherited))
                    .unwrap_or(true);

                let eye_icon = if is_visible {
                    icons.eye.clone()
                } else {
                    icons.eye_off.clone()
                };

                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)), // Transparent
                    VisibilityToggleButton {
                        target_entity: tree_entity.entity,
                    },
                    Pickable {
                        should_block_lower: true,
                        is_hoverable: true,
                    },
                ))
                .with_children(|button| {
                    button.spawn((
                        ImageNode::new(eye_icon),
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                    ));
                });

                // Expand/collapse indicator (if has children)
                if tree_entity.has_children {
                    let symbol = if hierarchy_state.expanded.contains(&tree_entity.entity) {
                        "▼"
                    } else {
                        "▶"
                    };
                    row.spawn((
                        Text::new(symbol),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        Node {
                            margin: UiRect::right(Val::Px(4.0)),
                            ..default()
                        },
                    ));
                } else {
                    // Spacer for entities without children
                    row.spawn(Node {
                        width: Val::Px(16.0),
                        ..default()
                    });
                }

                // Entity name (grayed out if hidden)
                let name_color = if is_visible {
                    Color::srgb(0.9, 0.9, 0.9)
                } else {
                    Color::srgb(0.5, 0.5, 0.5) // Gray for hidden
                };

                row.spawn((
                    Text::new(&tree_entity.name),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(name_color),
                    EntityNameText {
                        target_entity: tree_entity.entity,
                    },
                ));
            });
        }
    });
}
