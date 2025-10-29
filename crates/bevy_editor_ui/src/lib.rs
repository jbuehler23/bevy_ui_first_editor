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
                handle_tree_row_right_clicks,
                handle_context_menu_actions,
                handle_visibility_toggle_clicks,
                handle_hierarchy_keyboard_navigation,
                handle_tree_row_drag_start,
                handle_tree_row_drag_over,
                handle_tree_row_drop,
                handle_search_input,
                handle_clear_search_button,
                close_context_menu_on_click_outside,
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

/// Handle right-clicks on tree rows to show context menu
fn handle_tree_row_right_clicks(
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
fn handle_context_menu_actions(
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
fn close_context_menu_on_click_outside(
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

/// Handle clicks on tree rows for selection and expand/collapse
fn handle_tree_row_clicks(
    interaction_query: Query<(&Interaction, &EntityTreeRow), (Changed<Interaction>, With<Button>)>,
    all_tree_rows: Query<&EntityTreeRow, With<Button>>,
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
                // Update anchor for future range selections
                hierarchy_state.selection_anchor = Some(entity);
            } else if shift {
                // Range selection
                if let Some(anchor) = hierarchy_state.selection_anchor {
                    // Build a list of all visible entities in order
                    let visible_entities: Vec<Entity> = all_tree_rows
                        .iter()
                        .map(|row| row.entity)
                        .collect();

                    // Find indices of anchor and current entity
                    if let (Some(anchor_idx), Some(current_idx)) = (
                        visible_entities.iter().position(|e| *e == anchor),
                        visible_entities.iter().position(|e| *e == entity),
                    ) {
                        // Select all entities in the range
                        let (start, end) = if anchor_idx <= current_idx {
                            (anchor_idx, current_idx)
                        } else {
                            (current_idx, anchor_idx)
                        };

                        for i in start..=end {
                            selection.add(visible_entities[i]);
                        }
                    }
                } else {
                    // No anchor, just add this entity
                    selection.add(entity);
                    hierarchy_state.selection_anchor = Some(entity);
                }
            } else {
                // Single selection
                selection.select(entity);
                // Set new anchor
                hierarchy_state.selection_anchor = Some(entity);

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

/// Handle keyboard navigation in the hierarchy tree
fn handle_hierarchy_keyboard_navigation(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    all_tree_rows: Query<&EntityTreeRow, With<Button>>,
    mut selection: ResMut<EditorSelection>,
    mut hierarchy_state: ResMut<HierarchyState>,
    search_focus_query: Query<&Interaction, With<SearchInputBox>>,
) {
    // Don't handle navigation if search box is focused
    for interaction in &search_focus_query {
        if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
            return;
        }
    }

    // Build a list of all visible entities in order
    let visible_entities: Vec<Entity> = all_tree_rows
        .iter()
        .map(|row| row.entity)
        .collect();

    if visible_entities.is_empty() {
        return;
    }

    // Get the current primary selection
    let current_selection = selection.primary();

    // Arrow Up: Move selection up
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if let Some(current) = current_selection {
            if let Some(current_idx) = visible_entities.iter().position(|e| *e == current) {
                if current_idx > 0 {
                    let new_selection = visible_entities[current_idx - 1];
                    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                        // Shift+Up: Extend selection
                        selection.add(new_selection);
                    } else {
                        // Just Up: Move selection
                        selection.select(new_selection);
                        hierarchy_state.selection_anchor = Some(new_selection);
                    }
                }
            }
        } else if !visible_entities.is_empty() {
            // No selection, select first entity
            selection.select(visible_entities[0]);
            hierarchy_state.selection_anchor = Some(visible_entities[0]);
        }
    }

    // Arrow Down: Move selection down
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if let Some(current) = current_selection {
            if let Some(current_idx) = visible_entities.iter().position(|e| *e == current) {
                if current_idx < visible_entities.len() - 1 {
                    let new_selection = visible_entities[current_idx + 1];
                    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                        // Shift+Down: Extend selection
                        selection.add(new_selection);
                    } else {
                        // Just Down: Move selection
                        selection.select(new_selection);
                        hierarchy_state.selection_anchor = Some(new_selection);
                    }
                }
            }
        } else if !visible_entities.is_empty() {
            // No selection, select first entity
            selection.select(visible_entities[0]);
            hierarchy_state.selection_anchor = Some(visible_entities[0]);
        }
    }

    // Arrow Right: Expand selected entity
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if let Some(current) = current_selection {
            hierarchy_state.expanded.insert(current);
        }
    }

    // Arrow Left: Collapse selected entity
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if let Some(current) = current_selection {
            hierarchy_state.expanded.remove(&current);
        }
    }

    // Enter: Toggle expand/collapse
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some(current) = current_selection {
            if hierarchy_state.expanded.contains(&current) {
                hierarchy_state.expanded.remove(&current);
            } else {
                hierarchy_state.expanded.insert(current);
            }
        }
    }

    // Delete: Delete selected entities
    if keyboard.just_pressed(KeyCode::Delete) {
        for entity in selection.selected().collect::<Vec<_>>() {
            commands.entity(entity).despawn();
        }
        selection.clear();
        hierarchy_state.selection_anchor = None;
    }

    // Ctrl+D: Duplicate selected entity
    if keyboard.just_pressed(KeyCode::KeyD) &&
       (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)) {
        if let Some(current) = current_selection {
            // TODO: Implement entity duplication
            println!("Duplicate entity {:?}", current);
        }
    }
}

