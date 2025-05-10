use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use std::io::Write;
use fs2::FileExt;

fn try_lock_with_timeout(file: &std::fs::File, timeout: Duration) -> std::io::Result<()> {
    let start = Instant::now();
    loop {
        match file.try_lock_exclusive() {
            Ok(_) => return Ok(()),
            Err(_) if start.elapsed() < timeout => {
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(e),
        }
    }
}
fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("app.log")?;
    // Try to acquire exclusive lock with a 5-second timeout
    match try_lock_with_timeout(&file, Duration::from_secs(5)) {
        Ok(()) => {
            writeln!(file, "[{}] Application started with timeout lock", chrono::Utc::now())?;
        }
        Err(e) => {
            eprintln!("Failed to acquire lock within timeout: {}", e);
        }
    }
    Ok(())
}
