mod glfw;
pub use glfw::GlfwWindow;

#[cfg(target_family = "windows")]
mod win32;
#[cfg(target_family = "windows")]
pub use win32::Win32Window;

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
mod x11;
#[cfg(all(target_family = "unix", not(target_os = "macos")))]
pub use x11::X11Window;

#[derive(Debug, Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub enum WindowBackend {
    Win32,
    Glfw,
    X11,
}

impl WindowBackend {
    #[allow(unreachable_code)]
    pub const fn preferred() -> Self {
        #[cfg(target_family = "windows")]
        return WindowBackend::Win32;

        #[cfg(target_os = "macos")]
        return WindowBackend::Glfw;

        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        return WindowBackend::Glfw;

        unreachable!("Compiler platform has no preferred window backend");
    }
}
