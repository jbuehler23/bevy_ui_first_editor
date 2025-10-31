//! Search widget for filtering entities in the hierarchy
//!
//! Provides a reusable search box with text input and clear button.

use bevy::prelude::*;
use crate::{HierarchyState, SearchInputBox, SearchInputText, ClearSearchButton};

/// Handle keyboard input for search box
pub fn handle_search_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: MessageReader<bevy::input::keyboard::KeyboardInput>,
    search_box_query: Query<&Interaction, (With<SearchInputBox>, Changed<Interaction>)>,
    mut hierarchy_state: ResMut<HierarchyState>,
    mut search_text_query: Query<&mut Text, With<SearchInputText>>,
) {
    // Check if search box was clicked (becomes focused)
    let mut _is_focused = false;
    for interaction in &search_box_query {
        if *interaction == Interaction::Pressed {
            _is_focused = true;
        }
    }

    // Simple approach: always capture input when user types (basic implementation)
    // Handle backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        hierarchy_state.search_filter.pop();
    }

    // Handle character input
    for event in char_events.read() {
        if let bevy::input::keyboard::KeyboardInput {
            key_code: key,
            state: bevy::input::ButtonState::Pressed,
            ..
        } = event
        {
            // Convert keycode to character (simplified)
            if let Some(ch) = keycode_to_char(*key, keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)) {
                hierarchy_state.search_filter.push(ch);
            }
        }
    }

    // Update search text display
    for mut text in &mut search_text_query {
        if hierarchy_state.search_filter.is_empty() {
            **text = "Search...".to_string();
        } else {
            **text = hierarchy_state.search_filter.clone();
        }
    }
}

/// Convert keycode to character (simplified implementation)
fn keycode_to_char(key: KeyCode, shift: bool) -> Option<char> {
    match key {
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}

/// Handle clear search button clicks
pub fn handle_clear_search_button(
    interaction_query: Query<&Interaction, (With<ClearSearchButton>, Changed<Interaction>)>,
    mut hierarchy_state: ResMut<HierarchyState>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            hierarchy_state.search_filter.clear();
        }
    }
}
