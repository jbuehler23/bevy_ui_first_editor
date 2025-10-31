//! Docking system for editor panels
//!
//! Complete implementation of a flexible docking system with:
//! - Split containers (horizontal/vertical resizable dividers)
//! - Tab containers (multiple panels in one dock)
//! - Floating windows
//! - Drag-to-dock with drop zones
//! - Layout persistence (save/load)

mod systems;
mod renderer;
mod panels;
mod drop_zones;
mod persistence;

pub use systems::*;
pub use renderer::*;
pub use panels::*;
pub use drop_zones::*;
pub use persistence::*;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Describes the complete docking layout including floating windows
#[derive(Debug, Resource, Clone, Serialize, Deserialize)]
pub struct DockingLayout {
    /// Root docking tree (main window content)
    pub root: Option<DockNode>,
    /// Floating windows (undocked panels)
    pub floating: Vec<FloatingWindow>,
}

impl Default for DockingLayout {
    fn default() -> Self {
        Self {
            root: Some(DockNode::default_layout()),
            floating: Vec::new(),
        }
    }
}

/// A node in the docking tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockNode {
    /// A panel container with tabs
    Panel {
        /// Panel IDs in this container
        panels: Vec<String>,
        /// Index of the active (visible) panel
        active: usize,
        /// Unique ID for this container (for UI queries)
        id: DockId,
    },
    /// A split container (divides space between two children)
    Split {
        /// Horizontal (left/right) or Vertical (top/bottom)
        direction: SplitDirection,
        /// Split ratio (0.0-1.0, represents first child's portion)
        ratio: f32,
        /// First child node
        first: Box<DockNode>,
        /// Second child node
        second: Box<DockNode>,
        /// Unique ID for this split (for divider interaction)
        id: DockId,
    },
}

impl DockNode {
    /// Create a default 3-panel layout (viewport + right sidebar split into 2)
    pub fn default_layout() -> Self {
        DockNode::Split {
            direction: SplitDirection::Horizontal,
            ratio: 0.7, // 70% viewport, 30% sidebar
            first: Box::new(DockNode::Panel {
                panels: vec!["Viewport".to_string()],
                active: 0,
                id: DockId::new(),
            }),
            second: Box::new(DockNode::Split {
                direction: SplitDirection::Vertical,
                ratio: 0.5, // Split sidebar 50/50
                first: Box::new(DockNode::Panel {
                    panels: vec!["Hierarchy".to_string()],
                    active: 0,
                    id: DockId::new(),
                }),
                second: Box::new(DockNode::Panel {
                    panels: vec!["Inspector".to_string()],
                    active: 0,
                    id: DockId::new(),
                }),
                id: DockId::new(),
            }),
            id: DockId::new(),
        }
    }

    /// Get all panel IDs in this subtree
    pub fn all_panels(&self) -> Vec<String> {
        match self {
            DockNode::Panel { panels, .. } => panels.clone(),
            DockNode::Split { first, second, .. } => {
                let mut result = first.all_panels();
                result.extend(second.all_panels());
                result
            }
        }
    }

    /// Find a panel container by panel ID
    pub fn find_container_mut(&mut self, panel_id: &str) -> Option<&mut DockNode> {
        match self {
            DockNode::Panel { panels, .. } => {
                if panels.iter().any(|p| p == panel_id) {
                    Some(self)
                } else {
                    None
                }
            }
            DockNode::Split { first, second, .. } => {
                first.find_container_mut(panel_id)
                    .or_else(|| second.find_container_mut(panel_id))
            }
        }
    }
}

/// Direction for split containers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Left and right (| divider)
    Horizontal,
    /// Top and bottom (— divider)
    Vertical,
}

/// Unique identifier for dock nodes (for UI entity mapping)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DockId(pub u64);

impl DockId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// A floating window (undocked panel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingWindow {
    /// Panel IDs in this window
    pub panels: Vec<String>,
    /// Active panel index
    pub active: usize,
    /// Window position (top-left corner)
    pub position: Vec2,
    /// Window size
    pub size: Vec2,
    /// Unique ID
    pub id: DockId,
}

/// Drop zone for drag-to-dock operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropZone {
    /// Dock to the left of target
    Left,
    /// Dock to the right of target
    Right,
    /// Dock above target
    Top,
    /// Dock below target
    Bottom,
    /// Add as tab to target container
    Center,
}

/// State for drag-to-dock operations
#[derive(Debug, Resource, Default)]
pub struct DockDragState {
    /// Currently dragged panel ID
    pub dragging: Option<String>,
    /// Source container ID (where the drag started)
    pub source_container: Option<DockId>,
    /// Current drop target container ID
    pub drop_target: Option<DockId>,
    /// Current drop zone within target
    pub drop_zone: Option<DropZone>,
    /// Mouse position during drag
    pub drag_position: Vec2,
    /// Potential drag panel (before threshold is crossed)
    pub potential_drag_panel: Option<String>,
    /// Potential drag container (before threshold is crossed)
    pub potential_drag_container: Option<DockId>,
    /// Initial mouse position when drag might start
    pub drag_start_position: Option<Vec2>,
}

/// State for resizing split dividers
#[derive(Debug, Resource, Default)]
pub struct DividerDragState {
    /// Currently dragged divider ID
    pub dragging: Option<DockId>,
    /// Original split ratio when drag started
    pub original_ratio: f32,
    /// Original mouse position when drag started
    pub start_position: Vec2,
}

// ==================== Component Markers ====================

/// Marker for a dock container UI entity
#[derive(Component)]
pub struct DockContainer {
    pub id: DockId,
}

/// Marker for a split divider UI entity
#[derive(Component)]
pub struct SplitDivider {
    pub split_id: DockId,
    pub direction: SplitDirection,
}

