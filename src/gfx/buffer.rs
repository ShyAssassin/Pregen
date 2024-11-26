macro_rules! GenericEnum {
    ($trait:ident $enum:ident $name:ident { $($variant:ident),*}) => {
        pub enum $enum {
            $($variant),*
        }
        $(impl From<$enum> for $name::$variant {
            fn from(_val: $enum) -> Self {
                return $name::$variant;
            }
        })*

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
pub struct Buffer<A: BufferAccessType> {
    access: A,
}

pub fn test() {
    use BufferAccess::*;
    let test: Buffer<Read>;
}
