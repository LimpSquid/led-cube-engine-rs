use cube::color::{Color, WHITE, STEEL_BLUE};

fn main() {
    let c1 = WHITE.translucent(0.5);
    let c2 = STEEL_BLUE.translucent(0.5);
    let c3 = c1.blend_into(c2);

    println!("{:?}, {:?}", c3, Color::from_str("#ff0000ff"));
}
