use wgpu::Device;
use std::sync::Arc;
use crate::gfx::RenderTargetFormat;

use super::Sampler;

pub struct RenderTexture {
    pub name: String,
    pub size: wgpu::Extent3d,
    pub sampler: Arc<Sampler>,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: RenderTargetFormat,
}

impl RenderTexture {
    pub fn new(device: &Device, name: Option<&str>, sampler: Arc<Sampler>, format: impl Into<RenderTargetFormat>, width: u32, height: u32) -> Self {
        let format = format.into();
        let name = name.unwrap_or("Unnamed RenderTexture");
        let size = wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size: size,
            sample_count: 1,
            mip_level_count: 1,
            format: format.into(),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &format.compatible_formats(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level: 0,
            mip_level_count: Some(1),
            format: Some(format.into()),
            aspect: wgpu::TextureAspect::All,
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        });

        return Self {
            name: name.to_string(),
            size: size,
            view: view,
            format: format,
            texture: texture,
            sampler: sampler,
        };
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.size.width = width;
        self.size.height = height;

        self.texture.destroy();
        self.texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&self.name),
            size: self.size,
            sample_count: 1,
            mip_level_count: 1,
            format: self.format.into(),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &self.format.compatible_formats(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        self.view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level: 0,
            mip_level_count: Some(1),
            format: Some(self.format.into()),
            aspect: wgpu::TextureAspect::All,
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        });
    }
}

impl std::fmt::Debug for RenderTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderTexture")
            .field("name", &self.name)
            .field("format", &self.format)
            .field("width", &self.size.width)
            .field("height", &self.size.height)
        .finish()
    }
}
