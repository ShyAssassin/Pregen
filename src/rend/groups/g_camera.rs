use crate::gfx::UniformBuffer;
use crate::impl_bind_group_state;
use crate::rend::uniforms::{CameraUniform, FrustumUniform};

#[derive(Debug)]
pub struct CameraBindGroup {
    pub u_camera: UniformBuffer<CameraUniform>,
    pub u_frustum: UniformBuffer<FrustumUniform>,
}

impl_bind_group_state!(
    "Camera Bind Group",
    CameraBindGroup,

    "Camera Uniform",
    u_camera: CameraUniform, 0,
    "Camera Frustum",
    u_frustum: FrustumUniform, 1,
);
