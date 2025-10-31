//! Systems for docking interactions
//!
//! Handles divider dragging, panel drag-to-dock, and tab switching.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::*;

// ==================== Split Divider Resizing ====================

/// Start dragging a divider
pub fn handle_divider_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    divider_query: Query<(&Interaction, &SplitDivider), (Changed<Interaction>, With<Button>)>,
    mut drag_state: ResMut<DividerDragState>,
    layout: Res<DockingLayout>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for (interaction, divider) in &divider_query {
            if *interaction == Interaction::Pressed {
                // Get current split ratio
                let ratio = if let Some(ref root) = layout.root {
                    find_split_ratio(root, divider.split_id).unwrap_or(0.5)
                } else {
                    0.5
                };

                if let Ok(window) = window.single() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        drag_state.dragging = Some(divider.split_id);
                        drag_state.original_ratio = ratio;
                        drag_state.start_position = cursor_pos;
                    }
                }
                break;
            }
        }
    }
}

/// Update divider position during drag
pub fn handle_divider_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DividerDragState>,
    mut layout: ResMut<DockingLayout>,
    window: Query<&Window, With<PrimaryWindow>>,
    divider_query: Query<&SplitDivider>,
) {
    if drag_state.dragging.is_none() || !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    if let Ok(window) = window.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            // Find the divider being dragged
            let dragged_id = drag_state.dragging.unwrap();

            for divider in &divider_query {
                if divider.split_id == dragged_id {
                    // Calculate new ratio based on mouse movement
                    let delta = match divider.direction {
                        SplitDirection::Horizontal => cursor_pos.x - drag_state.start_position.x,
                        SplitDirection::Vertical => cursor_pos.y - drag_state.start_position.y,
                    };

                    // Use window size as reference (simplified approach)
                    let parent_size = match divider.direction {
                        SplitDirection::Horizontal => window.width(),
                        SplitDirection::Vertical => window.height(),
                    };

                    let delta_ratio = delta / parent_size;
                    let new_ratio = (drag_state.original_ratio + delta_ratio).clamp(0.1, 0.9);

                    layout.update_split_ratio(dragged_id, new_ratio);
                    break;
                }
            }
        }
    }
}

/// Stop dragging divider
pub fn handle_divider_drag_end(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DividerDragState>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        drag_state.dragging = None;
    }
}

// Helper function to find split ratio
fn find_split_ratio(node: &DockNode, split_id: DockId) -> Option<f32> {
    match node {
        DockNode::Split { id, ratio, first, second, .. } => {
            if *id == split_id {
                Some(*ratio)
            } else {
                find_split_ratio(first, split_id)
                    .or_else(|| find_split_ratio(second, split_id))
            }
        }
        _ => None,
    }
}

// ==================== Panel Tab Switching ====================

/// Handle clicks on panel tabs to switch active panel
pub fn handle_tab_clicks(
    interaction_query: Query<(&Interaction, &PanelTab), (Changed<Interaction>, With<Button>)>,
    mut layout: ResMut<DockingLayout>,
) {
    for (interaction, tab) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Find the container and set active panel
            if let Some(ref mut root) = layout.root {
                set_active_panel_recursive(root, &tab.container_id, &tab.panel_id);
            }
        }
    }
}

fn set_active_panel_recursive(node: &mut DockNode, container_id: &DockId, panel_id: &str) {
    match node {
        DockNode::Panel { id, panels, active, .. } => {
            if id == container_id {
                if let Some(index) = panels.iter().position(|p| p == panel_id) {
                    *active = index;
                }
            }
        }
        DockNode::Split { first, second, .. } => {
            set_active_panel_recursive(first, container_id, panel_id);
            set_active_panel_recursive(second, container_id, panel_id);
        }
    }
}

// ==================== Drag-to-Dock ====================

/// Detect potential drag from panel header or tab (before threshold)
pub fn handle_panel_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    header_query: Query<(&Interaction, &PanelHeader), With<Button>>,
    tab_query: Query<(&Interaction, &PanelTab), With<Button>>,
    mut drag_state: ResMut<DockDragState>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // NO Ctrl requirement - drag directly from header/tab!

    if mouse_button.just_pressed(MouseButton::Left) {
        info!("🖱️ Mouse pressed - checking for drag targets");
        info!("  Headers count: {}", header_query.iter().count());
        info!("  Tabs count: {}", tab_query.iter().count());

        if let Ok(window) = window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Store initial position
                drag_state.drag_start_position = Some(cursor_pos);
                info!("  Cursor position: {:?}", cursor_pos);

                // Check for panel header clicks (single-panel containers)
                for (interaction, header) in &header_query {
                    info!("  Header '{}' interaction: {:?}", header.panel_id, interaction);
                    if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
                        drag_state.potential_drag_panel = Some(header.panel_id.clone());
                        drag_state.potential_drag_container = Some(header.container_id);
                        info!("✅ Potential drag set: {}", header.panel_id);
                        return;  // Found header, stop searching
                    }
                }

                // Check for tab clicks (multi-panel containers)
                for (interaction, tab) in &tab_query {
                    info!("  Tab '{}' interaction: {:?}", tab.panel_id, interaction);
                    if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
                        drag_state.potential_drag_panel = Some(tab.panel_id.clone());
                        drag_state.potential_drag_container = Some(tab.container_id);
                        info!("✅ Potential drag set: {}", tab.panel_id);
                        return;  // Found tab, stop searching
                    }
                }

                info!("❌ No interactive headers/tabs found at cursor position");
            }
        }
    }
}

