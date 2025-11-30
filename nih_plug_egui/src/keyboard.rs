//! Keyboard event conversion from VST3 to egui.

use egui_baseview::egui;
use nih_plug::editor::{KeyEvent, KeyEventType};

/// VST3 VirtualKeyCodes (from keycodes.h)
/// https://github.com/steinbergmedia/vst3_pluginterfaces/blob/master/base/keycodes.h
///
/// NOTE: These are VST3 SDK's OWN platform-independent codes,
/// NOT Windows VK_* or macOS kVK_* codes!
mod vst3_keys {
    pub const KEY_BACK: i16 = 1;
    pub const KEY_TAB: i16 = 2;
    pub const KEY_CLEAR: i16 = 3;
    pub const KEY_RETURN: i16 = 4;
    pub const KEY_PAUSE: i16 = 5;
    pub const KEY_ESCAPE: i16 = 6;
    pub const KEY_SPACE: i16 = 7;
    pub const KEY_NEXT: i16 = 8; // Page Down
    pub const KEY_END: i16 = 9;
    pub const KEY_HOME: i16 = 10;
    pub const KEY_LEFT: i16 = 11;
    pub const KEY_UP: i16 = 12;
    pub const KEY_RIGHT: i16 = 13;
    pub const KEY_DOWN: i16 = 14;
    pub const KEY_PAGEUP: i16 = 15;
    pub const KEY_PAGEDOWN: i16 = 16;
    pub const KEY_SELECT: i16 = 17;
    pub const KEY_PRINT: i16 = 18;
    pub const KEY_ENTER: i16 = 19;
    pub const KEY_SNAPSHOT: i16 = 20;
    pub const KEY_INSERT: i16 = 21;
    pub const KEY_DELETE: i16 = 22;
    pub const KEY_HELP: i16 = 23;
    // Numpad 0-9
    pub const KEY_NUMPAD0: i16 = 24;
    pub const KEY_NUMPAD1: i16 = 25;
    pub const KEY_NUMPAD2: i16 = 26;
    pub const KEY_NUMPAD3: i16 = 27;
    pub const KEY_NUMPAD4: i16 = 28;
    pub const KEY_NUMPAD5: i16 = 29;
    pub const KEY_NUMPAD6: i16 = 30;
    pub const KEY_NUMPAD7: i16 = 31;
    pub const KEY_NUMPAD8: i16 = 32;
    pub const KEY_NUMPAD9: i16 = 33;
    pub const KEY_MULTIPLY: i16 = 34;
    pub const KEY_ADD: i16 = 35;
    pub const KEY_SEPARATOR: i16 = 36;
    pub const KEY_SUBTRACT: i16 = 37;
    pub const KEY_DECIMAL: i16 = 38;
    pub const KEY_DIVIDE: i16 = 39;
    // F1-F12
    pub const KEY_F1: i16 = 40;
    pub const KEY_F2: i16 = 41;
    pub const KEY_F3: i16 = 42;
    pub const KEY_F4: i16 = 43;
    pub const KEY_F5: i16 = 44;
    pub const KEY_F6: i16 = 45;
    pub const KEY_F7: i16 = 46;
    pub const KEY_F8: i16 = 47;
    pub const KEY_F9: i16 = 48;
    pub const KEY_F10: i16 = 49;
    pub const KEY_F11: i16 = 50;
    pub const KEY_F12: i16 = 51;
    // Other
    pub const KEY_NUMLOCK: i16 = 52;
    pub const KEY_SCROLL: i16 = 53;
    pub const KEY_SHIFT: i16 = 54;
    pub const KEY_CONTROL: i16 = 55;
    pub const KEY_ALT: i16 = 56;
    pub const KEY_EQUALS: i16 = 57;
}

