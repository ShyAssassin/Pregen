use std::marker::PhantomData;
use bytemuck::{Pod, Zeroable};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::{Device, WgpuExt, Dynamic, Buffer};

pub struct StorageBuffer<T> {
    pub device: Device,
    generation: AtomicU64,
    phantom: PhantomData<T>,
    pub buffer: wgpu::Buffer,
    pub label: Option<String>,
}

impl<T> StorageBuffer<T> {
    pub fn size(&self) -> usize {
        return self.buffer.size() as usize;
    }

    pub fn generation(&self) -> u64 {
        return self.generation.load(Ordering::Acquire);
    }

    pub unsafe fn transmute<U>(self) -> StorageBuffer<U> {
        let this = std::mem::ManuallyDrop::new(self);
        return std::mem::transmute_copy(&*this);
    }
}

impl<T> AsRef<wgpu::Buffer> for StorageBuffer<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }
}

impl<T> AsRef<StorageBuffer<Dynamic>> for StorageBuffer<T> {
    fn as_ref(&self) -> &StorageBuffer<Dynamic> {
        unsafe { std::mem::transmute(self) }
    }
}


#[allow(unused)]
fn test(device: Device) {
    struct Lights {
        pub value: [f32; 4]
    }

    // If any "attribute" is unknown it is defined at runtime
    // If known this is a compile-time constant and can be used

    // known type; unknown size
    let _: StorageBuffer<f32>;

    // unknown type; unknown size
    let _: StorageBuffer<[u32; 4]>;

    // known type; unknown size
    let _: StorageBuffer<Lights>;

    // unknown type; unknown size
    let _: StorageBuffer<Dynamic>;

    // known type; known size
    let _: StorageBuffer<[Lights; 4]>;

    // unknown type; known size
    let _: StorageBuffer<[Dynamic; 6]>;

}
