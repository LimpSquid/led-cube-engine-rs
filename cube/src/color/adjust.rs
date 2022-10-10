
use super::Color;

impl Color {
    pub fn lighter(&self, factor: f32) -> Color {
        let _factor = factor.clamp(0.0, 1.0);

        Color::default() // @TODO: implement math map function to map factor to new color
    }

    pub fn darker(&self, factor: f32) -> Color {
        let _factor = factor.clamp(0.0, 1.0);

        Color::default() // @TODO: implement math map function to map factor to new color
    }
}
