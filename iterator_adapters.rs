fn main() {
    let numbers = vec![1,2,3,4];
    let plus_one = numbers.iter().map(|x| x + 1);
    plus_one.for_each(|x| println!("{x}"));

    let numbers2 = vec![1,2,3,4];
    //let larger_then_two = numbers2.iter().filter(|x| x > 2);
    let larger_than_two = numbers2.into_iter().filter(|&x| x > 2);

    larger_than_two.for_each(|x| println!("{x}"));

    let numbers3 = vec![1,2,3,4];
    let larger_than_three = numbers3.iter().filter(|&&x| x > 3);

    larger_than_three.for_each(|x| println!("{x}"));

    let numbers4 = vec![1,2,3,4,5];
    let larger_than_four = numbers4.iter().filter(|x| **x > 4);

    larger_than_four.for_each(|x| println!("{x}"));
}
