use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // propagate open error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // propagate read error
    Ok(contents)
}
fn main() {
    match read_file_contents("example.txt") {
        Ok(text) => println!("File contents:\n{}", text),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
