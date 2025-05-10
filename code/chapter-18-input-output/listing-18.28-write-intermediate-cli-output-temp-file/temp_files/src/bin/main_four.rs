use tempfile::NamedTempFile;
use std::io::Write;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut tmp = NamedTempFile::new()?;
    writeln!(tmp, "Hello from your CLI tool!")?;
    let output = Command::new("cat")
        .arg(tmp.path())
        .output()
        .expect("failed to execute process");
    println!("Cat output:\n{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
