use glam::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct CoordinatesUniform {
    /// The world space forward vector
    pub forward: Vec3,
    /// The world space right vector
    pub right: Vec3,
    /// The world space up vector
    pub up: Vec3,
}

impl Default for CoordinatesUniform {
    fn default() -> Self {
        Self {
            forward: Vec3::new(0.0, 0.0, 1.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
