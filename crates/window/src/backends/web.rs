use std::ptr::NonNull;
use crate::WindowEvent;
use crate::window::NativeWindow;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{WebCanvasWindowHandle, RawWindowHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

#[derive(Debug)]
pub struct WebWindow {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,
}

impl NativeWindow for WebWindow {
    fn init() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        // FIXME: hard coding this is bad, should be user defined and passed in, add a window hint?
        let canvas = document.get_element_by_id("surface").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        return Self {
            window: window,
            canvas: canvas,
            document: document,
        };
    }

    fn show(&mut self) {
        // This is a no-op on the web, but we will still ask for focus
        self.focus();
    }

    fn focus(&mut self) {
        match self.window.focus() {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to focus window: {:?}", err);
            }
        }
    }

    fn shutdown(&mut self) {
        todo!()
    }

    fn is_focused(&self) -> bool {
        match self.document.has_focus() {
            Ok(focus) => return focus,
            Err(err) => {
                log::error!("Web focus query failed: {:?}", err);
                return false;
            }
        };
    }

    fn poll(&mut self) -> Vec<WindowEvent> {
        todo!("We first need to add event listeners to the canvas");
    }

    fn resize(&mut self, width: u32, height: u32) {
        // While not a no-op the parent element should handle resizing the canvas
    }

    fn get_size(&self) -> (u32, u32) {
        let width = self.canvas.client_width();
        let height = self.canvas.client_height();
        return (width as u32, height as u32);
    }

    fn get_clipboard(&self) -> String {
        todo!()
    }

    fn get_content_scale(&self) -> (f32, f32) {
        // I don't think content scaling is a thing on the web
        return (1.0, 1.0);
    }

    fn get_cursor_position(&self) -> (f32, f32) {
        todo!()
    }

    fn set_title(&mut self, title: &str) {
        self.document.set_title(title);
    }

    fn set_clipboard(&mut self, text: &str) {
        todo!()
    }

    fn set_resizeable(&mut self, resizable: bool) {
        // This is a no-op on the web
    }

    fn set_cursor_visible(&mut self, visible: bool) {
        todo!("Will need to set the cursor style to none or default");
    }

    fn set_cursor_position(&mut self, x: f32, y: f32) {
        todo!()
    }
}

impl HasWindowHandle for WebWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            let value = JsValue::from(&self.canvas);
            let handle = WebCanvasWindowHandle::new(NonNull::from(&value).cast());
            return Ok(WindowHandle::borrow_raw(RawWindowHandle::WebCanvas(handle)));
        }
    }
}

impl HasDisplayHandle for WebWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        return Ok(DisplayHandle::web());
    }
}
