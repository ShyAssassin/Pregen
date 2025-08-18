// TODO: switch to windows / windows-sys
use winapi::um::winuser::*;
use winapi::shared::basetsd::LONG_PTR;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::{HWND, POINT, PRECTL};
use winapi::shared::winerror::ERROR_CLASS_ALREADY_EXISTS;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use winapi::shared::minwindef::{HINSTANCE, HIWORD, LOWORD, LPARAM, LPVOID, LRESULT, UINT, WPARAM};

use std::num::NonZero;
use std::sync::{Mutex, Arc};
use crate::window::NativeWindow;
use std::ffi::{CStr, CString, OsStr};
use std::ptr::{null_mut, copy_nonoverlapping};
use crate::{WindowEvent, Action, Key, MouseButton};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};
use raw_window_handle::{Win32WindowHandle, RawWindowHandle};
use raw_window_handle::{DisplayHandle, WindowHandle, HandleError};

#[derive(Debug)]
struct Win32WindowState {
    pub cursor_visible: Mutex<bool>,
    pub events: Mutex<Vec<WindowEvent>>,
    pub cursor_move_pos: Mutex<(i32, i32)>,
}

#[derive(Debug)]
pub struct Win32Window {
    hwnd: HWND,
    class: Vec<u16>,
    instance: HINSTANCE,
    // FIXME: use a rc<refcell>
    state: Arc<Win32WindowState>,
}

impl Win32Window {
    fn null() -> Self {
        Self {
            hwnd: null_mut(),
            class: Vec::new(),
            instance: null_mut(),
            state: Arc::new(Win32WindowState {
                events: Mutex::new(Vec::new()),
                cursor_visible: Mutex::new(true),
                cursor_move_pos: Mutex::new((0, 0)),
            }),
        }
    }

