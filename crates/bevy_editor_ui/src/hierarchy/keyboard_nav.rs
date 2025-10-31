//! Keyboard navigation system for the hierarchy panel
//!
//! Handles arrow key navigation, expand/collapse, and keyboard shortcuts.

use bevy::prelude::*;
use bevy_editor_core::EditorSelection;
use bevy_editor_hierarchy::EntityTreeRow;
use crate::HierarchyState;
use crate::SearchInputBox;

/// Handle keyboard navigation in the hierarchy tree
pub fn handle_hierarchy_keyboard_navigation(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    all_tree_rows: Query<&EntityTreeRow, With<Button>>,
    mut selection: ResMut<EditorSelection>,
    mut hierarchy_state: ResMut<HierarchyState>,
    search_focus_query: Query<&Interaction, With<SearchInputBox>>,
) {
    // Don't handle navigation if search box is focused
    for interaction in &search_focus_query {
        if matches!(interaction, Interaction::Hovered | Interaction::Pressed) {
            return;
        }
    }

    // Build a list of all visible entities in order
    let visible_entities: Vec<Entity> = all_tree_rows
        .iter()
        .map(|row| row.entity)
        .collect();

    if visible_entities.is_empty() {
        return;
    }

    // Get the current primary selection
    let current_selection = selection.primary();

    // Arrow Up: Move selection up
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if let Some(current) = current_selection {
            if let Some(current_idx) = visible_entities.iter().position(|e| *e == current) {
                if current_idx > 0 {
                    let new_selection = visible_entities[current_idx - 1];
                    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                        // Shift+Up: Extend selection
                        selection.add(new_selection);
                    } else {
                        // Just Up: Move selection
                        selection.select(new_selection);
                        hierarchy_state.selection_anchor = Some(new_selection);
                    }
                }
            }
        } else if !visible_entities.is_empty() {
            // No selection, select first entity
            selection.select(visible_entities[0]);
            hierarchy_state.selection_anchor = Some(visible_entities[0]);
        }
    }

    // Arrow Down: Move selection down
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if let Some(current) = current_selection {
            if let Some(current_idx) = visible_entities.iter().position(|e| *e == current) {
                if current_idx < visible_entities.len() - 1 {
                    let new_selection = visible_entities[current_idx + 1];
                    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                        // Shift+Down: Extend selection
                        selection.add(new_selection);
                    } else {
                        // Just Down: Move selection
                        selection.select(new_selection);
                        hierarchy_state.selection_anchor = Some(new_selection);
                    }
                }
            }
        } else if !visible_entities.is_empty() {
            // No selection, select first entity
            selection.select(visible_entities[0]);
            hierarchy_state.selection_anchor = Some(visible_entities[0]);
        }
    }

    // Arrow Right: Expand selected entity
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if let Some(current) = current_selection {
            hierarchy_state.expanded.insert(current);
        }
    }

    // Arrow Left: Collapse selected entity
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if let Some(current) = current_selection {
            hierarchy_state.expanded.remove(&current);
        }
    }

    // Enter: Toggle expand/collapse
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some(current) = current_selection {
            if hierarchy_state.expanded.contains(&current) {
                hierarchy_state.expanded.remove(&current);
            } else {
                hierarchy_state.expanded.insert(current);
            }
        }
    }

    // Delete: Delete selected entities
    if keyboard.just_pressed(KeyCode::Delete) {
        for entity in selection.selected().collect::<Vec<_>>() {
            commands.entity(entity).despawn();
        }
        selection.clear();
        hierarchy_state.selection_anchor = None;
    }

    // Ctrl+D: Duplicate selected entity
    if keyboard.just_pressed(KeyCode::KeyD) &&
       (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)) {
        if let Some(current) = current_selection {
            // TODO: Implement entity duplication
            println!("Duplicate entity {:?}", current);
        }
    }
}
