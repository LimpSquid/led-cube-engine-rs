use super::{Color, BLACK, WHITE};
use crate::math;
use crate::math::float::FloatExt;

impl Color {
    pub fn lighter(self, factor: f64) -> Color {
        math::map(factor, 0.0, 1.0, self, WHITE)
    }

    pub fn darker(self, factor: f64) -> Color {
        math::map(factor, 0.0, 1.0, self, BLACK)
    }

    pub fn adjust_brightness(self, factor: f64) -> Color {
        if factor.safe_lt(0.5) {
            math::map(factor, 0.5, 0.0, self, BLACK)
        } else if factor.safe_gt(0.5) {
            math::map(factor, 0.5, 1.0, self, WHITE)
        } else {
            self
        }
    }

    pub fn map(self, other: Self, factor: f64) -> Self {
        math::map(factor, 0.0, 1.0, self, other)
    }

    pub fn map_alpha(self, other: Self, factor: f64) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: math::map(factor, 0.0, 1.0, self.a, other.a),
        }
    }

    pub fn map_rgb(self, other: Self, factor: f64) -> Self {
        Color {
            r: math::map(factor, 0.0, 1.0, self.r, other.r),
            g: math::map(factor, 0.0, 1.0, self.g, other.g),
            b: math::map(factor, 0.0, 1.0, self.b, other.b),
            a: self.a,
        }
    }

    pub fn opaque(mut self) -> Self {
        self.a = u8::MAX;
        self
    }

    pub fn translucent(mut self, factor: f64) -> Self {
        self.a = math::map(factor, 0.0, 1.0, u8::MIN, u8::MAX);
        self
    }

    pub fn is_opaque(self) -> bool {
        self.a == u8::MAX
    }

    pub fn is_translucent(self) -> bool {
        !self.is_opaque()
    }
}

// TODO: add test
