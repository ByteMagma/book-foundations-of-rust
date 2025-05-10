use std::fs::File;
use std::io::{self, ErrorKind};

fn open_or_create_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            println!("File not found, creating a new one.");
            File::create(path)
        }
        Err(e) => Err(e),
    }
}

fn main() {
    match open_or_create_file("maybe_exists.txt") {
        Ok(_) => println!("File is ready."),
        Err(e) => eprintln!("Failed to open or create file: {}", e),
    }
}
