use wayland_client::{protocol::{wl_display::WlDisplay, wl_surface::WlSurface}, Connection};

pub struct WaylandWindow {
    pub display: WlDisplay,
    pub connection: Connection,
    pub surface: Option<WlSurface>,
}

