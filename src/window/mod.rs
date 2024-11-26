// STOP YELLING AT ME, ITS DEAD CAUSE IT ISNT USED YET
#![allow(dead_code)]

mod event;
mod window;
mod backends;

pub use window::Window;
pub use backends::WindowBackend;
pub use event::{WindowEvent, Action, Key, MouseButton};
