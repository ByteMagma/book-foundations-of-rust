use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("data.bin")?;
    let mut buffer = [0u8; 4]; // read 4 bytes at a time

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }
        if bytes_read < 4 {
            eprintln!("Unexpected end of file");
            break;
        }

        // Convert 4 bytes to a u32 (assuming little-endian format)
        let number = u32::from_le_bytes(buffer);
        println!("Number: {}", number);
    }

    Ok(())
}
