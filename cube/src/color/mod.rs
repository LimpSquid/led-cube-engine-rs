pub mod blend;
pub mod adjust;
#[macro_use]
mod macros;

#[derive(Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const TRANSPARENT: Color = translucent!("00000000");
pub const BLACK: Color = opaque!("000000");
pub const WHITE: Color = opaque!("ffffff");
pub const RED: Color = opaque!("ff0000");
pub const GREEN: Color = opaque!("00ff00");
pub const BLUE: Color = opaque!("0000ff");
pub const CYAN: Color = opaque!("00ffff");
pub const MAGENTA: Color = opaque!("ff00ff");
pub const YELLOW: Color = opaque!("ffff00");
pub const ORANGE: Color = opaque!("ff8000");
pub const PINK: Color = opaque!("ff60ff");
pub const STEEL_BLUE: Color = opaque!("468bb4");
