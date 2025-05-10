use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let file = File::open("example.txt")?;
    Ok(())
}
