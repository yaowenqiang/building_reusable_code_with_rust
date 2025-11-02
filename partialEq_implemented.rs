// Equality
// Symmetric: a == b ->b == a                   PartialEq
// Transitive: a == b and b == c -> a == c
// Refexive: a == a                             Eq
// Example: Nan != Nan for f32

use std::cmp::PartialEq;
// '==' and '!=' operator:PartialEq
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
} 

fn main() {
    let p1 = Point {x:10, y: 10};
    let p2 = Point {x:10, y: 10};
    println!("p1 == p2: {}", p1 == p2);
    let p3 = Point {x:10, y: 11};
    println!("p2 == p3: {}", p2 == p3);
}
