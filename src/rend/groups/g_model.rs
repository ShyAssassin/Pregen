use glam::Mat4;
use crate::math::Transform;
use crate::gfx::UniformBuffer;
use crate::impl_bind_group_state;

pub struct ModelBindGroup {
    pub u_model: UniformBuffer<Mat4>,
    pub u_transform: UniformBuffer<Transform>
}

impl_bind_group_state!(
    "Model Bind Group",
    ModelBindGroup,

    "Model Matrix",
    u_model: Mat4, 0,
    "Transform",
    u_transform: Transform, 1,
);
