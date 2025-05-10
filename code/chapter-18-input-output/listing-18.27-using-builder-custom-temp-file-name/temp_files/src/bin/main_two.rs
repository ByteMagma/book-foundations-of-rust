use tempfile::NamedTempFile;
use std::{io::Write, thread, time::Duration};

fn main() -> std::io::Result<()> {
    let mut named_file = NamedTempFile::new()?;
    writeln!(named_file, "Rust loves safe temp files!")?;
    // Print the full absolute path to the temp file
    println!("Temp file created at: {}", named_file.path().display());
    // Sleep to allow time for manual inspection
    println!("Sleeping for 5 seconds so you can inspect the file...");
    thread::sleep(Duration::from_secs(5));
    // File gets deleted automatically when named_file is dropped
    Ok(())
}
