trait ShowMySelf {
    fn show(&self) -> String;
}


impl ShowMySelf for u32 {
    fn show(&self) -> String {
        format!("I'm u32 {}", *self)
    }
}

impl ShowMySelf for String {
    fn show(&self) -> String {
        format!("I'm String {}", *self)
    }
}

/*
fn show_myself<T: ShowMySelf>(x: T) {
    println!("{}", x.show());
}
*/

// Compiler "expands" the function into all implementation
fn show_myself_u32(x: u32) {
    println!("{}", x.show());
}

fn show_myself_string(x: String) {
    println!("{}", x.show());
}
fn main() {
    let x: u32 = 42;
    let y: String = "Hello world".to_string();
    // Compiler choose the right implementation and inline it
    show_myself_u32(x);
    show_myself_string(y);
}
