/*
impl<T> Vec<T> {
    pub const fn new() ->Vec<T>
}
*/


fn main() {
    let mut list: Vec<i32> = Vec::new();
    //let mut list = Vec::new();
    //let mut list = Vec::<i32>::new(); //turbofish ::<>
    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:?}", list);
}

// see also:std::collections
// https://doc.rust-lang.org/std/collections/index.html
// Sequences: Vec, VecDeque, LinkedList
// Maps: HashMap, BTreeMap
// Sets: HashSet, BtreeSet
// Misc: BinaryHeap
//
