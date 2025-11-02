// Make MyNumber and i32 inter-convertible
#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(number: i32) ->Self {
        MyNumber { value:number }
    }
}

// when we implments From, we get the correspoding 
// 'impl Into<MyNumber>' fro i32' for free


fn main() {
    //convert an i32 into MyNumber using From<i32>
    //
    let num1 = MyNumber::from(42i32);
    println!("{:?}", num1);
    let num2: MyNumber = 42i32.into();
    println!("{:?}", num2);

}
