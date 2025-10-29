# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a modular, Bevy-native game editor built entirely using Bevy core infrastructure (main branch). The editor demonstrates what's possible with current Bevy functionality and serves as a foundation for the official Bevy Editor.

**Key Constraint:** Uses ONLY functionality available in Bevy core (main branch, commit 4679505b). No external UI frameworks like egui - pure bevy_ui only.

## Build Commands

```bash
# Check the workspace
cargo check --workspace

# Run the editor (debug mode)
cargo run

# Run the editor (release mode - recommended for performance)
cargo run --release

# Build with limited parallelism to avoid OOM on memory-constrained systems
cargo build -j 2
```

Note: Initial builds take ~10 minutes. Incremental builds are 1-2 minutes.

### WSL2 / Linux Environment Setup

**⚠️ IMPORTANT: GUI Display Required**

The Bevy Editor is a graphical application and requires a display server. In WSL2, this means you need an X11 server.

**Recommended Approach: Run on Windows Natively**

For the best experience, run the editor natively on Windows rather than through WSL2:
```powershell
# In Windows PowerShell or Command Prompt
cd C:\dev\workspace\bevy_ui_first_editor
cargo run --release
```

**Alternative: WSL2 with X Server**

If you must run in WSL2, you need:

1. **Windows 11 with WSLg** (has built-in GUI support), OR
2. **Windows 10 with an X server:**
   - Install VcXsrv, X410, or Xming on Windows
   - Start the X server with "Disable access control" enabled
   - In WSL2, set: `export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0`

3. **Verify X server is accessible:**
```bash
# Check DISPLAY is set
echo $DISPLAY

# Test X server connection (requires x11-apps)
sudo apt-get install x11-apps
xeyes  # Should open a window with eyes that follow your mouse
```

4. **Run the editor:**
```bash
cargo run
```

The code automatically forces X11 backend on Linux to avoid Wayland issues.

## Workspace Architecture

The editor is structured as a cargo workspace with 8 modular crates + 1 application crate:

### Core Crates

- **bevy_editor_core** - Editor state machine (`EditorState`), selection management (`EditorSelection`), core utilities
- **bevy_editor_ui** - Native bevy_ui docking system, panels, widgets (currently disabled to avoid blocking picking)
- **bevy_editor_viewport** - 3D/2D viewport with camera controls, gizmos, picking, grid rendering
- **bevy_editor_hierarchy** - Entity hierarchy tree view and operations
- **bevy_editor_inspector** - Reflection-based component property editing
- **bevy_editor_assets** - Asset browser with file system watching
- **bevy_editor_undo** - Command pattern-based undo/redo system
- **bevy_editor_project** - Project and scene management (RON-based)

### Application

- **editor_app** - Main application binary that assembles all editor plugins

## Critical Plugin Ordering

**IMPORTANT:** Plugin initialization order matters. The viewport plugin MUST be initialized before the UI plugin:

```rust
.add_plugins((
    EditorCorePlugin,
    EditorViewportPlugin,  // Must come before EditorUiPlugin
    EditorUiPlugin,
    // ... other plugins
))
```

**Reason:** Camera3d requires proper render graph setup. If EditorUiPlugin initializes first, it interferes with camera configuration, breaking the picking system.

## Architecture Patterns

### ECS-First Design

- Editor state is stored in resources (`EditorSelection`, `GridConfig`)
- UI panels, cameras, and tools are ECS entities
- Systems handle all editor logic
- Observer pattern for entity interactions (e.g., `On<Pointer<Click>>`)

### Camera System

The viewport uses a custom orbit camera controller with spherical coordinates:

```rust
// Location: crates/bevy_editor_viewport/src/camera.rs
x = radius * cos(pitch) * sin(yaw)
y = radius * sin(pitch)
z = radius * cos(pitch) * cos(yaw)

// Constraints
pitch: clamped to (-π/2 + 0.01, π/2 - 0.01)  // Prevents gimbal lock
radius: clamped to (0.1, 1000.0)
```

**Camera Controls:**
- Right Mouse Button + Drag: Orbit around focus point
- Middle Mouse Button + Drag: Pan the camera
- Scroll Wheel: Zoom in/out

### Selection System

Entity selection uses Bevy's built-in picking system:

1. `MeshPickingPlugin` provides 3D mesh ray casting
2. Entities spawn with `.observe(on_entity_click)` observers
3. Click events update `EditorSelection` resource
4. `draw_selection_outline()` system renders yellow/orange wireframe boxes using bevy_gizmos

**Current limitation:** UI nodes block 3D picking. The EditorUiPlugin temporarily has UI spawning disabled to allow viewport interaction.

### Grid Rendering

Grid is drawn using `bevy_gizmos` immediate-mode API:
- 100x100 grid (50 cells each direction)
- Major lines every 10 cells
- Color-coded axes: red=X, blue=Z
- Configurable via `GridConfig` resource

