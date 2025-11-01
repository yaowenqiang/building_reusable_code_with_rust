#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}
/*
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

*/

impl Point<i32> {
    fn get_x(&self) -> &i32 {
        &self.x
    }
}

impl Point<f32> {
    fn get_x(&self) -> &f32 {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5.0, y: 10.0};
    println!("p.x = {}", p.get_x());
}
