use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("big_file.txt")?;
    let mut buffer = [0u8; 512];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        // Example: print the chunk as text
        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}
