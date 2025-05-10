use std::fs::OpenOptions;
use std::io::Write;
use fs2::FileExt;
use chrono::Utc;
fn main() -> std::io::Result<()> {
    let log_path = "app.log";

    // Open (or create) the log file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path)?;
    // Lock the file exclusively before writing
    file.lock_exclusive()?; // blocks until the lock is available
    writeln!(file, "[{}] Application started", Utc::now())?;
    // File lock is automatically released when file is dropped
    Ok(())
}
