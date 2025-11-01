//! Editor camera controller with orbit, pan, and zoom

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_editor_core::EditorEntity;

/// Marker component for the editor camera
#[derive(Component)]
pub struct EditorCamera {
    /// Point the camera orbits around
    pub focus: Vec3,
    /// Distance from the focus point
    pub radius: f32,
    /// Orbital rotation around the focus point
    pub yaw: f32,
    pub pitch: f32,
    /// Movement speed modifiers
    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    /// Enabled state
    pub enabled: bool,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 10.0,
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_4, // 45 degrees
            orbit_sensitivity: 0.003,
            pan_sensitivity: 0.01,
            zoom_sensitivity: 0.1,
            enabled: true,
        }
    }
}

/// Plugin for editor camera functionality
pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_editor_camera)
            .add_systems(Update, (
                editor_camera_orbit,
                editor_camera_pan,
                editor_camera_zoom,
                update_camera_transform,
            ).chain());
    }
}

fn spawn_editor_camera(mut commands: Commands) {
    // Spawn the 2D camera for the editor viewport
    // All components must be added together in a single spawn() to allow Bevy's
    // required components system to process everything atomically
    commands.spawn((
        Camera2d,
        // For 2D in Bevy, camera should be at Z=999.9 (or any positive Z > sprites)
        // This is the default for 2D cameras - sprites are at Z=0
        Transform::from_xyz(0.0, 0.0, 999.9),
        Projection::from(OrthographicProjection {
            // Start with a good zoom level - this controls how much of the world is visible
            // Smaller values = more zoomed in
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        EditorCamera {
            focus: Vec3::ZERO,
            radius: 1.0, // For 2D, we'll use this as the zoom scale (1.0 = normal, 0.5 = zoomed in)
            yaw: 0.0,    // Not used for 2D
            pitch: 0.0,  // Not used for 2D
            orbit_sensitivity: 0.003,
            pan_sensitivity: 1.0, // Sensitivity for panning
            zoom_sensitivity: 0.1,
            enabled: true,
        },
        EditorEntity, // Mark as editor entity
        Name::new("Editor Camera"), // Give it a name for debugging
    ));
}

/// Handle orbit controls (right mouse button + drag)
fn editor_camera_orbit(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut query: Query<&mut EditorCamera>,
    ui_query: Query<&Interaction>,
) {
    if !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    // Check if mouse is over any UI element
    let mouse_over_ui = ui_query.iter().any(|interaction| {
        matches!(interaction, Interaction::Hovered | Interaction::Pressed)
    });

    // Don't process camera orbit if mouse is over UI
    if mouse_over_ui {
        return;
    }

    for mut camera in &mut query {
        if !camera.enabled {
            continue;
        }

        for event in motion_events.read() {
            camera.yaw += event.delta.x * camera.orbit_sensitivity;
            camera.pitch += event.delta.y * camera.orbit_sensitivity;

            // Clamp pitch to avoid gimbal lock
            camera.pitch = camera.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
        }
    }
}

/// Handle pan controls (middle mouse button + drag)
fn editor_camera_pan(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut EditorCamera, &Transform)>,
    ui_query: Query<&Interaction>,
) {
    if !mouse_button.pressed(MouseButton::Middle) {
        return;
    }

    // Check if mouse is over any UI element
    let mouse_over_ui = ui_query.iter().any(|interaction| {
        matches!(interaction, Interaction::Hovered | Interaction::Pressed)
    });

    // Don't process camera pan if mouse is over UI
    if mouse_over_ui {
        return;
    }

    for (mut camera, transform) in &mut query {
        if !camera.enabled {
            continue;
        }

        for event in motion_events.read() {
            // Pan in camera space
            let right = transform.right();
            let up = transform.up();

            let pan_amount = event.delta * camera.pan_sensitivity * camera.radius * 0.1;
            camera.focus -= right * pan_amount.x;
            camera.focus += up * pan_amount.y;
        }
    }
}

/// Handle zoom controls (mouse wheel) - for 2D, this changes orthographic scale
fn editor_camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    mut query: Query<&mut EditorCamera>,
    ui_query: Query<&Interaction>,
) {
    // Check if mouse is over any UI element
    let mouse_over_ui = ui_query.iter().any(|interaction| {
        matches!(interaction, Interaction::Hovered | Interaction::Pressed)
    });

    // Don't process camera zoom if mouse is over UI
    if mouse_over_ui {
        return;
    }

    for mut camera in &mut query {
        if !camera.enabled {
            continue;
        }

        for event in scroll_events.read() {
            // For 2D: radius is the orthographic scale
            // Negative Y scroll = zoom out (increase scale)
            // Positive Y scroll = zoom in (decrease scale)
            camera.radius -= event.y * camera.zoom_sensitivity * camera.radius * 0.1;
            // Clamp zoom: 0.1 = very zoomed in, 10.0 = very zoomed out
            camera.radius = camera.radius.clamp(0.1, 10.0);
        }
    }
}

/// Update camera transform for 2D (pan and zoom via projection scale)
fn update_camera_transform(
    mut query: Query<(&EditorCamera, &mut Transform, &mut Projection)>,
) {
    for (camera, mut transform, mut projection) in &mut query {
        // For 2D camera: position at focus point
        transform.translation = Vec3::new(
            camera.focus.x,
            camera.focus.y,
            999.9, // Keep Z at standard 2D camera position
        );

        // Keep camera looking down the -Z axis (no rotation for 2D)
        transform.rotation = Quat::IDENTITY;

        // Update orthographic projection scale for zoom
        // radius field is repurposed as zoom scale for 2D
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = camera.radius;
        }
    }
}

/// Frame the camera on a specific point
pub fn frame_camera_on_point(camera: &mut EditorCamera, point: Vec3, radius: f32) {
    camera.focus = point;
    camera.radius = radius;
}
