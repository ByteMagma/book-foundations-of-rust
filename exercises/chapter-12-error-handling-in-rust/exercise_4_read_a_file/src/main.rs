use std::fs::File;
use std::io::Read;

fn main() {
    match read_file("data.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?; // Propagate errors with ?
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
