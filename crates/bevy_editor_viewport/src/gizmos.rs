//! Transform gizmos for moving, rotating, and scaling entities

use bevy::prelude::*;

/// Active gizmo mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

impl Default for GizmoMode {
    fn default() -> Self {
        Self::Translate
    }
}

/// Coordinate space for gizmo operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GizmoSpace {
    Local,
    World,
}

impl Default for GizmoSpace {
    fn default() -> Self {
        Self::World
    }
}
