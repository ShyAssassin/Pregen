#[derive(Debug, Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub enum TextureFormat {
    Rgba8Unorm,
    Rgba8UnormSrgb,
}

impl TextureFormat {
    pub fn is_srgb(&self) -> bool {
        match self {
            TextureFormat::Rgba8Unorm => false,
            TextureFormat::Rgba8UnormSrgb => true,
        }
    }

    pub fn compatible_formats(&self) -> Vec<wgpu::TextureFormat> {
        let mut _format = match self {
            TextureFormat::Rgba8Unorm => vec![wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rgba8UnormSrgb],
            TextureFormat::Rgba8UnormSrgb => vec![wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureFormat::Rgba8Unorm],
        };
        #[cfg(feature = "opengl")]
        _format.truncate(1);

        return _format;
    }
}

impl From<wgpu::TextureFormat> for TextureFormat {
    fn from(format: wgpu::TextureFormat) -> Self {
        match format {
            wgpu::TextureFormat::Rgba8Unorm => TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rgba8UnormSrgb => TextureFormat::Rgba8UnormSrgb,
            _ => panic!("Unsupported texture format"),
        }
    }
}

impl From<TextureFormat> for wgpu::TextureFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub enum RenderTargetFormat {
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Depth32Float,
}

impl RenderTargetFormat {
    pub fn is_srgb(&self) -> bool {
        match self {
            RenderTargetFormat::Rgba8Unorm => false,
            RenderTargetFormat::Bgra8Unorm => false,
            RenderTargetFormat::Depth32Float => false,
            RenderTargetFormat::Rgba8UnormSrgb => true,
            RenderTargetFormat::Bgra8UnormSrgb => true,
        }
    }

    pub fn compatible_formats(&self) -> Vec<wgpu::TextureFormat> {
        let mut _format = match self {
            RenderTargetFormat::Depth32Float => vec![wgpu::TextureFormat::Depth32Float],
            RenderTargetFormat::Rgba8Unorm => vec![wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rgba8UnormSrgb],
            RenderTargetFormat::Bgra8Unorm => vec![wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureFormat::Bgra8UnormSrgb],
            RenderTargetFormat::Rgba8UnormSrgb => vec![wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureFormat::Rgba8Unorm],
            RenderTargetFormat::Bgra8UnormSrgb => vec![wgpu::TextureFormat::Bgra8UnormSrgb, wgpu::TextureFormat::Bgra8Unorm],
        };
        #[cfg(feature = "opengl")]
        _format.truncate(1);

        return _format;

    }
}

impl From<wgpu::TextureFormat> for RenderTargetFormat {
    fn from(format: wgpu::TextureFormat) -> Self {
        match format {
            wgpu::TextureFormat::Rgba8Unorm => RenderTargetFormat::Rgba8Unorm,
            wgpu::TextureFormat::Bgra8Unorm => RenderTargetFormat::Bgra8Unorm,
            wgpu::TextureFormat::Depth32Float => RenderTargetFormat::Depth32Float,
            wgpu::TextureFormat::Rgba8UnormSrgb => RenderTargetFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Bgra8UnormSrgb => RenderTargetFormat::Bgra8UnormSrgb,
            _ => panic!("Unsupported render target format"),
        }
    }
}

impl From<RenderTargetFormat> for wgpu::TextureFormat {
    fn from(format: RenderTargetFormat) -> wgpu::TextureFormat {
        match format {
            RenderTargetFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            RenderTargetFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            RenderTargetFormat::Depth32Float => wgpu::TextureFormat::Depth32Float,
            RenderTargetFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            RenderTargetFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
}
