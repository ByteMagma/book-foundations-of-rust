use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // Ensure prompt appears before input

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello, {}!", name.trim());
}
