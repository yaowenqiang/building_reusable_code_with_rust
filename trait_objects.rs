trait NoisyAnimal {
    fn make_noise(&self) -> &'static str;
}

struct Cat {}

impl NoisyAnimal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

struct Dog {}

impl NoisyAnimal for Dog {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

fn make_noises(animals: Vec<Box<dyn NoisyAnimal>>) {
    for animal in animals.iter() {
        println!("{}", animal.make_noise());
    }
}

/*
fn make_noises2(animals: Vec<Box<dyn NoisyAnimal>>) {
    for animal in animals.iter() {
        println!("{}", animal.make_noise());
    }
}
*/

fn main() {
    let animals: Vec<Box<dyn NoisyAnimal>> = vec![
        Box::new(Dog{}),
        Box::new(Cat{}),
    ];

    make_noises(animals);
}
