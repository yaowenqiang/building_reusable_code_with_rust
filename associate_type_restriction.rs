use std::fmt;

trait Iterator {
    type Item: fmt::Display;
    fn next(&mut self) ->Option<Self::Item>; // used here
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;  // associated type
    fn next(&mut self) ->Option<Self::Item> {
        Some(42)
    }
}

fn main() {
    let mut counter = Counter {};

    let n = counter.next();
    println!("{:?}", n);

}


