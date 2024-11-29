use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]

pub struct FrustumUniform {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect: f32,
}

impl Default for FrustumUniform {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 100.0,
            fov: 60.0,
            aspect: 1.0,
        }
    }
}

impl FrustumUniform {
    pub fn new(near: f32, far: f32, fov: f32, aspect: f32) -> Self {
        Self {
            near,
            far,
            fov,
            aspect,
        }
    }
}
