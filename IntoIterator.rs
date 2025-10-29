fn main() {
    let myList = vec!(1,2,3,4);
    for element in myList {
        println!("{element}");
    }

    let mut iter = IntoIterator::into_iter(myList);
    loop {
        match iter.next() {
            Some(x) => {
                print("{x}");
            },
            None =>{ break }
        }
    }

}
