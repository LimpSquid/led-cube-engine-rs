#[macro_export]
macro_rules! opaque {
    ($color:literal) => {{
        let b = hex_literal::hex!($color);
        assert!(b.len() == 3);
        Color {
            r: b[0],
            g: b[1],
            b: b[2],
            a: u8::MAX,
        }
    }};
}

#[macro_export]
macro_rules! translucent {
    ($color:literal) => {{
        let b = hex_literal::hex!($color);
        assert!(b.len() == 4);
        Color {
            r: b[0],
            g: b[1],
            b: b[2],
            a: b[3],
        }
    }};
}
