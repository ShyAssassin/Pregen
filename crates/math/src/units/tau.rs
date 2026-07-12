use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Tau<T>(pub T);
