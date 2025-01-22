use crate::Device;

// TODO: consider moving this somewhere else
pub type FilterMode = wgpu::FilterMode;
pub type AddressMode = wgpu::AddressMode;

#[derive(Debug, Clone, Copy)]
#[derive(Hash, Eq, PartialEq)]
pub struct SamplerMode {
    pub address: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mip_filter: FilterMode,
}

impl SamplerMode {
    pub const REPEAT: Self = Self {
        address: AddressMode::Repeat,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Nearest,
        mip_filter: FilterMode::Nearest,
    };

    pub const CLAMP: Self = Self {
        address: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Nearest,
        mip_filter: FilterMode::Nearest,
    };
}

impl From<SamplerMode> for wgpu::SamplerDescriptor<'_> {
    fn from(mode: SamplerMode) -> Self {
        return Self {
            label: None,
            min_filter: mode.min_filter.into(),
            mag_filter: mode.mag_filter.into(),
            mipmap_filter: mode.mip_filter.into(),
            address_mode_u: mode.address.into(),
            address_mode_v: mode.address.into(),
            address_mode_w: mode.address.into(),
            ..Default::default()
        };
    }
}

pub struct Sampler {
    pub mode: SamplerMode,
    pub name: Option<String>,
    pub sampler: wgpu::Sampler,
}

impl Sampler {
    pub fn new(device: &Device, name: Option<String>, mode: SamplerMode) -> Self {
        let name = name.unwrap_or("Unnamed Sampler".into());
        todo!()
    }
}

impl std::fmt::Debug for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sampler")
            .field("name", &self.name)
            .field("sampler", &(&self.sampler as *const _))
        .finish()
    }
}
