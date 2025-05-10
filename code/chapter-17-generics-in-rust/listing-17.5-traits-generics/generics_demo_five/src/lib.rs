use std::fmt::Display;

pub fn log(msg: impl Display) {
    println!("[LOG]: {}", msg);
}
