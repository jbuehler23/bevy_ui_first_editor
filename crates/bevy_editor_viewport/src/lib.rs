//! 3D/2D viewport with editor camera and gizmos

use bevy::prelude::*;
use bevy::picking::prelude::*;
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
            // Initialize gizmo resources
            .init_resource::<GizmoMode>()
            .init_resource::<GizmoDragState>()
            // Add systems
            .add_systems(Update, (
                draw_grid,
                draw_selection_outline,
                // Gizmo systems
                draw_gizmos,
                handle_gizmo_mode_shortcuts,
                handle_gizmo_drag_start,
                handle_gizmo_drag,
                handle_gizmo_drag_end,
            ))
            // Add test scene for now
            .add_systems(Startup, spawn_test_scene);
    }
}

/// Spawn many test sprites for 2D scene editing (testing picking and scrolling)
fn spawn_test_scene(
    mut commands: Commands,
) {
    // Spawn a grid of sprites to test both picking and panel scrolling
    // Grid: 6 rows x 5 columns = 30 sprites
    let colors = [
        (Color::srgb(0.2, 0.5, 1.0), "Blue"),      // Blue
        (Color::srgb(1.0, 0.2, 0.2), "Red"),       // Red
        (Color::srgb(1.0, 0.9, 0.2), "Yellow"),    // Yellow
        (Color::srgb(0.2, 1.0, 0.2), "Green"),     // Green
        (Color::srgb(1.0, 0.5, 0.2), "Orange"),    // Orange
        (Color::srgb(0.8, 0.2, 1.0), "Purple"),    // Purple
    ];

    let spacing = 100.0;
    let start_x = -200.0;
    let start_y = 200.0;

    for row in 0..6 {
        for col in 0..5 {
            let x = start_x + (col as f32 * spacing);
            let y = start_y - (row as f32 * spacing);
            let (color, color_name) = colors[row % colors.len()];

            let size = 50.0 + (row * 5) as f32; // Vary size slightly

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
                Name::new(format!("{}_{}_{}", color_name, row, col)),
                Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
            ))
            .observe(on_entity_click);
        }
    }
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
    query: Query<(&Transform, Option<&Sprite>)>,
) {
    // Color for selection outline
    let selection_color = Color::srgb(1.0, 0.8, 0.0); // Bright yellow/orange

    for entity in selection.selected() {
        // Get the transform and sprite of the selected entity
        if let Ok((transform, sprite)) = query.get(entity) {
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

            // Draw a 2D rectangle outline around the sprite (with rotation)
            draw_2d_rect_outline_rotated(
                &mut gizmos,
                transform,
                scaled_size,
                selection_color
            );
        }
    }
}

/// Helper function to draw a 2D rectangle outline (accounts for rotation)
fn draw_2d_rect_outline_rotated(
    gizmos: &mut Gizmos,
    transform: &Transform,
    size: Vec2,
    color: Color,
) {
    let half_size = size / 2.0;

    // Define the 4 corners in local space
    let local_corners = [
        Vec2::new(-half_size.x, -half_size.y),
        Vec2::new(half_size.x, -half_size.y),
        Vec2::new(half_size.x, half_size.y),
        Vec2::new(-half_size.x, half_size.y),
    ];

    // Extract 2D rotation angle from quaternion
    let rotation_z = transform.rotation.to_euler(bevy::math::EulerRot::XYZ).2;
    let cos_angle = rotation_z.cos();
    let sin_angle = rotation_z.sin();

    // Transform corners to world space (apply rotation and position)
    let world_corners: Vec<Vec2> = local_corners
        .iter()
        .map(|&corner| {
            // Apply rotation
            let rotated_x = corner.x * cos_angle - corner.y * sin_angle;
            let rotated_y = corner.x * sin_angle + corner.y * cos_angle;
            // Apply translation
            Vec2::new(
                transform.translation.x + rotated_x,
                transform.translation.y + rotated_y,
            )
        })
        .collect();

    // Draw the 4 edges of the rectangle
    gizmos.line_2d(world_corners[0], world_corners[1], color);
    gizmos.line_2d(world_corners[1], world_corners[2], color);
    gizmos.line_2d(world_corners[2], world_corners[3], color);
    gizmos.line_2d(world_corners[3], world_corners[0], color);
}

/// Helper function to draw a 2D rectangle outline (legacy, no rotation)
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
