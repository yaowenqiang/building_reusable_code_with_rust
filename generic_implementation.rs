trait Iterator<T> {
    fn next(&mut self) ->Option<T>;
}

struct Counter;

impl Iterator<i32> for Counter {
    fn next(&mut self) ->Option<i32> {
        // Implement here
    }
}

impl Iterator<String> for Counter {
    fn next(&mut self) ->Option<String> {
        // Implement here
    }
}

fn main() {
    let mut counter = Counter {};

    let n_int: Option<i32> = counter.next();
    println!("{:?}", n_int);

    let n_string: Option<String> = counter.next();
    println!("{:?}", n_string);

    let n_unknown = counter.next();
    println!("{:?}", n_unknown);

};

