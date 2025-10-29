fn main() {
    for element in 1..10 {
        println!("{element}");
    }

    let myList = [1,2,3,4];
    for element in myList.into_iter() {
        println!("{element}");
    }
}
