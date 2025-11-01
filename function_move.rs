fn main() {
    let numbers = vec![1,2,3];
    consuming_print(&numbers);
    println!("{}", numbers[0]);

}

fn consuming_print(numbers: &Vec<i32>) {
    for number in numbers {
        println!("{number}");
    }
}
