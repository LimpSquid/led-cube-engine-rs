use cube::color::{RED};

fn main() {
    println!("Hello, world from mock!");

    let x = RED;
    let y = x.adjust_brightness(0.8);

    println!("{:?}, {:?}", x, y);
}
