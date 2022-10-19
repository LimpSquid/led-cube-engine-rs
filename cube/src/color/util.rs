use super::{Color, NAMED_COLORS, BLACK, WHITE};
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
        if factor.ltf(0.5) {
            math::map(factor, 0.5, 0.0, self, BLACK)
        } else if factor.gtf(0.5) {
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
            a: math::map(factor, 0.0, 1.0, self.a as f64, other.a as f64).round() as u8,
        }
    }

    pub fn map_rgb(self, other: Self, factor: f64) -> Self {
        Color {
            r: math::map(factor, 0.0, 1.0, self.r as f64, other.r as f64).round() as u8,
            g: math::map(factor, 0.0, 1.0, self.g as f64, other.g as f64).round() as u8,
            b: math::map(factor, 0.0, 1.0, self.b as f64, other.b as f64).round() as u8,
            a: self.a,
        }
    }

    pub fn opaque(mut self) -> Self {
        self.a = u8::MAX;
        self
    }

    pub fn translucent(mut self, factor: f64) -> Self {
        self.a = math::map(factor, 0.0, 1.0, u8::MAX, u8::MIN);
        self
    }

    pub fn is_opaque(self) -> bool {
        self.a == u8::MAX
    }

    pub fn is_translucent(self) -> bool {
        self.a != u8::MAX
    }

    pub fn from_str(src: &str) -> Result<Self, String> {
        if src.starts_with('#') {
            u32::from_str_radix(&src[1..], 16)
                .map(|v| v.into())
                .map_err(|e| e.to_string())
        } else {
            NAMED_COLORS.iter()
                .find(|t| t.0 == src)
                .map(|t| t.1)
                .ok_or(String::from(format!("unknown color '{}'", src)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::color::{RED, GREEN, BLUE, WHITE, BLACK, TRANSPARENT};

    #[test]
    fn color_mapping() {
        assert_eq!(RED.lighter(1.0), WHITE);
        assert_eq!(RED.lighter(100.0), WHITE);
        assert_eq!(RED.lighter(0.0), RED);
        assert_eq!(RED.lighter(-100.0), RED);
        assert_eq!(RED.lighter(0.5), Color{r: 255, g: 128, b: 128, a: 255});
        assert_eq!(TRANSPARENT.lighter(0.5), Color{r: 128, g: 128, b: 128, a: 128});

        assert_eq!(RED.darker(1.0), BLACK);
        assert_eq!(RED.darker(100.0), BLACK);
        assert_eq!(RED.darker(0.0), RED);
        assert_eq!(RED.darker(-100.0), RED);
        assert_eq!(RED.darker(0.5), Color{r: 128, g: 0, b: 0, a: 255});
        assert_eq!(TRANSPARENT.darker(0.5), Color{r: 0, g: 0, b: 0, a: 128});

        assert_eq!(RED.adjust_brightness(0.5), RED);
        assert_eq!(RED.adjust_brightness(0.25), RED.darker(0.5));
        assert_eq!(RED.adjust_brightness(0.75), RED.lighter(0.5));
        assert_eq!(RED.adjust_brightness(1.0), WHITE);
        assert_eq!(RED.adjust_brightness(0.0), BLACK);
        assert_eq!(RED.adjust_brightness(100.0), WHITE);
        assert_eq!(RED.adjust_brightness(-100.0), BLACK);

        assert_eq!(RED.map(GREEN, 1.0), GREEN);
        assert_eq!(RED.map(GREEN, 0.0), RED);
        assert_eq!(RED.map(GREEN, 100.0), GREEN);
        assert_eq!(RED.map(GREEN, -100.0), RED);
        assert_eq!(RED.map(GREEN, 0.5), Color{r: 128, g: 128, b: 0, a: 255});

        assert_eq!(RED.map_alpha(TRANSPARENT, 0.0), RED);
        assert_eq!(RED.map_alpha(TRANSPARENT, 1.0), Color{r: 255, g: 0, b: 0, a: 0});
        assert_eq!(RED.map_alpha(TRANSPARENT, 0.5), Color{r: 255, g: 0, b: 0, a: 128});
        assert_eq!(RED.map_alpha(TRANSPARENT, 100.0), Color{r: 255, g: 0, b: 0, a: 0});
        assert_eq!(RED.map_alpha(TRANSPARENT, -100.0), Color{r: 255, g: 0, b: 0, a: 255});

        assert_eq!(RED.map_rgb(TRANSPARENT, 0.0), RED);
        assert_eq!(RED.map_rgb(TRANSPARENT, 1.0), Color{r: 0, g: 0, b: 0, a: 255});
        assert_eq!(RED.map_rgb(TRANSPARENT, 0.5), Color{r: 128, g: 0, b: 0, a: 255});
        assert_eq!(RED.map_rgb(TRANSPARENT, 100.0), Color{r: 0, g: 0, b: 0, a: 255});
        assert_eq!(RED.map_rgb(TRANSPARENT, -100.0), Color{r: 255, g: 0, b: 0, a: 255});
    }

    #[test]
    fn opacity() {
        assert!(RED.is_opaque());
        assert!(!RED.is_translucent());
        assert!(TRANSPARENT.is_translucent());
        assert!(!TRANSPARENT.is_opaque());
        assert_eq!(RED.translucent(0.5), Color{r: 255, g: 0, b: 0, a: 127});
        assert_eq!(RED.translucent(1.0), Color{r: 255, g: 0, b: 0, a: 0});
        assert_eq!(RED.translucent(0.0), RED);
        assert_eq!(TRANSPARENT.opaque(), BLACK);
        assert!(TRANSPARENT.opaque().is_opaque());
        assert!(!TRANSPARENT.opaque().is_translucent());
        assert!(RED.translucent(0.0).is_opaque());
        assert!(RED.translucent(1.0).is_translucent());
    }

    #[test]
    fn from_str() {
        assert_eq!(RED, Color::from_str("#ff0000ff").unwrap());
        assert_eq!(GREEN, Color::from_str("#00ff00ff").unwrap());
        assert_eq!(BLUE, Color::from_str("#0000ffff").unwrap());
        assert_eq!(TRANSPARENT, Color::from_str("#00000000").unwrap());
        assert_ne!(RED, Color::from_str("#00000000").unwrap());

        assert_eq!(RED, Color::from_str("red").unwrap());
        assert_eq!(GREEN, Color::from_str("green").unwrap());
        assert_eq!(BLUE, Color::from_str("blue").unwrap());
        assert_eq!(TRANSPARENT, Color::from_str("transparent").unwrap());
        assert_ne!(RED, Color::from_str("transparent").unwrap());

        assert_eq!("unknown color 'blibla'", Color::from_str("blibla").unwrap_err());
        assert_eq!("invalid digit found in string", Color::from_str("#blibla").unwrap_err());
    }
}
