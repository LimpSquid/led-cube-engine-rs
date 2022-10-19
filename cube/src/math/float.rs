use ieee754::Ieee754;
use num_traits::Float;

pub trait FloatExt
where
    Self: Float + Ieee754
{
    fn safe_eq(self, rhs: Self) -> bool {
        // We need this check here since inf.next() or inf.prev() is not a number
        if self.is_infinite() && rhs.is_infinite() {
            return self == rhs; // inf != -inf
        }

        self.prev() <= rhs && self.next() >= rhs
    }

    fn safe_lt(self, rhs: Self) -> bool {
        self.next() < rhs
    }

    fn safe_gt(self, rhs: Self) -> bool {
        self.prev() > rhs
    }

    fn safe_le(self, rhs: Self) -> bool {
        self.safe_lt(rhs) || self.safe_eq(rhs)
    }

    fn safe_ge(self, rhs: Self) -> bool {
        self.safe_gt(rhs) || self.safe_eq(rhs)
    }

    fn safe_clamp(self, min: Self, max: Self) -> Self {
        if self.safe_le(min) {
            return min
        }
        if self.safe_ge(max) {
            return max
        }
        self
    }
}

impl<T: Float + Ieee754> FloatExt for T { }

#[cfg(test)]
mod tests {
    use super::FloatExt;
    use super::Ieee754;

    #[test]
    fn is_equal() {
        assert!(0f64.safe_eq(0f64));
        assert!(1.0f64.safe_eq(1.0f64.next()));
        assert!(1.0f64.safe_eq(1.0f64.prev()));
        assert!(0.0000000001f64.safe_eq(0.0000000001f64.next()));
        assert!(0.0000000001f64.safe_eq(0.0000000001f64.prev()));
        assert!(1000000000.0f64.safe_eq(1000000000.0f64.next()));
        assert!(1000000000.0f64.safe_eq(1000000000.0f64.prev()));
        assert!(!1.0f64.safe_eq(1.0f64.next().next()));
        assert!(!1.0f64.safe_eq(1.0f64.prev().prev()));
        assert!(f64::MIN.safe_eq(f64::MIN));
        assert!(f64::MAX.safe_eq(f64::MAX));
        assert!(!f64::MIN.safe_eq(f64::MAX));
        assert!(!f64::MAX.safe_eq(f64::MIN));
        assert_eq!(f64::INFINITY.safe_eq(f64::INFINITY), f64::INFINITY == f64::INFINITY);
        assert_eq!(f64::INFINITY.safe_eq(f64::NEG_INFINITY), f64::INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NEG_INFINITY.safe_eq(f64::NEG_INFINITY), f64::NEG_INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NEG_INFINITY.safe_eq(f64::INFINITY), f64::INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NAN.safe_eq(f64::NAN), f64::NAN == f64::NAN);

        assert!(0f32.safe_eq(0f32));
        assert!(1.0f32.safe_eq(1.0f32.next()));
        assert!(1.0f32.safe_eq(1.0f32.prev()));
        assert!(0.0000000001f32.safe_eq(0.0000000001f32.next()));
        assert!(0.0000000001f32.safe_eq(0.0000000001f32.prev()));
        assert!(1000000000.0f32.safe_eq(1000000000.0f32.next()));
        assert!(1000000000.0f32.safe_eq(1000000000.0f32.prev()));
        assert!(!1.0f32.safe_eq(1.0f32.next().next()));
        assert!(!1.0f32.safe_eq(1.0f32.prev().prev()));
        assert!(f32::MIN.safe_eq(f32::MIN));
        assert!(f32::MAX.safe_eq(f32::MAX));
        assert!(!f32::MIN.safe_eq(f32::MAX));
        assert!(!f32::MAX.safe_eq(f32::MIN));
        assert_eq!(f32::INFINITY.safe_eq(f32::INFINITY), f32::INFINITY == f32::INFINITY);
        assert_eq!(f32::INFINITY.safe_eq(f32::NEG_INFINITY), f32::INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NEG_INFINITY.safe_eq(f32::NEG_INFINITY), f32::NEG_INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NEG_INFINITY.safe_eq(f32::INFINITY), f32::INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NAN.safe_eq(f32::NAN), f32::NAN == f32::NAN);

        let lhs: f32 = 0.35;
        let rhs: f32 = 0.20 + 0.15;
        assert_ne!(lhs, rhs);
        assert!(lhs.safe_eq(rhs));
    }

    // TODO: add more tests
}
