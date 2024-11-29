use glam::{Mat4, Vec4, Vec3};
use bytemuck::{Pod, Zeroable};

use crate::math::Transform;

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct CameraUniform { // Vec4::w is only used for padding, probably a bad idea
    pub view: Mat4,
    pub position: Vec4,
    pub direction: Vec4,
    pub projection: Mat4,
    pub view_projection: Mat4,
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view: Mat4::IDENTITY,
            position: Vec4::ZERO,
            direction: Vec4::ZERO,
            projection: Mat4::IDENTITY,
            view_projection: Mat4::IDENTITY,
        }
    }
}

impl CameraUniform {
    pub fn new(position: Vec3, direction: Vec3, view: Mat4, proj: Mat4) -> Self {
        Self {
            view: view,
            projection: proj,
            view_projection: proj * view,
            position: position.extend(0.0),
            direction: direction.extend(0.0),
        }
    }

    pub fn from_transform(transform: Transform, view: Mat4, proj: Mat4) -> Self {
        let position = transform.translation;
        let direction = transform.rotation * Vec3::Z;
        return Self::new(position, direction, view, proj);
    }
}
