use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_dir("data")?;
    fs::remove_dir_all("temp_data")?;
    Ok(())
}
