# Bevy First-Party Editor

A modular, Bevy-native game editor built entirely using Bevy core infrastructure. This project aims to demonstrate what's possible with current Bevy functionality and serve as a foundation for the official Bevy Editor.

## Architecture

### Modular Workspace Structure

The editor is built as a cargo workspace with 8 core crates:

- **bevy_editor_core** - Editor state machine, selection management, core utilities
- **bevy_editor_ui** - Native bevy_ui docking system, panels, widgets
- **bevy_editor_viewport** - 3D/2D viewport with camera controls, gizmos, picking
- **bevy_editor_hierarchy** - Entity hierarchy tree view and operations
- **bevy_editor_inspector** - Reflection-based component property editing
- **bevy_editor_assets** - Asset browser with file system watching
- **bevy_editor_undo** - Command pattern-based undo/redo system
- **bevy_editor_project** - Project and scene management (RON-based)

### Using Only Bevy Core

This editor uses **ONLY** functionality available in Bevy core (main branch):

#### ✅ Available in Bevy Core

| Module | Status | Usage |
|--------|--------|-------|
| `bevy_gizmos` | ✅ Default | Visual debug drawing, grid rendering |
| `bevy_picking` | ✅ Default | Mouse picking, entity selection, interaction |
| `bevy_ui` | ✅ Default | Layout system (Flexbox/Grid), basic widgets |
| `bevy_scene` | ✅ Default | Scene serialization, hot-reload, prefabs |
| `bevy_reflect` | ✅ Default | Runtime type introspection, generic inspector |
| `bevy_asset` | ✅ With features | Asset loading, hot-reload (`file_watcher`) |
| `bevy_hierarchy` | ✅ Default | Parent/Child relationships, tree traversal |
| `bevy_input` | ✅ Default | Keyboard, mouse, gamepad input |

#### ⚠️ Experimental / To Be Investigated

- `bevy_feathers` - May exist as a module but not as a cargo feature
- `bevy_ui_widgets` - May exist as a module but not as a cargo feature
- `bevy_camera_controller` - May exist as a module but not as a cargo feature
- `bevy_input_focus` - May exist as a module but not as a cargo feature

These will be investigated and used directly if available in the codebase.

#### ❌ Not Available (Need to Implement)

- **Docking system** - Build custom using bevy_ui flex layout
- **Interactive gizmos** - Transform handles using bevy_picking + bevy_gizmos
- **Orbit camera** - Extend existing camera systems
- **Tree view widget** - Custom hierarchical list widget
- **Text input widget** - Custom input field
- **Menu system** - Menu bars and context menus

## Current Status

### Phase 1: Foundation ✅

- ✅ Cargo workspace with 8 modular crates
- ✅ Basic plugin structure
- ✅ Editor state machine (NoProject, Editing, Playing, Paused, Building)
- ✅ Selection management resource
- ✅ Command-based undo/redo architecture
- ✅ Project configuration system

### Phase 2: Viewport & Camera ✅

- ✅ Orbit camera controller (right-click + drag)
- ✅ Pan camera (middle-click + drag)
- ✅ Zoom camera (scroll wheel)
- ✅ Grid rendering with bevy_gizmos
- ✅ Test scene with objects and lighting
- ✅ Using MessageReader (Bevy main API)

### Phase 3: Next Steps (In Progress)

- [ ] Setup bevy_picking for entity selection
- [ ] Implement selection highlighting
- [ ] Build docking system with bevy_ui
- [ ] Create basic panels (Hierarchy, Inspector, Viewport, Assets)

## Design Principles

### 1. Bevy-Native Only
Use only what's available in Bevy core. If something doesn't exist, we build it in a way that can be upstreamed to Bevy itself.

### 2. ECS-Integrated
The editor is built with Bevy's ECS, not bolted on top. Editor state, UI, and tools are all ECS entities/resources/systems.

### 3. Modular & Reusable
Each crate is designed to be usable standalone. The docking system, inspector, etc. could be used in any Bevy application.

### 4. Upstream-Ready
All custom implementations follow Bevy conventions and are documented for potential upstreaming.

## Building

```bash
# Check the workspace
cargo check --workspace

# Run the editor (debug mode)
cargo run

# Run the editor (release mode - recommended)
cargo run --release
```

## Using the Editor

### Camera Controls
- **Right Mouse Button + Drag** - Orbit around focus point
- **Middle Mouse Button + Drag** - Pan the camera
- **Scroll Wheel** - Zoom in/out

### What You'll See
- A 3D viewport with a grid (red X-axis, blue Z-axis)
- Three test objects: Cube (beige), Sphere (blue), Cylinder (red)
- Directional lighting with shadows
- The camera starts orbiting around the center of the scene

## Development Notes

- Built with Bevy main branch (commit 4679505b)
- Uses `MessageReader` instead of deprecated `EventReader`
- `AmbientLight` is now a component, not a resource
- All Bevy core modules (gizmos, picking, ui, scene, reflect) are available by default

## Roadmap

- **Week 1-2:** Viewport, camera, picking, selection
- **Week 2-3:** Docking system, basic panels
- **Week 3-4:** Inspector with reflection, hierarchy tree
- **Week 4:** Gizmos, undo/redo, basic scene save/load

## Contributing

Since this is designed as the foundation for the official Bevy Editor, all contributions should follow Bevy's code style and conventions. See CONTRIBUTING.md (TBD) for details.

## License

Dual-licensed under MIT or Apache-2.0, matching Bevy's licensing.
