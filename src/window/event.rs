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
    Other(u32),
    // Arrow keys
    Left, Right, Up, Down,
    // Modifier keys
    LShift, RShift, LCtrl, RCtrl,
    // ASCII keys
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    // "Action" keys
    Space, Enter, Escape, Tab, Backspace, Insert, Delete, Home, End, PageUp, PageDown,
    // "Operation" keys
    Minus, Equals, LeftBracket, RightBracket, Backslash, Semicolon, Apostrophe, Grave, Comma, Period, Slash,
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
    /// The window has been created.
    Created,

    /// The window has gained focus.
    FocusGained,

    // The window has lost focus.
    FocusLost,

    /// The window has been forcfully closed
    Destroyed,

    /// The window has been asked closed.
    CloseRequested,

    // The window has been minimized.
    Minimized,

    // The window has maximized.
    Maximized,

    /// General event for all types of keyboard input.
    KeyboardInput,

    /// The user has pressed a key while within the window.
    KeyInput(Key, u32, Action),

    /// General event for all types of mouse input.
    MouseInput,

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
    pub fn id(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::mem::discriminant(self).hash(&mut hasher);
        return hasher.finish()
    }
}
