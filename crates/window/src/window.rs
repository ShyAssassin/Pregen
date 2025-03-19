use super::{Key, Action};
use std::collections::HashSet;
use super::{WindowBackend, WindowEvent};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

pub(crate) trait NativeWindow: HasWindowHandle + HasDisplayHandle {
    fn init() -> Self where Self: Sized;
    fn show(&mut self);
    fn focus(&mut self);
    fn shutdown(&mut self);
    fn is_focused(&self) -> bool;
    fn lock_cursor(&mut self, lock: bool);
    fn poll(&mut self) -> Vec<WindowEvent>;
    fn resize(&mut self, width: u32, height: u32);

    fn get_size(&self) -> (u32, u32);
    fn get_clipboard(&self) -> String;
    fn get_content_scale(&self) -> (f32, f32);
    fn get_cursor_position(&self) -> (u32, u32);

    fn set_title(&mut self, title: &str);
    fn set_clipboard(&mut self, text: &str);
    fn set_resizeable(&mut self, resizable: bool);
    fn set_cursor_visible(&mut self, visible: bool);
    fn set_cursor_position(&mut self, x: u32, y: u32);
}

pub struct Window {
    width: u32,
    height: u32,
    title: String,
    backend: WindowBackend,
    window: Box<dyn NativeWindow>,

    // Window state
    is_focused: bool,
    scale: (f32, f32),
    lock_cursor: bool,
    should_close: bool,
    cursor_visible: bool,
    mouse_delta: (f32, f32),
    mouse_position: (u32, u32),
    pressed_keys: HashSet<Key>,
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32, resizeable: bool, backend: WindowBackend) -> Self {
        log::info!("Creating window with backend: {:?}", backend);
        let mut window: Box<dyn NativeWindow> = match backend {
            #[cfg(not(target_family = "wasm"))]
            WindowBackend::Glfw => {
                use crate::backends::GlfwWindow;
                Box::new(GlfwWindow::init())
            }
            #[cfg(target_family = "windows")]
            WindowBackend::Win32 => {
                use crate::backends::Win32Window;
                Box::new(Win32Window::init())
            },
            #[cfg(target_family = "wasm")]
            WindowBackend::Web => {
                use crate::backends::WebWindow;
                Box::new(WebWindow::init())
            },
            #[cfg(target_os = "linux")]
            WindowBackend::X11 => {
                use crate::backends::X11Window;
                Box::new(X11Window::init())
            }
            _ => panic!("Unsupported window backend selected: {:?}", backend),
        };
        window.show();
        window.set_title(name);
        window.resize(width, height);
        window.set_resizeable(resizeable);

        let scale = window.get_content_scale();
        let (fwidth, fheight) = window.get_size();
        log::debug!("Content scale: {:?}", scale);
        log::debug!("Window size: {}x{}", width, height);
        log::debug!("Framebuffer size: {}x{}", fwidth, fheight);

        let focus = window.is_focused();
        return Self {
            title: name.to_string(),
            backend: backend,
            window: window,
            scale: scale,
            width: fwidth,
            height: fheight,
            is_focused: focus,
            lock_cursor: false,
            should_close: false,
            cursor_visible: true,
            mouse_position: (0, 0),
            mouse_delta: (0.0, 0.0),
            pressed_keys: HashSet::new(),
        };
    }

    pub fn poll(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        let mut seen_events = HashSet::new();
        // Remove all duplicate events but retain newest
        for event in self.window.poll().iter().rev() {
            if seen_events.insert(event.id()) {
                events.push(*event);
            }
        }

        for event in &events {
            match event {
                WindowEvent::FocusLost => {
                    self.is_focused = false;
                }
                WindowEvent::FocusGained => {
                    self.is_focused = true;
                }
                WindowEvent::CloseRequested => {
                    self.should_close = true;
                }
                WindowEvent::Resize { width, height } => {
                    self.width = *width;
                    self.height = *height;
                }
                WindowEvent::KeyboardInput(key, _, action) => {
                    match action {
                        Action::Pressed => {
                            self.pressed_keys.insert(*key);
                        }
                        Action::Released => {
                            self.pressed_keys.remove(key);
                        }
                    }
                }
                WindowEvent::CursorPosition { mouse_x, mouse_y } => {
                    self.mouse_delta = (
                        *mouse_x as f32 - self.mouse_position.0 as f32,
                        *mouse_y as f32 - self.mouse_position.1 as f32,
                    );
                    self.mouse_position = (*mouse_x, *mouse_y);
                }
                WindowEvent::ScaleFactorChanged { scale_x, scale_y } => {
                    self.scale = (*scale_x, *scale_y);
                }
                _ => {}
            }
        }

        return events;
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        let x = self.width as f32 * self.scale.0;
        let y = self.height as f32 * self.scale.1;
        return (x as i32, y as i32);
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        return self.pressed_keys.contains(&key);
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        return self.mouse_delta;
    }

    pub fn get_pressed_keys(&self) -> &HashSet<Key> {
        return &self.pressed_keys;
    }

    #[cfg(target_family = "wasm")]
    pub fn canvas(&self) -> web_sys::HtmlCanvasElement {
        use std::any::Any;
        use crate::backends::WebWindow;

        // some sketchy shit right here
        let window: &dyn Any = &self.window;
        assert_eq!(self.backend, WindowBackend::Web);
        return window.downcast_ref::<WebWindow>().unwrap().canvas.clone();
    }
}

impl Window {
    pub fn backend(&self) -> WindowBackend {
        return self.backend;
    }

    pub fn should_close(&self) -> bool {
        return self.should_close;
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.should_close = should_close;
    }

    pub fn close(mut self) {
        self.window.shutdown();
        self.set_should_close(true);
    }

    pub fn focus(&mut self) {
        self.window.focus();
        self.is_focused = true;
    }

    pub fn get_focus(&self) -> bool {
        return self.is_focused;
    }

    pub fn lock_cursor(&mut self, lock: bool) {
        self.lock_cursor = lock;
        self.window.lock_cursor(lock);
    }

    pub fn set_cursor_position(&mut self, x: u32, y: u32) {
        if self.is_focused {
            if self.lock_cursor {
                if self.get_cursor_position() != (x, y) {
                    self.mouse_position = (x, y);
                    self.mouse_delta = (0.0, 0.0);
                    self.window.set_cursor_position(x, y);
                }
                return;
            }
            log::warn!("Attempted to set cursor position while cursor is not locked");
        }
    }

    pub fn get_cursor_position(&self) -> (u32, u32) {
        return self.window.get_cursor_position();
    }

    pub fn set_clipboard(&mut self, text: &str) {
        self.window.set_clipboard(text);
    }

    pub fn get_clipboard(&self) -> String {
        return self.window.get_clipboard();
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
        self.window.set_cursor_visible(visible);
    }

    pub fn get_cursor_visible(&self) -> bool {
        return self.cursor_visible;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        return self.width as f32 / self.height as f32;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.into();
        self.window.set_title(title);
    }

    pub fn get_title(&self) -> &str {
        return &self.title;
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.shutdown();
        self.should_close = true;
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        return self.window.window_handle();
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        return self.window.display_handle();
    }
}
