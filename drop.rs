use std::ops::Drop;

struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    println!("Declaring");
    let x = MyType;
    println!("About to exit main.");
}
