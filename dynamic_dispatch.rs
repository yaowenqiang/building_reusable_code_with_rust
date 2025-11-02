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

fn show_myself<T: ShowMySelf>(x: T) {
    println!("{}", x.show());
}

fn main() {
    let x: &ShowMySelf = &42;
    let y = &"Hello world".to_string() as &ShowMySelf;
    show_myself(x);
    show_myself(y);
}
