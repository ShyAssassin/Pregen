#[cfg(not(target_family = "wasm"))]
mod glfw;
#[cfg(not(target_family = "wasm"))]
pub use glfw::GlfwWindow;

#[cfg(target_family = "windows")]
mod win32;
#[cfg(target_family = "windows")]
pub use win32::Win32Window;

#[cfg(target_family = "wasm")]
mod web;
#[cfg(target_family = "wasm")]
pub use web::WebWindow;

#[cfg(target_os = "linux")]
mod x11;
#[cfg(target_os = "linux")]
pub use x11::X11Window;

#[cfg(target_os = "linux")]
mod wayland;
#[cfg(target_os = "linux")]
pub use wayland::WaylandWindow;

#[derive(Debug, Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub enum WindowBackend {
    X11,
    Web,
    Glfw,
    Win32,
    Unkown,
    Wayland,
}

impl WindowBackend {
    #[allow(unreachable_code)]
    pub const fn preferred() -> Self {
        #[cfg(target_family = "windows")]
        return WindowBackend::Win32;

        #[cfg(target_family = "wasm")]
        return WindowBackend::Web;

        #[cfg(target_os = "linux")]
        return WindowBackend::Glfw;

        #[cfg(target_os = "macos")]
        return WindowBackend::Glfw;

        unreachable!("Compiler platform has no preferred window backend");
    }
}
