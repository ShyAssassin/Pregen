// TODO: should this crate contain a context for state management?
// TODO: Should the raw gfx object be aracanised or should it the use object?
// TODO: if there is a context / device should that handle object creation instead of the object its self?
// TODO: instead of this lib handling that might be better for rend to rexport the gfx objects wrapped in a arc?

mod shader;
pub use shader::Shader;
