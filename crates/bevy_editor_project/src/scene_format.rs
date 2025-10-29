//! RON-based scene serialization

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Save a scene to a .bscn file
pub fn save_scene(scene: &DynamicScene, path: &Path) -> Result<(), std::io::Error> {
    // TODO: Serialize scene to RON format
    Ok(())
}

/// Load a scene from a .bscn file
pub fn load_scene(path: &Path) -> Result<DynamicScene, std::io::Error> {
    // TODO: Deserialize scene from RON format
    todo!()
}

/// Scene metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneMetadata {
    pub name: String,
    pub created: String,
    pub modified: String,
    pub entities: usize,
}
