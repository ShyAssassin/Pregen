use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Clone, Copy, PartialEq)]
pub struct KeyboardUniform {
    pub _padding: [f32; 4],
}

impl Default for KeyboardUniform {
    fn default() -> Self {
        return Self {
            _padding: [0.0; 4],
        }
    }
}
