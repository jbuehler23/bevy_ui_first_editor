//! Docking UI renderer
//!
//! Builds and updates the UI hierarchy from the docking tree structure.

use bevy::prelude::*;
use bevy::picking::Pickable;
use bevy::ui::RelativeCursorPosition;
use bevy_editor_core::EditorEntity;
use super::*;

/// Marker for the root docking container
#[derive(Component)]
pub struct DockingRoot;

/// Marker for panel content areas (to be filled by specific panels)
#[derive(Component)]
pub struct PanelContent {
    pub panel_id: String,
}

/// Build the docking UI from the layout tree
pub fn build_docking_ui(
    mut commands: Commands,
    layout: Res<DockingLayout>,
    root_query: Query<Entity, With<DockingRoot>>,
    existing_containers: Query<Entity, With<DockContainer>>,
) {
    // Build on first run (no root exists) OR when layout changed
    // This ensures UI builds initially and whenever user rearranges panels
    if !layout.is_changed() && !root_query.is_empty() {
        return;
    }

    // Despawn all existing dock containers and their children
    // Note: In Bevy main, we need to manually despawn children first
    for entity in &existing_containers {
        commands.entity(entity).despawn();
    }

    // Find or create root
    let root_entity = if let Ok(entity) = root_query.single() {
        entity
    } else {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            DockingRoot,
            EditorEntity,
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        )).id()
    };

    // Build tree from layout
    if let Some(ref root_node) = layout.root {
        let root_id = commands.entity(root_entity).id();
        build_node_ui(&mut commands, root_id, root_node);
    }

    // Build floating windows
    for window in &layout.floating {
        build_floating_window(&mut commands, window);
    }
}

/// Build UI for a dock node and attach to parent
fn build_node_ui(
    commands: &mut Commands,
    parent: Entity,
    node: &DockNode,
) {
    match node {
        DockNode::Panel { panels, active, id } => {
            build_panel_container(commands, parent, panels, *active, *id);
        }
        DockNode::Split { direction, ratio, first, second, id } => {
            build_split_container(commands, parent, *direction, *ratio, first, second, *id);
        }
    }
}

