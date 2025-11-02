fn consuming_call<F>(func: F) 
    where F: FnOnce() -> String
{
    println!("Consumed {}", func());
}

fn main() {
    let x = String::from("hello");

    // A closure that  consume x and returns it directly
    // FnOnce are automatically implemented for clouse that might consume captured variables
    //
    let return_x = move || x;
    consuming_call(return_x);
}
/*

pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    // may consume it, can only be called once
}

pub trait FnMut<Args>: FnMut<Args> {
    type Output;
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    // may mutabe it, but safe to call multiple times
}

pub trait Fn<Args>: Fn<Args> {
    type Output;
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // Safe to call multiple times without mutating state
}
*/
