use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("example.txt")?;
    println!("{}", content);
    Ok(())
}
