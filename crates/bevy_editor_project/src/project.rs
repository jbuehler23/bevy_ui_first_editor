//! Project file management

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Current project state
#[derive(Resource)]
pub struct CurrentProject {
    pub path: Option<PathBuf>,
    pub config: Option<ProjectConfig>,
}

impl Default for CurrentProject {
    fn default() -> Self {
        Self {
            path: None,
            config: None,
        }
    }
}

/// Project configuration file (.bevy_project/project.ron)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub bevy_version: String,
    pub default_scene: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "New Project".to_string(),
            version: "0.1.0".to_string(),
            bevy_version: "0.15".to_string(),
            default_scene: None,
        }
    }
}
