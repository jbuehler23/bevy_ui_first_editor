//! Transform gizmos for moving, rotating, and scaling entities visually in the viewport
//!
//! Provides interactive handles for Move, Rotate, and Scale operations on selected entities.

use bevy::prelude::*;
use bevy_editor_core::{EditorSelection, UiFocus};

/// Active gizmo mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

impl Default for GizmoMode {
    fn default() -> Self {
        Self::Translate
    }
}

/// Coordinate space for gizmo operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GizmoSpace {
    Local,
    World,
}

impl Default for GizmoSpace {
    fn default() -> Self {
        Self::World
    }
}

/// Axis for move and scale gizmos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoAxis {
    X,
    Y,
    XY, // For center handle or free movement
}

/// State tracking current gizmo drag operation
#[derive(Resource, Default)]
pub struct GizmoDragState {
    /// Whether we're currently dragging
    pub is_dragging: bool,
    /// The entity being transformed
    pub target_entity: Option<Entity>,
    /// Initial mouse position in world space when drag started
    pub drag_start_world_pos: Vec2,
    /// Initial transform when drag started
    pub initial_transform: Option<Transform>,
    /// Which axis is being dragged
    pub drag_axis: Option<GizmoAxis>,
}

/// Keyboard shortcut handler for switching gizmo modes
pub fn handle_gizmo_mode_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    ui_focus: Res<UiFocus>,
    mut gizmo_mode: ResMut<GizmoMode>,
) {
    // Only handle shortcuts when UI doesn't have focus
    if ui_focus.focused_entity.is_some() {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyW) {
        *gizmo_mode = GizmoMode::Translate;
        info!("Switched to Translate mode (W)");
    } else if keyboard.just_pressed(KeyCode::KeyE) {
        *gizmo_mode = GizmoMode::Rotate;
        info!("Switched to Rotate mode (E)");
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        *gizmo_mode = GizmoMode::Scale;
        info!("Switched to Scale mode (R)");
    }
}

/// Draw gizmos for the currently selected entity
pub fn draw_gizmos(
    selection: Res<EditorSelection>,
    gizmo_mode: Res<GizmoMode>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    // Only draw if exactly one entity is selected
    let Some(selected_entity) = selection.selected().next() else {
        return;
    };

    let Ok(transform) = transforms.get(selected_entity) else {
        return;
    };

    match *gizmo_mode {
        GizmoMode::Translate => draw_move_gizmo(&mut gizmos, transform),
        GizmoMode::Rotate => draw_rotate_gizmo(&mut gizmos, transform),
        GizmoMode::Scale => draw_scale_gizmo(&mut gizmos, transform),
    }
}

/// Draw the move gizmo (X and Y axis arrows)
fn draw_move_gizmo(gizmos: &mut Gizmos, transform: &Transform) {
    let pos = transform.translation.truncate();
    const ARROW_LENGTH: f32 = 50.0;
    const ARROW_HEAD_SIZE: f32 = 10.0;

    // X axis (red arrow pointing right)
    gizmos.line_2d(pos, pos + Vec2::new(ARROW_LENGTH, 0.0), Color::srgb(1.0, 0.0, 0.0));
    // Arrow head
    gizmos.line_2d(
        pos + Vec2::new(ARROW_LENGTH, 0.0),
        pos + Vec2::new(ARROW_LENGTH - ARROW_HEAD_SIZE, ARROW_HEAD_SIZE / 2.0),
        Color::srgb(1.0, 0.0, 0.0),
    );
    gizmos.line_2d(
        pos + Vec2::new(ARROW_LENGTH, 0.0),
        pos + Vec2::new(ARROW_LENGTH - ARROW_HEAD_SIZE, -ARROW_HEAD_SIZE / 2.0),
        Color::srgb(1.0, 0.0, 0.0),
    );

    // Y axis (green arrow pointing up)
    gizmos.line_2d(pos, pos + Vec2::new(0.0, ARROW_LENGTH), Color::srgb(0.0, 1.0, 0.0));
    // Arrow head
    gizmos.line_2d(
        pos + Vec2::new(0.0, ARROW_LENGTH),
        pos + Vec2::new(ARROW_HEAD_SIZE / 2.0, ARROW_LENGTH - ARROW_HEAD_SIZE),
        Color::srgb(0.0, 1.0, 0.0),
    );
    gizmos.line_2d(
        pos + Vec2::new(0.0, ARROW_LENGTH),
        pos + Vec2::new(-ARROW_HEAD_SIZE / 2.0, ARROW_LENGTH - ARROW_HEAD_SIZE),
        Color::srgb(0.0, 1.0, 0.0),
    );

    // Center handle (white square for XY movement)
    const CENTER_SIZE: f32 = 8.0;
    gizmos.rect_2d(pos, Vec2::splat(CENTER_SIZE), Color::srgb(1.0, 1.0, 1.0));
}

/// Draw the rotate gizmo (circular handle)
fn draw_rotate_gizmo(gizmos: &mut Gizmos, transform: &Transform) {
    let pos = transform.translation.truncate();
    const CIRCLE_RADIUS: f32 = 50.0;

    // Draw circle
    gizmos.circle_2d(pos, CIRCLE_RADIUS, Color::srgb(0.3, 0.6, 1.0));

    // Draw rotation indicator (small line from center, rotated with entity)
    let rotation_z = transform.rotation.to_euler(bevy::math::EulerRot::XYZ).2;
    let indicator_end = pos + Vec2::new(
        CIRCLE_RADIUS * rotation_z.cos(),
        CIRCLE_RADIUS * rotation_z.sin(),
    );
    gizmos.line_2d(pos, indicator_end, Color::srgb(1.0, 1.0, 1.0));
}

