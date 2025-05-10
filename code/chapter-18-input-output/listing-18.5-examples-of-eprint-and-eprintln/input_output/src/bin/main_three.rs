use std::io;

fn main() {
    loop {
        println!("Enter a number (or type 'quit' to exit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("quit") {
            break;
        }

        match trimmed.parse::<i32>() {
            Ok(num) => println!("You entered: {}", num),
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}
