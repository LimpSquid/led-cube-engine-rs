pub mod blend;
pub mod util;
#[macro_use]
mod macros;

use glm::{Vector4, DVec4, Primitive};
use lazy_static::lazy_static;
use crate::interpolate;
use crate::math::interpolate::Interpolate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        (self.r as u32) << 24 |
        (self.g as u32) << 16 |
        (self.b as u32) <<  8 |
        (self.a as u32)
    }
}

impl From<u32> for Color {
    fn from(rgba: u32) -> Self {
        Color {
            r: (rgba >> 24) as u8,
            g: (rgba >> 16) as u8,
            b: (rgba >>  8) as u8,
            a: (rgba      ) as u8,
        }
    }
}

impl<T> Into<Vector4<T>> for Color
where
    T: Copy + Primitive + From<u8>,
{
    fn into(self) -> Vector4<T> {
        Vector4 {
            x: self.r.into(),
            y: self.g.into(),
            z: self.b.into(),
            w: self.a.into(),
        }
    }
}

impl<T> From<Vector4<T>> for Color
where
    T: Copy + Primitive + Into<f64>,
{
    fn from(v: Vector4<T>) -> Self {
        let x: f64 = v.x.into();
        let y: f64 = v.y.into();
        let z: f64 = v.z.into();
        let w: f64 = v.w.into();

        Color {
            r: x.round() as u8,
            g: y.round() as u8,
            b: z.round() as u8,
            a: w.round() as u8,
        }
    }
}

impl Interpolate for Color {
    fn interpolate(x: f64, x_start: f64, x_end: f64, y_start: Self, y_end: Self) -> Self {
        let y_start: DVec4 = y_start.into();
        let y_end: DVec4 = y_end.into();

        interpolate!(x, x_start, x_end, y_start, y_end).into()
    }
}

// TODO: move to separate file?
macro_rules! count {
    () => (0usize);
    ($x:tt $($xs:tt)*) => (1usize + count!($($xs)*));
}

macro_rules! impl_named_colors {
    ($($name:ident = $color:expr),+) => {
        lazy_static! {
            static ref NAMED_COLORS: [(String, Color); count!($($name)+)] = [
                $((String::from(stringify!($name)).to_lowercase(), $name),)+
            ];
        }
        $(pub const $name: Color = $color;)+
    }
}

impl_named_colors!
{
    // Translucent colors
    TRANSPARENT = translucent!("00000000"),

    // Opaque colors
    BLACK       = opaque!("000000"),
    WHITE       = opaque!("ffffff"),
    RED         = opaque!("ff0000"),
    GREEN       = opaque!("00ff00"),
    BLUE        = opaque!("0000ff"),
    CYAN        = opaque!("00ffff"),
    MAGENTA     = opaque!("ff00ff"),
    YELLOW      = opaque!("ffff00"),
    ORANGE      = opaque!("ff8000"),
    PINK        = opaque!("ff60ff"),
    STEEL_BLUE  = opaque!("468bb4")
}
