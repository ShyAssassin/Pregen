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

use xkbcommon::xkb;
use xkbcommon::xkb::keysyms;
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
    // pub wlglobals: WaylandGlobals,

    pub wl_seat: WlSeat,
    pub wl_display: WlDisplay,
    pub xdg_wm_base: XdgWmBase,
    pub wl_compositor: WlCompositor,

    pub wl_pointer: WlPointer,
    pub wl_surface: WlSurface,
    pub xdg_surface: XdgSurface,
    pub wl_keyboard: WlKeyboard,
    pub xdg_toplevel: XdgToplevel,

    pub xkb_context: xkb::Context,
    pub xkb_state: Option<xkb::State>,
    pub xkb_keymap: Option<xkb::Keymap>,
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

        let xkb_context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

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

            xkb_state: None,
            xkb_keymap: None,
            xkb_context: xkb_context,
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
        wlstate: &mut Self, _: &WlKeyboard, event: wl_keyboard::Event,
        _: &(), _: &Connection, _: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                log::debug!("Keyboard keymap event: format={:?}, size={}", format, size);
                if format != wayland_client::WEnum::Value(wl_keyboard::KeymapFormat::XkbV1) {
                    log::warn!("Unsupported keymap format: {:?}", format);
                }
                match unsafe { xkb::Keymap::new_from_fd(&wlstate.xkb_context, fd,
                        size as usize, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS) } {
                    Ok(Some(keymap)) => {
                        log::info!("Loaded keymap with {} layouts", keymap.num_layouts());
                        let state = xkb::State::new(&keymap);
                        wlstate.xkb_keymap = Some(keymap);
                        wlstate.xkb_state = Some(state);
                    }
                    Ok(None) => {
                        log::error!("Failed to compile xkb keymap from compositor");
                    }
                    Err(e) => {
                        log::error!("Failed to create xkb keymap from fd: {}", e);
                    }
                }
            }
            wl_keyboard::Event::Key { serial: _, time: _, key, state } => {
                // xkb wants evdev + 8, for some reason...
                let xkb_keycode = xkb::Keycode::new(key + 8);
                let action = match state.into_result().unwrap() {
                    wl_keyboard::KeyState::Pressed => Action::Pressed,
                    wl_keyboard::KeyState::Released => Action::Released,
                    _ => unreachable!("Unknown wayland key state, somehow"),
                };

                if let Some(xkb_state) = &wlstate.xkb_state {
                    let keysym = xkb_state.key_get_one_sym(xkb_keycode);
                    wlstate.events.push(WindowEvent::KeyboardInput(keysym.into(), key, action));
                } else {
                    wlstate.events.push(WindowEvent::KeyboardInput(Key::Other(key), key, action));
                    log::warn!("Keyboard event before xkb state initialized, using raw scancode {}", key);
                }
            }
            wl_keyboard::Event::Modifiers { serial: _, mods_depressed, mods_latched, mods_locked, group } => {
                if let Some(xkb_state) = &mut wlstate.xkb_state {
                    xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
                } else {
                    log::warn!("Received keyboard modifiers event before xkb state initialized");
                }
            }
            wl_keyboard::Event::Enter { serial: _, surface: _, keys: _ } => {
                wlstate.events.push(WindowEvent::FocusGained);
            }
            wl_keyboard::Event::Leave { serial: _, surface: _ } => {
                wlstate.events.push(WindowEvent::FocusLost);
            }
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                log::debug!("Keyboard repeat info: rate={}, delay={}", rate, delay);
            }
            _ => {
                log::debug!("Unhandled keyboard event: {:?}", event);
            }
        }
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

impl From<xkb::Keysym> for Key {
    fn from(keysym: xkb::Keysym) -> Self {
        let raw = keysym.raw();
        match raw {
            // A-Z and a-z
            65..=90 | 97..=122 => {
                let keycode = raw as u8;
                return Key::from_char(keycode as char);
            }

            // 0-9
            48..=57 => {
                let keycode = raw as u8;
                return Key::from_digit(keycode as char);
            }

            // Arrow keys
            keysyms::KEY_Up => Key::Up, keysyms::KEY_Down => Key::Down,
            keysyms::KEY_Left => Key::Left, keysyms::KEY_Right => Key::Right,

            // Modifier keys
            keysyms::KEY_Alt_L => Key::LAlt, keysyms::KEY_Alt_R => Key::RAlt,
            keysyms::KEY_Shift_L => Key::LShift, keysyms::KEY_Shift_R => Key::RShift,
            keysyms::KEY_Control_L => Key::LCtrl, keysyms::KEY_Control_R => Key::RCtrl,

            // Operation keys
            keysyms::KEY_minus => Key::Minus, keysyms::KEY_equal => Key::Equals,
            keysyms::KEY_apostrophe => Key::Apostrophe, keysyms::KEY_grave => Key::Grave,
            keysyms::KEY_backslash => Key::Backslash, keysyms::KEY_semicolon => Key::Semicolon,
            keysyms::KEY_bracketleft => Key::LeftBracket, keysyms::KEY_bracketright => Key::RightBracket,
            keysyms::KEY_comma => Key::Comma, keysyms::KEY_period => Key::Period, keysyms::KEY_slash => Key::Slash,

            // Action keys
            keysyms::KEY_Escape => Key::Escape, keysyms::KEY_Home => Key::Home,
            keysyms::KEY_End => Key::End, keysyms::KEY_Tab => Key::Tab, keysyms::KEY_BackSpace => Key::Backspace,
            keysyms::KEY_Insert => Key::Insert, keysyms::KEY_Delete => Key::Delete, keysyms::KEY_Page_Up => Key::PageUp,
            keysyms::KEY_Page_Down => Key::PageDown, keysyms::KEY_space => Key::Space, keysyms::KEY_Return => Key::Enter,


            // Function keys
            keysyms::KEY_F1 => Key::F1, keysyms::KEY_F2 => Key::F2, keysyms::KEY_F3 => Key::F3, keysyms::KEY_F4 => Key::F4,
            keysyms::KEY_F5 => Key::F5, keysyms::KEY_F6 => Key::F6, keysyms::KEY_F7 => Key::F7, keysyms::KEY_F8 => Key::F8,
            keysyms::KEY_F9 => Key::F9, keysyms::KEY_F10 => Key::F10, keysyms::KEY_F11 => Key::F11, keysyms::KEY_F12 => Key::F12,
            keysyms::KEY_F13 => Key::F13, keysyms::KEY_F14 => Key::F14, keysyms::KEY_F15 => Key::F15, keysyms::KEY_F16 => Key::F16,
            keysyms::KEY_F17 => Key::F17, keysyms::KEY_F18 => Key::F18, keysyms::KEY_F19 => Key::F19, keysyms::KEY_F20 => Key::F20,
            keysyms::KEY_F21 => Key::F21, keysyms::KEY_F22 => Key::F22, keysyms::KEY_F23 => Key::F23, keysyms::KEY_F24 => Key::F24,

            _ => {
                log::warn!("Unknown xkb keysym: 0x{:x}", raw);
                Key::Other(raw)
            }
        }
    }
}
