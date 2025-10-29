# Bevy Editor Development Roadmap

This document outlines the iterative, feature-by-feature development plan for building a complete Godot/Unity-style game editor for Bevy.

## Development Philosophy

- **Iterative:** Each phase is fully implemented and tested before proceeding
- **Integrated:** New features integrate with existing functionality
- **Upstream-Ready:** All code follows Bevy conventions for potential upstreaming
- **Well-Documented:** Each phase includes examples and documentation

## Phase 1: Fixed Layout UI System ✅ COMPLETED

**Goal:** Solve UI blocking picking, establish 4-panel layout matching Figma design

### Completed Features
- ✅ Fixed layout: viewport (center), scene tree (right top), inspector (right bottom), asset browser (bottom)
- ✅ Transparent viewport container allows 3D picking to work
- ✅ Panel borders and spacing with bevy_ui
- ✅ Panel titles displayed
- ✅ Dark theme styling

### Technical Details
- Root flex container with column layout
- Top row contains viewport + right sidebar (scene tree + inspector stacked vertically)
- Bottom panel spans full width for assets
- Viewport area is transparent (`BackgroundColor(Color::NONE)`) to allow mouse events to reach 3D picking system

### Files Modified
- `crates/bevy_editor_ui/src/lib.rs` - Complete UI layout rewrite

---

## Phase 2: Hierarchy Panel (Scene Tree)

**Goal:** Complete entity management interface with tree view

### Features to Implement
- [ ] Tree widget with expand/collapse for parent/child entities
- [ ] Display entity names with icons based on components
- [ ] Click to select (single selection working, need multi-select)
- [ ] Multi-select with Shift (range) and Ctrl (toggle)
- [ ] Visibility toggles (eye icons) to show/hide entities in viewport
- [ ] Filter/search functionality to find entities by name
- [ ] Right-click context menu (delete, duplicate, reparent, add child)
- [ ] Drag-and-drop to reparent entities
- [ ] Keyboard navigation (arrow keys, Enter to rename)

### Technical Approach
1. Query all entities with `Name` component
2. Build tree structure from `Parent`/`Children` relationships
3. Create custom tree widget using bevy_ui flexbox
4. Store expanded/collapsed state in resource
5. Sync selection with `EditorSelection` resource
6. Update viewport highlight when selection changes

### Dependencies
- `bevy_editor_core::EditorSelection` (already exists)
- New: `HierarchyState` resource for UI state
- New: Tree widget components

---

## Phase 3: Inspector Panel with Reflection

**Goal:** View and edit component properties through reflection

### Features to Implement
- [ ] List all components on selected entity
- [ ] Transform component editor (translation/rotation/scale XYZ fields with labels)
- [ ] "Add Component" button with searchable dropdown/modal
- [ ] Collapsible component sections
- [ ] Generic property editors using `bevy_reflect`:
  - Numbers (f32, f64, i32, u32, etc.) with drag-to-edit
  - Booleans (checkbox)
  - Strings (text input)
  - Vec2/Vec3/Vec4 (multi-field with labels)
  - Colors (color picker)
  - Enums (dropdown)
  - Options (checkbox + nested fields)
- [ ] "Remove Component" button per component
- [ ] Live updates (changes reflect immediately in viewport)
- [ ] Undo/redo integration

### Technical Approach
1. Query selected entity for all components with `Reflect` trait
2. Use `TypeRegistry` to get component metadata
3. Generate UI based on reflected field types
4. Create `PropertyEditor` trait for custom editors
5. Implement built-in editors for common types
6. Wire up change events to update ECS components
7. Create commands for undo/redo

### Dependencies
- `bevy::reflect::TypeRegistry`
- `bevy_editor_undo::Command` (for undo)
- New: Property editor widgets

---

## Phase 4: Interactive Transform Gizmos

**Goal:** Manipulate objects directly in viewport with visual handles

