use bytemuck::{Pod, Zeroable};

#[repr(C)]
pub struct Rad<T>(pub T);
