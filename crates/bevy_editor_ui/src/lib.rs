//! Native bevy_ui-based editor UI framework
//!
//! Provides docking, panels, and UI widgets using only bevy_ui.

use bevy::prelude::*;
use bevy::picking::Pickable;
use bevy_editor_hierarchy::{HierarchyState, build_entity_tree_flat, EntityTreeRow};
use bevy_editor_core::{EditorSelection, EditorEntity};

pub mod docking;
pub mod panel;
pub mod widgets;

pub use docking::*;
pub use panel::*;
pub use widgets::*;

/// Marker component for editor panel UI nodes
#[derive(Component)]
pub struct EditorPanel {
    pub name: String,
}

/// Marker component for the Scene Tree panel content area
#[derive(Component)]
pub struct SceneTreePanel;

/// Marker component for the Inspector panel content area
#[derive(Component)]
pub struct InspectorPanel;

/// Marker component for the Assets panel content area
#[derive(Component)]
pub struct AssetsPanel;

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

/// Marker component for the search input box
#[derive(Component)]
pub struct SearchInputBox;

/// Marker component for the search text display
#[derive(Component)]
pub struct SearchInputText;

/// Marker component for the clear search button
#[derive(Component)]
pub struct ClearSearchButton;

/// Plugin for the native bevy_ui editor UI system
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DockingLayout>()
            .add_systems(Startup, setup_editor_ui)
            .add_systems(Update, (
                update_docking_layout,
                handle_panel_resize,
                handle_tree_row_clicks,
                handle_visibility_toggle_clicks,
                handle_search_input,
                handle_clear_search_button,
                update_scene_tree_panel,
                update_tree_row_visibility_appearance,
            ));
    }
}

fn setup_editor_ui(mut commands: Commands) {
    // Root container - Column layout for top content + bottom asset browser
    // CRITICAL: Pickable with should_block_lower: false allows clicks through to 3D viewport
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Pickable {
                should_block_lower: false,  // Let clicks through to 3D scene
                is_hoverable: true,          // Panel can still respond to interactions
            },
            EditorEntity, // Mark as editor entity
        ))
        .with_children(|root| {
            // Top content row (viewport + right sidebar)
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
            ))
            .with_children(|content_row| {
                // Viewport area (center/left) - no UI nodes, allows 3D picking
                // This transparent spacer ensures the 3D scene is visible and clickable
                content_row.spawn((
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(100.0),
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: false,  // Spacer doesn't need interaction
                    },
                ));

                // Right sidebar - contains Scene Tree and Inspector stacked vertically
                content_row
                    .spawn((
                        Node {
                            width: Val::Px(350.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                    ))
                    .with_children(|sidebar| {
                        // Scene Tree panel (top right)
                        sidebar.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                flex_grow: 1.0,
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                            EditorPanel {
                                name: "Scene Tree".to_string(),
                            },
                            Pickable {
                                should_block_lower: false,
                                is_hoverable: true,
                            },
                            SceneTreePanel,  // Marker for query
                        ))
                        .with_children(|panel| {
                            // Search input row
                            panel.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(28.0),
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|search_row| {
                                // Search input box
                                search_row.spawn((
                                    Node {
                                        width: Val::Auto,
                                        height: Val::Percent(100.0),
                                        flex_grow: 1.0,
                                        padding: UiRect::all(Val::Px(4.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                                    SearchInputBox,
                                    Button, // Make it clickable
                                    Pickable {
                                        should_block_lower: true,
                                        is_hoverable: true,
                                    },
                                    EditorEntity,
                                ))
                                .with_children(|input_box| {
                                    input_box.spawn((
                                        Text::new("Search..."),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                        SearchInputText,
                                    ));
                                });

                                // Clear button (X)
                                search_row.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(24.0),
                                        height: Val::Percent(100.0),
                                        margin: UiRect::left(Val::Px(4.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                                    ClearSearchButton,
                                    Pickable {
                                        should_block_lower: true,
                                        is_hoverable: true,
                                    },
                                    EditorEntity,
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new("✕"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                });
                            });

                            // Panel title
                            panel.spawn((
                                Text::new("Scene Tree"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));
                        });

                        // Inspector panel (bottom right)
                        sidebar.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                flex_grow: 1.0,
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                            EditorPanel {
                                name: "Inspector".to_string(),
                            },
                            Pickable {
                                should_block_lower: false,
                                is_hoverable: true,
                            },
                        ))
                        .with_children(|panel| {
                            // Panel title
                            panel.spawn((
                                Text::new("Inspector"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));
                        });
                    });
            });

            // Asset Browser panel (bottom, full width)
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(200.0),
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
                BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                EditorPanel {
                    name: "Assets".to_string(),
                },
                Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
            ))
            .with_children(|panel| {
                // Panel title
                panel.spawn((
                    Text::new("Assets"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                ));
            });
        });
}

