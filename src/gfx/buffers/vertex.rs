use std::{fmt::Debug, ops::RangeBounds};
use bytemuck::{Pod, Zeroable};
pub trait VertexArrayObject: where Self: Pod + Zeroable {
    const SIZE: usize = size_of::<Self>();
    const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static>;
}

pub struct VertexBuffer<T: VertexArrayObject> {
    pub name: String,
    pub data: Vec<T>,
    pub buffer: wgpu::Buffer,
}

impl<T: VertexArrayObject> VertexBuffer<T> {
    pub fn new(device: &wgpu::Device, name: Option<&str>, vertices: Vec<T>) -> Self {
        let name = name.unwrap_or("Unnamed Vertex Buffer");
        let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;
        let unpadded_size = (vertices.len() * T::SIZE) as wgpu::BufferAddress;
        let padded_size = ((unpadded_size + align_mask) & !align_mask).max(wgpu::COPY_BUFFER_ALIGNMENT);

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(name),
            size: padded_size,
            mapped_at_creation: true,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
        });
        // no bounds checking on `get_mapped_range_mut` is a bit scary
        buffer.slice(..).get_mapped_range_mut().copy_from_slice(bytemuck::cast_slice(vertices.as_slice()));
        buffer.unmap();

        return Self {
            name: name.to_string(),
            data: vertices,
            buffer: buffer,
        };
    }

    pub fn slice<S: RangeBounds<u64>>(&self, range: S) -> wgpu::BufferSlice {
        return self.buffer.slice(range);
    }
}

impl<T: VertexArrayObject> Debug for VertexBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("name", &self.name)
            .field("data", &(&self.data as *const _))
            .field("buffer", &(&self.buffer as *const _))
        .finish()
    }
}
