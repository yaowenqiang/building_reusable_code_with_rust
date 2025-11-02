pub trait Add<RHS = self> { // right hand side
    type Output;
    fn add(self, rhs: RHC) ->Self::Output;
} 

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other:Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.x + other.y,
        }
    }
}
