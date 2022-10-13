use cube::color::{WHITE, BLACK};
use cube::math;

fn main() {
    let c = math::map(0.1, 0.0, 1.0, BLACK, WHITE);

    println!("{:?}", c);
}
