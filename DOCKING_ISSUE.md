# Docking System Drop Zone Rendering Issue

**Status:** DEFERRED - Detection works, rendering doesn't
**Priority:** Medium (polish feature, not blocking MVP)
**Date Discovered:** 2025-10-31

## Problem Summary

The docking system's drag-and-drop functionality has working detection logic but the visual drop zone overlays do not render on screen. Users can drag panels successfully and the logs confirm drop targets are detected, but they get no visual feedback about where the panel will dock.

## What Works ✅

1. **Drag Activation** - Panel dragging triggers correctly after 5px threshold
   - Logs show: `🎯 DRAG ACTIVATED! Panel: Hierarchy, Distance: 10.8px`

2. **Drop Target Detection** - Cursor position detection during drag is working
   - Fixed by adding `RelativeCursorPosition` component to containers (`renderer.rs:107`)
   - Rewritten `handle_panel_drag_over()` system uses normalized cursor position instead of `Interaction::Hovered`
   - Logs show: `✅ Found drop target: DockId(0) at pos (0.57, 0.00)`

3. **Drop Zone Calculation** - Zone determination (Left/Right/Top/Bottom/Center) works correctly
   - Based on 30% edge zones
   - Logs show: `Drop zone: Right`

4. **Overlay Entity Creation** - Drop zone overlay entities are spawned
   - Logs show: `✨ Creating overlays for target container: DockId(0)`
   - 5 overlays created per container (Left, Right, Top, Bottom, Center)
   - Entities have child relationship to parent container

## What Doesn't Work ❌

**Visual Rendering** - Overlay entities don't appear on screen despite being spawned

## Technical Details

### Files Involved

- `crates/bevy_editor_ui/src/docking/drop_zones.rs` - Overlay creation and rendering
- `crates/bevy_editor_ui/src/docking/systems.rs` - Drop target detection
- `crates/bevy_editor_ui/src/docking/renderer.rs` - Container UI creation
- `crates/bevy_editor_ui/src/lib.rs` - System ordering

### Key Code: Overlay Creation

```rust
// drop_zones.rs:89-125
fn create_drop_zone_overlay(
    commands: &mut Commands,
    parent: Entity,
    zone: DropZone,
    position: Vec2,
    size: Vec2,
) {
    let color = Color::srgba(0.2, 0.8, 1.0, 0.7); // Bright cyan, 70% opaque

    let overlay = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x * 100.0),
            top: Val::Percent(position.y * 100.0),
            width: Val::Percent(size.x * 100.0),
            height: Val::Percent(size.y * 100.0),
            border: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(color),
        BorderColor::all(Color::srgb(1.0, 0.0, 1.0)),  // Bright magenta border
        DropZoneOverlay { zone },
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        bevy::ui::ZIndex(1000),  // Force to top layer
    )).id();

    commands.entity(parent).add_child(overlay);
}
```

### Debugging Attempts

1. **Increased Visibility** (no effect)
   - Changed opacity from 30% to 70%
   - Added bright cyan background (`Color::srgba(0.2, 0.8, 1.0, 0.7)`)
   - Added bright magenta 4px border
   - Added `ZIndex(1000)` to force top layer

2. **Verified Entity Creation** (working)
   - Logs confirm entity spawning: `✅ Overlay entity spawned: Entity(...)`
   - Each overlay has correct position/size in logs
   - Parent-child relationship established

3. **Fixed Drop Detection** (working)
   - Originally used `Interaction::Hovered` which doesn't work during drag
   - Switched to `RelativeCursorPosition` which updates every frame
   - Detection now works perfectly

## Hypotheses

### Most Likely

1. **Parent Container Clipping**
   - Overlays use `PositionType::Absolute` with percentage positioning
   - Parent container may have `Overflow::clip()` or similar cutting off children
   - Check `DockContainer` node styling in `renderer.rs:96-113`

2. **Coordinate System Mismatch**
   - Absolute positioning in bevy_ui may not work as expected with percentage values
   - UI coordinate system: (0,0) at top-left, Y increases downward
   - Overlay position calculation may need adjustment

3. **Render Layer / Z-Index Issue**
   - Despite `ZIndex(1000)`, overlays may be rendered behind other elements
   - Bevy UI's layering system may require explicit render layer assignment
   - Check if parent container's ZIndex affects child rendering

### Less Likely

4. **Spawning Timing**
   - Overlays spawned but immediately despawned before rendering
   - System ordering issue (though logs show creation happening)

5. **Component Conflict**
   - Some component on parent or overlay preventing rendering
   - `Pickable` settings might affect rendering?

## Next Debugging Steps

1. **Check parent overflow settings** (`renderer.rs:96-113`)
   - Add `overflow: Overflow::visible()` to `DockContainer` node
   - Or ensure no clipping is applied

2. **Test with absolute pixel positioning**
   - Replace percentage-based positioning with fixed pixel values
   - e.g., `left: Val::Px(100.0)` instead of `left: Val::Percent(30.0)`
   - This isolates whether it's a percentage calculation issue

3. **Verify overlays exist in hierarchy**
   - Use Bevy inspector or debug print to check entity hierarchy after spawn
   - Confirm overlays are actually children of expected parent
   - Verify components are attached correctly

4. **Simplify overlay to minimal case**
   - Remove all components except Node + BackgroundColor
   - Start with opaque color and no border
   - Spawn as direct child of DockingRoot instead of DockContainer
   - Gradually add back complexity to find breaking point

5. **Test without parent-child relationship**
   - Spawn overlays as separate top-level entities
   - Position them globally instead of relative to parent
   - If this works, confirms parent clipping or coordinate issue

## Workaround Options

If this proves difficult to fix:

1. **Change visual feedback approach**
   - Use hover highlight on container borders instead of overlay zones
   - Show preview of where panel will be placed (ghost outline)
   - Text label indicating drop zone ("Drop here: Left")

2. **Simplify drop zones**
   - Use entire container as single drop zone (no edge/center distinction)
   - Determine placement based on cursor position at drop time
   - Less visual feedback but functionally equivalent

## References

- Similar issue in egui_dock: Uses colored borders instead of overlay fills
- ImGui: Uses thin colored lines at drop edges, not filled rectangles
- Qt Designer: Uses blue highlight around entire drop target

## Related Files

- `crates/bevy_editor_ui/src/docking/drop_zones.rs` (overlay system)
- `crates/bevy_editor_ui/src/docking/systems.rs` (drag detection - WORKING)
- `crates/bevy_editor_ui/src/docking/renderer.rs` (container creation)
- `crates/bevy_editor_ui/src/lib.rs` (system ordering)

## User Logs Example

```
2025-10-31T08:01:07.043924Z  INFO bevy_editor_ui::docking::systems: 🎯 DRAG ACTIVATED! Panel: Inspector, Distance: 11.0px
2025-10-31T08:01:07.043954Z  INFO bevy_editor_ui::docking::systems: ✅ Found drop target: DockId(0) at pos (0.91, 0.03)
2025-10-31T08:01:07.043961Z  INFO bevy_editor_ui::docking::systems:   Drop zone: Right
2025-10-31T08:01:07.043976Z  INFO bevy_editor_ui::docking::drop_zones: 🎨 Showing drop zones for panel: Some("Inspector"), drop_target: Some(DockId(0))
2025-10-31T08:01:07.043989Z  INFO bevy_editor_ui::docking::drop_zones:   ✨ Creating overlays for target container: DockId(0)
```

Note: Despite logs showing creation, user reports "i still don't see the drop zone in the UI"
