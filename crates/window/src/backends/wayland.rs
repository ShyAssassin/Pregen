// SHUT THE FUCK UP
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ptr::NonNull;
use crate::window::NativeWindow;
use crate::{WindowEvent, Action, Key, MouseButton};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{WaylandWindowHandle, RawWindowHandle};
use raw_window_handle::{WaylandDisplayHandle, RawDisplayHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

use wayland_client::protocol::*;
use wayland_protocols::xdg::shell::client::*;
use wayland_client::globals::{GlobalList, GlobalListContents, registry_queue_init};
use wayland_client::{Connection, EventQueue, QueueHandle, Proxy, Dispatch, delegate_noop};
use wayland_client::protocol::{wl_seat::WlSeat, wl_keyboard::WlKeyboard, wl_pointer::WlPointer};
use wayland_protocols::xdg::shell::client::{xdg_wm_base::XdgWmBase, xdg_toplevel::XdgToplevel, xdg_surface::XdgSurface};
use wayland_client::protocol::{wl_registry::WlRegistry, wl_display::WlDisplay, wl_surface::WlSurface, wl_compositor::WlCompositor};

#[derive(Default)]
pub struct WaylandState {
    pub width: i32,
    pub height: i32,
}

pub struct WaylandGlobals {
    pub wl_seat: WlSeat,
    pub wl_display: WlDisplay,
    pub xdg_wm_base: XdgWmBase,
    pub wl_compositor: WlCompositor,
}

pub struct WaylandWindow {
    pub registry: GlobalList,
    pub wlstate: WaylandState,
    pub connection: Connection,
    pub queue: EventQueue<Self>,
    pub events: Vec<WindowEvent>,

    pub wl_seat: WlSeat,
    pub wl_display: WlDisplay,
    pub xdg_wm_base: XdgWmBase,
    pub wl_compositor: WlCompositor,

    pub wl_pointer: WlPointer,
    pub wl_surface: WlSurface,
    pub xdg_surface: XdgSurface,
    pub wl_keyboard: WlKeyboard,
    pub xdg_toplevel: XdgToplevel,
}

delegate_noop!(WaylandWindow: WlDisplay);
delegate_noop!(WaylandWindow: WlRegistry);
delegate_noop!(WaylandWindow: WlCompositor);

// TODO: dont ignore, these are important
delegate_noop!(WaylandWindow: ignore WlSeat);
delegate_noop!(WaylandWindow: ignore WlSurface);

#[profiling::all_functions]
impl NativeWindow for WaylandWindow {
    fn init() -> Self {
        let state = WaylandState::default();
        let conn = Connection::connect_to_env().unwrap();
        let (registry, queue) = registry_queue_init(&conn).unwrap();

        let wl_display: WlDisplay = conn.display();
        for global in registry.contents().clone_list() {
            log::debug!(
                "Found global {} version {}",
                global.interface, global.version
            );
        }

        // Version >4 changed behavour to not allow the reuse of seats after removal
        let wl_seat: WlSeat = registry.bind::<WlSeat, _, _>(&queue.handle(), 1..=4, ()).unwrap();
        let xdg_wm_base: XdgWmBase = registry.bind::<XdgWmBase, _, _>(&queue.handle(), 1..=3, ()).unwrap();
        let wl_compositor: WlCompositor = registry.bind::<WlCompositor, _, _>(&queue.handle(), 1..=6, ()).unwrap();

        let wl_pointer = wl_seat.get_pointer(&queue.handle(), ());
        let wl_keyboard = wl_seat.get_keyboard(&queue.handle(), ());
        let wl_surface = wl_compositor.create_surface(&queue.handle(), ());
        let xdg_surface = xdg_wm_base.get_xdg_surface(&wl_surface, &queue.handle(), ());
        let xdg_toplevel = xdg_surface.get_toplevel(&queue.handle(), ());

        return Self {
            queue: queue,
            wlstate: state,
            connection: conn,
            registry: registry,
            events: Vec::default(),

            wl_seat: wl_seat,
            wl_display: wl_display,
            xdg_wm_base: xdg_wm_base,
            wl_compositor: wl_compositor,

            wl_pointer: wl_pointer,
            wl_surface: wl_surface,
            xdg_surface: xdg_surface,
            wl_keyboard: wl_keyboard,
            xdg_toplevel: xdg_toplevel,
        };
    }

    fn show(&mut self) {
        // todo!()
    }

    fn focus(&mut self) {
        // todo!()
    }

    fn shutdown(&mut self) {
        // todo!()
        self.connection.flush().unwrap();
    }

    fn is_focused(&self) -> bool {
        // todo!()
        return true;
    }

    fn lock_cursor(&mut self, lock: bool) {
        // todo!()
    }

    fn poll(&mut self) -> Vec<WindowEvent> {
        self.connection.flush().unwrap();
        unsafe {
            let self_ptr = self as *mut Self;
            (*self_ptr).queue.roundtrip(&mut *self_ptr).unwrap();
            (*self_ptr).queue.dispatch_pending(&mut *self_ptr).unwrap();
        }

        return self.events.drain(..).collect()
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.wlstate.width = width as i32;
        self.wlstate.height = height as i32;
        // Wayland currently doesnt have a way for clients to suggest a size to the compositor,
        // the way resizing is handled is by attaching a new buffer of the desired size to
        // the wl_surface. Since we dont handle the wl_surface buffer directly we push
        // a resize event and hope the renderer handles resizing the buffer for us.
        self.events.push(WindowEvent::Resize {
            width: width as u32,
            height: height as u32,
        });
    }

    fn get_size(&self) -> (u32, u32) {
        let width = self.wlstate.width as u32;
        let height = self.wlstate.height as u32;

        return (width, height);
    }

    fn get_clipboard(&self) -> String {
        // todo!()
        return String::new();
    }

    fn get_content_scale(&self) -> (f32, f32) {
        // TODO: this
        return (1.0, 1.0);
    }

    fn set_title(&mut self, title: &str) {
        self.xdg_toplevel.set_title(title.to_string());
    }

    fn set_clipboard(&mut self, text: &str) {
        // todo!()
    }

    fn set_resizeable(&mut self, resizable: bool) {
        // todo!()
    }

    fn set_cursor_visible(&mut self, visible: bool) {
        // todo!()
    }

    fn set_cursor_position(&mut self, x: u32, y: u32) {
        // todo!()
    }

    fn get_cursor_position(&self) -> (u32, u32) {
        // todo!()
        return (0, 0);
    }
}


impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for WaylandWindow {
    fn event(
        _: &mut Self, _: &WlRegistry, event: wl_registry::Event,
        _: &GlobalListContents, _: &Connection, _: &QueueHandle<Self>,
    ) {
        log::debug!("Registry event: {:?}", event);
    }
}

impl Dispatch<XdgToplevel, ()> for WaylandWindow {
    fn event(
        wlstate: &mut Self, _: &XdgToplevel, event: xdg_toplevel::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        if let xdg_toplevel::Event::Close = event {
            wlstate.events.push(WindowEvent::CloseRequested);
        }
        // TODO: acording to IRC we should only send resize events to the queue
        // after we recieve an xdg_surface configure event, so cache and send later
        if let xdg_toplevel::Event::Configure { width, height, states: _ } = event {
            if width > 0 && height > 0 {
                wlstate.wlstate.width = width;
                wlstate.wlstate.height = height;
                wlstate.events.push(WindowEvent::Resize {
                    width: width as u32,
                    height: height as u32,
                });
            }
        }
    }
}

impl Dispatch<WlKeyboard, ()> for WaylandWindow {
    fn event(
        _: &mut Self, _: &WlKeyboard, event: wl_keyboard::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        log::debug!("Keyboard event: {:?}", event);
    }
}

impl Dispatch<WlPointer, ()> for WaylandWindow {
    fn event(
        wlstate: &mut Self, _: &WlPointer, event: wl_pointer::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Motion { time: _, surface_x, surface_y } => {
                wlstate.events.push(WindowEvent::CursorPosition {
                    mouse_x: surface_x,
                    mouse_y: surface_y,
                });
            }
            wl_pointer::Event::Button { serial: _, time: _, button, state } => {
                let button = match button {
                    0x110 => MouseButton::Left,
                    0x111 => MouseButton::Right,
                    0x112 => MouseButton::Middle,
                    _ => MouseButton::Other(button),
                };
                let action = match state.into_result().unwrap() {
                    wl_pointer::ButtonState::Pressed => Action::Pressed,
                    wl_pointer::ButtonState::Released => Action::Released,
                    _ => unreachable!("Unknown wayland button state??????"),
                };
                wlstate.events.push(WindowEvent::MouseButton(button, action));
            }
            _ => {}
        }
    }
}

impl Dispatch<XdgSurface, ()> for WaylandWindow {
    fn event(
        _: &mut Self, xdg_surface: &XdgSurface, event: xdg_surface::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            xdg_surface.ack_configure(serial);
        }
    }
}

impl Dispatch<XdgWmBase, ()> for WaylandWindow {
    fn event(
        _: &mut Self, xdg_wm_base: &XdgWmBase, event: xdg_wm_base::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            xdg_wm_base.pong(serial);
        }
    }
}

impl HasWindowHandle for WaylandWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            let handle = WaylandWindowHandle::new(NonNull::new(
                self.wl_surface.id().as_ptr()
                .cast::<std::ffi::c_void>().into()
            ).unwrap());

            let rwh = RawWindowHandle::Wayland(handle);
            return Ok(WindowHandle::borrow_raw(rwh));
        }
    }
}

impl HasDisplayHandle for WaylandWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            let handle = WaylandDisplayHandle::new(NonNull::new(
                self.wl_display.id().as_ptr()
                .cast::<std::ffi::c_void>().into()
            ).unwrap());

            let rdh = RawDisplayHandle::Wayland(handle);
            return Ok(DisplayHandle::borrow_raw(rdh));
        }
    }
}
