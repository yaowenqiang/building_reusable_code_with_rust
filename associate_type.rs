trait Iterator {
    type Item;  // associated type
    fn next(&mut self) ->Option<Self::Item>; // used here
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;  // associated type
    fn next(&mut self) ->Option<Self::Item> {
        Some(42)
    }
}

/*
impl Iterator for Counter {
    type Item = String;  // associated type
    fn next(&mut self) ->Option<Self::Item> {
        Some("42".to_string())
    }
}
*/

fn main() {
    let mut counter = Counter {};

    let n = counter.next();
    println!("{:?}", n);

}