/// Handle drag start for tree rows (left click + drag)
fn handle_tree_row_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    tree_row_query: Query<(&Interaction, &EntityTreeRow), With<Button>>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    // Start dragging when left mouse button is pressed on a tree row
    if mouse_button.just_pressed(MouseButton::Left) {
        for (interaction, tree_row) in &tree_row_query {
            if matches!(interaction, Interaction::Pressed) {
                hierarchy_state.dragging = Some(tree_row.entity);
                break;
            }
        }
    }
}

/// Update drop target during drag
fn handle_tree_row_drag_over(
    mouse_button: Res<ButtonInput<MouseButton>>,
    tree_row_query: Query<(&Interaction, &EntityTreeRow), With<Button>>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    // Only track drop target if we're currently dragging
    if hierarchy_state.dragging.is_some() && mouse_button.pressed(MouseButton::Left) {
        hierarchy_state.drop_target = None;

        for (interaction, tree_row) in &tree_row_query {
            if matches!(interaction, Interaction::Hovered) {
                // Don't allow dropping on self
                if Some(tree_row.entity) != hierarchy_state.dragging {
                    hierarchy_state.drop_target = Some(tree_row.entity);
                }
                break;
            }
        }
    }
}

/// Handle drop and perform reparenting
fn handle_tree_row_drop(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut hierarchy_state: ResMut<HierarchyState>,
    children_query: Query<&Children>,
) {
    // Perform reparenting when mouse is released
    if mouse_button.just_released(MouseButton::Left) {
        if let (Some(dragged), Some(target)) = (hierarchy_state.dragging, hierarchy_state.drop_target) {
            // Check if target is not a descendant of dragged (prevent circular hierarchy)
            let mut is_descendant = false;
            let mut check_entity = target;

            // Walk up the hierarchy to check if we'd create a cycle
            loop {
                if check_entity == dragged {
                    is_descendant = true;
                    break;
                }

                // Check if this entity has a parent
                if let Ok(children) = children_query.get(check_entity) {
                    // This entity has children, but we need to check its parent
                    // We'll break here for now and implement proper parent checking later
                    break;
                } else {
                    break;
                }
            }

            if !is_descendant {
                // Remove from old parent (if any) and add to new parent
                commands.entity(target).add_children(&[dragged]);
                println!("Reparented {:?} under {:?}", dragged, target);
            } else {
                println!("Cannot reparent: would create circular hierarchy");
            }
        }

        // Clear drag state
        hierarchy_state.dragging = None;
        hierarchy_state.drop_target = None;
    }

    // Also clear if mouse button is released without a valid drop target
    if mouse_button.just_released(MouseButton::Left) {
        hierarchy_state.dragging = None;
        hierarchy_state.drop_target = None;
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
