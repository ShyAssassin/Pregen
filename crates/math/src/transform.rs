use crate::{Quat, Vec3};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Transform {
    pub scale: Vec3<f32>,
    pub rotation: Quat<f32>,
    pub translation: Vec3<f32>,
}
