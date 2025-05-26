use std::io::{self, Write};
use walkdir::WalkDir;

fn main() {
    print!("Enter a directory to walk: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed

    let mut dir_input = String::new();
    io::stdin().read_line(&mut dir_input).unwrap();
    let dir_input = dir_input.trim(); // Remove newline

    for entry in WalkDir::new(dir_input) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                continue;
            }
        };
        
        let path = entry.path();

        if entry.file_type().is_dir() {
            println!("Directory: {}", path.display());
        } else if entry.file_type().is_file() {
            println!("File: {}", path.display());
        }
    }
}
