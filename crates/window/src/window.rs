use std::collections::HashSet;
use super::{Key, Action, Cursor};
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

    // TODO: Implement this!
    /// Sets the cursor icon to the specified type.
    fn set_cursor(&mut self, cursor: Cursor) {
        let _ = cursor;
    }
}


pub struct Window {
    backend: WindowBackend,
    window: Box<dyn NativeWindow>,

    focus: bool,
    title: String,
    size: (u32, u32),
    scale: (f32, f32),
    fbsize: (u32, u32),
    capture_cursor: bool,
    close_requested: bool,
    mouse_delta: (f64, f64),
    active_keys: HashSet<Key>,
    mouse_position: (f64, f64),
}

impl Window {
    pub fn new(title: &str, size: (u32, u32), backend: WindowBackend) -> Self {
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

        window.resize(size.0, size.1);
        return Self::from_native(title, backend, window);
    }

    pub fn from_native(title: &str, backend: WindowBackend, mut window: Box<dyn NativeWindow>) -> Self {
        window.set_title(title);
        let size = window.get_size();
        let focus = window.is_focused();
        let dpi = window.get_content_scale();
        let fbwidth = (size.0 as f32 * dpi.0) as u32;
        let fbheight = (size.1 as f32 * dpi.1) as u32;

        log::debug!("Window content scale: {:?}", dpi);
        log::debug!("Window size: {}x{}", size.0, size.1);
        log::debug!("Framebuffer size: {}x{}", fbwidth, fbheight);

        return Self {
            title: title.to_string(),
            size: size,
            scale: dpi,
            focus: focus,
            window: window,
            backend: backend,
            capture_cursor: false,
            close_requested: false,
            mouse_delta: (0.0, 0.0),
            mouse_position: (0.0, 0.0),
            fbsize: (fbwidth, fbheight),
            active_keys: HashSet::new(),
        };
    }

    pub fn poll(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        self.mouse_delta = (0.0, 0.0);

        // Except for some special cases
        // Only keep the most recent event
        let raw_events = self.window.poll();
        let mut seen_ids: HashSet<u64> = HashSet::new();
        let deduped: Vec<&WindowEvent> = raw_events.iter().rev().filter(|event| {
            let exempt = matches!(event, WindowEvent::MouseButton(_, _))
                      || matches!(event, WindowEvent::CursorPosition{ .. })
                      || matches!(event, WindowEvent::KeyboardInput(_, _, _));
            exempt || seen_ids.insert(event.id())
        }).collect::<Vec<_>>().into_iter().rev().collect();

        for event in deduped {
            match event {
                WindowEvent::CloseRequested => {
                    self.close_requested = true;
                }
                WindowEvent::FocusGained => {
                    self.focus = true;
                }
                WindowEvent::FocusLost => {
                    self.focus = false;
                    self.active_keys.clear();
                    self.capture_cursor(false);
                    self.mouse_delta = (0.0, 0.0);
                }
                WindowEvent::KeyboardInput(key, _, action) => {
                    match action {
                        Action::Pressed => {
                            self.active_keys.insert(*key);
                        }
                        Action::Released => {
                            self.active_keys.remove(key);
                        }
                    }
                }
                WindowEvent::Resize { width, height } => {
                    self.size = (*width, *height);
                    self.fbsize = (
                        (*width as f32 * self.scale.0) as u32,
                        (*height as f32 * self.scale.1) as u32
                    );

                    // Backends are only required to emit logical resize events.
                    // We synthesize the framebuffer resize event here for consumers.
                    // TODO: Backends really should emit this instead of us faking it here
                    events.push(WindowEvent::FramebufferResize {
                        width: self.fbsize.0,
                        height: self.fbsize.1,
                    });
                }
                WindowEvent::ScaleFactorChanged { scale_x, scale_y } => {
                    self.scale = (*scale_x, *scale_y);
                    self.fbsize = (
                        (self.size.0 as f32 * self.scale.0) as u32,
                        (self.size.1 as f32 * self.scale.1) as u32
                    );

                    // TODO: also force backends to emit this
                    // Similarly when the scale factor changes, we recalculate
                    // the framebuffer size and emit a corresponding resize event.
                    events.push(WindowEvent::FramebufferResize {
                        width: self.fbsize.0,
                        height: self.fbsize.1,
                    });
                }
                WindowEvent::CursorPosition { mouse_x, mouse_y } => {
                    let new_position = (*mouse_x, *mouse_y);
                    if new_position != self.mouse_position {
                        let dx = *mouse_x - self.mouse_position.0;
                        let dy = *mouse_y - self.mouse_position.1;

                        // Ignore delta if it is the exact inverse of the accumulated delta
                        // This filters out "echoed" events from programmatic cursor movement
                        if (dx, dy) != (-self.mouse_delta.0 as f64, -self.mouse_delta.1 as f64) {
                            self.mouse_delta.0 += dx as f64;
                            self.mouse_delta.1 += dy as f64;
                        } else {
                            log::trace!("Ignoring cursor position event with inverse delta: ({}, {})", dx, dy);
                        }

                        self.mouse_position = new_position;
                    }
                }
                _ => {}
            }

            events.push(*event);
            log::trace!("{:?}", event);
        }

        return events;
    }
}

