use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("test_file.txt")?;

    writeln!(file, "Adding this line to the file!")?;
    Ok(())
}
