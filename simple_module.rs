mod cat {
    pub fn meow() { // should be 'pub fn' 
        println!("meow");
    }
}
fn main() {
    cat::meow();
}
