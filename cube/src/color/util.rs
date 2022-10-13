use super::{Color, BLACK, WHITE};
use crate::math;
use crate::math::float::FloatExt;

impl Color {
    pub fn lighter(self, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        math::map(factor, 0.0, 1.0, self, WHITE)
    }

    pub fn darker(self, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        math::map(factor, 0.0, 1.0, self, BLACK)
    }

    pub fn adjust_brightness(self, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);

        if factor.cmp_lt(0.5) {
            return math::map(factor, 0.5, 0.0, self, BLACK);
        }
        if factor.cmp_gt(0.5) {
            return math::map(factor, 0.5, 1.0, self, WHITE);
        }
        self
    }
}

// @TODO: add test
