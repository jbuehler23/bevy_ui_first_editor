//! Fixed layout setup for the editor UI
//!
//! Provides the startup system that builds the initial 4-panel editor layout.
//! This will eventually be replaced with a dynamic docking system.

use bevy::prelude::*;
use bevy::picking::Pickable;
use bevy_editor_core::EditorEntity;
use crate::{
    PanelMarker, SceneTreePanel,
    SearchInputBox, SearchInputText, ClearSearchButton,
    InspectorPanel,
    EditorIcons,
};

/// Set up the fixed editor UI layout
///
/// Creates a 4-panel layout:
/// - Center/Left: Viewport (transparent spacer, allows 3D picking)
/// - Right Top: Scene Tree panel with search
/// - Right Bottom: Inspector panel
/// - Bottom: Asset Browser panel (full width)
pub fn setup_editor_ui(mut commands: Commands, icons: Res<EditorIcons>) {
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
                is_hoverable: false,         // Container should not be hoverable
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
                    is_hoverable: false,  // Container should not be hoverable
                },
            ))
            .with_children(|content_row| {
                // Scene Tree panel (left side)
                content_row.spawn((
                            Node {
                                width: Val::Px(350.0), // Fixed width for left panel
                                height: Val::Percent(100.0),
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::clip(), // Clip content that exceeds bounds
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                            PanelMarker {
                                name: "Scene Tree".to_string(),
                            },
                            Pickable {
                                should_block_lower: false,
                                is_hoverable: true,
                            },
                        ))
                        .with_children(|panel_wrapper| {
                            // Search input row
                            panel_wrapper.spawn((
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
                                        ImageNode::new(icons.x.clone()),
                                        Node {
                                            width: Val::Px(12.0),
                                            height: Val::Px(12.0),
                                            ..default()
                                        },
                                    ));
                                });
                            });

                            // Panel title
                            panel_wrapper.spawn((
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

                            // Scrollable content area for hierarchy
                            panel_wrapper.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(0.0), // Start at 0, grow to fill parent
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::scroll_y(), // Scrollable!
                                    ..default()
                                },
                                ScrollPosition(Vec2::ZERO), // Track scroll position
                                BackgroundColor(Color::NONE), // Transparent background
                                Pickable {
                                    should_block_lower: false,
                                    is_hoverable: true,  // Needs to be hoverable for scroll detection
                                },
                                SceneTreePanel,  // Marker for hierarchy update system
                                EditorEntity,
                            ));
                        });

                // Viewport area (middle) - grows to fill available space
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

                // Inspector panel (right side)
                content_row.spawn((
                            Node {
                                width: Val::Px(350.0), // Fixed width for right panel
                                height: Val::Percent(100.0),
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::all(Val::Px(8.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            BorderColor::all(Color::srgb(0.25, 0.25, 0.25)),
                            PanelMarker {
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

                            // Inspector content area
                            panel.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(0.0), // Start at 0, grow to fill parent
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::scroll_y(),
                                    ..default()
                                },
                                ScrollPosition(Vec2::ZERO), // Track scroll position
                                BackgroundColor(Color::NONE), // Transparent background
                                Pickable {
                                    should_block_lower: false,
                                    is_hoverable: true,  // Needs to be hoverable for scroll detection
                                },
                                InspectorPanel,
                                EditorEntity,
                            ));
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
                PanelMarker {
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