/// Build a panel container with tabs
fn build_panel_container(
    commands: &mut Commands,
    parent: Entity,
    panels: &[String],
    active: usize,
    container_id: DockId,
) {
    let container = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
        DockContainer { id: container_id },
        RelativeCursorPosition::default(),  // For drop target detection during drag
        EditorEntity,
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
    )).id();

    commands.entity(parent).add_child(container);

    // Header: Always show either panel header (single panel) or tab bar (multiple panels)
    if panels.len() == 1 {
        // Single panel: spawn a panel header with drag handle
        let panel_id = &panels[0];

        let header = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(28.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(8.0)),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
            Button,  // Make entire header draggable
            PanelHeader {
                panel_id: panel_id.clone(),
                container_id,
            },
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            },
            EditorEntity,
        )).id();

        commands.entity(container).add_child(header);

        // Header content
        commands.entity(header).with_children(|header_row| {
            // Left side: drag handle + title
            header_row.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|left_side| {
                // Drag handle icon
                left_side.spawn((
                    Text::new("≡"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    Node {
                        margin: UiRect::right(Val::Px(8.0)),
                        ..default()
                    },
                ));

                // Panel title
                left_side.spawn((
                    Text::new(panel_id),
                    TextFont { font_size: 12.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

            // Right side: menu button (⋮) - TODO: implement dropdown functionality
            header_row.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|right_side| {
                right_side.spawn((
                    Text::new("⋮"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });
        });
    } else {
        // Multiple panels: spawn tab bar
        let tab_bar = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(32.0),
                flex_direction: FlexDirection::Row,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
        )).id();

        commands.entity(container).add_child(tab_bar);

        // Spawn tabs
        for (i, panel_id) in panels.iter().enumerate() {
            let is_active = i == active;
            let bg_color = if is_active {
                Color::srgb(0.2, 0.2, 0.2)
            } else {
                Color::srgb(0.12, 0.12, 0.12)
            };

            let tab = commands.spawn((
                Button,
                Node {
                    width: Val::Px(120.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::right(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                PanelTab {
                    panel_id: panel_id.clone(),
                    container_id,
                },
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
                EditorEntity,
            )).id();

            commands.entity(tab_bar).add_child(tab);

            // Tab text
            let text = commands.spawn((
                Text::new(panel_id),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )).id();

            commands.entity(tab).add_child(text);
        }
    }

    // Active panel content area
    if let Some(panel_id) = panels.get(active) {
        // Special styling for Viewport (transparent, no blocking)
        let (bg_color, padding, pickable_blocking, overflow) = if panel_id == "Viewport" {
            (
                BackgroundColor(Color::NONE), // Transparent
                UiRect::all(Val::Px(0.0)),    // No padding
                false,                          // Don't block picking
                Overflow::visible(),           // No scrolling
            )
        } else {
            (
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                UiRect::all(Val::Px(8.0)),
                false,
                Overflow::scroll_y(),
            )
        };

        let content_area = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                padding,
                overflow,
                ..default()
            },
            bg_color,
            PanelContent {
                panel_id: panel_id.clone(),
            },
            Pickable {
                should_block_lower: pickable_blocking,
                is_hoverable: true,
            },
            EditorEntity,
        )).id();

        commands.entity(container).add_child(content_area);

        // NO placeholder content - panels populate themselves via routing system
    }
}

/// Build a split container with divider
fn build_split_container(
    commands: &mut Commands,
    parent: Entity,
    direction: SplitDirection,
    ratio: f32,
    first: &DockNode,
    second: &DockNode,
    split_id: DockId,
) {
    let flex_direction = match direction {
        SplitDirection::Horizontal => FlexDirection::Row,
        SplitDirection::Vertical => FlexDirection::Column,
    };

    let split = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction,
            ..default()
        },
    )).id();

    commands.entity(parent).add_child(split);

    // First child container
    let first_container = commands.spawn((
        Node {
            width: if matches!(direction, SplitDirection::Horizontal) {
                Val::Percent(ratio * 100.0)
            } else {
                Val::Percent(100.0)
            },
            height: if matches!(direction, SplitDirection::Vertical) {
                Val::Percent(ratio * 100.0)
            } else {
                Val::Percent(100.0)
            },
            ..default()
        },
    )).id();

    commands.entity(split).add_child(first_container);
    build_node_ui(commands, first_container, first);

    // Divider (resizable)
    let divider_size = Val::Px(4.0);
    let divider = commands.spawn((
        Button,
        Node {
            width: if matches!(direction, SplitDirection::Horizontal) {
                divider_size
            } else {
                Val::Percent(100.0)
            },
            height: if matches!(direction, SplitDirection::Vertical) {
                divider_size
            } else {
                Val::Percent(100.0)
            },
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        SplitDivider {
            split_id,
            direction,
        },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        EditorEntity,
    )).id();

    commands.entity(split).add_child(divider);

    // Second child container
    let second_container = commands.spawn((
        Node {
            width: Val::Auto,
            height: Val::Auto,
            flex_grow: 1.0,
            ..default()
        },
    )).id();

    commands.entity(split).add_child(second_container);
    build_node_ui(commands, second_container, second);
}

/// Build a floating window
fn build_floating_window(
    commands: &mut Commands,
    window: &FloatingWindow,
) {
    let floating = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(window.position.x),
            top: Val::Px(window.position.y),
            width: Val::Px(window.size.x),
            height: Val::Px(window.size.y),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        FloatingWindowMarker { window_id: window.id },
        EditorEntity,
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
    )).id();

    // Title bar
    let title_bar = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(28.0),
            padding: UiRect::all(Val::Px(8.0)),
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
    )).id();

    commands.entity(floating).add_child(title_bar);

    if let Some(panel_id) = window.panels.get(window.active) {
        let title_text = commands.spawn((
            Text::new(panel_id),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )).id();

        commands.entity(title_bar).add_child(title_text);

        // Content area
        let content = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            PanelContent {
                panel_id: panel_id.clone(),
            },
        )).id();

        commands.entity(floating).add_child(content);

        let content_text = commands.spawn((
            Text::new(format!("Floating: {}", panel_id)),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        )).id();

        commands.entity(content).add_child(content_text);
    }
}
