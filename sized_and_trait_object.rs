trait NoisyAnimal: Sized {
    fn make_noise(&self) -> &'static str;
}

struct Cat{}

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

fn make_noise(animals: Vec<Box<dyn NoisyAnimal>>) {
    for animal in animals.iter() {
        animal.make_noise();
    }
}


fn main() {
    let animals: Vec<Box<dyn NoisyAnimal>> = vec![
        Box::new(Cat{}),
        Box::new(Dog{}),
    ];
    make_noise(animals);

}

