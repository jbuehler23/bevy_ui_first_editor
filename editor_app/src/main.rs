//! Bevy Native Editor
//!
//! A modular, plugin-based game editor built entirely with Bevy and bevy_ui.

use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;

// Import all editor plugins
use bevy_editor_assets::EditorAssetsPlugin;
use bevy_editor_core::EditorCorePlugin;
use bevy_editor_hierarchy::EditorHierarchyPlugin;
use bevy_editor_inspector::EditorInspectorPlugin;
use bevy_editor_project::EditorProjectPlugin;
use bevy_editor_ui::EditorUiPlugin;
use bevy_editor_undo::EditorUndoPlugin;
use bevy_editor_viewport::EditorViewportPlugin;

fn main() -> AppExit {
    // CRITICAL: Force X11 backend on Linux to avoid Wayland compositor panics in WSL2
    // This must be set BEFORE creating the Bevy App, as winit reads it during initialization
    #[cfg(target_os = "linux")]
    {
        // SAFETY: We set this environment variable before any threads are spawned
        // and before winit initializes. This is safe as it happens at the very start
        // of main() and no other code has read this variable yet.
        unsafe {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }

        // Check if DISPLAY is set
        match std::env::var("DISPLAY") {
            Ok(display) => println!("✓ X11 backend forced, DISPLAY={}", display),
            Err(_) => {
                eprintln!("═══════════════════════════════════════════════════════════════");
                eprintln!("ERROR: DISPLAY environment variable not set!");
                eprintln!("═══════════════════════════════════════════════════════════════");
                eprintln!();
                eprintln!("The Bevy Editor requires a display server to run.");
                eprintln!();
                eprintln!("For WSL2, you have these options:");
                eprintln!("  1. Use Windows 11 with WSLg (GUI support built-in)");
                eprintln!("  2. Install an X server on Windows (VcXsrv, X410, Xming)");
                eprintln!("  3. Run the editor on Windows natively (recommended)");
                eprintln!();
                eprintln!("If you have an X server running, set DISPLAY:");
                eprintln!("  export DISPLAY=:0");
                eprintln!();
                eprintln!("Then run: cargo run");
                eprintln!("═══════════════════════════════════════════════════════════════");
                std::process::exit(1);
            }
        }
    }
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Editor".into(),
                resolution: (1600, 900).into(),
                ..default()
            }),
            ..default()
        }))
        // Add mesh picking backend for entity selection
        // Note: DefaultPlugins in Bevy main now includes DefaultPickingPlugins
        .add_plugins(MeshPickingPlugin)
        // Add all editor plugins
        // IMPORTANT: EditorViewportPlugin MUST come before EditorUiPlugin to ensure
        // proper camera initialization and render graph setup
        .add_plugins((
            EditorCorePlugin,
            EditorViewportPlugin,
            EditorUiPlugin,
            EditorHierarchyPlugin,
            EditorInspectorPlugin,
            EditorAssetsPlugin,
            EditorUndoPlugin,
            EditorProjectPlugin,
        ))
        .run()
}
