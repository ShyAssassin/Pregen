use glam::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct MeshUniform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}
