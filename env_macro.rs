fn main() {
    let path: &'static str = env!("PATH");
    println!("the $PATH variable at the time of compiling was: {}", path);

    // env! will cause compilation error is not defined

    let key: Option<&'static str> = option_env!("SECRECT_KEY");
    println!("the secrect key might be : {:?}", key);
}
