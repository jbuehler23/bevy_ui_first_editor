//! Hierarchy panel and entity tree view
//!
//! Provides the scene tree view with entity selection, visibility toggles,
//! context menus, search, keyboard navigation, and drag-and-drop reparenting.

mod context_menu;
mod visibility;
mod keyboard_nav;
mod interactions;
mod search;
mod panel;

// Re-export public items
pub use context_menu::{
    ContextMenu, ContextMenuAction,
    handle_tree_row_right_clicks,
    handle_context_menu_actions,
    close_context_menu_on_click_outside,
};

pub use visibility::{
    VisibilityToggleButton, EntityNameText,
    handle_visibility_toggle_clicks,
    update_tree_row_visibility_appearance,
};

pub use keyboard_nav::{
    handle_hierarchy_keyboard_navigation,
};

pub use interactions::{
    handle_tree_row_clicks,
    handle_tree_row_drag_start,
    handle_tree_row_drag_over,
    handle_tree_row_drop,
};

pub use search::{
    handle_search_input,
    handle_clear_search_button,
};

pub use panel::{
    update_scene_tree_panel,
};
