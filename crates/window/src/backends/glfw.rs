use crate::window::NativeWindow;
use crate::{Key, Action, MouseButton, WindowEvent};
use glfw::{Glfw, GlfwReceiver, PWindow, WindowEvent as GlfwWindowEvent};
use raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle, HandleError};

#[derive(Debug)]
pub struct GlfwWindow {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, GlfwWindowEvent)>
}

impl NativeWindow for GlfwWindow {
    fn init() -> Self {
        let mut glfw = glfw::init(|err: glfw::Error, desc: String| {
            log::error!("{:?}: {}", err, desc);
        }).expect("Failed to initialize GLFW");

        glfw.set_swap_interval(glfw::SwapInterval::None);
        glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        let (mut window, events) = glfw.create_window(100, 100, "Pregen", glfw::WindowMode::Windowed).unwrap();
        window.set_all_polling(true);

        return Self {
            glfw: glfw,
            window: window,
            events: events,
        };
    }

    fn show(&mut self) {
        self.window.show();
    }

    fn focus(&mut self) {
        self.window.focus();
    }

    fn shutdown(&mut self) {
        self.window.set_should_close(true);
    }

    fn is_focused(&self) -> bool {
        return self.window.is_focused();
    }

    fn lock_cursor(&mut self, lock: bool) {
        self.window.set_cursor_mode(if lock { glfw::CursorMode::Disabled } else { glfw::CursorMode::Normal });
    }

    fn poll(&mut self) -> Vec<WindowEvent> {
        self.glfw.poll_events();
        let mut events = Vec::new();
        let glfw_events: Vec<_> = glfw::flush_messages(&self.events).collect();
        for (_, event) in glfw_events {
            match event {
                GlfwWindowEvent::Close => {
                    events.push(WindowEvent::CloseRequested);
                }
                GlfwWindowEvent::Size(width, height) => {
                    // HACK: On Systems with async window resizing the resize event is fired over multiple frames
                    // To avoid this we poll the window size until it is no longer changing, still spams but not as much
                    events.append(&mut self.poll());
                    events.push(WindowEvent::Resize { width: width as u32, height: height as u32 });
                }
                GlfwWindowEvent::ContentScale(scale_x, scale_y) => {
                    events.push(WindowEvent::ScaleFactorChanged { scale_x, scale_y });
                }
                GlfwWindowEvent::Focus(false) => {
                    events.push(WindowEvent::FocusLost);
                }
                GlfwWindowEvent::Focus(true) => {
                    events.push(WindowEvent::FocusGained);
                }
                GlfwWindowEvent::Maximize(true) => {
                    events.push(WindowEvent::Maximized);
                }
                GlfwWindowEvent::Maximize(false) => {
                    events.push(WindowEvent::Minimized);
                }
                GlfwWindowEvent::CursorPos(x, y) => {
                    events.push(WindowEvent::CursorPosition { mouse_x: x as u32, mouse_y: y as u32 });
                }
                GlfwWindowEvent::MouseButton(button, action, _) => {
                    events.push(WindowEvent::MouseButton(button.into(), action.into()));
                }
                GlfwWindowEvent::Key(key, code, action, _) => {
                    events.push(WindowEvent::KeyboardInput(key.into(), code as u32, action.into()));
                }
                _ => {}
            }
        }

        return events;
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.window.set_size(width as i32, height as i32);
        // Under XWayland the window size is not updated until the next frame
        self.glfw.poll_events();
    }

    fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_size();
        return (width as u32, height as u32);
    }

    fn get_clipboard(&self) -> String {
        return self.window.get_clipboard_string().unwrap_or_default()
    }

    fn get_content_scale(&self) -> (f32, f32) {
        return self.window.get_content_scale();
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    fn set_clipboard(&mut self, text: &str) {
        self.window.set_clipboard_string(text);
    }

    fn set_resizeable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    fn set_cursor_visible(&mut self, visible: bool) {
        let mode = if visible { glfw::CursorMode::Normal } else { glfw::CursorMode::Hidden };
        self.window.set_cursor_mode(mode);
    }

    fn set_cursor_position(&mut self, x: u32, y: u32) {
        self.window.set_cursor_pos(x as f64, y as f64);
    }

    fn get_cursor_position(&self) -> (u32, u32) {
        let (x, y) = self.window.get_cursor_pos();
        return (x as u32, y as u32);
    }
}

impl HasDisplayHandle for GlfwWindow {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        return self.window.display_handle();
    }
}

impl HasWindowHandle for GlfwWindow {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        return self.window.window_handle();
    }
}