    fn to_wstring(s: &str) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        return OsStr::new(s).encode_wide().chain(Some(0)).collect();
    }

    unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        // SAFETY: Since WM_CREATE is the first message sent to the window we can assume that GWLP_USERDATA is non-null
        let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Win32WindowState;
        let state = &*state_ptr;

        match msg {
            WM_CREATE => {
                let info = &*(l_param as *const CREATESTRUCTW);
                let window_state_ptr = info.lpCreateParams as LONG_PTR;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_state_ptr);
                return LRESULT::from(0u8);
            }

            WM_DESTROY => {
                if !state_ptr.is_null() {
                    state.events.lock().unwrap().push(WindowEvent::Destroyed);
                    drop(Arc::from_raw(state_ptr));
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                }
                DestroyWindow(hwnd);
                return LRESULT::from(0u8);
            }

            WM_SETCURSOR => {
                // The cursor will be changed by windows itself when outside the client area
                // When going back into the client area we just put the cursor back to the previous state
                if LOWORD(l_param as u32) == HTCLIENT as u16 {
                    if !(*state.cursor_visible.lock().unwrap()) {
                        SetCursor(null_mut());
                        return LRESULT::from(true);
                    }
                }
                // MSDN says this should return false, it lied
                return DefWindowProcW(hwnd, msg, w_param, l_param);
            }

            WM_DPICHANGED => {
                let rect = &*(l_param as PRECTL);
                // Resize based on lParam suggestions to maintain scale
                SetWindowPos(hwnd, null_mut(), // This should emit WM_SIZE?
                    rect.left, rect.top,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    SWP_NOZORDER | SWP_NOMOVE | SWP_FRAMECHANGED
                );

                let dpi = f32::from(HIWORD(w_param as u32));
                state.events.lock().unwrap().push(WindowEvent::ScaleFactorChanged {
                    scale_x: dpi / USER_DEFAULT_SCREEN_DPI as f32,
                    scale_y: dpi / USER_DEFAULT_SCREEN_DPI as f32,
                });
                return LRESULT::from(0u8);
            }

            WM_SIZE => {
                let width = LOWORD(l_param as u32) as u32;
                let height = HIWORD(l_param as u32) as u32;
                match w_param {
                    SIZE_MINIMIZED => state.events.lock().unwrap().push(WindowEvent::Minimized),
                    SIZE_MAXIMIZED => state.events.lock().unwrap().push(WindowEvent::Maximized),
                    _ => {}
                }
                state.events.lock().unwrap().push(WindowEvent::Resize { width, height });
                return LRESULT::from(0u8);
            }

            WM_KEYDOWN | WM_KEYUP => {
                let mut queue = state.events.lock().unwrap();
                let is_extended = (l_param & 0x01000000) != 0;
                let scancode = ((l_param >> 16) & 0xFF) as u32;
                // Some keys are not correctly mapped by the windows API
                let key_code = match w_param as i32 {
                    VK_MENU => if is_extended {VK_RMENU} else {VK_LMENU}
                    VK_SHIFT => if scancode == 0x36 {VK_RSHIFT} else {VK_LSHIFT}
                    VK_CONTROL => if is_extended {VK_RCONTROL} else {VK_LCONTROL}
                    _ => w_param as i32,
                };
                let is_pressed = Action::from(msg == WM_KEYDOWN);
                queue.push(WindowEvent::KeyboardInput(Key::from(key_code as WPARAM), scancode, is_pressed));
                return LRESULT::from(0u8);
            }

            WM_MOUSEMOVE => {
                let mouse_x = GET_X_LPARAM(l_param);
                let mouse_y = GET_Y_LPARAM(l_param);
                // Should probably move over to the rawinput windows API for this stuff
                state.events.lock().unwrap().push(WindowEvent::CursorPosition {
                    mouse_x: mouse_x as u32,
                    mouse_y: mouse_y as u32
                });
                return LRESULT::from(0u8);
                // FIXME: sometimes the cursor doesnt allign with the actual position
                // if (mouse_x - state.cursor_move_pos.lock().unwrap().0).abs() > 1 ||
                //    (mouse_y - state.cursor_move_pos.lock().unwrap().1).abs() > 1 {
                //     state.events.lock().unwrap().push(WindowEvent::CursorPosition {
                //         mouse_x: mouse_x as u32,
                //         mouse_y: mouse_y as u32
                //     });
                // }
            }

            WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_MBUTTONDOWN | WM_MBUTTONUP => {
                const DOWN_EVENTS: [UINT; 3] = [WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_MBUTTONDOWN];
                let mut queue = state.events.lock().unwrap();
                let button = match msg {
                    WM_LBUTTONDOWN | WM_LBUTTONUP => MouseButton::Left,
                    WM_RBUTTONDOWN | WM_RBUTTONUP => MouseButton::Right,
                    WM_MBUTTONDOWN | WM_MBUTTONUP => MouseButton::Middle,
                    _ => MouseButton::Other(msg),
                };
                let is_pressed = Action::from(DOWN_EVENTS.contains(&msg));
                queue.push(WindowEvent::MouseButton(button, is_pressed));
                return LRESULT::from(0u8);
            }

            WM_CLOSE => {
                state.events.lock().unwrap().push(WindowEvent::CloseRequested);
                return LRESULT::from(0u8);
            }

            WM_ACTIVATE => {
                let active = LOWORD(w_param as u32);
                if active == WA_ACTIVE || active == WA_CLICKACTIVE {
                    state.events.lock().unwrap().push(WindowEvent::FocusGained);
                } else {
                    state.events.lock().unwrap().push(WindowEvent::FocusLost);
                }
                return LRESULT::from(0u8);
            }
            // Needs to manually be handled to avoid locking
            WM_SYSCHAR | WM_SYSKEYDOWN | WM_SYSKEYUP => {
                return LRESULT::from(0u8);
            }
            _ => DefWindowProcW(hwnd, msg, w_param, l_param)
        }
    }
}

#[profiling::all_functions]
impl NativeWindow for Win32Window {
    fn init() -> Self {
        // TODO: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiawarenesscontext
        // TODO: https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows
        unsafe {
            let mut window: Win32Window = Self::null();
            let h_instance = GetModuleHandleW(null_mut());
            let class_name = Self::to_wstring("PregenWindowClass");
            let wnd_class = WNDCLASSW {
                hInstance: h_instance,
                lpszMenuName: null_mut(),
                hbrBackground: null_mut(),
                style: CS_VREDRAW | CS_HREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                lpszClassName: class_name.as_ptr(),
                hCursor: LoadCursorW(null_mut(), IDC_ARROW),
                hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
                ..Default::default()
            };

            if RegisterClassW(&wnd_class) == 0 {
                let error = GetLastError();
                // We dont care if the class already exists
                if error != ERROR_CLASS_ALREADY_EXISTS {
                    panic!("Failed to register window class: {}", error);
                }
            }

            let hwnd = CreateWindowExW(
                0, // Optional styles
                // Class name
                class_name.as_ptr(),
                // Window name
                Self::to_wstring("Pregen").as_ptr(),
                // Window style
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                // Position and size
                CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
                // Parent window
                null_mut(),
                // Menu
                null_mut(),
                // Instance handle
                h_instance,
                // Aditional data
                Arc::into_raw(window.state.clone()) as LPVOID,
            );

            if hwnd.is_null() {
                panic!("Failed to create window: {}", GetLastError());
            }

            window.hwnd = hwnd;
            window.class = class_name;
            window.instance = h_instance;
            return window;
        }
    }

