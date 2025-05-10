use std::fs::File;
use std::io::{BufRead, BufReader};
use fs2::FileExt;

fn main() -> std::io::Result<()> {
    let file = File::open("config.ini")?;
    file.lock_shared()?; // Acquire a shared lock

    let reader = BufReader::new(&file);
    for line in reader.lines() {
        println!("Config Line: {}", line?);
    }

    // Lock is automatically released here
    Ok(())
}
