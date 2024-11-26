use bytemuck::{Pod, Zeroable};
use std::{fmt::Debug, ops::{Deref, DerefMut}};

pub trait Uniform: Pod + Zeroable + PartialEq + Default {}
impl<T: Pod + Zeroable + PartialEq + Default> Uniform for T {}

pub struct UniformBuffer<T: Uniform> {
    pub name: String,
    pub data: T,
    pub updated: bool,
    pub buffer: wgpu::Buffer,
}

#[profiling::all_functions]
impl<T: Uniform> UniformBuffer<T> {
    // TODO: figure out how to make this work when T is wrapped in a Vec<_>
    // dont think i can use bytemuck::cast_slice(&[data]) in this case since it is trait bound by NonUninit
    // a niave but functional aproach would be to create a custom vec struct for gpu-cpu shared data, i really miss C++ right now
    // we could also create a seperate type specifically for this use case, but that would be a lot of boilerplate
    // either way this is kinda cancer, but it is what it is
    pub fn new(device: &wgpu::Device, name: Option<&str>, data: T) -> Self {
        let name = name.unwrap_or("Unnamed Buffer");
        let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;
        let unpadded_size = size_of::<T>() as wgpu::BufferAddress;
        let padded_size = ((unpadded_size + align_mask) & !align_mask).max(wgpu::COPY_BUFFER_ALIGNMENT);

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(name),
            size: padded_size,
            mapped_at_creation: true,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM
        });
        // no bounds checking on `get_mapped_range_mut` is a bit scary
        buffer.slice(..).get_mapped_range_mut().copy_from_slice(bytemuck::cast_slice(&[data]));
        buffer.unmap();

        Self {
            name: name.to_string(),
            updated: false,
            buffer: buffer,
            data: data,
        }
    }

    pub fn set(&mut self, data: T) {
        if self.data != data {
            self.data = data;
            self.updated = true;
        }
    }

    pub fn write_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]));
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.updated == true {
            self.updated = false;
            self.write_buffer(queue);
        }
    }

    pub fn as_entire_binding(&self) -> wgpu::BindingResource<'_> {
        return self.buffer.as_entire_binding();
    }
}

impl<T: Uniform> Deref for UniformBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}

impl<T: Uniform> DerefMut for UniformBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.updated = true;
        return &mut self.data;
    }
}

impl<T: Uniform + Debug> Debug for UniformBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("name", &self.name)
            .field("updated", &self.updated)
            .field("data", &self.data)
        .finish()
    }
}
