use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Deg<T>(pub T);
