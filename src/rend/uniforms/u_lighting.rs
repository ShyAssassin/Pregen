use glam::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct LightingUniform {
    pub color: Vec3,
    pub _padding: f32,
    pub position: Vec3,
    pub intensity: f32,
}

impl Default for LightingUniform {
    fn default() -> Self {
        Self {
            color: Vec3::ZERO,
            position: Vec3::ZERO,
            intensity: 0.0,
            _padding: 0.0,
        }
    }
}
