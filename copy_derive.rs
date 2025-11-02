#[derive(Debug, Copy, Clone)]
struct MyStruct {
}

#[allow(unused_variables)]
pub fn main() {
    let x = MyStruct{};
    let y = x;
    println!("{:?}", x);
}
