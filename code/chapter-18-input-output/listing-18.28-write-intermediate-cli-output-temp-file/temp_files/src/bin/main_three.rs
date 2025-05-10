use tempfile::Builder;

fn main() -> std::io::Result<()> {
    let custom_file = Builder::new()
        .prefix("rusty_temp_")
        .suffix(".log")
        .tempfile()?; // Unnamed temporary file
    println!("Custom temp file created");
    Ok(())
}
