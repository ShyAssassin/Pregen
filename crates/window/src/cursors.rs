// Cursor types here adhere *mostly* to the web's CSS cursor properties
// https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/cursor

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub enum Cursor {
    Default, Pointer,
    Hidden, Text, Crosshair,
}

impl Default for Cursor {
    fn default() -> Self {
        return Cursor::Default;
    }
}
