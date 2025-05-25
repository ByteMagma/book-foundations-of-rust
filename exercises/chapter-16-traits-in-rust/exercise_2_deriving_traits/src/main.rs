trait Greetable {
    // Method without default implementation — must be implemented
    fn say_hello(&self);

    // Method with default implementation — optional to override
    fn say_goodbye(&self) {
        println!("Goodbye from Greetable!");
    }
}

// Implementing the trait for a struct
#[derive(Debug, Clone, PartialEq)]
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

    // Use the Debug trait
    println!("Debug: {:?}", person);

    // Use the Clone trait
    let person2 = person.clone();
    println!("Cloned person: {:?}", person2);

    // Use the PartialEq trait
    if person == person2 {
        println!("person1 and person2 are equal!");
    } else {
        println!("person1 and person2 are not equal!");
    }

    // Now modify one to show inequality
    let person3 = Person {
        name: "Bob".to_string(),
    };

    println!("Another person: {:?}", person3);
    if person != person3 {
        println!("person1 and person3 are different!");
    }
}
