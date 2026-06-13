use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use std::sync::atomic::{Ordering, AtomicU64};
use crate::{Device, WgpuExt, Dynamic, Buffer};

pub struct VertexBuffer<T> {
    pub count: usize,
    pub stride: usize,
    pub device: Device,
    pub capacity: usize,
    generation: AtomicU64,
    phantom: PhantomData<T>,
    pub buffer: wgpu::Buffer,
    pub label: Option<String>,
}

impl<T> VertexBuffer<T> {
    pub fn size(&self) -> usize {
        return self.buffer.size() as usize;
    }

    pub fn generation(&self) -> u64 {
        return self.generation.load(Ordering::Acquire);
    }

    pub unsafe fn transmute<U>(self) -> VertexBuffer<U> {
        let this = std::mem::ManuallyDrop::new(self);
        return std::mem::transmute_copy(&*this);
    }
}

impl<T> AsRef<wgpu::Buffer> for VertexBuffer<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }
}

impl<T> AsRef<VertexBuffer<Dynamic>> for VertexBuffer<T> {
    fn as_ref(&self) -> &VertexBuffer<Dynamic> {
        unsafe { std::mem::transmute(self) }
    }
}


#[allow(unused)]
fn test(device: Device) {
    struct Vertex {
        pub position: [u32; 3]
    }

    // If any "attribute" is unknown it is defined at runtime
    // If known this is a compile-time constant and can be used

    // known type; unknown size
    let _: VertexBuffer<Vertex>;

    // unknown type; unknown size
    let _: VertexBuffer<Dynamic>;

    // known type; known size
    let _: VertexBuffer<[Vertex; 3]>;

    // known type; unknown size
    let _: VertexBuffer<[Dynamic; 6]>;
}