impl Window {
    #[cfg(target_family = "wasm")]
    // TODO: Return a reference instead of cloning?
    pub fn canvas(&self) -> web_sys::HtmlCanvasElement {
        use std::any::Any;
        use crate::backends::WebWindow;

        // some sketchy shit right here
        let window: &dyn Any = &self.window;
        assert_eq!(self.backend, WindowBackend::Web);
        return window.downcast_ref::<WebWindow>().unwrap().canvas.clone();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            if width != self.size.0 || height != self.size.1 {
                log::debug!("Requesting window resize to: {}x{}", width, height);
                // Why yes obsevant reader, we are blindly *trusting* the backend here
                // Depending on the backend the wm / compositor may not respect our request
                // but there is no way for us to verify this without querying the wm / compositor
                // so we need to wait for a new `Resize` event in `Poll` in order to update our state
                self.window.resize(width, height);
            } else {
                log::trace!("Window resize ignored, is already: {}x{}", width, height);
            }
        } else {
            log::warn!("Window attempted to resize to invalid geometry: {}x{}", width, height);
        }
    }

    pub fn move_cursor(&mut self, x: u32, y: u32) {
        if self.focus {
            if self.capture_cursor {
                if (x as f64, y as f64) != self.mouse_position {
                    // self.mouse_delta = (0.0, 0.0);
                    self.window.set_cursor_position(x, y);
                    self.mouse_position = (x as f64, y as f64);
                }
            } else {
                log::warn!("Attempted to move cursor while cursor is not captured");
            }
        }
    }

    // TODO: rename to confine during refactor
    pub fn capture_cursor(&mut self, capture: bool) {
        if self.focus || !capture {
            self.capture_cursor = capture;
            self.window.lock_cursor(capture);
        } else {
            log::warn!("Attempted to capture cursor without window focus");
        }
    }

    pub fn rename(&mut self, title: String) {
        if self.title != title {
            self.title = title.clone();
            self.window.set_title(&title);
        }
    }

    pub fn focus(&mut self) {
        // We are yet again fully trusting the backend here (—ᴗ—)
        // Since `focus()` is a request to the window manager / compositor
        // we are not able to guarantee that we will receive immediate input focus
        // so we will need to wait for a `FocusGained` event in `poll()` to update our state
        if !self.focus { self.window.focus() }
    }

    pub fn close(mut self) {
        self.window.shutdown();
        self.active_keys.clear();
        self.close_requested = true;
    }
}

impl Window {
    pub fn dpi(&self) -> f32 {
        return self.scale.0;
    }

    pub fn title(&self) -> &str {
        return &self.title;
    }

    pub fn focused(&self) -> bool {
        return self.focus;
    }

    pub fn should_close(&self) -> bool {
        return self.close_requested;
    }

    pub fn cursor_captured(&self) -> bool {
        return self.capture_cursor;
    }

    pub fn backend(&self) -> WindowBackend {
        return self.backend;
    }

    pub fn aspect_ratio(&self) -> f32 {
        return self.size.0 as f32 / self.size.1 as f32;
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        return self.mouse_delta;
    }

    // TODO: maybe rename this function?
    pub fn key_pressed(&self, key: Key) -> bool {
        return self.active_keys.contains(&key);
    }
}

impl Window {
    pub fn width(&self) -> u32 {
        return self.size.0;
    }

    pub fn height(&self) -> u32 {
        return self.size.1;
    }

    pub fn size(&self) -> (u32, u32) {
        return (self.size.0, self.size.1);
    }

    pub fn framebuffer_width(&self) -> u32 {
        return self.fbsize.0;
    }

    pub fn framebuffer_height(&self) -> u32 {
        return self.fbsize.1;
    }

    pub fn framebuffer_size(&self) -> (u32, u32) {
        return (self.fbsize.0, self.fbsize.1);
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
