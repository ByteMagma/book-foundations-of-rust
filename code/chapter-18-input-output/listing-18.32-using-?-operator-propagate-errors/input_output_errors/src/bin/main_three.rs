use std::fs::File;
use std::io::{self, ErrorKind};

fn load_config(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Configuration file not found.");
                Err(e)
            }
            ErrorKind::PermissionDenied => {
                println!("Access denied to config file.");
                Err(e)
            }
            _ => {
                println!("Other error: {:?}", e);
                Err(e)
            }
        },
    }
}

fn main() {
    match load_config("config.toml") {
        Ok(_) => println!("Configuration loaded."),
        Err(e) => eprintln!("Error loading config: {}", e),
    }
}
