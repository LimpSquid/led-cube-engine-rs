use cube::color::{WHITE, BLACK};

fn main() {
    let c1 = WHITE.translucent(0.5);
    let c2 = BLACK.translucent(0.5);
    let c3 = c1.blend_into(c2);
    println!("{:?}", c3);
}
