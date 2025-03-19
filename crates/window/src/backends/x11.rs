use crate::WindowEvent;
use crate::window::NativeWindow;
use raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle, HandleError};

#[derive(Debug)]
pub struct X11Window;

impl NativeWindow for X11Window {
    fn init() -> Self {
        todo!()
    }

    fn show(&mut self) {
        todo!()
    }

    fn focus(&mut self) {
        todo!()
    }

    fn shutdown(&mut self) {
        todo!()
    }

    fn is_focused(&self) -> bool {
        todo!()
    }

    fn lock_cursor(&mut self, lock: bool) {
        todo!()
    }

    fn poll(&mut self) -> Vec<WindowEvent> {
        todo!()
    }

    fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }

    fn get_size(&self) -> (u32, u32) {
        todo!()
    }

    fn get_clipboard(&self) -> String {
        todo!()
    }

    fn get_content_scale(&self) -> (f32, f32) {
        todo!()
    }

    fn set_title(&mut self, title: &str) {
        todo!()
    }

    fn set_clipboard(&mut self, text: &str) {
        todo!()
    }

    fn set_resizeable(&mut self, resizable: bool) {
        todo!()
    }

    fn set_cursor_visible(&mut self, visible: bool) {
        todo!()
    }

    fn set_cursor_position(&mut self, x: u32, y: u32) {
        todo!()
    }

    fn get_cursor_position(&self) -> (u32, u32) {
        todo!()
    }
}

impl HasWindowHandle for X11Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        todo!()
    }
}

impl HasDisplayHandle for X11Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        todo!()
    }
}
