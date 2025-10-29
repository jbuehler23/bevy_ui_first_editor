//! Entity selection management
//!
//! Tracks which entities are currently selected in the editor.

use bevy::prelude::*;
use std::collections::HashSet;

/// Tracks currently selected entities in the editor
#[derive(Debug, Default, Resource)]
pub struct EditorSelection {
    selected: HashSet<Entity>,
    primary: Option<Entity>,
}

impl EditorSelection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Select a single entity (clears previous selection)
    pub fn select(&mut self, entity: Entity) {
        self.selected.clear();
        self.selected.insert(entity);
        self.primary = Some(entity);
    }

    /// Add an entity to the selection
    pub fn add(&mut self, entity: Entity) {
        self.selected.insert(entity);
        if self.primary.is_none() {
            self.primary = Some(entity);
        }
    }

    /// Remove an entity from the selection
    pub fn remove(&mut self, entity: Entity) {
        self.selected.remove(&entity);
        if self.primary == Some(entity) {
            self.primary = self.selected.iter().next().copied();
        }
    }

    /// Toggle entity selection
    pub fn toggle(&mut self, entity: Entity) {
        if self.selected.contains(&entity) {
            self.remove(entity);
        } else {
            self.add(entity);
        }
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.selected.clear();
        self.primary = None;
    }

    /// Check if an entity is selected
    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected.contains(&entity)
    }

    /// Get all selected entities
    pub fn selected(&self) -> impl Iterator<Item = Entity> + '_ {
        self.selected.iter().copied()
    }

    /// Get the primary selection (for inspector display)
    pub fn primary(&self) -> Option<Entity> {
        self.primary
    }

    /// Get the number of selected entities
    pub fn len(&self) -> usize {
        self.selected.len()
    }

    /// Check if selection is empty
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}
