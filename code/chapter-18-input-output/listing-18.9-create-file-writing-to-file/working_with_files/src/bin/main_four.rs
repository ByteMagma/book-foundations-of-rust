use std::fs::File;
use std::io::{self, Write};
fn main() -> io::Result<()> {
    let mut file = File::create("hello.txt")?;
    writeln!(file, "Hello, world!")?;
    Ok(())
}