    fn show(&mut self) {
        unsafe {
            ShowWindow(self.hwnd, SW_RESTORE);
        }
    }

    fn focus(&mut self) {
        unsafe {
            self.show();
            SetFocus(self.hwnd);
        }
    }

    fn shutdown(&mut self) {
        // Cant use PostQuitMessage since that would effect every window
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }

    fn is_focused(&self) -> bool {
        unsafe {
            return GetFocus() == self.hwnd;
        }
    }

    fn lock_cursor(&mut self, lock: bool) {
        if lock == true {
            self.set_cursor_visible(false);
            unsafe { SetCapture(self.hwnd) };
        } else {
            unsafe { ReleaseCapture() };
            self.set_cursor_visible(true);
        }
    }

    fn poll(&mut self) -> Vec<WindowEvent> {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                match msg.message {
                    _ => { }
                };
            }
            // We need to perform the standard message loop of win32 first to not softlock the mutex
            // This mutex is kinda slow and should be replaced with a lock free alternative
            let mut queue = self.state.events.lock().unwrap();
            if !queue.is_empty() {
                return queue.drain(..).collect();
            }
            return Vec::new();
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        // This is a bit of a hack since we need to adjust the window size to include the border
        unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            let mut client_rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut client_rect);

            let style = GetWindowLongPtrW(self.hwnd, GWL_STYLE);
            AdjustWindowRect(&mut rect, style as u32, false as i32);
            let win_width = width as i32 + (rect.right - rect.left) - (client_rect.right - client_rect.left);
            let win_height = height as i32 + (rect.bottom - rect.top) - (client_rect.bottom - client_rect.top);
            SetWindowPos(self.hwnd, null_mut(), 0, 0, win_width, win_height, SWP_NOZORDER | SWP_NOMOVE | SWP_FRAMECHANGED);
        }
    }

    fn get_size(&self) -> (u32, u32) {
        // Same hack as `resize` but in reverse
        unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            let mut client_rect = std::mem::zeroed();
            GetClientRect(self.hwnd, &mut client_rect);

            let width = (client_rect.right - client_rect.left) as u32;
            let height = (client_rect.bottom - client_rect.top) as u32;
            return (width, height);
        }
    }

    fn get_clipboard(&self) -> String {
        let mut text = String::new();
        unsafe {
            if OpenClipboard(self.hwnd) == 0 {
                panic!("Failed to open clipboard: {}", GetLastError());
            };

            let data = GetClipboardData(CF_TEXT);
            if !data.is_null() {
                let c_str = CStr::from_ptr(GlobalLock(data) as *const i8);
                text = c_str.to_string_lossy().into_owned();
                GlobalUnlock(data);
            }
            CloseClipboard();
        };
        return text.to_string();
    }

    fn get_content_scale(&self) -> (f32, f32) {
        unsafe {
            let scale = GetDpiForWindow(self.hwnd) as f32 / USER_DEFAULT_SCREEN_DPI as f32;
            return (scale, scale);
        }
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let title = Self::to_wstring(title);
            SetWindowTextW(self.hwnd, title.as_ptr());
        }
    }

    fn set_clipboard(&mut self, text: &str) {
        unsafe {
            if OpenClipboard(self.hwnd) == 0 {
                panic!("Failed to open clipboard: {}", GetLastError());
            }

            if EmptyClipboard() == 0 {
                CloseClipboard();
                panic!("Failed to empty clipboard: {}", GetLastError());
            }

            let text = CString::new(text).unwrap();
            let length = text.as_bytes().len();
            let mem = GlobalAlloc(GMEM_MOVEABLE, length);
            if mem.is_null() {
                CloseClipboard();
                panic!("Failed to allocate clipboard memory: {}", GetLastError());
            }

            let ptr = GlobalLock(mem) as *mut i8;
            if ptr.is_null() {
                GlobalFree(mem);
                CloseClipboard();
                panic!("Failed to lock clipboard memory: {}", GetLastError());
            }

            copy_nonoverlapping(text.as_ptr(), ptr, length);
            GlobalUnlock(mem);

            if SetClipboardData(CF_TEXT, mem).is_null() {
                GlobalFree(mem);
                CloseClipboard();
                panic!("Failed to set clipboard data: {}", GetLastError());
            }

            CloseClipboard();
        }
    }

    fn set_resizeable(&mut self, resizable: bool) {
        unsafe {
            let style = GetWindowLongPtrW(self.hwnd, GWL_STYLE) as u32;
            let new_style = if resizable {
                style | WS_OVERLAPPEDWINDOW
            } else {
                // FIXME: is this correct?
                style & !WS_OVERLAPPEDWINDOW | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MINIMIZEBOX | WS_OVERLAPPED
            };
            SetWindowLongPtrW(self.hwnd, GWL_STYLE, new_style as LONG_PTR);
            SetWindowPos(self.hwnd, null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOZORDER | SWP_NOSIZE | SWP_FRAMECHANGED);
        }
    }

    fn set_cursor_visible(&mut self, visible: bool) {
        unsafe {
            *(self.state.cursor_visible.lock().unwrap()) = visible;
            let cursor = if visible { LoadCursorW(self.instance, IDC_ARROW) } else { null_mut() };
            SetCursor(cursor);
        }
    }

    fn set_cursor_position(&mut self, x: u32, y: u32) {
        unsafe {
            *self.state.cursor_move_pos.lock().unwrap() = (x as i32, y as i32);
            let mut point = POINT { x: x as i32, y: y as i32 };
            ClientToScreen(self.hwnd, &mut point);
            SetCursorPos(point.x, point.y);
        }
    }

    fn get_cursor_position(&self) -> (u32, u32) {
        unsafe {
            let mut point = POINT { x: 0, y: 0 };
            GetCursorPos(&mut point);
            ScreenToClient(self.hwnd, &mut point);
            return (point.x as u32, point.y as u32);
        }
    }
}

