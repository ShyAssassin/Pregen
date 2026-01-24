use gfx_macros::arcanize;
use crate::{Device, WgpuExt};

#[derive(Debug, Clone, Copy)]
#[derive(Hash, Eq, PartialEq)]
pub struct SamplerMode {
    pub address: wgpu::AddressMode,
    pub mag_filter: wgpu::FilterMode,
    pub min_filter: wgpu::FilterMode,
    pub mip_filter: wgpu::FilterMode,
}

impl SamplerMode {
    pub const REPEAT: Self = Self {
        address: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mip_filter: wgpu::FilterMode::Nearest,
    };

    pub const CLAMP: Self = Self {
        address: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mip_filter: wgpu::FilterMode::Nearest,
    };
}

#[arcanize]
pub struct Sampler {
    pub device: Device,
    pub mode: SamplerMode,
    pub label: Option<String>,
    pub sampler: wgpu::Sampler,
}

impl Sampler {
    pub fn new(device: &Device, label: Option<String>, mode: SamplerMode) -> Self {
        let sampler = device.wgpu_create_sampler(&wgpu::SamplerDescriptor {
            label: label.as_deref(),
            min_filter: mode.min_filter,
            mag_filter: mode.mag_filter,
            address_mode_u: mode.address,
            address_mode_v: mode.address,
            address_mode_w: mode.address,
            mipmap_filter: mode.mip_filter,
            ..Default::default()
        });

        return SamplerInner {
            mode: mode,
            label: label,
            sampler: sampler,
            device: device.clone(),
        }.into();
    }
}
