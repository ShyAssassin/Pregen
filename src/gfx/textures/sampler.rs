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

    pub fn into_wgpu(self, name: &str) -> wgpu::SamplerDescriptor<'_> {
        return wgpu::SamplerDescriptor {
            label: Some(name),
            min_filter: self.min_filter,
            mag_filter: self.mag_filter,
            address_mode_u: self.address,
            address_mode_v: self.address,
            address_mode_w: self.address,
            mipmap_filter: self.mip_filter,
            ..Default::default()
        };
    }
}

pub struct Sampler {
    pub name: String,
    pub mode: SamplerMode,
    pub sampler: wgpu::Sampler,
}

impl Sampler {
    pub fn new(device: &wgpu::Device, name: Option<&str>, mode: SamplerMode) -> Self {
        let name = name.unwrap_or("Unnamed Sampler");
        let sampler = device.create_sampler(&mode.into_wgpu(name));

        return Self {
            name: name.to_string(),
            sampler: sampler,
            mode: mode,
        };
    }

    pub fn as_raw(&self) -> &wgpu::Sampler {
        return &self.sampler;
    }
}

impl std::fmt::Debug for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sampler")
            .field("name", &self.name)
            .field("mode", &self.mode)
            .field("sampler", &(&self.sampler as *const _))
        .finish()
    }
}
