use glam::{Vec3, Quat};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    __padding1: f32,
    pub scale: Vec3,
    pub rotation: Quat,
    pub translation: Vec3,
    __padding2: f32,
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            scale: scale,
            rotation: rotation,
            translation: translation,
            __padding1: f32::default(),
            __padding2: f32::default(),
        }
    }

    pub fn new_with_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn new_with_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }

    pub fn new_with_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::default(), Quat::default(), Vec3::splat(1.0))
    }
}
