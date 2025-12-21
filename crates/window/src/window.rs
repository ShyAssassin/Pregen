use super::{Key, Action};
use std::collections::HashSet;
use super::{WindowBackend, WindowEvent};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

pub trait NativeWindow: HasWindowHandle + HasDisplayHandle {
    /// Creates and initializes a new hidden window.
    ///
    /// Performs any necessary one-time setup for the windowing system and creates
    /// a new native window. The window should remain invisible until `show()` is called.
    fn init() -> Self where Self: Sized;

    /// Bring the window to the foreground and make it visible if hidden.
    ///
    /// If the window is already visible, this method should have no effect.
    /// When possible this should also request focus from the window manager.
    /// This is a request to the window manager which may choose to ignore it.
    fn show(&mut self);

    /// Force input focus and bring the window to the foreground.
    ///
    /// This is a request to the window manager, which may choose to ignore it.
    /// Calling this function implicitly invokes `show()` if the window is hidden.
    /// If forcing focus is not available implementations should request focus instead.
    fn focus(&mut self);

    /// Destroys the window and releases all associated resources.
    ///
    /// After calling this method, the window should no longer be used.
    /// Implementations must ensure all resources are properly cleaned up.
    fn shutdown(&mut self);

    /// Returns whether the window currently has input focus.
    ///
    /// Implementations must track focus changes and return the correct state.
    /// Focus changes must be reported via `FocusGained` and `FocusLost` events.
    fn is_focused(&self) -> bool;

    /// Controls whether the cursor is confined to the window and hidden.
    ///
    /// When enabled, hides the cursor and prevents it from leaving the window client area.
    /// The lock may be automatically released by the window manager when the window loses focus.
    // TODO: rename, this function name is really not good, should also add cursor icon control sometime
    fn lock_cursor(&mut self, lock: bool);

    /// Poll and return pending window events without blocking.
    ///
    /// Processes all available events and returns them. Returns an empty vector
    /// if no events are pending. Most coordinates and sizes in returned events
    /// use logical pixels except in cases where physical pixels are expected
    fn poll(&mut self) -> Vec<WindowEvent>;

    /// Sets the window's client area size in logical pixels.
    ///
    /// The implementation handles conversion to physical pixels based on the
    /// current DPI scale factor. The drawable region of the window (client area)
    /// must match the requested size after accounting for scaling and decorations.
    fn resize(&mut self, width: u32, height: u32);

    /// Returns the current client area size in logical pixels.
    ///
    /// Size changes must be reported via `WindowEvent::Resize` events.
    /// The size excludes any window decorations like borders or title bars.
    /// Implementations must track size changes and return the correct dimensions.
    fn get_size(&self) -> (u32, u32);

    /// Retrieves text content from the system clipboard.
    ///
    /// If the clipboard is empty or unavailable, returns an empty string.
    fn get_clipboard(&self) -> String;

    /// Returns the DPI scaling factor for the display containing the window.
    ///
    /// The scale factor may change if the window moves to a different display,
    /// Implementations must emit `WindowEvent::ScaleFactorChanged` when this occurs.
    fn get_content_scale(&self) -> (f32, f32);

    /// Returns the cursor position in logical pixels.
    ///
    /// Coordinates are relative to the top left corner of the client area.
    /// General cursor movement must be reported via `WindowEvent::CursorPosition` events.
    /// Implementations can safely retrun (0, 0) if no cursor is present or outside client area.
    fn get_cursor_position(&self) -> (u32, u32);

    /// Sets the window's title bar text.
    ///
    /// If no title bar is present, this method should have no effect.
    fn set_title(&mut self, title: &str);

    /// Sets the system clipboard content to the provided text.
    ///
    /// Implementations must handle any necessary encoding conversions.
    /// If the clipboard is unavailable, this method should have no effect.
    fn set_clipboard(&mut self, text: &str);

    /// Sets whether the user can resize the window.
    ///
    /// This must only affect user-initiated resizing, programmatic resizing
    /// via `resize()` should still function normally regardless of this option.
    /// This is a request to the window manager, which may choose to ignore it.
    fn set_resizeable(&mut self, resizable: bool);

    /// Controls cursor visibility within the window's client area.
    ///
    /// This may be reversed automatically by the window manager when the window
    /// loses focus, implementations are not expected to track focus changes for this.
    fn set_cursor_visible(&mut self, visible: bool);

