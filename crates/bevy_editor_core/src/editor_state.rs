//! Editor state machine
//!
//! Manages the overall state of the editor including project loading,
//! editing mode, play mode, and building.

use bevy::prelude::*;

/// Top-level editor state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum EditorState {
    /// No project is currently loaded
    #[default]
    NoProject,
    /// Project is being loaded
    ProjectLoading,
    /// Normal editing mode - game is stopped
    Editing,
    /// Game is running in the editor
    Playing,
    /// Game is paused
    Paused,
    /// Project is being built
    Building,
}

/// Manages play mode state and controls
#[derive(Resource)]
pub struct PlayModeController {
    pub state: PlayModeState,
    /// Snapshot of the game world before entering play mode
    pub game_snapshot: Option<DynamicScene>,
}

impl Default for PlayModeController {
    fn default() -> Self {
        Self {
            state: PlayModeState::Stopped,
            game_snapshot: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayModeState {
    Stopped,
    Playing,
    Paused,
    /// Frame-by-frame stepping with remaining frames
    Stepping { frames_left: u32 },
}

impl PlayModeController {
    pub fn is_playing(&self) -> bool {
        matches!(self.state, PlayModeState::Playing | PlayModeState::Stepping { .. })
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.state, PlayModeState::Paused)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, PlayModeState::Stopped)
    }
}
