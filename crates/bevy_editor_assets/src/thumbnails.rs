//! Thumbnail generation for assets

use bevy::prelude::*;
use std::path::Path;

/// Cache for asset thumbnails
#[derive(Resource, Default)]
pub struct ThumbnailCache {
    // TODO: Store Handle<Image> for each asset path
}

impl ThumbnailCache {
    pub fn get_or_generate(&mut self, path: &Path) -> Option<Handle<Image>> {
        // TODO: Load or generate thumbnail
        None
    }
}
