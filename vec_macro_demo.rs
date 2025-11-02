fn main() {
    let mut vec = vec![1,2,3,4];
    vec.push(5);
    assert_eq!(vec,[1,2,3,4,5] );

    let vec = vec![0;5];
    assert_eq!(vec, [0,0,0,0,0]);
}
