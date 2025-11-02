trait Vehicle {
    fn new(name: &'static str) -> Self;//static method
    fn move(&self) -> ();// instance method
    fn to_string(&self) {
        println!("Vehicle {}", self.name); //default implementation
    }
}

#[derive(Debug)]
struct Airplane {
    name: &'static str
}

#[derive(Debug)]
struct Bicycle {
    name: &'static str
}

#[derive(Debug)]
struct Car {
    name: &'static str
}

impl Vehicle for Airplane {
    fn new(name: &'static str) -> Self {
        Airplane { name: name}
    }
    fn move(&self) {
        self.fly();
    }
}

impl Vehicle for Bicycle {
    fn new(name: &'static str) -> Self {
        Bicycle { name: name}
    }
    fn move(&self) {
        self.pedal();
    }
}

impl Vehicle for Car {
    fn new(name: &'static str) -> Self {
        Car { name: name}
    }
    fn move(&self) {
        self.drive();
    }
}

fn from_amsterdam_to_pairs<T: Vehicle>(Vehicle: T) {
    while (location_of(Vehicle) != "pairs") {
        Vehicle.move();
    }
}
