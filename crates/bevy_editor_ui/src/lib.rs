//! Native bevy_ui-based editor UI framework
//!
//! Provides docking, panels, and UI widgets using only bevy_ui.

use bevy::prelude::*;
use bevy_editor_hierarchy::HierarchyState;

pub mod docking;
pub mod panel;
pub mod widgets;
pub mod inspector;
pub mod hierarchy;
pub mod components;
pub mod layout;

pub use docking::*;
pub use panel::*;
pub use widgets::*;
pub use inspector::{InspectorPanel, TransformField, TransformEditor, TransformEditState};
pub use hierarchy::{
    ContextMenu, ContextMenuAction,
    VisibilityToggleButton, EntityNameText,
};
pub use components::*;
pub use layout::setup_editor_ui;


/// Plugin for the native bevy_ui editor UI system
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<DockingLayout>()
            .init_resource::<DockDragState>()
            .init_resource::<DividerDragState>()
            .init_resource::<TransformEditState>()
            .init_resource::<HierarchyState>()
            // Startup systems
            .add_systems(Startup, (
                // setup_editor_ui,  // Disabled: replaced by docking system
                docking::auto_load_layout,
            ))
            // Docking systems with explicit ordering
            .add_systems(Update, (
                // Phase 1: Build UI structure
                docking::build_docking_ui,

                // Phase 2: Populate panel content (after UI exists)
                docking::route_panel_content
                    .after(docking::build_docking_ui),

                // Phase 3: Handle interactions (after UI is ready)
                (
                    docking::handle_divider_drag_start,
                    docking::handle_panel_drag_start,
                    docking::handle_tab_clicks,
                ).after(docking::route_panel_content),

                // Phase 4: Process drag state (after initial detection)
                (
                    docking::activate_drag_on_threshold,
                    docking::handle_divider_drag,
                    docking::handle_panel_drag_over,
                ).after(docking::handle_panel_drag_start),

                // Phase 5: Visual feedback (after drag state updated)
                docking::show_drop_zones
                    .after(docking::handle_panel_drag_over),

                // Phase 6: Finalize (after everything)
                (
                    docking::handle_divider_drag_end,
                    docking::handle_panel_drop,
                    docking::auto_save_layout,
                ).after(docking::show_drop_zones),
            ))
            // Hierarchy systems
            .add_systems(Update, (
                hierarchy::handle_tree_row_clicks,
                hierarchy::handle_tree_row_right_clicks,
                hierarchy::handle_context_menu_actions,
                hierarchy::handle_visibility_toggle_clicks,
                hierarchy::handle_hierarchy_keyboard_navigation,
                hierarchy::handle_tree_row_drag_start,
                hierarchy::handle_tree_row_drag_over,
                hierarchy::handle_tree_row_drop,
                hierarchy::handle_search_input,
                hierarchy::handle_clear_search_button,
                hierarchy::close_context_menu_on_click_outside,
                hierarchy::update_scene_tree_panel,
                hierarchy::update_tree_row_visibility_appearance,
            ))
            // Inspector systems
            .add_systems(Update, inspector::update_inspector_panel);
    }
}

