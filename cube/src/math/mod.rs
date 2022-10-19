pub mod float;
pub mod interpolate;

use interpolate::Interpolate;
use float::FloatExt;

pub fn map<TIn, TOut>(
    value: TIn,
    from_start: TIn,
    from_end: TIn,
    to_start: TOut,
    to_end: TOut,
) -> TOut
where
    TIn: Copy + Into<f64>,
    TOut: Interpolate
{
    let from_start: f64 = from_start.into();
    let from_end: f64 = from_end.into();
    let value: f64 = value.into();

    TOut::interpolate(value.clipf(from_start, from_end),
        from_start, from_end,
        to_start, to_end)
}

// TODO: add test
