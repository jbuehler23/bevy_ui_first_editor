//! Panel content routing system
//!
//! Routes panel IDs to their actual content (hierarchy, inspector, etc.)

use bevy::prelude::*;
use crate::{SceneTreePanel, InspectorPanel};
use super::PanelContent;

/// System to populate panel content areas with actual panel components
pub fn route_panel_content(
    mut commands: Commands,
    content_query: Query<(Entity, &PanelContent), Added<PanelContent>>,
) {
    for (entity, content) in &content_query {
        match content.panel_id.as_str() {
            "Hierarchy" => {
                // Mark this as the scene tree panel
                commands.entity(entity).insert(SceneTreePanel);
            }
            "Inspector" => {
                // Mark this as the inspector panel
                commands.entity(entity).insert(InspectorPanel);
            }
            "Viewport" => {
                // Viewport is transparent, no special marking needed
                // Just ensure it doesn't block picking
            }
            "Assets" => {
                // TODO: Add AssetsPanel marker when implemented
            }
            _ => {
                // Unknown panel type, leave as placeholder
            }
        }
    }
}
