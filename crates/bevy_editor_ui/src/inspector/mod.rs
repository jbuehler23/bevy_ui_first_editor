//! Inspector panel for displaying and editing entity properties
//!
//! The inspector shows detailed information about the currently selected entity,
//! including all its components and their properties.

mod panel;
mod transform_editor;
mod sprite_editor;

// Re-export public items
pub use panel::{InspectorPanel, update_inspector_panel};
pub use transform_editor::{
    TransformField, TransformEditor, TransformEditState,
    handle_transform_editor_click, handle_transform_edit_input, update_transform_editor_display,
};
pub use sprite_editor::{
    SpriteFlipXCheckbox, SpriteFlipYCheckbox, SpriteTextureButton, PendingTextureSelection,
    handle_texture_button, apply_pending_texture,
};
