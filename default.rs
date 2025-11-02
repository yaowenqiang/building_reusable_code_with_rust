#[derive(Debug, Default)]
struct MyData {
    int_field: i32,
    float_field: f32,
}

fn main() {
    let data1 : MyData = Default::default();
    println!("{:?}", data1);

    let data2 = MyData {
        int_field: 42,
        ..Default::default()
    };
    println!("{:?}", data2);
}