    /// Moves the cursor to the provided position in logical pixels.
    ///
    /// The implementation handles conversion to physical coordinates.
    /// Coordinates are relative to the top-left corner of the client area.
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
    mouse_position: (f64, f64),
    pressed_keys: HashSet<Key>,
    cursor_move_pos: (f32, f32),
}

#[profiling::all_functions]
impl Window {
    pub fn new(title: &str, width: u32, height: u32, resizeable: bool, backend: WindowBackend) -> Self {
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
            #[cfg(target_os = "linux")]
            WindowBackend::Wayland => {
                use crate::backends::WaylandWindow;
                Box::new(WaylandWindow::init())
            }
            _ => panic!("Unsupported window backend selected: {:?}", backend),
        };

        window.show();
        window.set_title(title);
        window.resize(width, height);
        window.set_resizeable(resizeable);
        return Self::from_native(title, window, backend);
    }

    pub fn from_native(title: &str, window: Box<dyn NativeWindow>, backend: WindowBackend) -> Self {
        let size = window.get_size();
        let focus = window.is_focused();
        let scale = window.get_content_scale();
        let fbwidth = (size.0 as f32 * scale.0) as u32;
        let fbheight = (size.1 as f32 * scale.1) as u32;

        log::debug!("Content scale: {:?}", scale);
        log::debug!("Window size: {}x{}", size.0, size.1);
        log::debug!("Framebuffer size: {}x{}", fbwidth, fbheight);

        return Self {
            title: title.to_string(),
            backend: backend,
            window: window,
            scale: scale,
            width: fbwidth,
            height: fbheight,
            is_focused: focus,
            lock_cursor: false,
            should_close: false,
            cursor_visible: true,
            mouse_delta: (0.0, 0.0),
            mouse_position: (0.0, 0.0),
            cursor_move_pos: (0.0, 0.0),
            pressed_keys: HashSet::new(),
        };
    }

    pub fn poll(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        // reset delta from last poll
        self.mouse_delta = (0.0, 0.0);
        let mut seen_events = HashSet::new();

        for event in self.window.poll().iter().rev() {
            // FIXME: inputs with differing key enums are not being considered unique
            // May no longer be needed after the refactor to use WindowEvent::id()
            let is_kb = matches!(event, WindowEvent::KeyboardInput(_, _, _));
            if seen_events.insert(event.id()) || is_kb {
                match event {
                    WindowEvent::FocusLost => {
                        self.is_focused = false;
                        self.pressed_keys.clear();
                        self.mouse_delta = (0.0, 0.0);
                    }
                    WindowEvent::FocusGained => {
                        self.is_focused = true;
                        self.pressed_keys.clear();
                        self.mouse_delta = (0.0, 0.0);
                    }
                    WindowEvent::CloseRequested => {
                        self.should_close = true;
                    }
                    WindowEvent::Resize { width, height } => {
                        self.width = *width;
                        self.height = *height;
                        events.push(WindowEvent::FramebufferResize {
                            width: (self.width as f32 * self.scale.0) as u32,
                            height: (self.height as f32 * self.scale.1) as u32,
                        });
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
                        if (*mouse_x, *mouse_y) != self.mouse_position {
                            // TODO: maybe check for inversed mouse delta here instead?
                            // Account for movement which occurs from set_cursor_position
                            if self.cursor_move_pos != (*mouse_x as f32, *mouse_y as f32) {
                                let dx = *mouse_x as f32 - self.mouse_position.0 as f32;
                                let dy = *mouse_y as f32 - self.mouse_position.1 as f32;
                                self.mouse_delta = (self.mouse_delta.0 + dx, self.mouse_delta.1 + dy);
                            }
                            self.mouse_position = (*mouse_x, *mouse_y);
                        }
                    }
                    WindowEvent::ScaleFactorChanged { scale_x, scale_y } => {
                        self.scale = (*scale_x, *scale_y);
                    }
                    _ => {}
                }

                events.push(*event);
                log::trace!("Window Event: {:?}", event);
            }
        }

        return events;
    }

    pub fn get_size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    pub fn get_framebuffer_size(&self) -> (u32, u32) {
        let x = self.width as f32 * self.scale.0;
        let y = self.height as f32 * self.scale.1;
        return (x as u32, y as u32);
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

    pub fn native_window(&self) -> &Box<dyn NativeWindow> {
        return &self.window
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
                    self.mouse_delta = (0.0, 0.0);
                    self.window.set_cursor_position(x, y);
                    self.mouse_position = (x as f64, y as f64);
                    self.cursor_move_pos = (x as f32, y as f32);
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
