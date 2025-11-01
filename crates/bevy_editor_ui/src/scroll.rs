//! Mouse wheel scrolling support for UI panels
//!
//! Based on the official Bevy UI scroll example.

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;

const LINE_HEIGHT: f32 = 20.0; // Pixels per line scroll

/// UI scrolling entity event (uses observer pattern)
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub struct Scroll {
    pub entity: Entity,
    /// Scroll delta in logical coordinates
    pub delta: Vec2,
}

/// Sends scroll events to hovered UI nodes
pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        // Negate delta to match natural scroll direction
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        // Trigger scroll event for all hovered entities
        for pointer_map in hover_map.values() {
            for &entity in pointer_map.keys() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

/// Handles scroll events and updates ScrollPosition
pub fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut scroll_query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = scroll_query.get_mut(scroll.entity) else {
        return;
    };

    let mut delta = scroll.delta;

    // Calculate maximum scroll offset (content size - visible size)
    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    // Handle vertical scrolling
    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
        // Check if already scrolled all the way in the direction of the scroll
        let at_max = if delta.y > 0.0 {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.0
        };

        if !at_max {
            scroll_position.y = (scroll_position.y + delta.y).clamp(0.0, max_offset.y.max(0.0));
            delta.y = 0.0;
        }
    }

    // Handle horizontal scrolling
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
        let at_max = if delta.x > 0.0 {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.0
        };

        if !at_max {
            scroll_position.x = (scroll_position.x + delta.x).clamp(0.0, max_offset.x.max(0.0));
            delta.x = 0.0;
        }
    }

    // Stop propagation if we consumed the scroll
    if delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
