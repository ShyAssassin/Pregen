use crate::Vec2;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}
