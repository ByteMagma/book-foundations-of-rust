// Example of opening a file and handling the error
// use std::fs::File;
// use std::io::Read;

// fn main() {
//     match read_file("data.txt") {
//         Ok(content) => println!("File content: {}", content),
//         Err(e) => println!("Error reading file: {}", e),
//     }
// }
// fn read_file(filename: &str) -> Result<String, std::io::Error> {
//     let mut file = File::open(filename)?; // Propagate errors with ?
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     Ok(content)
// }

// Example of protecting against division by zero
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
