#[derive(Debug)]
struct MyStruct {
}
// pub trait Copy: Clone {} // Clone is a supertrait of Copy
// Copy is trivial bitwise copy, can't be ovreload
// Notice: it's empty, this is what we called marker trait


impl Copy for MyStruct {}

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}

#[allow(unused_variables)]
pub fn main() {
    let x = MyStruct{};
    let y = x;
    println!("{:?}", x);
}