## Bevy Core API Notes

### MessageReader vs EventReader

Bevy main has renamed `EventReader` to `MessageReader`. Use `MessageReader` for all event handling:

```rust
// Old (pre-main)
fn system(mut events: EventReader<MyEvent>) { }

// New (main branch)
fn system(mut events: MessageReader<MyEvent>) { }
```

### AmbientLight Component

`AmbientLight` is now a component, not a resource:

```rust
// Spawn ambient light as an entity
commands.spawn((
    AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        affects_lightmapped_meshes: false,  // Required field
    },
    Name::new("Ambient Light"),
));
```

### Camera Spawning

Always spawn Camera3d with all components atomically:

```rust
// Correct pattern
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y),
));

// Wrong: Adding components post-spawn can break required component registration
let entity = commands.spawn(Camera3d::default()).id();
commands.entity(entity).insert(Transform::default());  // Avoid this
```

### Picking System

Bevy core includes a complete picking system (no external crates needed):

```rust
use bevy::picking::mesh_picking::MeshPickingPlugin;

// Available events
On<Pointer<Click>>   // Click event
On<Pointer<Over>>    // Hover enter
On<Pointer<Out>>     // Hover exit
On<Pointer<Drag>>    // Drag event

// Usage
commands.spawn((Mesh3d, Transform))
    .observe(my_click_handler);
```

## Current Implementation Status

### Phase 1: Fixed Layout UI System (✅ COMPLETED)
- ✅ 4-panel fixed layout matching Figma design
- ✅ Viewport (center/left) - transparent, allows 3D picking
- ✅ Scene Tree panel (right top)
- ✅ Inspector panel (right bottom)
- ✅ Asset Browser panel (bottom, full width)
- ✅ Panel styling with borders and dark theme
- ✅ Panel titles displayed

### Phase 2-10: In Development
See comprehensive 10-phase development plan in project documentation.

### Working Features
- ✅ 3D viewport rendering
- ✅ Orbit/pan/zoom camera controls
- ✅ Grid with axis indicators
- ✅ Test scene with lit objects and shadows
- ✅ Entity selection via clicking
- ✅ Selection highlighting with wireframe boxes
- ✅ Fixed UI layout with 4 panels

### Next Priorities
- Multi-selection (Shift+Click, Ctrl+Click)
- Click-to-deselect on empty space
- Populate hierarchy panel with entity tree
- Inspector panel with reflection-based component editing
- Interactive transform gizmos
- Play mode system
- Scene save/load

## Design Principles

1. **Bevy-Native Only** - Use only what's in Bevy core. Build missing features in an upstream-ready way.
2. **ECS-Integrated** - Editor is built with Bevy's ECS, not bolted on top.
3. **Modular & Reusable** - Each crate can be used standalone in any Bevy application.
4. **Upstream-Ready** - All custom implementations follow Bevy conventions for potential upstreaming.

## Known Issues

1. **WSL2 Wayland Panic (FIXED):** The editor would panic on startup in WSL2 with "NoCompositor" error. **Solution:** Added `.cargo/config.toml` to automatically force X11 backend. Ensure you have an X11 server running (VcXsrv, X410, or WSLg) and `DISPLAY` is set.

2. **UI Layout Fixed (Not Docking):** Current UI uses fixed panel sizes and positions. Resizing and docking will be implemented in a later phase.

3. **Build Memory Usage:** Initial workspace build can cause OOM on systems with limited memory. Use `cargo build -j 2` to limit parallelism.

4. **Mystery Camera Entity:** A bare `Camera` component is created somewhere (entity 19v0) after PostStartup, likely by Bevy internals. Not currently causing issues but warrants investigation.

## File Locations

Key implementation files:
- Camera controller: `crates/bevy_editor_viewport/src/camera.rs`
- Selection system: `crates/bevy_editor_viewport/src/lib.rs:91-180`
- Grid rendering: `crates/bevy_editor_viewport/src/grid.rs`
- Editor state: `crates/bevy_editor_core/src/editor_state.rs`
- Selection resource: `crates/bevy_editor_core/src/selection.rs`
- Main application: `editor_app/src/main.rs`

## Development Workflow

When adding new features:

1. Determine which crate the feature belongs to based on the module structure
2. Add dependencies to that crate's Cargo.toml if needed
3. Implement as a system or plugin following ECS patterns
4. Use observers (`On<Event>`) for entity-specific interactions
5. Add to the appropriate plugin's `build()` method
6. Test that plugin ordering doesn't break existing functionality
7. Document any Bevy API quirks or ordering requirements

## References

- Bevy main branch: https://github.com/bevyengine/bevy (commit 4679505b)
- Development progress: See PROGRESS.md for detailed session notes
- Architecture overview: See README.md for high-level design