### Features to Implement
- [ ] **Move Gizmo:** 3 arrows (X=red, Y=green, Z=blue)
  - Click arrow to constrain movement to axis
  - Click center to move freely in camera plane
  - Show current delta while dragging
- [ ] **Rotate Gizmo:** 3 circles for rotation around each axis
  - X=red, Y=green, Z=blue circles
  - Drag circle to rotate around axis
  - Show angle delta while rotating
- [ ] **Scale Gizmo:** 3 cubes on axes
  - Click cube to scale along axis
  - Click center to scale uniformly
  - Show scale factor while dragging
- [ ] **Gizmo Mode Toggle:** W/E/R hotkeys (Unity-style)
- [ ] **Local vs World Space Toggle:** Keyboard shortcut to switch
- [ ] **Snapping:** Configurable snap increments (position, rotation, scale)
- [ ] **Visual Feedback:** Highlight selected axis/plane during drag
- [ ] **Inspector Sync:** Update inspector values in real-time during manipulation

### Technical Approach
1. Render gizmos using `bevy_gizmos` (non-pickable, always-on-top)
2. Create separate pickable meshes for each gizmo handle
3. Use `bevy_picking` to detect handle clicks
4. On drag: raycast against constraint plane/axis
5. Update transform based on drag delta
6. Support multi-selection (edit all selected at once)
7. Create command for undo when drag completes

### Dependencies
- `bevy_picking` for handle interaction
- `bevy_gizmos` for visual rendering
- Math utilities for raycasting and constraint planes
- `bevy_editor_undo::Command` for undo

---

## Phase 5: Play Mode System

**Goal:** Test scenes while editing with proper isolation

### Features to Implement
- [ ] **Play Button:** Start play mode from current scene state
- [ ] **Pause Button:** Pause game systems while preserving state
- [ ] **Stop Button:** Exit play mode and revert to edit mode
- [ ] **World Duplication:** Create separate play world, preserve edit world
- [ ] **System Filtering:** Game systems run only in play mode
- [ ] **Camera Preservation:** Keep editor camera active, disable game cameras in edit mode
- [ ] **Visual Indicator:** Tint/border when in play mode
- [ ] **Revert on Stop:** Discard play world changes unless explicitly saved
- [ ] **Hot Reload:** Optionally keep changes (checkbox or modal prompt)

### Technical Approach
1. Extend `EditorState` enum: Edit, Playing, Paused
2. Clone entire scene to separate world on play
3. Use `AppState` or schedule to control which systems run
4. Keep editor camera in edit world, create proxy camera for viewport
5. Add visual overlay when in play mode
6. On stop: despawn play world, restore edit world
7. Optional: Diff worlds and allow selective merges

### Dependencies
- `bevy::ecs::World` for world duplication
- `bevy_scene` for serialization/cloning
- `bevy_editor_core::EditorState`

---

## Phase 6: Scene Save/Load & Project Management

**Goal:** Persist work and manage multiple scenes

### Features to Implement
- [ ] **Scene Serialization:** Save current scene to RON format
- [ ] **Scene Deserialization:** Load scene from RON file
- [ ] **Project Structure:** `/assets` and `/scenes` folders
- [ ] **New Scene Command:** Clear current, start fresh
- [ ] **Open Scene Command:** Browse and load existing scene
- [ ] **Save Scene Command:** Save to current file
- [ ] **Save As Command:** Choose new file location
- [ ] **Scene Tab System:** Browser-style tabs for multiple open scenes
- [ ] **Modified Indicator:** Show `*` in tab when scene has unsaved changes
- [ ] **Auto-save:** Periodic background saves to temp location
- [ ] **Crash Recovery:** Restore auto-saved scenes on startup

### Technical Approach
1. Use `DynamicScene` for serialization
2. Serialize to RON format (human-readable, merge-friendly)
3. Track current scene path in resource
4. Track dirty flag for unsaved changes
5. Implement file browser UI with bevy_ui
6. Auto-save every N seconds to `.editor/autosave/`
7. Check for autosaves on startup and prompt recovery

