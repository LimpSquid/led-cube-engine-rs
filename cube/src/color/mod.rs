pub mod blend;
pub mod util;
#[macro_use]
mod macros;

use glm::{Vector4, DVec4, Primitive};
use crate::interpolate;
use crate::math::interpolate::Interpolate;

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl<T> Into<Vector4<T>> for Color
where
    T: Primitive + From<u8>,
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
    T: Primitive + Into<f64>,
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
