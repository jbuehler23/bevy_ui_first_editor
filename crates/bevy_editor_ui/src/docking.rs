//! Docking system for editor panels

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Describes the layout of docked panels
#[derive(Debug, Resource, Default, Serialize, Deserialize)]
pub struct DockingLayout {
    pub root: Option<DockNode>,
}

/// A node in the docking tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockNode {
    /// A panel container with tabs
    Panel {
        panels: Vec<String>,
        active: usize,
    },
    /// A split container
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<DockNode>,
        second: Box<DockNode>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}
