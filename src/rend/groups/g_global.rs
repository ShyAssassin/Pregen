use glam::Vec2;
use crate::gfx::UniformBuffer;
use crate::impl_bind_group_state;
use crate::rend::{KeyboardUniform, MouseUniform, CoordinatesUniform, LightingUniform};

pub struct GlobalBindGroup {
    pub u_time: UniformBuffer<f32>,
    pub u_frame: UniformBuffer<u32>,
    pub u_rand_seed: UniformBuffer<u32>,
    pub u_resolution: UniformBuffer<Vec2>,
    pub u_mouse: UniformBuffer<MouseUniform>,
    pub u_keyboard: UniformBuffer<KeyboardUniform>,
    pub u_lights: UniformBuffer<[LightingUniform; 8]>,
    pub u_coordinates: UniformBuffer<CoordinatesUniform>,
}

impl_bind_group_state!(
    "Global Frame Bind Group",
    GlobalBindGroup,

    "Game Time",
    u_time: f32, 0,
    "Frame Count",
    u_frame: u32, 1,
    "Random Seed",
    u_rand_seed: u32, 2,
    "Window Resolution",
    u_resolution: Vec2, 3,
    "Mouse State",
    u_mouse: MouseUniform, 4,
    "Keyboard State",
    u_keyboard: KeyboardUniform, 5,
    "Coordinate System",
    u_coordinates: CoordinatesUniform, 6,
    "Lighting",
    u_lights: [LightingUniform; 8], 7
);
