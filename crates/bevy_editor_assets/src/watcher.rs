//! File system watcher for hot reloading

use bevy::prelude::*;
use notify::Watcher;

/// Resource that watches the file system for changes
#[derive(Resource)]
pub struct FileSystemWatcher {
    // TODO: Integrate notify crate
}

impl FileSystemWatcher {
    pub fn new() -> Self {
        Self {}
    }
}
