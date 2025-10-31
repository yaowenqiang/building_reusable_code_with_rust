fn main() {
    let numbers :Vec<i32> = (1..)
        .map(|x| x + 1)
        .map(|x| x * x)
        .take(3)
        .collect();
    println!("{:?}", numbers);
}