fn update_docking_layout() {
    // Placeholder for docking system
}

fn handle_panel_resize() {
    // Placeholder for panel resizing
}

/// Handle clicks on visibility toggle buttons
fn handle_visibility_toggle_clicks(
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

/// Handle clicks on tree rows for selection and expand/collapse
fn handle_tree_row_clicks(
    interaction_query: Query<(&Interaction, &EntityTreeRow), (Changed<Interaction>, With<Button>)>,
    mut selection: ResMut<EditorSelection>,
    mut hierarchy_state: ResMut<HierarchyState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, tree_row) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let entity = tree_row.entity;

            // Check for modifier keys
            let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
            let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

            // Handle selection with modifier keys
            if ctrl {
                // Toggle selection
                selection.toggle(entity);
            } else if shift {
                // Range selection (TODO: implement properly)
                selection.add(entity);
            } else {
                // Single selection
                selection.select(entity);

                // Toggle expand/collapse when clicking on entity with children
                if hierarchy_state.expanded.contains(&entity) {
                    hierarchy_state.expanded.remove(&entity);
                } else {
                    hierarchy_state.expanded.insert(entity);
                }
            }
        }
    }
}

/// Handle keyboard input for search box
fn handle_search_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    search_box_query: Query<&Interaction, (With<SearchInputBox>, Changed<Interaction>)>,
    mut hierarchy_state: ResMut<HierarchyState>,
    mut search_text_query: Query<&mut Text, With<SearchInputText>>,
) {
    // Check if search box was clicked (becomes focused)
    let mut is_focused = false;
    for interaction in &search_box_query {
        if *interaction == Interaction::Pressed {
            is_focused = true;
        }
    }

    // Simple approach: always capture input when user types (basic implementation)
    // Handle backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        hierarchy_state.search_filter.pop();
    }

    // Handle character input
    for event in char_events.read() {
        if let bevy::input::keyboard::KeyboardInput {
            key_code: key,
            state: bevy::input::ButtonState::Pressed,
            ..
        } = event
        {
            // Convert keycode to character (simplified)
            if let Some(ch) = keycode_to_char(*key, keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)) {
                hierarchy_state.search_filter.push(ch);
            }
        }
    }

    // Update search text display
    for mut text in &mut search_text_query {
        if hierarchy_state.search_filter.is_empty() {
            **text = "Search...".to_string();
        } else {
            **text = hierarchy_state.search_filter.clone();
        }
    }
}

/// Convert keycode to character (simplified implementation)
fn keycode_to_char(key: KeyCode, shift: bool) -> Option<char> {
    match key {
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}

/// Handle clear search button clicks
fn handle_clear_search_button(
    interaction_query: Query<&Interaction, (With<ClearSearchButton>, Changed<Interaction>)>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            hierarchy_state.search_filter.clear();
        }
    }
}

/// Update tree row appearance when entity visibility changes
fn update_tree_row_visibility_appearance(
    visibility_changed: Query<Entity, Changed<Visibility>>,
    visibility_query: Query<&Visibility>,
    // Update eye icons
    mut toggle_buttons: Query<(&VisibilityToggleButton, &Children)>,
    mut button_text: Query<&mut Text>,
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
            let eye_symbol = if is_visible { "👁" } else { "🚫" };

            // Find the text child and update it
            for child in children.iter() {
                if let Ok(mut text) = button_text.get_mut(child) {
                    **text = eye_symbol.to_string();
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

/// Update the Scene Tree panel with the current entity hierarchy
fn update_scene_tree_panel(
    mut commands: Commands,
    scene_tree_query: Query<Entity, With<SceneTreePanel>>,
    hierarchy_state: Res<HierarchyState>,
    selection: Res<EditorSelection>,
    world: &World,
    all_entities: Query<(Entity, Option<&Name>)>,
    children_query: Query<&Children>,
    entity_row_query: Query<Entity, With<EntityTreeRow>>,
) {
    // Only update if hierarchy state or selection changed
    if !hierarchy_state.is_changed() && !selection.is_changed() {
        return;
    }

    let Ok(panel_entity) = scene_tree_query.single() else {
        return;
    };

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

                let eye_symbol = if is_visible { "👁" } else { "🚫" };

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
                        Text::new(eye_symbol),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
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
