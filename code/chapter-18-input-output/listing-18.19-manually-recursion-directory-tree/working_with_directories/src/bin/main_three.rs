use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry in fs::read_dir("logs")? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?; // Follows symlinks

        if metadata.is_dir() {
            println!("Directory: {}", path.display());
        } else if metadata.is_file() {
            println!("File: {}", path.display());
        } else {
            println!("Other: {}", path.display());
        }
    }

    Ok(())
}
