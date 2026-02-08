use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Hash)]
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

#[derive(PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Hash)]
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
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
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
            _ => unreachable!("Unknown digit: {}", digit),
        }
    }
}

#[derive(PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    /// The window has been asked closed.
    CloseRequested,
    /// The window has been forcefully closed
    Destroyed,

    /// The window has lost focus.
    FocusLost,
    /// The window has gained focus.
    FocusGained,

    /// The window has been minimized.
    Minimized,
    /// The window has maximized.
    Maximized,

    /// The user has pressed a key while within the window.
    KeyboardInput(Key, u32, Action),

    /// A Mouse button has been pressed.
    MouseButton(MouseButton, Action),

    /// The user has scrolled the mouse wheel.
    MouseWheel {
        scroll_x: f32,
        scroll_y: f32,
    },

    /// The Mouse has moved within the window.
    CursorPosition {
        mouse_x: f64,
        mouse_y: f64,
    },

    /// The window's scale factor has changed.
    ScaleFactorChanged {
        scale_x: f32,
        scale_y: f32,
    },

    /// The window has been resized.
    /// Does not account for scale factor.
    /// If you want to know the size of the framebuffer, use `FramebufferResize`.
    Resize {
        width: u32,
        height: u32,
    },

    /// The window has been resized.
    /// Accounts for scale factor.
    /// If you want to know the size of the window, use `Resize`.
    FramebufferResize {
        width: u32,
        height: u32,
    },
}

impl WindowEvent {
    // FIXME: in a perfect world this would be static
    /// Returns a unique identifier for events and their variants.
    pub fn id(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        match self {
            WindowEvent::MouseButton(button, action) => {
                std::mem::discriminant(self).hash(&mut hasher);
                button.hash(&mut hasher);
                action.hash(&mut hasher);
            }
            WindowEvent::KeyboardInput(key, keycode, action) => {
                std::mem::discriminant(self).hash(&mut hasher);
                key.hash(&mut hasher);
                action.hash(&mut hasher);
                keycode.hash(&mut hasher);
            }
            WindowEvent::CursorPosition { mouse_x, mouse_y } => {
                std::mem::discriminant(self).hash(&mut hasher);
                mouse_x.to_bits().hash(&mut hasher);
                mouse_y.to_bits().hash(&mut hasher);
            }
            _ => {
                std::mem::discriminant(self).hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}
