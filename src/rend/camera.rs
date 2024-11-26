use std::f32::consts::PI;
use crate::math::Transform;
use glam::{Mat3, Mat4, Quat, Vec3};
use crate::gfx::{BindGroup, RenderContext};
use super::{CameraBindGroup, CameraUniform, FrustumUniform};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct CameraDescriptor {
    pub fov: f32,
    pub zoom: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            fov: 90.0,
            zoom: 1.00,
            z_near: 0.1,
            z_far: 100.0,
            aspect_ratio: 1.0,
            transform: Transform::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub enum CameraProjection {
    Perspective,
    Orthographic,
}

#[derive(Debug)]
pub struct Camera {
    pub fov: f32,
    pub zoom: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub projection: CameraProjection,
    pub group: BindGroup<CameraBindGroup>,
}

impl Camera {
    pub fn new(ctx: &mut RenderContext, projection: CameraProjection, desc: CameraDescriptor) -> Self {
        let group = ctx.create_bind_group::<CameraBindGroup>(None);
        let mut camera = Self {
           group: group,
            fov: desc.fov,
            zoom: desc.zoom,
            z_far: desc.z_far,
            z_near: desc.z_near,
            projection: projection,
            transform: desc.transform,
            aspect_ratio: desc.aspect_ratio,
        };
        camera.update(&ctx.queue);
        return camera;
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        let view = Mat4::from_quat(self.transform.rotation.inverse()) * Mat4::from_translation(-self.transform.translation);
        let uniform = match self.projection {
            CameraProjection::Perspective => self.update_perspective(view),
            CameraProjection::Orthographic => self.update_orthographic(view),
        };
        self.group.u_camera.set(uniform);
        self.group.u_frustum.set(FrustumUniform::new(self.z_near, self.z_far, self.fov, self.aspect_ratio));
        self.group.update(&queue);
    }

    fn update_perspective(&mut self, view: Mat4) -> CameraUniform {
        let projection = Mat4::perspective_lh(self.fov * (PI/180.0), self.aspect_ratio, self.z_near, self.z_far);
        return CameraUniform::from_transform(self.transform, view, projection)
    }

    fn update_orthographic(&mut self, view: Mat4) -> CameraUniform {
        let left = -self.aspect_ratio * self.zoom;
        let right = self.aspect_ratio * self.zoom;
        let projection = Mat4::orthographic_lh(left, right, -self.zoom, self.zoom, self.z_near, self.z_far);
        return CameraUniform::from_transform(self.transform, view, projection);
    }

    pub fn look_at(&mut self, target: Vec3) {
        let forward = (target - self.transform.translation).normalize();
        let right = Vec3::Y.cross(forward).normalize();
        let up = forward.cross(right).normalize();

        self.transform.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
    }
}
