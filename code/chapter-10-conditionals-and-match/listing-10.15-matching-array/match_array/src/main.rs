fn main() {
    let arr = [1, 2, 3];
    match arr {
        [1, _, 3] => println!("Pattern matched"),
        _ => println!("Different array"),
    }
}