impl HasWindowHandle for Win32Window {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        unsafe {
            let mut handle = Win32WindowHandle::new(NonZero::new(self.hwnd as isize).unwrap());
            handle.hinstance = NonZero::new(self.instance as isize);
            return Ok(WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)));
        }
    }
}

impl HasDisplayHandle for Win32Window {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        return Ok(DisplayHandle::windows());
    }
}

impl From<WPARAM> for Key {
    fn from(key: WPARAM) -> Self {
        match key as i32 {
            65..=90 => Key::from_char((key as u8) as char), // A-Z
            48..=57 => Key::from_digit((key as u8) as char), // 0-9

            // Function keys
            VK_F1 => Key::F1, VK_F2 => Key::F2, VK_F3 => Key::F3, VK_F4 => Key::F4,
            VK_F5 => Key::F5, VK_F6 => Key::F6, VK_F7 => Key::F7, VK_F8 => Key::F8,
            VK_F9 => Key::F9, VK_F10 => Key::F10, VK_F11 => Key::F11, VK_F12 => Key::F12,
            VK_F13 => Key::F13, VK_F14 => Key::F14, VK_F15 => Key::F15, VK_F16 => Key::F16,
            VK_F17 => Key::F17, VK_F18 => Key::F18, VK_F19 => Key::F19, VK_F20 => Key::F20,
            VK_F21 => Key::F21, VK_F22 => Key::F22, VK_F23 => Key::F23, VK_F24 => Key::F24,

            // Modifier keys
            VK_LMENU => Key::LAlt, VK_RMENU => Key::RAlt,
            VK_LSHIFT => Key::LShift, VK_RSHIFT => Key::RShift,
            VK_LCONTROL => Key::LCtrl, VK_RCONTROL => Key::RCtrl,

            // Arrow keys
            VK_LEFT => Key::Left, VK_UP => Key::Up, VK_RIGHT => Key::Right, VK_DOWN => Key::Down,

            VK_END => Key::End, VK_PRIOR => Key::PageUp, VK_NEXT => Key::PageDown, VK_OEM_PLUS => Key::Equals,
            VK_INSERT => Key::Insert, VK_DELETE => Key::Delete, VK_HOME => Key::Home, VK_OEM_MINUS => Key::Minus,
            VK_SPACE => Key::Space, VK_RETURN => Key::Enter, VK_TAB => Key::Tab, VK_BACK => Key::Backspace, VK_ESCAPE => Key::Escape,

            // FIXME: determine if this is layout dependent
            219 => Key::LeftBracket, 221 => Key::RightBracket, 220 => Key::Backslash, 186 => Key::Semicolon,

            // VK_MENU is triggered by both LMENU and RMENU
            VK_MENU => Key::Other(VK_MENU as u32),
            // VK_SHIFT is triggered by both LSHIFT and RSHIFT
            VK_SHIFT => Key::Other(VK_SHIFT as u32),
            // VK_CONTROL is triggered by both LCONTROL and RCONTROL
            VK_CONTROL => Key::Other(VK_CONTROL as u32),
            _ => {
                log::warn!("Unknown key code: {}", key);
                Key::Other(key as u32)
            },
        }
    }
}
