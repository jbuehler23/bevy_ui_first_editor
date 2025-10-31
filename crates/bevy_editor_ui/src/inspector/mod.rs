//! Inspector panel for displaying and editing entity properties
//!
//! The inspector shows detailed information about the currently selected entity,
//! including all its components and their properties.

mod panel;
mod transform_editor;

// Re-export public items
pub use panel::{InspectorPanel, update_inspector_panel};
pub use transform_editor::{TransformField, TransformEditor, TransformEditState};
