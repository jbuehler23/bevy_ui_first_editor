//! Tree row interaction systems for selection and drag-and-drop reparenting
//!
//! Handles click selection (single, multi, range), drag-and-drop reparenting,
//! and expand/collapse interactions.

use bevy::prelude::*;
use bevy_editor_core::EditorSelection;
use bevy_editor_hierarchy::EntityTreeRow;
use crate::HierarchyState;

/// Handle clicks on tree rows for selection and expand/collapse
pub fn handle_tree_row_clicks(
    interaction_query: Query<(&Interaction, &EntityTreeRow), (Changed<Interaction>, With<Button>)>,
    all_tree_rows: Query<&EntityTreeRow, With<Button>>,
    mut selection: ResMut<EditorSelection>,
    mut hierarchy_state: ResMut<HierarchyState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, tree_row) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let entity = tree_row.entity;

            // Check for modifier keys
            let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
            let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

            // Handle selection with modifier keys
            if ctrl {
                // Toggle selection
                selection.toggle(entity);
                // Update anchor for future range selections
                hierarchy_state.selection_anchor = Some(entity);
            } else if shift {
                // Range selection
                if let Some(anchor) = hierarchy_state.selection_anchor {
                    // Build a list of all visible entities in order
                    let visible_entities: Vec<Entity> = all_tree_rows
                        .iter()
                        .map(|row| row.entity)
                        .collect();

                    // Find indices of anchor and current entity
                    if let (Some(anchor_idx), Some(current_idx)) = (
                        visible_entities.iter().position(|e| *e == anchor),
                        visible_entities.iter().position(|e| *e == entity),
                    ) {
                        // Select all entities in the range
                        let (start, end) = if anchor_idx <= current_idx {
                            (anchor_idx, current_idx)
                        } else {
                            (current_idx, anchor_idx)
                        };

                        for i in start..=end {
                            selection.add(visible_entities[i]);
                        }
                    }
                } else {
                    // No anchor, just add this entity
                    selection.add(entity);
                    hierarchy_state.selection_anchor = Some(entity);
                }
            } else {
                // Single selection
                selection.select(entity);
                // Set new anchor
                hierarchy_state.selection_anchor = Some(entity);

                // Toggle expand/collapse when clicking on entity with children
                if hierarchy_state.expanded.contains(&entity) {
                    hierarchy_state.expanded.remove(&entity);
                } else {
                    hierarchy_state.expanded.insert(entity);
                }
            }
        }
    }
}


/// Handle drag start for tree rows (left click + drag)
pub fn handle_tree_row_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    tree_row_query: Query<(&Interaction, &EntityTreeRow), With<Button>>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    // Start dragging when left mouse button is pressed on a tree row
    if mouse_button.just_pressed(MouseButton::Left) {
        for (interaction, tree_row) in &tree_row_query {
            if matches!(interaction, Interaction::Pressed) {
                hierarchy_state.dragging = Some(tree_row.entity);
                break;
            }
        }
    }
}

/// Update drop target during drag
pub fn handle_tree_row_drag_over(
    mouse_button: Res<ButtonInput<MouseButton>>,
    tree_row_query: Query<(&Interaction, &EntityTreeRow), With<Button>>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    // Only track drop target if we're currently dragging
    if hierarchy_state.dragging.is_some() && mouse_button.pressed(MouseButton::Left) {
        hierarchy_state.drop_target = None;

        for (interaction, tree_row) in &tree_row_query {
            if matches!(interaction, Interaction::Hovered) {
                // Don't allow dropping on self
                if Some(tree_row.entity) != hierarchy_state.dragging {
                    hierarchy_state.drop_target = Some(tree_row.entity);
                }
                break;
            }
        }
    }
}

/// Handle drop and perform reparenting
pub fn handle_tree_row_drop(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut hierarchy_state: ResMut<HierarchyState>,
    children_query: Query<&Children>,
) {
    // Perform reparenting when mouse is released
    if mouse_button.just_released(MouseButton::Left) {
        if let (Some(dragged), Some(target)) = (hierarchy_state.dragging, hierarchy_state.drop_target) {
            // Check if target is not a descendant of dragged (prevent circular hierarchy)
            let mut is_descendant = false;
            let mut check_entity = target;

            // Walk up the hierarchy to check if we'd create a cycle
            loop {
                if check_entity == dragged {
                    is_descendant = true;
                    break;
                }

                // Check if this entity has a parent
                if let Ok(_children) = children_query.get(check_entity) {
                    // This entity has children, but we need to check its parent
                    // We'll break here for now and implement proper parent checking later
                    break;
                } else {
                    break;
                }
            }

            if !is_descendant {
                // Remove from old parent (if any) and add to new parent
                commands.entity(target).add_children(&[dragged]);
                println!("Reparented {:?} under {:?}", dragged, target);
            } else {
                println!("Cannot reparent: would create circular hierarchy");
            }
        }

        // Clear drag state
        hierarchy_state.dragging = None;
        hierarchy_state.drop_target = None;
    }

    // Also clear if mouse button is released without a valid drop target
    if mouse_button.just_released(MouseButton::Left) {
        hierarchy_state.dragging = None;
        hierarchy_state.drop_target = None;
    }
}
