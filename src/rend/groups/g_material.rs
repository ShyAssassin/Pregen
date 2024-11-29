use std::sync::Arc;
use crate::gfx::BindGroupState;
use crate::gfx::{Sampler, Texture};

pub struct MaterialBindGroup {
    pub albedo: Arc<Texture>,
    pub albedo_sampler: Arc<Sampler>,
    pub normal: Arc<Texture>,
    pub normal_sampler: Arc<Sampler>,
    pub ambient: Arc<Texture>,
    pub ambient_sampler: Arc<Sampler>,
}

impl MaterialBindGroup {
    pub fn new(albedo: Arc<Texture>, normal: Arc<Texture>, ambient: Arc<Texture>) -> Self {
        MaterialBindGroup {
            albedo: albedo.clone(),
            albedo_sampler: albedo.sampler.clone(),
            normal: normal.clone(),
            normal_sampler: normal.sampler.clone(),
            ambient: ambient.clone(),
            ambient_sampler: ambient.sampler.clone(),
        }
    }
}

impl BindGroupState for MaterialBindGroup {
    fn init(_device: &wgpu::Device) -> Self {
        panic!("MaterialBindGroup requires a pre init state!");
    }

    fn get_name(&self) -> &'static str {
        return "Material Bind Group";
    }

    fn create_bind_group(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        return device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.albedo.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.albedo_sampler.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&self.normal.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&self.normal_sampler.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&self.ambient.view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&self.ambient_sampler.sampler),
                },
            ],
            label: Some("MaterialBindGroup"),
        });
    }

    fn get_layout_descriptor(&self) -> wgpu::BindGroupLayoutDescriptor {
        return wgpu::BindGroupLayoutDescriptor {
            label: Some("MaterialBindGroup"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        };
    }

    fn update(&mut self, _queue: &wgpu::Queue) {}
}
