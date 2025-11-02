//! Sprite component editor with interactive controls
//!
//! Provides editable controls for Sprite properties including flip toggles and texture selection.

use bevy::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;

/// Marker component for flip X checkbox
#[derive(Component)]
pub struct SpriteFlipXCheckbox {
    pub target_entity: Entity,
}

/// Marker component for flip Y checkbox
#[derive(Component)]
pub struct SpriteFlipYCheckbox {
    pub target_entity: Entity,
}

/// Marker component for texture selection button
#[derive(Component)]
pub struct SpriteTextureButton {
    pub target_entity: Entity,
}

/// Resource to hold async file dialog result
#[derive(Resource, Default)]
pub struct PendingTextureSelection {
    pub target_entity: Option<Entity>,
    pub path: Option<PathBuf>,
}

/// Handle texture selection button clicks
pub fn handle_texture_button(
    interaction_query: Query<(&Interaction, &SpriteTextureButton), Changed<Interaction>>,
    mut pending: ResMut<PendingTextureSelection>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Opening file dialog for texture selection...");

            // Open file dialog (blocking - will freeze UI briefly)
            // TODO: Make this async using bevy_tasks
            if let Some(path) = FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "webp"])
                .pick_file()
            {
                info!("Selected texture: {:?}", path);
                pending.target_entity = Some(button.target_entity);
                pending.path = Some(path);
            }
        }
    }
}

/// Apply pending texture selection
pub fn apply_pending_texture(
    mut pending: ResMut<PendingTextureSelection>,
    mut sprite_query: Query<&mut Sprite>,
    asset_server: Res<AssetServer>,
) {
    if let (Some(entity), Some(path)) = (pending.target_entity, pending.path.take()) {
        if let Ok(mut sprite) = sprite_query.get_mut(entity) {
            // Load the new texture
            let texture_handle: Handle<Image> = asset_server.load(path.clone());
            sprite.image = texture_handle;
            info!("Applied texture: {:?}", path);
        }
        pending.target_entity = None;
    }
}
