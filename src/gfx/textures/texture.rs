use super::Sampler;
use std::sync::Arc;
use wgpu::{Device, Queue};
use crate::{gfx::TextureFormat, asset::Image};

pub struct Texture {
    pub name: String,
    pub image: Arc<Image>,
    pub sampler: Arc<Sampler>,
    pub format: TextureFormat,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub fn new(device: &Device, queue: &Queue, name: Option<&str>, format: impl Into<TextureFormat>, sampler: Arc<Sampler>, image: Arc<Image>) -> Self {
        let format = format.into();
        let name = name.unwrap_or("Unnamed Texture");

        let size = wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size: size,
            sample_count: 1,
            format: format.into(),
            mip_level_count: image.mip_levels,
            dimension: wgpu::TextureDimension::D2,
            view_formats: &format.compatible_formats(),
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        for level in 0..image.mip_levels {
            let data = image.data[level as usize].clone();
            Self::write_wgpu_texture(queue, &texture, level, size, data);
        };

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(format!("{} View", name).as_str()),
            base_mip_level: 0,
            format: Some(format.into()),
            aspect: wgpu::TextureAspect::All,
            mip_level_count: Some(image.mip_levels),
            ..Default::default()
        });

        return Self {
            name: name.to_string(),
            view: view,
            image: image,
            format: format,
            texture: texture,
            sampler: sampler,
        };
    }

    fn write_wgpu_texture(queue: &Queue, texture: &wgpu::Texture, level: u32, size: wgpu::Extent3d, data: Vec<u8>) {
        let width = size.width >> level;
        let height = size.height >> level;
        if width * height * 4 != data.len() as u32 {
            panic!("Data size: `{}` does not match mip level size: `{}`", data.len(), width * height * 4);
        }

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                mip_level: level,
                texture: texture,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                rows_per_image: Some(height),
                // 4 bytes per pixel to store RGBA
                bytes_per_row: Some(4 * width),
            },
            wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
        );
    }
}

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("name", &self.name)
            .field("format", &self.format)
            .field("sampler", &self.sampler)
            .field("width", &self.image.width)
            .field("height", &self.image.height)
            .field("mip_map_levels", &self.image.mip_levels)
            .field("data", &format_args!("{} bytes", self.image.data.iter().map(|x| x.len()).sum::<usize>()))
            .field("view", &(&self.view as *const _))
            .field("texture", &(&self.texture as *const _))
        .finish()
    }
}
