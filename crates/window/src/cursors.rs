// Cursor types here adhere *mostly* to the web's CSS cursor property
// https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/cursor

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub enum Cursor {
    Text, Crosshair,
    Default, Pointer,
}
