//! RON-based scene serialization

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeSeed;
use std::path::Path;

/// Save a scene to a .bscn file
pub fn save_scene(scene: &DynamicScene, path: &Path, type_registry: &bevy::reflect::TypeRegistry) -> Result<(), std::io::Error> {
    // Serialize the DynamicScene to RON using Bevy's serializer
    let serializer = bevy::scene::serde::SceneSerializer {
        scene,
        registry: type_registry,
    };

    let serialized = ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let byte_count = serialized.len();

    // Write to file
    std::fs::write(path, serialized)?;

    info!("Scene saved to {:?} ({} bytes)", path, byte_count);
    Ok(())
}

/// Load a scene from a .bscn file (returns RON string to be loaded by Bevy's scene spawner)
pub fn load_scene(path: &Path) -> Result<String, std::io::Error> {
    // Read the file and return the RON string
    // Bevy's SceneSpawner will handle deserialization
    let contents = std::fs::read_to_string(path)?;
    info!("Scene loaded from {:?}", path);
    Ok(contents)
}

/// Scene metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneMetadata {
    pub name: String,
    pub created: String,
    pub modified: String,
    pub entities: usize,
}
