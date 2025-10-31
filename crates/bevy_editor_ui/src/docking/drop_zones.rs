//! Visual drop zone highlights
//!
//! Displays colored overlay zones when dragging panels to show where they will dock.

use bevy::prelude::*;
use bevy::picking::Pickable;
use super::{DockDragState, DropZone, DockContainer};

/// Marker for drop zone overlay entities
#[derive(Component)]
pub struct DropZoneOverlay {
    pub zone: DropZone,
}

/// Show drop zone overlays when dragging a panel
pub fn show_drop_zones(
    mut commands: Commands,
    drag_state: Res<DockDragState>,
    container_query: Query<(Entity, &DockContainer, &Node, &GlobalTransform)>,
    existing_overlays: Query<Entity, With<DropZoneOverlay>>,
) {
    // Clear existing overlays if not dragging
    if drag_state.dragging.is_none() {
        let overlay_count = existing_overlays.iter().count();
        if overlay_count > 0 {
            info!("🧹 Clearing {} drop zone overlays", overlay_count);
        }
        for entity in &existing_overlays {
            commands.entity(entity).despawn();
        }
        return;
    }

    info!("🎨 Showing drop zones for panel: {:?}, drop_target: {:?}", drag_state.dragging, drag_state.drop_target);

    // Clear old overlays
    for entity in &existing_overlays {
        commands.entity(entity).despawn();
    }

    // Show overlays for the current drop target
    if let Some(target_id) = drag_state.drop_target {
        info!("  ✨ Creating overlays for target container: {:?}", target_id);
        // Find the target container
        for (container_entity, container, _node, _transform) in &container_query {
            if container.id == target_id {
                // Create 5 drop zone overlays (4 edges + center)
                create_drop_zone_overlay(
                    &mut commands,
                    container_entity,
                    DropZone::Left,
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.3, 1.0),
                );
                create_drop_zone_overlay(
                    &mut commands,
                    container_entity,
                    DropZone::Right,
                    Vec2::new(0.7, 0.0),
                    Vec2::new(0.3, 1.0),
                );
                create_drop_zone_overlay(
                    &mut commands,
                    container_entity,
                    DropZone::Top,
                    Vec2::new(0.3, 0.0),
                    Vec2::new(0.4, 0.3),
                );
                create_drop_zone_overlay(
                    &mut commands,
                    container_entity,
                    DropZone::Bottom,
                    Vec2::new(0.3, 0.7),
                    Vec2::new(0.4, 0.3),
                );
                create_drop_zone_overlay(
                    &mut commands,
                    container_entity,
                    DropZone::Center,
                    Vec2::new(0.3, 0.3),
                    Vec2::new(0.4, 0.4),
                );
                break;
            }
        }
    }
}

/// Create a drop zone overlay
fn create_drop_zone_overlay(
    commands: &mut Commands,
    parent: Entity,
    zone: DropZone,
    position: Vec2,
    size: Vec2,
) {
    // TESTING: Very bright, high opacity color for debugging
    let color = Color::srgba(0.2, 0.8, 1.0, 0.7); // Bright cyan, 70% opaque

    info!("  📦 Creating overlay for {:?} at ({:.0}%, {:.0}%) size ({:.0}% x {:.0}%)",
        zone, position.x * 100.0, position.y * 100.0, size.x * 100.0, size.y * 100.0);

    let overlay = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x * 100.0),
            top: Val::Percent(position.y * 100.0),
            width: Val::Percent(size.x * 100.0),
            height: Val::Percent(size.y * 100.0),
            border: UiRect::all(Val::Px(4.0)),  // Thicker border
            ..default()
        },
        BackgroundColor(color),
        BorderColor::all(Color::srgb(1.0, 0.0, 1.0)),  // Bright magenta border
        DropZoneOverlay { zone },
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        bevy::ui::ZIndex(1000),  // Force to top layer
    )).id();

    commands.entity(parent).add_child(overlay);
    info!("  ✅ Overlay entity spawned: {:?}", overlay);
}

/// Update drop zone based on cursor position within container
pub fn update_drop_zone_from_cursor(
    mut drag_state: ResMut<DockDragState>,
    container_query: Query<(&DockContainer, &Node, &GlobalTransform)>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if drag_state.dragging.is_none() {
        return;
    }

    if let Some(target_id) = drag_state.drop_target {
        if let Ok(window) = window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Find the target container
                for (container, _node, _transform) in &container_query {
                    if container.id == target_id {
                        // Calculate relative cursor position (0.0-1.0)
                        // TODO: Proper coordinate transformation
                        // For now, use simple zone detection

                        // Use window-space approximation
                        let rel_x = (cursor_pos.x / window.width()).clamp(0.0, 1.0);
                        let rel_y = (cursor_pos.y / window.height()).clamp(0.0, 1.0);

                        // Determine zone based on position
                        let zone = if rel_x < 0.3 {
                            DropZone::Left
                        } else if rel_x > 0.7 {
                            DropZone::Right
                        } else if rel_y < 0.3 {
                            DropZone::Top
                        } else if rel_y > 0.7 {
                            DropZone::Bottom
                        } else {
                            DropZone::Center
                        };

                        drag_state.drop_zone = Some(zone);
                        break;
                    }
                }
            }
        }
    }
}
