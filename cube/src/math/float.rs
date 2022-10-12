use num::Float;

pub trait FloatExt : Float {
    fn pow2(self) -> Self;
}

impl FloatExt for f32 {
    fn pow2(self) -> Self {
        self.powf(2.0)
    }
}

impl FloatExt for f64 {
    fn pow2(self) -> Self {
        self.powf(2.0)
    }
}

pub trait FloatCmpExt
where
    Self: FloatExt
{
    fn cmp_eq(self, rhs: Self) -> bool {
        // Epsilon is proportional to log base 2 of the value
        // More info: https://stackoverflow.com/a/35536839
        (self - rhs).abs() < Self::epsilon() * self.abs().max(rhs.abs()).log2().floor().pow2()
    }

    fn cmp_lt(self, rhs: Self) -> bool {
        (rhs - self) > Self::epsilon() * self.abs().max(rhs.abs()).log2().floor().pow2()
    }

    fn cmp_gt(self, rhs: Self) -> bool {
        (self - rhs) > Self::epsilon() * self.abs().max(rhs.abs()).log2().floor().pow2()
    }

    fn cmp_le(self, rhs: Self) -> bool {
        self.cmp_lt(rhs) || self.cmp_eq(rhs)
    }

    fn cmp_ge(self, rhs: Self) -> bool {
        self.cmp_gt(rhs) || self.cmp_eq(rhs)
    }
}

impl<T: FloatExt> FloatCmpExt for T { }

// @TODO: add test
