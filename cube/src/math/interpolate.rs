pub trait Interpolate
where
    Self: Copy
{
    fn compute(x: f64, x_start: f64, x_end: f64, y_start: Self, y_end: Self) -> Self;
}

#[macro_export]
macro_rules! interpolate {
    ($x:ident, $x_start:ident, $x_end:ident, $y_start:ident, $y_end:ident) => {
        $y_start + ($y_end - $y_start) * ($x - $x_start) / ($x_end - $x_start)
    }
}

macro_rules! impl_interpolate {
    ($($t: ty),+) => {
        $(
            impl Interpolate for $t {
                fn compute(x: f64, x_start: f64, x_end: f64, y_start: Self, y_end: Self) -> Self {
                    let y_start: f64 = y_start.into();
                    let y_end: f64 = y_end.into();
                    interpolate!(x, x_start, x_end, y_start, y_end) as $t
                }
            }
        )+
    }
}

impl_interpolate! { f64, f32 }
impl_interpolate! { i32, i16, i8 }
impl_interpolate! { u32, u16, u8 }
