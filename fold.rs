fn main() {
    let numbers = vec![1,2,3,4];
    let sum :i32 = numbers.iter().fold(0, | sum, val| sum + val);
    print!("{sum}");
}