### Dependencies
- `bevy_scene::DynamicScene`
- `ron` for serialization
- File system APIs for reading/writing
- `bevy_editor_project` crate (already scaffolded)

---

## Phase 7: Asset Browser

**Goal:** Manage and import project assets visually

### Features to Implement
- [ ] **Folder Navigation:** Breadcrumb trail showing current path
- [ ] **Asset Grid/List View:** Toggle between icon grid and detailed list
- [ ] **File System Watcher:** Auto-update when files change externally
- [ ] **Asset Thumbnails:** Generate previews for meshes, textures, scenes
- [ ] **Drag to Spawn:** Drag mesh/prefab into viewport to instantiate
- [ ] **Search and Filter:** Find assets by name, type, tags
- [ ] **Import Settings:** Configure texture compression, mesh optimization
- [ ] **Supported Types:** Meshes (.gltf/.glb), textures (.png/.jpg), scenes (.scn.ron)
- [ ] **Context Menu:** Import, delete, rename, reveal in file manager

### Technical Approach
1. Use `notify` crate for file system watching
2. Scan asset folders on startup
3. Generate thumbnails:
   - Meshes: Render small preview with bevy_render
   - Textures: Load and display
   - Scenes: Icon or preview from metadata
4. Create asset entry components with metadata
5. Implement drag-and-drop with `bevy_picking`
6. Spawn entities on drop using asset handles

### Dependencies
- `notify` for file watching
- `bevy_asset` for loading
- `bevy_render` for thumbnail generation
- `bevy_editor_assets` crate (already scaffolded)

---

## Phase 8: Undo/Redo System

**Goal:** Make editor forgiving and professional

### Features to Implement
- [ ] **Command Pattern:** All operations wrapped in reversible commands
- [ ] **Undo Stack:** Limited history (configurable, default 100 actions)
- [ ] **Redo Stack:** Cleared on new action, rebuilt on undo
- [ ] **Keyboard Shortcuts:** Ctrl+Z (undo), Ctrl+Shift+Z (redo)
- [ ] **Supported Operations:**
  - Entity spawn/delete
  - Component add/remove
  - Property value changes
  - Transform modifications (gizmo drags)
  - Hierarchy changes (reparenting)
- [ ] **Batching:** Group related changes (e.g., multi-selection edit) into single undo
- [ ] **Clear on Play:** Reset undo stack when entering play mode
- [ ] **UI Indicator:** Show current action name in status bar

### Technical Approach
1. Define `Command` trait with `execute()`, `undo()`, `redo()`
2. Implement commands for each operation type
3. Store commands in `CommandHistory` resource
4. Wrap all editor mutations in commands
5. Execute + push to stack on action
6. Pop and call `undo()` on Ctrl+Z
7. Batch commands with `CompositeCommand`

### Dependencies
- `bevy_editor_undo` crate (already scaffolded)
- Integration with inspector, gizmos, hierarchy

---

## Phase 9: Menu System & Toolbar

**Goal:** Polish top-level navigation

### Features to Implement
- [ ] **Menu Bar:** File, Edit, View, Entity, Help
- [ ] **File Menu:**
  - New Scene
  - Open Scene
  - Save Scene
  - Save Scene As
  - Open Project
  - Quit
- [ ] **Edit Menu:**
  - Undo
  - Redo
  - Preferences (open settings dialog)
- [ ] **View Menu:**
  - Toggle Scene Tree
  - Toggle Inspector
  - Toggle Assets
  - Camera Views (Top, Front, Side, Perspective)
- [ ] **Entity Menu:**
  - Create Empty
  - Create Cube/Sphere/Cylinder/etc.
  - Create Light (Directional, Point, Spot)
  - Create Camera
- [ ] **Help Menu:**
  - Documentation Link
  - About
