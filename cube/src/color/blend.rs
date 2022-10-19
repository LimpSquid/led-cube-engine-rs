use super::Color;

impl Color {
    pub fn alpha_blend_into<T: Into<Color>>(self, bucket: T)-> Self {
        let bucket: Color = bucket.into();
        let alpha: u32 = self.a.into();
        let inv_alpha: u32 = (u8::MAX - self.a).into();

        // Blend color into the bucket
        Color {
            r: ((alpha * (self.r as u32) + inv_alpha * (bucket.r as u32)) >> u8::BITS) as u8,
            g: ((alpha * (self.g as u32) + inv_alpha * (bucket.g as u32)) >> u8::BITS) as u8,
            b: ((alpha * (self.b as u32) + inv_alpha * (bucket.b as u32)) >> u8::BITS) as u8,
            a: u8::MAX,
        }
    }

    pub fn blend_into<T: Into<Color>>(self, bucket: T) -> Self {
        if self.is_opaque() {
            self
        } else {
            self.alpha_blend_into(bucket)
        }
    }
}

// TODO: tests
