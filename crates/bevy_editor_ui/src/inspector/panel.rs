//! Inspector panel UI generation
//!
//! Displays properties of the selected entity including components and their values.

use bevy::prelude::*;
use bevy::math::EulerRot;
use bevy::feathers::controls::checkbox;
use bevy::ui::Checked;
use bevy::ui_widgets::ValueChange;
use bevy::ecs::spawn::Spawn;
use bevy_editor_core::{EditorSelection, EditorEntity};
use super::transform_editor::{TransformEditor, TransformField};
use super::sprite_editor::{SpriteFlipXCheckbox, SpriteFlipYCheckbox, SpriteTextureButton};

/// Marker component for the Inspector panel content area
#[derive(Component)]
pub struct InspectorPanel;

/// Update the Inspector panel with the currently selected entity's information
pub fn update_inspector_panel(
    mut commands: Commands,
    inspector_query: Query<(Entity, Option<&Children>), With<InspectorPanel>>,
    selection: Res<EditorSelection>,
    world: &World,
) {
    // Only update if selection changed
    if !selection.is_changed() {
        return;
    }

    let Ok((inspector_entity, children_opt)) = inspector_query.single() else {
        return;
    };

    // Clear existing inspector content by despawning all children
    if let Some(children) = children_opt {
        for child in children {
            commands.entity(*child).despawn();
        }
    }

    // Get the primary selected entity
    let Some(selected_entity) = selection.primary() else {
        // No selection - show empty state
        commands.entity(inspector_entity).with_children(|inspector| {
            inspector.spawn((
                Text::new("No entity selected"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
            ));
        });
        return;
    };

    // Check if entity still exists
    if !world.entities().contains(selected_entity) {
        commands.entity(inspector_entity).with_children(|inspector| {
            inspector.spawn((
                Text::new("Selected entity no longer exists"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.3, 0.3)),
                Node {
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
            ));
        });
        return;
    }

    let entity_ref = world.entity(selected_entity);

    // Build inspector UI
    commands.entity(inspector_entity).with_children(|inspector| {
        // Entity header section
        inspector.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::bottom(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|header| {
            // Entity name or ID
            let entity_name = entity_ref
                .get::<Name>()
                .map(|n| n.as_str().to_string())
                .unwrap_or_else(|| format!("Entity {}", selected_entity.index()));

            header.spawn((
                Text::new(&entity_name),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
            ));

            // Entity ID
            header.spawn((
                Text::new(&format!("ID: {:?}", selected_entity)),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });

        // Components section
        inspector.spawn((
            Text::new("Components"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
        ));

        // Special handling for Transform component
        if let Some(transform) = entity_ref.get::<Transform>() {
            inspector.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::vertical(Val::Px(2.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|component_ui| {
                // Component header
                component_ui.spawn((
                    Text::new("Transform"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.9, 1.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(6.0)),
                        ..default()
                    },
                ));

                // Translation (Position)
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Position:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // X field
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::PositionX,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("X: {:.2}", transform.translation.x)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.5, 0.5)),
                        ));
                    });

                    // Y field
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::PositionY,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Y: {:.2}", transform.translation.y)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                        ));
                    });

                    // Z field
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::PositionZ,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Z: {:.2}", transform.translation.z)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 1.0)),
                        ));
                    });
                });

                // Rotation (convert to euler angles for readability)
                let (rot_x, rot_y, rot_z) = transform.rotation.to_euler(EulerRot::XYZ);
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Rotation:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // X rotation
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::RotationX,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("X: {:.1}°", rot_x.to_degrees())),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.5, 0.5)),
                        ));
                    });

                    // Y rotation
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::RotationY,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Y: {:.1}°", rot_y.to_degrees())),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                        ));
                    });

                    // Z rotation
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::RotationZ,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Z: {:.1}°", rot_z.to_degrees())),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 1.0)),
                        ));
                    });
                });

                // Scale
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Scale:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // X scale
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::ScaleX,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("X: {:.2}", transform.scale.x)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.5, 0.5)),
                        ));
                    });

                    // Y scale
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::ScaleY,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Y: {:.2}", transform.scale.y)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                        ));
                    });

                    // Z scale
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            min_width: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                        TransformEditor {
                            target_entity: selected_entity,
                            field: TransformField::ScaleZ,
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&format!("Z: {:.2}", transform.scale.z)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 1.0)),
                        ));
                    });
                });
            });
        }

        // Special handling for Sprite component
        if let Some(sprite) = entity_ref.get::<Sprite>() {
            inspector.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::vertical(Val::Px(2.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|component_ui| {
                // Component header
                component_ui.spawn((
                    Text::new("Sprite"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.9, 1.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(6.0)),
                        ..default()
                    },
                ));

                // Color swatch with RGBA values
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Color:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // Color swatch
                    row.spawn((
                        Node {
                            width: Val::Px(30.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(sprite.color),
                        BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                    ));

                    // RGBA values
                    let [r, g, b] = sprite.color.to_srgba().to_u8_array_no_alpha();
                    row.spawn((
                        Text::new(&format!("R:{} G:{} B:{} A:{:.2}", r, g, b, sprite.color.alpha())),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });

                // Texture selection button
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Texture:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // Texture select button
                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                        BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                        SpriteTextureButton {
                            target_entity: selected_entity,
                        },
                        EditorEntity,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Select Image..."),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });
                });

                // Flip toggles (feathers checkboxes)
                component_ui.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Flip:"),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));

                    // Flip X checkbox using feathers
                    let flip_x_checked = sprite.flip_x;
                    let flip_x_entity = selected_entity;
                    let mut checkbox_x = row.spawn((
                        checkbox(
                            (),
                            Spawn((
                                Text::new("Flip X"),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                            ))
                        ),
                        SpriteFlipXCheckbox {
                            target_entity: flip_x_entity,
                        },
                    ));
                    if flip_x_checked {
                        checkbox_x.insert(Checked);
                    }
                    checkbox_x.observe(move |trigger: On<ValueChange<bool>>,
                                   mut sprite_query: Query<&mut Sprite>| {
                        if let Ok(mut sprite) = sprite_query.get_mut(flip_x_entity) {
                            sprite.flip_x = trigger.event().value;
                            info!("Set flip_x: {}", sprite.flip_x);
                        }
                    });

                    // Flip Y checkbox using feathers
                    let flip_y_checked = sprite.flip_y;
                    let flip_y_entity = selected_entity;
                    let mut checkbox_y = row.spawn((
                        checkbox(
                            (),
                            Spawn((
                                Text::new("Flip Y"),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                            ))
                        ),
                        SpriteFlipYCheckbox {
                            target_entity: flip_y_entity,
                        },
                    ));
                    if flip_y_checked {
                        checkbox_y.insert(Checked);
                    }
                    checkbox_y.observe(move |trigger: On<ValueChange<bool>>,
                                   mut sprite_query: Query<&mut Sprite>| {
                        if let Ok(mut sprite) = sprite_query.get_mut(flip_y_entity) {
                            sprite.flip_y = trigger.event().value;
                            info!("Set flip_y: {}", sprite.flip_y);
                        }
                    });
                });

                // Custom size (if set)
                if let Some(size) = sprite.custom_size {
                    component_ui.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(2.0)),
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                    ))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Size:"),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        ));

                        row.spawn((
                            Text::new(&format!("{:.1} x {:.1}", size.x, size.y)),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    });
                }
            });
        }

        // List other components
        let archetype = entity_ref.archetype();
        for component_id in archetype.components() {
            if let Some(component_info) = world.components().get_info(*component_id) {
                // Use debug formatting to get the name as a string
                let component_name = format!("{:?}", component_info.name());

                // Skip editor-specific components and specially handled components
                if component_name.starts_with("bevy_editor")
                    || component_name.contains("Transform")
                    || component_name.contains("Sprite") {
                    continue;
                }

                // Create component entry
                inspector.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::vertical(Val::Px(2.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                    BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                ))
                .with_children(|component_ui| {
                    // Component name
                    component_ui.spawn((
                        Text::new(&component_name),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                    ));
                });
            }
        }
    });
}