/// Activate drag after threshold is crossed (5px of movement)
pub fn activate_drag_on_threshold(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DockDragState>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    const DRAG_THRESHOLD: f32 = 5.0;  // pixels

    // If we have a potential drag and mouse is still pressed
    if drag_state.potential_drag_panel.is_some() && mouse_button.pressed(MouseButton::Left) {
        if let Ok(window) = window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Some(start_pos) = drag_state.drag_start_position {
                    let distance = cursor_pos.distance(start_pos);

                    // If mouse moved more than threshold, activate drag!
                    if distance > DRAG_THRESHOLD {
                        let panel_name = drag_state.potential_drag_panel.as_ref().unwrap();
                        info!("🎯 DRAG ACTIVATED! Panel: {}, Distance: {:.1}px", panel_name, distance);

                        drag_state.dragging = drag_state.potential_drag_panel.take();
                        drag_state.source_container = drag_state.potential_drag_container.take();
                        drag_state.drag_position = cursor_pos;
                        drag_state.drag_start_position = None;
                    }
                }
            }
        }
    }

    // Clear potential drag on mouse release (it was just a click, not a drag)
    if mouse_button.just_released(MouseButton::Left) {
        if drag_state.potential_drag_panel.is_some() {
            info!("⬆️ Mouse released before threshold - treating as click");
        }
        drag_state.potential_drag_panel = None;
        drag_state.potential_drag_container = None;
        drag_state.drag_start_position = None;
    }
}

/// Update drop target during panel drag
pub fn handle_panel_drag_over(
    container_query: Query<(&DockContainer, &bevy::ui::RelativeCursorPosition)>,
    mut drag_state: ResMut<DockDragState>,
) {
    if drag_state.dragging.is_none() {
        return;
    }

    // Clear previous target
    drag_state.drop_target = None;
    drag_state.drop_zone = None;

    // Find container under cursor using RelativeCursorPosition
    // This works during drag because RelativeCursorPosition updates every frame
    // regardless of mouse button state (unlike Interaction::Hovered)
    for (container, cursor_pos) in &container_query {
        if let Some(pos) = cursor_pos.normalized {
            // normalized is Some if cursor is over the node, with (0,0) = top-left, (1,1) = bottom-right
            // Check if cursor is inside container bounds
            if (0.0..=1.0).contains(&pos.x) && (0.0..=1.0).contains(&pos.y) {
                // Cursor is inside this container!
                drag_state.drop_target = Some(container.id);
                info!("✅ Found drop target: {:?} at pos ({:.2}, {:.2})", container.id, pos.x, pos.y);

                // Determine drop zone based on position within container
                // Edge zones are 30% of container size
                let zone = if pos.x < 0.3 {
                    DropZone::Left
                } else if pos.x > 0.7 {
                    DropZone::Right
                } else if pos.y < 0.3 {
                    DropZone::Top
                } else if pos.y > 0.7 {
                    DropZone::Bottom
                } else {
                    DropZone::Center
                };

                drag_state.drop_zone = Some(zone);
                info!("  Drop zone: {:?}", zone);
                break;  // Found the topmost container under cursor
            }
        }
    }
}

/// Complete panel docking on drag release
pub fn handle_panel_drop(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DockDragState>,
    mut layout: ResMut<DockingLayout>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        if let Some(ref panel_id) = drag_state.dragging {
            if let (Some(target_container), Some(drop_zone)) =
                (drag_state.drop_target, drag_state.drop_zone) {

                // Remove panel from source
                layout.remove_panel(panel_id);

                // Add to target based on drop zone
                match drop_zone {
                    DropZone::Center => {
                        layout.add_panel_to_container(panel_id.clone(), target_container);
                    }
                    DropZone::Left => {
                        layout.split_container(
                            target_container,
                            SplitDirection::Horizontal,
                            panel_id.clone(),
                            0.5,
                        );
                    }
                    DropZone::Right => {
                        layout.split_container(
                            target_container,
                            SplitDirection::Horizontal,
                            panel_id.clone(),
                            0.5,
                        );
                    }
                    DropZone::Top => {
                        layout.split_container(
                            target_container,
                            SplitDirection::Vertical,
                            panel_id.clone(),
                            0.5,
                        );
                    }
                    DropZone::Bottom => {
                        layout.split_container(
                            target_container,
                            SplitDirection::Vertical,
                            panel_id.clone(),
                            0.5,
                        );
                    }
                }
            }
        }

        // Clear drag state
        drag_state.dragging = None;
        drag_state.source_container = None;
        drag_state.drop_target = None;
        drag_state.drop_zone = None;
    }
}
