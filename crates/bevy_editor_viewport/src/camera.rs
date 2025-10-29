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
    // Spawn the 3D camera for the editor viewport
    // All components must be added together in a single spawn() to allow Bevy's
    // required components system to process everything atomically
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        EditorCamera::default(),
        EditorEntity, // Mark as editor entity
        Name::new("Editor Camera"), // Give it a name for debugging
    ));
}

/// Handle orbit controls (right mouse button + drag)
fn editor_camera_orbit(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut query: Query<&mut EditorCamera>,
) {
    if !mouse_button.pressed(MouseButton::Right) {
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
) {
    if !mouse_button.pressed(MouseButton::Middle) {
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

/// Handle zoom controls (mouse wheel)
fn editor_camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    mut query: Query<&mut EditorCamera>,
) {
    for mut camera in &mut query {
        if !camera.enabled {
            continue;
        }

        for event in scroll_events.read() {
            camera.radius -= event.y * camera.zoom_sensitivity * camera.radius * 0.1;
            camera.radius = camera.radius.clamp(0.1, 1000.0);
        }
    }
}

/// Update camera transform based on orbit parameters
fn update_camera_transform(
    mut query: Query<(&EditorCamera, &mut Transform)>,
) {
    for (camera, mut transform) in &mut query {
        // Calculate position from spherical coordinates
        let x = camera.radius * camera.pitch.cos() * camera.yaw.sin();
        let y = camera.radius * camera.pitch.sin();
        let z = camera.radius * camera.pitch.cos() * camera.yaw.cos();

        transform.translation = camera.focus + Vec3::new(x, y, z);
        transform.look_at(camera.focus, Vec3::Y);
    }
}

/// Frame the camera on a specific point
pub fn frame_camera_on_point(camera: &mut EditorCamera, point: Vec3, radius: f32) {
    camera.focus = point;
    camera.radius = radius;
}
