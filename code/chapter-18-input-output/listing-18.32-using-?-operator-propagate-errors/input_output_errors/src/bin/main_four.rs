use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn print_lines(path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn main() {
    if let Err(e) = print_lines("notes.txt") {
        eprintln!("Failed to print lines: {}", e);
    }
}
