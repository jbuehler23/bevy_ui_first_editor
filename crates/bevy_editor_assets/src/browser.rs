//! Asset browser UI

use bevy::prelude::*;
use std::path::PathBuf;

/// State for the asset browser panel
#[derive(Resource)]
pub struct AssetBrowserState {
    pub current_path: PathBuf,
    pub selected_asset: Option<PathBuf>,
    pub view_mode: ViewMode,
}

impl Default for AssetBrowserState {
    fn default() -> Self {
        Self {
            current_path: PathBuf::from("assets"),
            selected_asset: None,
            view_mode: ViewMode::Grid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Grid,
}
