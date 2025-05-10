use std::io;

fn main() {
    println!("Enter your age:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let age: u32 = 
        input.trim().parse().expect("Please enter a valid number");
    println!("In 5 years, you’ll be {}.", age + 5);
}
