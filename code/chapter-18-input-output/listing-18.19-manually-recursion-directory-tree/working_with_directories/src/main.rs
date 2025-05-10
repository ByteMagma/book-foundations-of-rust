use std::fs;
fn main() -> std::io::Result<()> {
    fs::create_dir("data")?;
    fs::create_dir_all("data/reports/2025")?;
    Ok(())
}
