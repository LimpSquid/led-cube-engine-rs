use ieee754::Ieee754;
use num_traits::Float;

pub trait FloatExt
where
    Self: Float + Ieee754
{
    fn eqf(self, rhs: Self) -> bool {
        // We need this check here since inf.next() or inf.prev() is not a number
        if self.is_infinite() && rhs.is_infinite() {
            self == rhs // inf != -inf
        } else {
            self.prev() <= rhs && self.next() >= rhs
        }
    }

    fn ltf(self, rhs: Self) -> bool {
        self.next() < rhs
    }

    fn gtf(self, rhs: Self) -> bool {
        self.prev() > rhs
    }

    fn lef(self, rhs: Self) -> bool {
        self.ltf(rhs) || self.eqf(rhs)
    }

    fn gef(self, rhs: Self) -> bool {
        self.gtf(rhs) || self.eqf(rhs)
    }

    fn clipf(self, start: Self, end: Self) -> Self {
        let x = start.ltf(end);
        let min = if x { start } else { end   };
        let max = if x { end   } else { start };

        if self.ltf(min) {
            min
        } else if self.gtf(max) {
            max
        } else {
            self
        }
    }
}

impl<T: Float + Ieee754> FloatExt for T { }

#[cfg(test)]
mod tests {
    use super::FloatExt;
    use super::Ieee754;

    #[test]
    fn is_equal() {
        assert!(0f64.eqf(0f64));
        assert!(1.0f64.eqf(1.0f64.next()));
        assert!(1.0f64.eqf(1.0f64.prev()));
        assert!(0.0000000001f64.eqf(0.0000000001f64.next()));
        assert!(0.0000000001f64.eqf(0.0000000001f64.prev()));
        assert!(1000000000.0f64.eqf(1000000000.0f64.next()));
        assert!(1000000000.0f64.eqf(1000000000.0f64.prev()));
        assert!(!1.0f64.eqf(1.0f64.next().next()));
        assert!(!1.0f64.eqf(1.0f64.prev().prev()));
        assert!(f64::MIN.eqf(f64::MIN));
        assert!(f64::MAX.eqf(f64::MAX));
        assert!(!f64::MIN.eqf(f64::MAX));
        assert!(!f64::MAX.eqf(f64::MIN));
        assert_eq!(f64::INFINITY.eqf(f64::INFINITY), f64::INFINITY == f64::INFINITY);
        assert_eq!(f64::INFINITY.eqf(f64::NEG_INFINITY), f64::INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NEG_INFINITY.eqf(f64::NEG_INFINITY), f64::NEG_INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NEG_INFINITY.eqf(f64::INFINITY), f64::INFINITY == f64::NEG_INFINITY);
        assert_eq!(f64::NAN.eqf(f64::NAN), f64::NAN == f64::NAN);

        assert!(0f32.eqf(0f32));
        assert!(1.0f32.eqf(1.0f32.next()));
        assert!(1.0f32.eqf(1.0f32.prev()));
        assert!(0.0000000001f32.eqf(0.0000000001f32.next()));
        assert!(0.0000000001f32.eqf(0.0000000001f32.prev()));
        assert!(1000000000.0f32.eqf(1000000000.0f32.next()));
        assert!(1000000000.0f32.eqf(1000000000.0f32.prev()));
        assert!(!1.0f32.eqf(1.0f32.next().next()));
        assert!(!1.0f32.eqf(1.0f32.prev().prev()));
        assert!(f32::MIN.eqf(f32::MIN));
        assert!(f32::MAX.eqf(f32::MAX));
        assert!(!f32::MIN.eqf(f32::MAX));
        assert!(!f32::MAX.eqf(f32::MIN));
        assert_eq!(f32::INFINITY.eqf(f32::INFINITY), f32::INFINITY == f32::INFINITY);
        assert_eq!(f32::INFINITY.eqf(f32::NEG_INFINITY), f32::INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NEG_INFINITY.eqf(f32::NEG_INFINITY), f32::NEG_INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NEG_INFINITY.eqf(f32::INFINITY), f32::INFINITY == f32::NEG_INFINITY);
        assert_eq!(f32::NAN.eqf(f32::NAN), f32::NAN == f32::NAN);

        let lhs: f32 = 0.35;
        let rhs: f32 = 0.20 + 0.15;
        assert_ne!(lhs, rhs);
        assert!(lhs.eqf(rhs));
    }

    // TODO: add more tests
}
