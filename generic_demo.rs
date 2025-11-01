/*
#[derive(Debug)]
struct PointInteger {
    x: i32,
    y: i32
}
#[derive(Debug)]
struct PointFloat {
    x: f32,
    y: f32
}
*/
fn largest_i32_reference(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32_fixed_length(list: [i32; 5]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    //let integer = PointInteger{x:5, y: 10};
    //let float = PointFloat{x: 1.0, y: 4.0};

    //let number_list :[i32] = [34, 50, 25, 100, 65];
    //let result = largest::<i32>(number_list);
    //println!("{result}");
    let number_list: [i32; 5] = [34, 50, 25, 100, 65];
    let char_list = vec!['a', 'b','d', 'g','z'];
    let result = largest_i32_fixed_length(number_list);
    let result2 = largest_i32_reference(&number_list);
    let result3 = largest(&number_list);
    let result4 = largest_char(&char_list);
    println!("{result}");
    println!("{result2}");
    println!("{result3}");
    println!("{result4}");

}
