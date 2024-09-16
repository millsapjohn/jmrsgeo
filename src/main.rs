pub mod point;

use point::*;

fn main() {
    let point1 = Pointz::new(0.0, 0.0, 0.0);
    let point2 = Pointxy::new(1.0, 1.0);

    let dist = dist2d(&point1.cast_to_2d(), &point2);
    println!("{}", dist);
}
