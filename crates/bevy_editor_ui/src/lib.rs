//! Native bevy_ui-based editor UI framework
//!
//! Provides docking, panels, and UI widgets using only bevy_ui.

use bevy::prelude::*;
use bevy::picking::Pickable;

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

/// Plugin for the native bevy_ui editor UI system
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DockingLayout>()
            .add_systems(Startup, setup_editor_ui)
            .add_systems(Update, (update_docking_layout, handle_panel_resize));
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
                        ))
                        .with_children(|panel| {
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
