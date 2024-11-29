use glam::{Vec2, Vec3};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct MouseUniform {
    pub state: Vec3, // X = left, right = Y, middle = Z
    pub position: Vec2,
}

impl Default for MouseUniform {
    fn default() -> Self {
        return Self {
            state: Vec3::ZERO,
            position: Vec2::ZERO,
        }
    }
}