/// Convert VST3 key code to egui Key.
///
/// IMPORTANT: key_code contains VST3 VirtualKeyCodes for special keys.
/// Regular characters (A-Z, 0-9) come in the `key` (char16) parameter,
/// not in key_code!
pub fn vst3_keycode_to_egui_key(key_code: i16) -> Option<egui::Key> {
    use vst3_keys::*;

    match key_code {
        // Navigation
        KEY_BACK => Some(egui::Key::Backspace),
        KEY_TAB => Some(egui::Key::Tab),
        KEY_RETURN | KEY_ENTER => Some(egui::Key::Enter),
        KEY_ESCAPE => Some(egui::Key::Escape),
        KEY_SPACE => Some(egui::Key::Space),
        KEY_DELETE => Some(egui::Key::Delete),
        KEY_INSERT => Some(egui::Key::Insert),

        // Arrow keys
        KEY_LEFT => Some(egui::Key::ArrowLeft),
        KEY_UP => Some(egui::Key::ArrowUp),
        KEY_RIGHT => Some(egui::Key::ArrowRight),
        KEY_DOWN => Some(egui::Key::ArrowDown),

        // Page/Home/End
        KEY_HOME => Some(egui::Key::Home),
        KEY_END => Some(egui::Key::End),
        KEY_PAGEUP => Some(egui::Key::PageUp),
        KEY_PAGEDOWN | KEY_NEXT => Some(egui::Key::PageDown),

        // Numpad
        KEY_NUMPAD0 => Some(egui::Key::Num0),
        KEY_NUMPAD1 => Some(egui::Key::Num1),
        KEY_NUMPAD2 => Some(egui::Key::Num2),
        KEY_NUMPAD3 => Some(egui::Key::Num3),
        KEY_NUMPAD4 => Some(egui::Key::Num4),
        KEY_NUMPAD5 => Some(egui::Key::Num5),
        KEY_NUMPAD6 => Some(egui::Key::Num6),
        KEY_NUMPAD7 => Some(egui::Key::Num7),
        KEY_NUMPAD8 => Some(egui::Key::Num8),
        KEY_NUMPAD9 => Some(egui::Key::Num9),
        KEY_ADD => Some(egui::Key::Plus),
        KEY_SUBTRACT => Some(egui::Key::Minus),

        // Function keys
        KEY_F1 => Some(egui::Key::F1),
        KEY_F2 => Some(egui::Key::F2),
        KEY_F3 => Some(egui::Key::F3),
        KEY_F4 => Some(egui::Key::F4),
        KEY_F5 => Some(egui::Key::F5),
        KEY_F6 => Some(egui::Key::F6),
        KEY_F7 => Some(egui::Key::F7),
        KEY_F8 => Some(egui::Key::F8),
        KEY_F9 => Some(egui::Key::F9),
        KEY_F10 => Some(egui::Key::F10),
        KEY_F11 => Some(egui::Key::F11),
        KEY_F12 => Some(egui::Key::F12),

        // Unknown
        _ => None,
    }
}

/// Convert Unicode character to egui Key.
/// This handles regular characters (A-Z, 0-9, etc.)
pub fn char_to_egui_key(c: char) -> Option<egui::Key> {
    match c.to_ascii_uppercase() {
        'A' => Some(egui::Key::A),
        'B' => Some(egui::Key::B),
        'C' => Some(egui::Key::C),
        'D' => Some(egui::Key::D),
        'E' => Some(egui::Key::E),
        'F' => Some(egui::Key::F),
        'G' => Some(egui::Key::G),
        'H' => Some(egui::Key::H),
        'I' => Some(egui::Key::I),
        'J' => Some(egui::Key::J),
        'K' => Some(egui::Key::K),
        'L' => Some(egui::Key::L),
        'M' => Some(egui::Key::M),
        'N' => Some(egui::Key::N),
        'O' => Some(egui::Key::O),
        'P' => Some(egui::Key::P),
        'Q' => Some(egui::Key::Q),
        'R' => Some(egui::Key::R),
        'S' => Some(egui::Key::S),
        'T' => Some(egui::Key::T),
        'U' => Some(egui::Key::U),
        'V' => Some(egui::Key::V),
        'W' => Some(egui::Key::W),
        'X' => Some(egui::Key::X),
        'Y' => Some(egui::Key::Y),
        'Z' => Some(egui::Key::Z),
        '0' => Some(egui::Key::Num0),
        '1' => Some(egui::Key::Num1),
        '2' => Some(egui::Key::Num2),
        '3' => Some(egui::Key::Num3),
        '4' => Some(egui::Key::Num4),
        '5' => Some(egui::Key::Num5),
        '6' => Some(egui::Key::Num6),
        '7' => Some(egui::Key::Num7),
        '8' => Some(egui::Key::Num8),
        '9' => Some(egui::Key::Num9),
        '.' => Some(egui::Key::Period),
        ',' => Some(egui::Key::Comma),
        '-' => Some(egui::Key::Minus),
        '+' => Some(egui::Key::Plus),
        _ => None,
    }
}

