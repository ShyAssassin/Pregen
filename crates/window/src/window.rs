use std::collections::HashSet;
use super::{WindowBackend, WindowEvent};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

pub trait NativeWindow: HasWindowHandle + HasDisplayHandle + /* Send + Sync */ {
    fn init() -> Self where Self: Sized;
    fn show(&mut self);
    fn focus(&mut self);
    fn shutdown(&mut self);
    fn is_focused(&self) -> bool;
    fn poll(&mut self) -> Vec<WindowEvent>;
    fn resize(&mut self, width: u32, height: u32);

    fn get_size(&self) -> (u32, u32);
    fn get_clipboard(&self) -> String;
    fn get_content_scale(&self) -> (f32, f32);
    fn get_cursor_position(&self) -> (f32, f32);

    fn set_title(&mut self, title: &str);
    fn set_clipboard(&mut self, text: &str);
    fn set_resizeable(&mut self, resizable: bool);
    fn set_cursor_visible(&mut self, visible: bool);
    fn set_cursor_position(&mut self, x: f32, y: f32);
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
    should_close: bool,
    cursor_visible: bool,
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32, resizeable: bool, backend: WindowBackend) -> Self {
        println!("Creating window with backend: {:?}", backend);
        let mut window: Box<dyn NativeWindow> = match backend {
            WindowBackend::Glfw => {
                use crate::backends::GlfwWindow;
                Box::new(GlfwWindow::init())
            }
            #[cfg(target_family = "windows")]
            WindowBackend::Win32 => {
                use crate::backends::Win32Window;
                Box::new(Win32Window::init())
            },
            #[cfg(all(target_family = "unix", not(target_os = "macos")))]
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
        assert_eq!((fwidth, fheight), (width, height));

        let focus = window.is_focused();
        return Self {
            title: name.to_string(),
            backend: backend,
            window: window,
            scale: scale,
            width: fwidth,
            height: fheight,
            is_focused: focus,
            should_close: false,
            cursor_visible: true,
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

    pub fn is_focused(&self) -> bool {
        return self.is_focused;
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
