use crate::Device;
use raw_window_handle::HasWindowHandle;
use raw_window_handle::HasDisplayHandle;

pub struct Surface {
    pub device: Device,
    pub surface: wgpu::Surface<'static>,
    pub handle: wgpu::SurfaceTargetUnsafe,
    pub config: wgpu::SurfaceConfiguration,
}

impl Surface {
    pub unsafe fn new<T>(device: Device, handle: &T, framebuffer: (u32, u32)) -> Self
    where T: HasWindowHandle + HasDisplayHandle {
        let target = wgpu::SurfaceTargetUnsafe::from_window(handle).unwrap();
        let surface = device.instance.create_surface_unsafe(target).unwrap();

        // WGPU currently doesnt have a way to query prefered formats
        // So just pick the first one which should be the prefered one
        // Although order is entirely dependent on the drivers goodwill
        let prefered = surface.get_capabilities(&device.adapter).formats[0];

        // We want an srgb format if possible but if there isnt one
        // just go with the prefered format and manually apply tonemapping
        let format = surface.get_capabilities(&device.adapter).formats.iter()
            .find(|format| format.is_srgb())
        .copied().unwrap_or(prefered);

        let config = wgpu::SurfaceConfiguration {
            format: format,
            width: framebuffer.0,
            height: framebuffer.1,
            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![format.add_srgb_suffix(), format.remove_srgb_suffix()],
        };
        surface.configure(&device.device, &config);

        let wshandle = wgpu::SurfaceTargetUnsafe::from_window(handle).unwrap();
        log::debug!("Created surface for {:p} {:?} ({}x{})", handle, format, framebuffer.0, framebuffer.1);

        return Self {
            device: device,
            config: config,
            handle: wshandle,
            surface: surface,
        };
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            if (width, height) != (self.config.width, self.config.height) {
                self.config.width = width;
                self.config.height = height;
                log::debug!("Surface resized to ({},{})", width, height);
                self.surface.configure(&self.device.device, &self.config);
            } else {
                log::trace!("Surface resize ignored, is already ({},{})", width, height);
            }
        } else {
            log::warn!("Surface attempted to resize to invalid geometry ({},{})", width, height);
        }
    }

    pub fn width(&self) -> u32 {
        return self.config.width;
    }

    pub fn height(&self) -> u32 {
        return self.config.height;
    }

    pub fn size(&self) -> (u32, u32) {
        return (self.width(), self.height());
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        return self.config.format;
    }
}
