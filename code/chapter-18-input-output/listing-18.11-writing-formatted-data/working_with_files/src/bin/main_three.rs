use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;

    writeln!(file, "New log entry")?;
    Ok(())
}
