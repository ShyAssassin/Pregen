use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::{Device, WgpuExt, Dynamic, Buffer};

pub struct UniformBuffer<T> {
    pub device: Device,
    generation: AtomicU64,
    phantom: PhantomData<T>,
    pub buffer: wgpu::Buffer,
    pub label: Option<String>,
}

impl<T> UniformBuffer<T> {
    pub fn size(&self) -> usize {
        return self.buffer.size() as usize;
    }

    pub fn generation(&self) -> u64 {
        return self.generation.load(Ordering::Acquire);
    }

    pub unsafe fn transmute<U>(self) -> UniformBuffer<U> {
        let this = std::mem::ManuallyDrop::new(self);
        return std::mem::transmute_copy(&*this);
    }
}

impl<T> AsRef<wgpu::Buffer> for UniformBuffer<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }
}

impl<T> AsRef<UniformBuffer<Dynamic>> for UniformBuffer<T> {
    fn as_ref(&self) -> &UniformBuffer<Dynamic> {
        unsafe { std::mem::transmute(self) }
    }
}


#[allow(unused)]
fn test(device: Device) {
    struct Position {
        pub value: [f32; 4]
    }

    // If any "attribute" is unknown it is defined at runtime
    // If known this is a compile-time constant and can be used

    // known type; unknown size
    let _: UniformBuffer<u32>;

    // unknown type; unknown size
    let _: UniformBuffer<Dynamic>;

    // known type; known size
    let _: UniformBuffer<[f32; 4]>;

    // known type; unknown size
    let _: UniformBuffer<Position>;

    // unknown type; known size
    let _: UniformBuffer<[Dynamic; 6]>;

    // known type; known size
    let _: UniformBuffer<[Position; 3]>;
}