/// Draw the scale gizmo (corner handles that scale with entity)
fn draw_scale_gizmo(gizmos: &mut Gizmos, transform: &Transform) {
    let pos = transform.translation.truncate();

    // Scale the gizmo handles with the entity for better visual feedback
    let avg_scale = (transform.scale.x + transform.scale.y) / 2.0;
    let scaled_distance = 40.0 * avg_scale.max(0.5);
    const HANDLE_SIZE: f32 = 8.0;

    // Four corner handles
    let corners = [
        Vec2::new(scaled_distance, scaled_distance),   // Top-right
        Vec2::new(-scaled_distance, scaled_distance),  // Top-left
        Vec2::new(-scaled_distance, -scaled_distance), // Bottom-left
        Vec2::new(scaled_distance, -scaled_distance),  // Bottom-right
    ];

    for corner in corners {
        let handle_pos = pos + corner;
        gizmos.rect_2d(
            handle_pos,
            Vec2::splat(HANDLE_SIZE),
            Color::srgb(1.0, 1.0, 0.0),
        );
    }

    // Draw box outline (scales with entity)
    gizmos.rect_2d(
        pos,
        Vec2::splat(scaled_distance * 2.0),
        Color::srgb(0.5, 0.5, 0.5),
    );
}

/// Handle mouse down on gizmo handles to start dragging
pub fn handle_gizmo_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    gizmo_mode: Res<GizmoMode>,
    mut drag_state: ResMut<GizmoDragState>,
    selection: Res<EditorSelection>,
    transforms: Query<&Transform>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Get mouse position in world space
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Check if clicking on a gizmo handle
    let Some(selected_entity) = selection.selected().next() else {
        return;
    };

    let Ok(transform) = transforms.get(selected_entity) else {
        return;
    };

    let entity_pos = transform.translation.truncate();

    // Hit test radius should scale with the entity's scale for better UX
    let avg_scale = (transform.scale.x + transform.scale.y) / 2.0;
    let hit_radius = 50.0 * avg_scale.max(0.5); // Scale with entity, minimum 0.5x

    let distance = world_pos.distance(entity_pos);

    // Hit test for gizmo (scales with entity size)
    if distance < hit_radius {
        drag_state.is_dragging = true;
        drag_state.target_entity = Some(selected_entity);
        drag_state.drag_start_world_pos = world_pos;
        drag_state.initial_transform = Some(*transform);
        drag_state.drag_axis = Some(GizmoAxis::XY);

        match *gizmo_mode {
            GizmoMode::Translate => info!("Started translating entity"),
            GizmoMode::Rotate => info!("Started rotating entity"),
            GizmoMode::Scale => info!("Started scaling entity"),
        }
    }
}

/// Handle mouse drag to update entity transform
pub fn handle_gizmo_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    gizmo_mode: Res<GizmoMode>,
    drag_state: Res<GizmoDragState>,
    mut transforms: Query<&mut Transform>,
) {
    if !drag_state.is_dragging || !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    let Some(target_entity) = drag_state.target_entity else {
        return;
    };

    let Some(initial_transform) = drag_state.initial_transform else {
        return;
    };

    // Get current mouse position in world space
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Ok(current_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Apply transform based on gizmo mode
    let Ok(mut transform) = transforms.get_mut(target_entity) else {
        return;
    };

    match *gizmo_mode {
        GizmoMode::Translate => {
            // Move: Calculate delta from drag start
            let delta = current_world_pos - drag_state.drag_start_world_pos;
            transform.translation = initial_transform.translation + delta.extend(0.0);
        }
        GizmoMode::Rotate => {
            // Rotate: Calculate angle from entity center
            let entity_pos = initial_transform.translation.truncate();

            // Vector from entity to initial mouse position
            let initial_vec = drag_state.drag_start_world_pos - entity_pos;
            // Vector from entity to current mouse position
            let current_vec = current_world_pos - entity_pos;

            // Calculate angle difference
            let initial_angle = initial_vec.y.atan2(initial_vec.x);
            let current_angle = current_vec.y.atan2(current_vec.x);
            let angle_delta = current_angle - initial_angle;

            // Apply rotation (rotate around Z axis in 2D)
            transform.rotation = initial_transform.rotation * Quat::from_rotation_z(angle_delta);
        }
        GizmoMode::Scale => {
            // Scale: Calculate distance ratio from entity center
            let entity_pos = initial_transform.translation.truncate();

            let initial_distance = (drag_state.drag_start_world_pos - entity_pos).length();
            let current_distance = (current_world_pos - entity_pos).length();

            // Avoid division by zero
            if initial_distance > 0.01 {
                let scale_factor = current_distance / initial_distance;

                // Apply uniform scale (maintain aspect ratio)
                let new_scale = initial_transform.scale * scale_factor;
                // Clamp scale to reasonable values
                transform.scale = new_scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            }
        }
    }
}

/// Handle mouse up to end dragging
pub fn handle_gizmo_drag_end(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<GizmoDragState>,
) {
    if mouse_button.just_released(MouseButton::Left) && drag_state.is_dragging {
        info!("Ended drag operation");
        // TODO: Create undo command here
        drag_state.is_dragging = false;
        drag_state.target_entity = None;
        drag_state.initial_transform = None;
        drag_state.drag_axis = None;
    }
}
