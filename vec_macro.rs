let x: Vec<i32>= vec![1,2,3];

// expand to
let x: Vec<i32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};

macro_rules! vec {
    ($($x:expr), *) => (
        { // block
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    );
}
