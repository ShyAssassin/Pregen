use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Quat<T> {
    pub x: T, pub y: T,
    pub z: T, pub w: T,
}
