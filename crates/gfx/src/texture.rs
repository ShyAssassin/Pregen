pub struct Texture {
    pub name: Option<String>,
    pub texture: wgpu::Texture,
    pub group: Option<wgpu::BindGroup>,
    pub view: Option<wgpu::TextureView>,
}

impl Texture {
    pub fn new(name: Option<String>) -> Self {
        let name = name.unwrap_or("Unnamed Texture".into());
        todo!()
    }
}

macro_rules! GenericEnum {
    ($trait:ident $enum:ident $name:ident { $($variant:ident),*}) => {
        pub enum $enum {
            $($variant),*
        }
        pub trait $trait {}
        mod $name {
            use super::{$trait, $enum};
            $(pub struct $variant;)*
            $(impl $trait for $variant{})*
            $(impl From<$variant> for $enum {
                fn from(_val: $variant) -> Self {
                    return $enum::$variant;
                }
            })*
        }
    }
}

GenericEnum!(
    BufferAccessType
    BufferAccessTypes
    BufferAccess {
        Read,
        Write,
        RedWrite
    }
);

pub struct Buffer<A = BufferAccess::Read> {
    access: A,
}

pub fn test() {
    use BufferAccess::*;
    let test: Buffer<Write>;
}
