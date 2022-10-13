pub mod float;
pub mod interpolate;

use self::interpolate::Interpolate;

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
    Interpolate::compute(value.into(),
        from_start.into(), from_end.into(),
        to_start, to_end)
}

// @TODO: add test
