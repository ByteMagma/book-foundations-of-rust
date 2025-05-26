use std::io::{self, Write};

fn main() {
    println!("Enter some text, or exit to quit: ");

    loop {
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); // Remove any trailing newline or spaces
        
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }
        println!("{}", input);
    }
}
