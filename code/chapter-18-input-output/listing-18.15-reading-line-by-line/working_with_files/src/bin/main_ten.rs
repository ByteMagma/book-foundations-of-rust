use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("lines.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
