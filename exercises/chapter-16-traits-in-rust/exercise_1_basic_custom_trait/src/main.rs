trait Greetable {
    // Method without default implementation — must be implemented
    fn say_hello(&self);

    // Method with default implementation — optional to override
    fn say_goodbye(&self) {
        println!("Goodbye from Greetable!");
    }
}

// Implementing the trait for a struct
struct Person {
    name: String,
}

impl Greetable for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}!", self.name);
    }

    // say_goodbye() is not overridden — will use the default implementation
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
    };

    person.say_hello();    // Uses custom implementation
    person.say_goodbye();  // Uses default implementation
}
