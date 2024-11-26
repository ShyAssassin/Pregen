use std::{fmt::Debug, ops::RangeBounds};


pub struct IndexBuffer {
    pub name: String,
    pub count: usize,
    pub buffer: wgpu::Buffer,
    pub indices: Vec<IndexType>,
}

type IndexType = u32;
impl IndexBuffer {
    pub fn new<T: Into<IndexType>>(device: &wgpu::Device, name: Option<&str>, indices: Vec<T>) -> Self {
        let count = indices.len();
        let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;
        let name = name.unwrap_or("Unnamed Buffer").to_string();
        let indices = indices.into_iter().map(|i| i.into()).collect::<Vec<u32>>();
        let unpadded_size = (size_of::<IndexType>() * count) as wgpu::BufferAddress;
        let padded_size = ((unpadded_size + align_mask) & !align_mask).max(wgpu::COPY_BUFFER_ALIGNMENT);

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&name),
            size: padded_size,
            mapped_at_creation: true,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::INDEX,
        });
        // no bounds checking on `get_mapped_range_mut` is a bit scary
        buffer.slice(..).get_mapped_range_mut().copy_from_slice(bytemuck::cast_slice(&indices));
        buffer.unmap();

        return Self {
            name: name,
            count: count,
            buffer: buffer,
            indices: indices,
        };
    }

    pub fn slice<S: RangeBounds<u64>>(&self, range: S) -> wgpu::BufferSlice {
        return self.buffer.slice(range);
    }
}

impl Debug for IndexBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("name", &self.name)
            .field("count", &self.count)
            .field("buffer", &(&self.buffer as *const _))
            .field("indices", &(&self.indices as *const _))
        .finish()
    }
}
