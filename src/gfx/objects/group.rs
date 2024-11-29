use std::sync::Arc;
use std::{fmt::Debug, ops::{Deref, DerefMut}};

pub trait BindGroupState: Sized {
    /// The name of the bind group
    fn get_name(&self) -> &'static str;
    /// Default initialization of the state
    fn init(device: &wgpu::Device) -> Self;
    /// Update all of the bound buffers attached to the bind group
    fn update(&mut self, queue: &wgpu::Queue);
    /// The layout which describes how data is bound in the bind group
    fn get_layout_descriptor(&self) -> wgpu::BindGroupLayoutDescriptor;
    /// Create a new bind group adhering to the layout defined in `get_layout_descriptor`
    fn create_bind_group(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup;
}

pub struct BindGroup<T: BindGroupState> {
    pub name: String,
    pub state: T,
    pub bind_group: wgpu::BindGroup,
    pub layout: Arc<wgpu::BindGroupLayout>,
}

#[profiling::all_functions]
impl<T: BindGroupState> BindGroup<T> {
    pub fn new(device: &wgpu::Device, name: Option<&str>, state: T, layout: Arc<wgpu::BindGroupLayout>) -> Self {
        let name = name.unwrap_or("Unnamed BindGroup").to_string();
        let bind_group = state.create_bind_group(&device, &layout);
        return BindGroup {
            name,
            state,
            bind_group,
            layout,
        };
    }

    /// Recreate the bind group with the current state
    pub fn recreate(&mut self, device: &wgpu::Device) {
        self.bind_group = self.state.create_bind_group(&device, &self.layout);
    }

    /// Update all of the bound buffers attached to the bind group
    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.state.update(queue);
    }

    /// Get the raw underlying bind group
    pub fn as_raw(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
}

impl<T: BindGroupState> Deref for BindGroup<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.state;
    }
}

impl<T: BindGroupState> DerefMut for BindGroup<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.state;
    }
}

impl<T: BindGroupState + Debug> Debug for BindGroup<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layout = <T as BindGroupState>::get_layout_descriptor(&self);
        f.debug_struct("BindGroup")
            .field("name", &self.name)
            .field("layout", &layout)
            .field("state", &self.state)
        .finish()
    }
}
