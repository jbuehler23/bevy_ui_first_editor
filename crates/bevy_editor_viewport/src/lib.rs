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

/// Spawn some test objects to see in the editor
fn spawn_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a cube - with click observer
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Name::new("Cube"),
    ))
    .observe(on_entity_click);

    // Spawn a sphere - with click observer
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.7, 0.9))),
        Transform::from_xyz(2.0, 0.5, 0.0),
        Name::new("Sphere"),
    ))
    .observe(on_entity_click);

    // Spawn a cylinder - with click observer
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.3, 0.3))),
        Transform::from_xyz(-2.0, 0.5, 0.0),
        Name::new("Cylinder"),
    ))
    .observe(on_entity_click);

    // Spawn a directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        Name::new("Sun"),
    ));

    // Spawn ambient light as a component now
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 200.0,
            affects_lightmapped_meshes: false,
        },
        Name::new("Ambient Light"),
    ));
}

/// Handle entity click events to update selection
fn on_entity_click(
    trigger: On<Pointer<Click>>,
    mut selection: ResMut<EditorSelection>,
) {
    // Update selection (for now, just single selection - no multi-select yet)
    selection.select(trigger.entity);
}

/// Draw selection outline using gizmos
fn draw_selection_outline(
    mut gizmos: Gizmos,
    selection: Res<EditorSelection>,
    query: Query<(&Transform, Option<&Name>)>,
) {
    // Color for selection outline
    let selection_color = Color::srgb(1.0, 0.8, 0.0); // Bright yellow/orange

    for entity in selection.selected() {
        // Get the transform of the selected entity
        if let Ok((transform, name)) = query.get(entity) {
            // Use fixed sizes for our test objects (we know what they are)
            // Later we can compute actual bounding boxes from meshes
            let size = if let Some(name) = name {
                match name.as_str() {
                    "Cube" => Vec3::new(1.0, 1.0, 1.0),
                    "Sphere" => Vec3::new(1.0, 1.0, 1.0), // diameter
                    "Cylinder" => Vec3::new(1.0, 1.0, 1.0), // diameter x height
                    _ => Vec3::splat(1.0),
                }
            } else {
                Vec3::splat(1.0)
            };

            // Scale by transform and add offset to make box slightly larger
            let scaled_size = size * transform.scale * 1.1;

            // Draw a wireframe box around the selected entity
            draw_oriented_box(&mut gizmos, transform.translation, transform.rotation, scaled_size, selection_color);
        }
    }
}

/// Helper function to draw an oriented bounding box
fn draw_oriented_box(
    gizmos: &mut Gizmos,
    position: Vec3,
    rotation: Quat,
    size: Vec3,
    color: Color,
) {
    let half_size = size / 2.0;

    // Define the 8 corners of the box in local space
    let corners = [
        Vec3::new(-half_size.x, -half_size.y, -half_size.z),
        Vec3::new(half_size.x, -half_size.y, -half_size.z),
        Vec3::new(half_size.x, half_size.y, -half_size.z),
        Vec3::new(-half_size.x, half_size.y, -half_size.z),
        Vec3::new(-half_size.x, -half_size.y, half_size.z),
        Vec3::new(half_size.x, -half_size.y, half_size.z),
        Vec3::new(half_size.x, half_size.y, half_size.z),
        Vec3::new(-half_size.x, half_size.y, half_size.z),
    ];

    // Transform corners to world space
    let world_corners: Vec<Vec3> = corners
        .iter()
        .map(|&corner| position + rotation * corner)
        .collect();

    // Draw the 12 edges of the box
    // Bottom face (4 edges)
    gizmos.line(world_corners[0], world_corners[1], color);
    gizmos.line(world_corners[1], world_corners[2], color);
    gizmos.line(world_corners[2], world_corners[3], color);
    gizmos.line(world_corners[3], world_corners[0], color);

    // Top face (4 edges)
    gizmos.line(world_corners[4], world_corners[5], color);
    gizmos.line(world_corners[5], world_corners[6], color);
    gizmos.line(world_corners[6], world_corners[7], color);
    gizmos.line(world_corners[7], world_corners[4], color);

    // Vertical edges (4 edges)
    gizmos.line(world_corners[0], world_corners[4], color);
    gizmos.line(world_corners[1], world_corners[5], color);
    gizmos.line(world_corners[2], world_corners[6], color);
    gizmos.line(world_corners[3], world_corners[7], color);
}
