use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
  let mut file = File::create("data.txt")?;
  file.write_all("Hello".as_bytes())?; // a string as bytes
  file.write_all(b"Raw byte data\n")?; // another string as bytes
  Ok(())
}
