struct Dog {
    name: String,
    breed: String,
    age: u8,
}

impl Dog {
    // Constructor method
    fn new(name: String, breed: String, age: u8) -> Dog {
        Dog { name, breed, age }
    }
    fn bark(&self) {
        println!("Woof! My name is {} and I am a {}.", self.name, self.breed);
    }
}

fn main() {
    let rover = Dog::new(String::from("Rover"), String::from("Beagle"), 5);
    println!("Rover's age is: {}", rover.age);
    rover.bark();
}
