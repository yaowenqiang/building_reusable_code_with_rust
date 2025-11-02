trait Animal {
    fn walk(&self);
    fn make_noise(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn walk(&self) {
        unimplemented!();
    }
    fn make_noise(&self) {
        unimplemented!();
    }
}


fn main() {
    let cat = Cat {};
    cat.walk();
}
