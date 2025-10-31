//! Project and scene management

use bevy::prelude::*;
use bevy::scene::DynamicSceneBuilder;
use bevy_editor_core::EditorEntity;
use serde::de::DeserializeSeed;
use std::path::PathBuf;

pub mod project;
pub mod scene_format;

pub use project::*;
pub use scene_format::*;

/// Current scene being edited
#[derive(Resource, Debug, Clone)]
pub struct CurrentScene {
    pub path: PathBuf,
    pub modified: bool,
}

impl Default for CurrentScene {
    fn default() -> Self {
        Self {
            path: PathBuf::from("scenes/main.bscn"),
            modified: false,
        }
    }
}

/// Plugin for project management
pub struct EditorProjectPlugin;

impl Plugin for EditorProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentProject>()
            .init_resource::<CurrentScene>()
            .add_systems(Update, (
                handle_save_scene,
                handle_load_scene,
            ));
    }
}

/// Handle Ctrl+S to save the current scene (exclusive system)
fn handle_save_scene(world: &mut World) {
    // Get keyboard state
    let keyboard = world.resource::<ButtonInput<KeyCode>>();
    let ctrl_pressed = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let s_just_pressed = keyboard.just_pressed(KeyCode::KeyS);

    if !ctrl_pressed || !s_just_pressed {
        return;
    }

    info!("💾 Save scene requested (Ctrl+S)");

    // Get scene path
    let scene_path = world.resource::<CurrentScene>().path.clone();

    // Create scenes directory if it doesn't exist
    if let Some(parent) = scene_path.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("Failed to create scenes directory: {}", e);
                return;
            }
        }
    }

    // Build DynamicScene from world, excluding editor entities
    let mut builder = DynamicSceneBuilder::from_world(world);

    // Filter out entities with EditorEntity component (UI, camera, etc.)
    builder = builder.deny_all_resources(); // Don't save resources

    // Collect game entities (non-editor entities)
    let game_entities: Vec<Entity> = world.iter_entities()
        .filter(|entity_ref| !entity_ref.contains::<EditorEntity>())
        .map(|entity_ref| entity_ref.id())
        .collect();

    let scene = builder.extract_entities(game_entities.iter().copied()).build();

    // Save to file (in a scope to drop the type_registry guard)
    let save_result = {
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        save_scene(&scene, &scene_path, &type_registry)
    };

    // Process save result
    match save_result {
        Ok(_) => {
            info!("✅ Scene saved successfully to {:?}", scene_path);
            // Mark scene as unmodified (now safe to mutably borrow world)
            world.resource_mut::<CurrentScene>().modified = false;
        }
        Err(e) => {
            error!("❌ Failed to save scene: {}", e);
        }
    }
}

/// Handle Ctrl+O to load a scene
fn handle_load_scene(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    current_scene: Res<CurrentScene>,
    entities_query: Query<Entity, Without<EditorEntity>>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    type_registry: Res<AppTypeRegistry>,
    mut scenes: ResMut<Assets<DynamicScene>>,
) {
    // Check for Ctrl+O (Left Ctrl or Right Ctrl)
    let ctrl_pressed = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let o_just_pressed = keyboard.just_pressed(KeyCode::KeyO);

    if ctrl_pressed && o_just_pressed {
        info!("📂 Load scene requested (Ctrl+O)");

        let scene_path = current_scene.path.clone();

        // Check if file exists
        if !scene_path.exists() {
            warn!("Scene file does not exist: {:?}", scene_path);
            return;
        }

        // Load scene from file
        match load_scene(&scene_path) {
            Ok(ron_string) => {
                info!("✅ Scene file read successfully from {:?}", scene_path);

                // Clear existing game entities (keep editor entities)
                for entity in &entities_query {
                    commands.entity(entity).despawn();
                }

                // Deserialize the scene using Bevy's SceneDeserializer
                let type_registry = type_registry.read();
                let scene_deserializer = bevy::scene::serde::SceneDeserializer {
                    type_registry: &type_registry,
                };

                let mut deserializer = ron::de::Deserializer::from_str(&ron_string)
                    .expect("Failed to create RON deserializer");

                let scene: DynamicScene = scene_deserializer.deserialize(&mut deserializer)
                    .expect("Failed to deserialize scene");

                info!("Scene deserialized: {} entities", scene.entities.len());

                // Add scene to assets and spawn it
                let scene_handle = scenes.add(scene);
                scene_spawner.spawn_dynamic(scene_handle);

                info!("Scene entities spawned");
            }
            Err(e) => {
                error!("❌ Failed to load scene: {}", e);
            }
        }
    }
}
