//! Core UI component markers
//!
//! Defines marker components for identifying UI elements in the editor.
//! These components are used for querying and updating specific UI panels and widgets.

use bevy::prelude::*;

/// Marker component for editor panel UI nodes
///
/// Note: This is distinct from the `EditorPanel` trait in panel.rs
/// which defines the interface for panel implementations.
#[derive(Component)]
pub struct PanelMarker {
    pub name: String,
}

/// Marker component for the Scene Tree panel content area
#[derive(Component)]
pub struct SceneTreePanel;

/// Marker component for the Assets panel content area
#[derive(Component)]
pub struct AssetsPanel;

/// Marker component for the search input box
#[derive(Component)]
pub struct SearchInputBox;

/// Marker component for the search text display
#[derive(Component)]
pub struct SearchInputText;

/// Marker component for the clear search button
#[derive(Component)]
pub struct ClearSearchButton;
