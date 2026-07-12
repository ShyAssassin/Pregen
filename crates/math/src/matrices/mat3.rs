use crate::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}
