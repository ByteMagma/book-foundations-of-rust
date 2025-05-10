use std::io::{self, Write};

fn main() {
    print!("What's your favorite color? ");
    io::stdout().flush().unwrap(); // Make sure the prompt shows
    let mut color = String::new();
    io::stdin().read_line(&mut color).unwrap();
    println!("Nice! {} is a great color.", color.trim());
}
// fn main() {
//     let items = ["apple", "banana", "cherry"];
//     println!("You have {} items in your cart:", items.len());

//     for item in items {
//         println!("- {}", item);
//     }
// }
// fn main() {
//     let name = "Greg";
//     let language = "Rust";
//     println!("Hello, {name}! Ready to write some {language} code?");
// }