/// Convert a KeyEvent to egui Events.
/// Returns both Key and Text events as needed for TextEdit.
pub fn convert_to_egui_events(event: &KeyEvent) -> Vec<egui::Event> {
    let mut events = Vec::new();

    // Try to get egui::Key from key_code first, then from character
    let key = vst3_keycode_to_egui_key(event.key_code)
        .or_else(|| event.character.and_then(char_to_egui_key));

    let modifiers = egui::Modifiers {
        alt: event.modifiers.alt,
        ctrl: event.modifiers.ctrl,
        shift: event.modifiers.shift,
        mac_cmd: cfg!(target_os = "macos") && event.modifiers.meta,
        command: if cfg!(target_os = "macos") {
            event.modifiers.meta
        } else {
            event.modifiers.ctrl
        },
    };

    // Key event (if we recognized the key)
    if let Some(key) = key {
        events.push(egui::Event::Key {
            key,
            physical_key: None,
            pressed: event.event_type == KeyEventType::KeyDown,
            repeat: false,
            modifiers,
        });
    }

    // Text event (only on KeyDown and if there's a character)
    if event.event_type == KeyEventType::KeyDown {
        if let Some(c) = event.character {
            // Don't send Text events for control characters or when Ctrl/Cmd is pressed
            if !c.is_control() && !modifiers.ctrl && !modifiers.command {
                events.push(egui::Event::Text(c.to_string()));
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use vst3_keys::*;

    #[test]
    fn test_vst3_keycode_to_egui_key() {
        // VST3 VirtualKeyCodes - NOT Windows VK_* codes!
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_BACK),
            Some(egui::Key::Backspace)
        );
        assert_eq!(vst3_keycode_to_egui_key(KEY_TAB), Some(egui::Key::Tab));
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_RETURN),
            Some(egui::Key::Enter)
        );
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_ESCAPE),
            Some(egui::Key::Escape)
        );
        assert_eq!(vst3_keycode_to_egui_key(KEY_SPACE), Some(egui::Key::Space));
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_DELETE),
            Some(egui::Key::Delete)
        );

        // Arrow keys
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_LEFT),
            Some(egui::Key::ArrowLeft)
        );
        assert_eq!(vst3_keycode_to_egui_key(KEY_UP), Some(egui::Key::ArrowUp));
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_RIGHT),
            Some(egui::Key::ArrowRight)
        );
        assert_eq!(
            vst3_keycode_to_egui_key(KEY_DOWN),
            Some(egui::Key::ArrowDown)
        );

        // Unknown returns None
        assert_eq!(vst3_keycode_to_egui_key(999), None);
    }

    #[test]
    fn test_char_to_egui_key() {
        assert_eq!(char_to_egui_key('a'), Some(egui::Key::A));
        assert_eq!(char_to_egui_key('A'), Some(egui::Key::A));
        assert_eq!(char_to_egui_key('0'), Some(egui::Key::Num0));
        assert_eq!(char_to_egui_key('9'), Some(egui::Key::Num9));
        assert_eq!(char_to_egui_key('.'), Some(egui::Key::Period));
        assert_eq!(char_to_egui_key('-'), Some(egui::Key::Minus));

        // Special characters return None
        assert_eq!(char_to_egui_key('@'), None);
        assert_eq!(char_to_egui_key('#'), None);
    }
}
