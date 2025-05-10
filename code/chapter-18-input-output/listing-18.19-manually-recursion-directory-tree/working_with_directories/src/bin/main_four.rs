use std::fs;
use std::path::Path;
fn visit_dirs(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("File: {}", path.display());
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    visit_dirs(Path::new("my_project"))?;
    Ok(())
}
