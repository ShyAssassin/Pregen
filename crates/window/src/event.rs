use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Pressed,
    Released,
}

impl From<bool> for Action {
    fn from(value: bool) -> Self {
        if value {
            Action::Pressed
        } else {
            Action::Released
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    // Arrow keys
    Left, Right, Up, Down,
    // ASCII keys
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Modifier keys
    LShift, RShift, LCtrl, RCtrl, LAlt, RAlt,
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    // "Action" keys
    Space, Enter, Escape, Tab, Backspace, Insert, Delete, Home, End, PageUp, PageDown,
    // "Operation" keys
    Minus, Equals, LeftBracket, RightBracket, Backslash, Semicolon, Apostrophe, Grave, Comma, Period, Slash,
    // Any key that doesnt have a specific enum yet
    Other(u32),
}

impl Key {
    pub fn from_char(letter: char) -> Self {
        match letter {
            'A' => Key::A, 'B' => Key::B, 'C' => Key::C, 'D' => Key::D, 'E' => Key::E, 'F' => Key::F, 'G' => Key::G,
            'H' => Key::H, 'I' => Key::I, 'J' => Key::J, 'K' => Key::K, 'L' => Key::L, 'M' => Key::M, 'N' => Key::N,
            'O' => Key::O, 'P' => Key::P, 'Q' => Key::Q, 'R' => Key::R, 'S' => Key::S, 'T' => Key::T, 'U' => Key::U,
            'V' => Key::V, 'W' => Key::W, 'X' => Key::X, 'Y' => Key::Y, 'Z' => Key::Z,
            _ => panic!("Unknown key: {}", letter),
        }
    }

    pub fn from_digit(digit: char) -> Self {
        match digit {
            '0' => Key::Num0, '1' => Key::Num1, '2' => Key::Num2, '3' => Key::Num3, '4' => Key::Num4,
            '5' => Key::Num5, '6' => Key::Num6, '7' => Key::Num7, '8' => Key::Num8, '9' => Key::Num9,
            _ => panic!("Unknown digit: {}", digit),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum WindowEvent {
    /// The window has been asked closed.
    CloseRequested,
    /// The window has been forcfully closed
    Destroyed,

    // The window has lost focus.
    FocusLost,
    /// The window has gained focus.
    FocusGained,

    // The window has been minimized.
    Minimized,
    // The window has maximized.
    Maximized,

    /// The user has pressed a key while within the window.
    KeyboardInput(Key, u32, Action),

    /// A Mouse button has been pressed.
    MouseButton(MouseButton, Action),

    /// The Mouse has moved within the window.
    CursorPosition {
        mouse_x: u32,
        mouse_y: u32,
    },

    /// The window's scale factor has changed.
    ScaleFactorChanged {
        scale_x: f32,
        scale_y: f32,
    },

    /// The window has been resized.
    Resize {
        width: u32,
        height: u32,
    },
}

impl WindowEvent {
    // FIXME: in a perfect world this would be static
    pub fn id(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::mem::discriminant(self).hash(&mut hasher);
        return hasher.finish()
    }
}
