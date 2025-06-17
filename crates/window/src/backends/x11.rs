use crate::WindowEvent;
use crate::window::NativeWindow;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use x11rb::protocol::xproto::*;
use x11rb::connection::Connection;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::rust_connection::RustConnection;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

#[derive(Debug)]
pub struct X11Window {
    atoms: Atoms,
    window: Window,
    connection: RustConnection,
}

#[profiling::all_functions]
impl NativeWindow for X11Window {
    fn init() -> Self {
        let (conn, screen_num) = x11rb::connect(None).expect("Failed to connect to X11 server");
        let win_id = conn.generate_id().expect("Failed to generate window ID");
        let atoms = Atoms::new(&conn).unwrap().reply().unwrap();
        let screen = &conn.setup().roots[screen_num];

        // conn.create_window();

        return Self {
            atoms: atoms,
            window: win_id,
            connection: conn,
        };
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
        self.connection.change_property8(
            PropMode::REPLACE,
            self.window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            title.as_bytes(),
        ).expect("Failed to set window title");
        self.connection.change_property8(
            PropMode::REPLACE,
            self.window,
            self.atoms._NET_WM_NAME,
            self.atoms.UTF8_STRING,
            title.as_bytes(),
        ).expect("Failed to set _NET_WM_NAME");
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
