use fs2::FileExt;
use std::fs::File;
fn main() -> std::io::Result<()> {
    let file = File::create("temp.txt")?;
    file.lock_exclusive()?; // Lock acquired
    // Do stuff...
    file.unlock()?; // Explicit unlock
    Ok(())
}
