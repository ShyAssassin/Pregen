use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use std::sync::atomic::{Ordering, AtomicU64};
use crate::{Device, WgpuExt, Dynamic, Buffer};

pub struct IndexBuffer<T> {
    pub count: usize,
    pub device: Device,
    pub capacity: usize,
    generation: AtomicU64,
    phantom: PhantomData<T>,
    pub buffer: wgpu::Buffer,
    pub label: Option<String>,
    pub format: wgpu::IndexFormat,
}

impl<T> IndexBuffer<T> {
    pub fn size(&self) -> usize {
        return self.buffer.size() as usize;
    }

    pub fn generation(&self) -> u64 {
        return self.generation.load(Ordering::Acquire);
    }

    pub unsafe fn transmute<U>(self) -> IndexBuffer<U> {
        let this = std::mem::ManuallyDrop::new(self);
        return std::mem::transmute_copy(&*this);
    }
}

impl<T> AsRef<wgpu::Buffer> for IndexBuffer<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }
}

impl<T> AsRef<IndexBuffer<Dynamic>> for IndexBuffer<T> {
    fn as_ref(&self) -> &IndexBuffer<Dynamic> {
        unsafe { std::mem::transmute(self) }
    }
}

#[allow(unused)]
fn test(device: Device) {
    // If any "attribute" is unknown it is defined at runtime
    // If known this is a compile-time constant and can be used

    // known type; unknown size
    let _: IndexBuffer<u16>;

    // unknown type; unknown size
    let _: IndexBuffer<Dynamic>;

    // known type; known size
    let _: IndexBuffer<[u32; 3]>;

    // unknown type; known size
    let _: IndexBuffer<[Dynamic; 6]>;
}
