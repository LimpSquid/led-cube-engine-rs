pub mod float;
pub use float::FloatCmpExt;

use std::ops::{Add, Div, Mul, Sub};

pub fn map<TIn, TOut>(
    value: TIn,
    from_start: TIn,
    from_end: TIn,
    to_start: TOut,
    to_end: TOut,
) -> TOut
where
    TIn: Copy + Sub<Output = TIn>,
    TOut: Copy
        + Add<Output = TOut>
        + Sub<Output = TOut>
        + Mul<TIn, Output = TOut>
        + Div<TIn, Output = TOut>,
{
    to_start + (to_end - to_start) * (value - from_start) / (from_end - from_start)
}

// @TODO: add test