impl From<glfw::MouseButton> for MouseButton {
    fn from(action: glfw::MouseButton) -> MouseButton {
        return match action {
            glfw::MouseButton::Button1 => MouseButton::Left,
            glfw::MouseButton::Button2 => MouseButton::Right,
            glfw::MouseButton::Button3 => MouseButton::Middle,
            _ => MouseButton::Other(action as u32),
        }
    }
}

impl From<glfw::Action> for Action {
    fn from(action: glfw::Action) -> Action {
        return match action {
            glfw::Action::Press => Action::Pressed,
            glfw::Action::Repeat => Action::Pressed,
            glfw::Action::Release => Action::Released,
        }
    }
}

impl From<glfw::Key> for Key {
    fn from(key: glfw::Key) -> Key {
        return match key {
            // A-Z
            glfw::Key::A => Key::A, glfw::Key::B => Key::B, glfw::Key::C => Key::C, glfw::Key::D => Key::D,
            glfw::Key::E => Key::E, glfw::Key::F => Key::F, glfw::Key::G => Key::G, glfw::Key::H => Key::H,
            glfw::Key::I => Key::I, glfw::Key::J => Key::J, glfw::Key::K => Key::K, glfw::Key::L => Key::L,
            glfw::Key::M => Key::M, glfw::Key::N => Key::N, glfw::Key::O => Key::O, glfw::Key::P => Key::P,
            glfw::Key::Q => Key::Q, glfw::Key::R => Key::R, glfw::Key::S => Key::S, glfw::Key::T => Key::T,
            glfw::Key::U => Key::U, glfw::Key::V => Key::V, glfw::Key::W => Key::W, glfw::Key::X => Key::X,
            glfw::Key::Y => Key::Y, glfw::Key::Z => Key::Z,

            // 0-9
            glfw::Key::Num0 => Key::Num0, glfw::Key::Num1 => Key::Num1, glfw::Key::Num2 => Key::Num2, glfw::Key::Num3 => Key::Num3,
            glfw::Key::Num4 => Key::Num4, glfw::Key::Num5 => Key::Num5, glfw::Key::Num6 => Key::Num6, glfw::Key::Num7 => Key::Num7,
            glfw::Key::Num8 => Key::Num8, glfw::Key::Num9 => Key::Num9,

            // Function keys
            glfw::Key::F1 => Key::F1, glfw::Key::F2 => Key::F2, glfw::Key::F3 => Key::F3, glfw::Key::F4 => Key::F4,
            glfw::Key::F5 => Key::F5, glfw::Key::F6 => Key::F6, glfw::Key::F7 => Key::F7, glfw::Key::F8 => Key::F8,
            glfw::Key::F9 => Key::F9, glfw::Key::F10 => Key::F10, glfw::Key::F11 => Key::F11, glfw::Key::F12 => Key::F12,
            glfw::Key::F13 => Key::F13, glfw::Key::F14 => Key::F14, glfw::Key::F15 => Key::F15, glfw::Key::F16 => Key::F16,
            glfw::Key::F17 => Key::F17, glfw::Key::F18 => Key::F18, glfw::Key::F19 => Key::F19, glfw::Key::F20 => Key::F20,
            glfw::Key::F21 => Key::F21, glfw::Key::F22 => Key::F22, glfw::Key::F23 => Key::F23, glfw::Key::F24 => Key::F24,

            // Modifier keys
            glfw::Key::LeftAlt => Key::LAlt, glfw::Key::RightAlt => Key::RAlt,
            glfw::Key::LeftShift => Key::LShift, glfw::Key::RightShift => Key::RShift,
            glfw::Key::LeftControl => Key::LCtrl, glfw::Key::RightControl => Key::RCtrl,

            // Arrow keys
            glfw::Key::Left => Key::Left, glfw::Key::Right => Key::Right, glfw::Key::Up => Key::Up, glfw::Key::Down => Key::Down,

            glfw::Key::Delete => Key::Delete, glfw::Key::Home => Key::Home, glfw::Key::End => Key::End,
            glfw::Key::Space => Key::Space, glfw::Key::Enter => Key::Enter, glfw::Key::Escape => Key::Escape,
            glfw::Key::Tab => Key::Tab, glfw::Key::Backspace => Key::Backspace, glfw::Key::Insert => Key::Insert,
            glfw::Key::PageUp => Key::PageUp, glfw::Key::PageDown => Key::PageDown, glfw::Key::Minus => Key::Minus,
            glfw::Key::Equal => Key::Equals, glfw::Key::LeftBracket => Key::LeftBracket, glfw::Key::RightBracket => Key::RightBracket,
            _ => {
                log::warn!("Unknown key code: {:?}", key);
                Key::Other(key as u32)
            },
        }
    }
}