- [ ] **Toolbar:** Icon buttons for common actions
  - Gizmo mode selector (Move/Rotate/Scale)
  - Play/Pause/Stop
  - Snap toggle
- [ ] **Keyboard Shortcuts:** Display in menus, customizable

### Technical Approach
1. Create menu bar with bevy_ui at top of screen
2. Implement dropdown menus with click-to-open
3. Close menus on outside click or action
4. Bind keyboard shortcuts to menu actions
5. Store shortcuts in settings resource
6. Create icon assets for toolbar
7. Update CLAUDE.md with all shortcuts

### Dependencies
- bevy_ui for menus and toolbar
- Settings/preferences system
- Icon assets (SVG or PNG)

---

## Phase 10: Advanced Features & Polish

**Goal:** Reach parity with Figma design and beyond

### Features to Implement
- [ ] **Resizable Panels:** Drag borders to resize scene tree/inspector width
- [ ] **Full Docking System:** Drag panels to dock in different positions
- [ ] **Layout Presets:** Default, Tall, Wide, etc.
- [ ] **Layout Save/Load:** Persist user layout preferences
- [ ] **Multi-Viewport Support:** Split viewport for orthographic views
- [ ] **Component Observers Tab:** View and edit ECS observers
- [ ] **Status Bar:** Git branch, notifications, FPS counter
- [ ] **Preferences Dialog:** Theme, keybinds, editor settings
- [ ] **Project Settings Dialog:** Physics, rendering, input config
- [ ] **Console Panel:** View logs, warnings, errors with filtering
- [ ] **Profiler Integration:** Performance graphs and frame timing
- [ ] **Plugin System:** Allow third-party editor extensions

### Technical Approach
1. Implement resize handles with drag detection
2. Build docking system with drop zones and preview overlay
3. Serialize layout to RON file in project folder
4. Add additional camera entities for split views
5. Create tabbed panel system for console/profiler
6. Integrate `bevy_diagnostic` for performance metrics
7. Design plugin API with traits and registration

### Dependencies
- Advanced bevy_ui layout techniques
- Drag-and-drop system enhancements
- Settings persistence
- Plugin API design

---

## Development Guidelines

### Before Starting Each Phase
1. Review dependencies from previous phases
2. Check if any foundation work is needed
3. Create todo list with specific tasks
4. Update CLAUDE.md with new patterns discovered

### During Implementation
1. Write code in small, testable increments
2. Test each feature as it's completed
3. Update PROGRESS.md with discoveries and decisions
4. Document any Bevy API quirks in CLAUDE.md

### After Completing Each Phase
1. Test integration with all previous features
2. Update CLAUDE.md Current Implementation Status
3. Create examples or demo scenes if applicable
4. Tag git commit with phase number
5. Get user feedback before proceeding

---

## Timeline Estimates

| Phase | Estimated Time | Complexity |
|-------|----------------|------------|
| Phase 1 | ✅ Complete | Low |
| Phase 2 | 2-3 days | Medium |
| Phase 3 | 3-5 days | High |
| Phase 4 | 4-6 days | High |
| Phase 5 | 2-3 days | Medium |
| Phase 6 | 2-4 days | Medium |
| Phase 7 | 3-5 days | Medium |
| Phase 8 | 2-3 days | Medium |
| Phase 9 | 2-3 days | Low |
| Phase 10 | 1-2 weeks | High |

**Total:** ~6-8 weeks for full feature set

---

## Success Criteria

The editor will be considered complete when:
1. ✅ It runs using only Bevy core (no external UI frameworks)
2. ✅ It has a professional, polished UI matching the Figma design
3. ✅ Users can create, edit, and save complete game scenes
4. ✅ It supports play mode for immediate testing
5. ✅ All features have undo/redo support
6. ✅ It performs well with complex scenes (1000+ entities)
7. ✅ Code is documented and follows Bevy conventions
8. ✅ It serves as a reference implementation for Bevy editors

