//! 3D/2D viewport with editor camera and gizmos

use bevy::prelude::*;
use bevy_editor_core::EditorSelection;

pub mod camera;
pub mod gizmos;
pub mod grid;
pub mod picking;

pub use camera::*;
pub use gizmos::*;
pub use grid::*;
pub use picking::*;

/// Plugin for viewport functionality
pub struct EditorViewportPlugin;

impl Plugin for EditorViewportPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add camera plugin
            .add_plugins(EditorCameraPlugin)
            // Initialize grid config
            .init_resource::<GridConfig>()
            // Add systems
            .add_systems(Update, (
                draw_grid,
                draw_selection_outline,
            ))
            // Add test scene for now
            .add_systems(Startup, spawn_test_scene);
    }
}

/// Spawn some test sprites for 2D scene editing
fn spawn_test_scene(
    mut commands: Commands,
) {
    // Spawn player sprite - blue square at origin
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.5, 1.0), // Blue
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Player"),
    ))
    .observe(on_entity_click);

    // Spawn enemy sprite - red square
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.2, 0.2), // Red
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(150.0, 0.0, 0.0),
        Name::new("Enemy"),
    ))
    .observe(on_entity_click);

    // Spawn collectible sprite - yellow/gold circle
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.9, 0.2), // Yellow/Gold
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 120.0, 0.0),
        Name::new("Collectible"),
    ))
    .observe(on_entity_click);

    // Spawn platform sprite - gray rectangle
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Gray
            custom_size: Some(Vec2::new(256.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -100.0, 0.0),
        Name::new("Platform"),
    ))
    .observe(on_entity_click);
}

/// Handle entity click events to update selection
fn on_entity_click(
    trigger: On<Pointer<Click>>,
    mut selection: ResMut<EditorSelection>,
) {
    // Update selection (for now, just single selection - no multi-select yet)
    selection.select(trigger.entity);
}

/// Draw selection outline using gizmos (2D rectangles for sprites)
fn draw_selection_outline(
    mut gizmos: Gizmos,
    selection: Res<EditorSelection>,
    query: Query<(&Transform, Option<&Sprite>, Option<&Name>)>,
) {
    // Color for selection outline
    let selection_color = Color::srgb(1.0, 0.8, 0.0); // Bright yellow/orange

    for entity in selection.selected() {
        // Get the transform and sprite of the selected entity
        if let Ok((transform, sprite, name)) = query.get(entity) {
            // Get sprite size
            let size = if let Some(sprite) = sprite {
                // Use sprite's custom_size if available
                if let Some(custom_size) = sprite.custom_size {
                    Vec2::new(custom_size.x, custom_size.y)
                } else {
                    // Default size if no custom_size
                    Vec2::new(64.0, 64.0)
                }
            } else {
                // Fallback size for entities without sprites
                Vec2::new(64.0, 64.0)
            };

            // Scale by transform and add offset to make rectangle slightly larger
            let scaled_size = size * transform.scale.truncate() * 1.1;

            // Draw a 2D rectangle outline around the sprite
            draw_2d_rect_outline(
                &mut gizmos,
                transform.translation.truncate(),
                scaled_size,
                selection_color
            );
        }
    }
}

/// Helper function to draw a 2D rectangle outline
fn draw_2d_rect_outline(
    gizmos: &mut Gizmos,
    position: Vec2,
    size: Vec2,
    color: Color,
) {
    let half_size = size / 2.0;

    // Define the 4 corners of the rectangle
    let corners = [
        Vec2::new(position.x - half_size.x, position.y - half_size.y),
        Vec2::new(position.x + half_size.x, position.y - half_size.y),
        Vec2::new(position.x + half_size.x, position.y + half_size.y),
        Vec2::new(position.x - half_size.x, position.y + half_size.y),
    ];

    // Draw the 4 edges of the rectangle
    gizmos.line_2d(corners[0], corners[1], color);
    gizmos.line_2d(corners[1], corners[2], color);
    gizmos.line_2d(corners[2], corners[3], color);
    gizmos.line_2d(corners[3], corners[0], color);
}
