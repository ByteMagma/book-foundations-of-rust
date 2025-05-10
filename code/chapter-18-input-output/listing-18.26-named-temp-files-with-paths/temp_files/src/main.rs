use tempfile::tempfile;
use std::io::{Write, Seek, SeekFrom, Read};

fn main() -> std::io::Result<()> {
    let mut file = tempfile()?; // Automatically deleted on drop
    writeln!(file, "Temporary data here!")?;
    file.seek(SeekFrom::Start(0))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}