/// Marker for a panel tab button
#[derive(Component)]
pub struct PanelTab {
    pub panel_id: String,
    pub container_id: DockId,
}

/// Marker for a panel header (for single-panel containers)
#[derive(Component)]
pub struct PanelHeader {
    pub panel_id: String,
    pub container_id: DockId,
}

/// Marker for a floating window UI entity
#[derive(Component)]
pub struct FloatingWindowMarker {
    pub window_id: DockId,
}

// ==================== Dock Tree Operations ====================

impl DockingLayout {
    /// Add a panel to a specific container
    pub fn add_panel_to_container(&mut self, panel_id: String, container_id: DockId) {
        if let Some(ref mut root) = self.root {
            if let Some(container) = Self::find_container_by_id_mut(root, container_id) {
                if let DockNode::Panel { panels, active, .. } = container {
                    panels.push(panel_id);
                    *active = panels.len() - 1;
                }
            }
        }
    }

    /// Remove a panel from its container
    pub fn remove_panel(&mut self, panel_id: &str) -> Option<String> {
        if let Some(ref mut root) = self.root {
            if let Some(container) = root.find_container_mut(panel_id) {
                if let DockNode::Panel { panels, active, .. } = container {
                    if let Some(pos) = panels.iter().position(|p| p == panel_id) {
                        let removed = panels.remove(pos);
                        // Adjust active index
                        if panels.is_empty() {
                            *active = 0;
                        } else if *active >= panels.len() {
                            *active = panels.len() - 1;
                        }
                        return Some(removed);
                    }
                }
            }
        }
        None
    }

    /// Split a container in a direction, creating two new containers
    pub fn split_container(
        &mut self,
        container_id: DockId,
        direction: SplitDirection,
        new_panel_id: String,
        ratio: f32,
    ) {
        if let Some(ref mut root) = self.root {
            Self::split_container_recursive(root, container_id, direction, new_panel_id, ratio);
        }
    }

    fn split_container_recursive(
        node: &mut DockNode,
        target_id: DockId,
        direction: SplitDirection,
        new_panel_id: String,
        ratio: f32,
    ) -> bool {
        match node {
            DockNode::Panel { id, .. } if *id == target_id => {
                // Replace this panel with a split containing the old panel and new panel
                let old_node = std::mem::replace(node, DockNode::Panel {
                    panels: vec![],
                    active: 0,
                    id: DockId::new(),
                });

                *node = DockNode::Split {
                    direction,
                    ratio,
                    first: Box::new(old_node),
                    second: Box::new(DockNode::Panel {
                        panels: vec![new_panel_id],
                        active: 0,
                        id: DockId::new(),
                    }),
                    id: DockId::new(),
                };
                true
            }
            DockNode::Split { first, second, .. } => {
                if Self::split_container_recursive(first, target_id, direction, new_panel_id.clone(), ratio) {
                    true
                } else {
                    Self::split_container_recursive(second, target_id, direction, new_panel_id, ratio)
                }
            }
            _ => false,
        }
    }

    /// Find a container by its ID
    fn find_container_by_id_mut(node: &mut DockNode, target_id: DockId) -> Option<&mut DockNode> {
        // Check if this is the target node first
        let is_target = match node {
            DockNode::Panel { id, .. } => *id == target_id,
            DockNode::Split { id, .. } => *id == target_id,
        };

        if is_target {
            return Some(node);
        }

        // Otherwise search children
        match node {
            DockNode::Split { first, second, .. } => {
                Self::find_container_by_id_mut(first, target_id)
                    .or_else(|| Self::find_container_by_id_mut(second, target_id))
            }
            _ => None,
        }
    }

    /// Update split ratio for a divider
    pub fn update_split_ratio(&mut self, split_id: DockId, new_ratio: f32) {
        if let Some(ref mut root) = self.root {
            Self::update_split_ratio_recursive(root, split_id, new_ratio);
        }
    }

    fn update_split_ratio_recursive(node: &mut DockNode, split_id: DockId, new_ratio: f32) {
        if let DockNode::Split { id, ratio, first, second, .. } = node {
            if *id == split_id {
                *ratio = new_ratio.clamp(0.1, 0.9);
            } else {
                Self::update_split_ratio_recursive(first, split_id, new_ratio);
                Self::update_split_ratio_recursive(second, split_id, new_ratio);
            }
        }
    }

    /// Create a floating window from a panel
    pub fn undock_panel(&mut self, panel_id: &str, position: Vec2, size: Vec2) {
        if let Some(removed_panel) = self.remove_panel(panel_id) {
            self.floating.push(FloatingWindow {
                panels: vec![removed_panel],
                active: 0,
                position,
                size,
                id: DockId::new(),
            });
        }
    }

    /// Dock a floating window back into the tree
    pub fn dock_floating_window(&mut self, window_id: DockId, target_container: DockId, zone: DropZone) {
        // Find and remove the floating window
        if let Some(pos) = self.floating.iter().position(|w| w.id == window_id) {
            let window = self.floating.remove(pos);

            // Add panels back to dock tree based on drop zone
            for panel in window.panels {
                match zone {
                    DropZone::Center => {
                        self.add_panel_to_container(panel, target_container);
                    }
                    DropZone::Left => {
                        self.split_container(target_container, SplitDirection::Horizontal, panel, 0.5);
                    }
                    DropZone::Right => {
                        self.split_container(target_container, SplitDirection::Horizontal, panel, 0.5);
                    }
                    DropZone::Top => {
                        self.split_container(target_container, SplitDirection::Vertical, panel, 0.5);
                    }
                    DropZone::Bottom => {
                        self.split_container(target_container, SplitDirection::Vertical, panel, 0.5);
                    }
                }
            }
        }
    }
}
