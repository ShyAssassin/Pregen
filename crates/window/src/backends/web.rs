use std::ptr::NonNull;
use crate::WindowEvent;
use std::sync::{Arc, Mutex};
use crate::window::NativeWindow;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{WebCanvasWindowHandle, RawWindowHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

#[derive(Debug)]
pub struct WebWindow {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,
    pub events: Arc<Mutex<Vec<WindowEvent>>>,
}

impl NativeWindow for WebWindow {
    fn init() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let events = Arc::new(Mutex::new(Vec::new()));
        // FIXME: hard coding this is bad, should be user defined and passed in, add a window hint?
        let canvas = document.get_element_by_id("view").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        // FIXME: calling .forget() on the closures will memory leak, need to store them somewhere
        let key_up = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            log::info!("Key up event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onkeyup(Some(key_up.as_ref().unchecked_ref()));
        key_up.forget();

        let key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            log::info!("Key down event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onkeydown(Some(key_down.as_ref().unchecked_ref()));
        key_down.forget();

        let mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log::info!("Mouse down event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onmousedown(Some(mouse_down.as_ref().unchecked_ref()));
        mouse_down.forget();

        let mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log::info!("Mouse up event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onmouseup(Some(mouse_up.as_ref().unchecked_ref()));
        mouse_up.forget();

        let blur = Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
            log::info!("Blur event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onblur(Some(blur.as_ref().unchecked_ref()));
        blur.forget();

        let focus = Closure::wrap(Box::new(move |event: web_sys::FocusEvent| {
            log::info!("Focus event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onfocus(Some(focus.as_ref().unchecked_ref()));
        focus.forget();

        let resize = Closure::wrap(Box::new(move |event: web_sys::UiEvent| {
            log::info!("Resize event: {:?}", event);
        }) as Box<dyn FnMut(_)>);
        canvas.set_onresize(Some(resize.as_ref().unchecked_ref()));
        resize.forget();

        return Self {
            window: window,
            canvas: canvas,
            events: events,
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
        // We *probably* don't need to do anything here...
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
        // FIXME: to not block the main thread we need to use either a web worker or put everything in `requestAnimationFrame` closure
        // Potentially we could make the thread async sleep for a bit to not block the main thread
        // We could also use `setInterval` / `setTimeout` but that would probably be a bad idea
        return Vec::new();
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
