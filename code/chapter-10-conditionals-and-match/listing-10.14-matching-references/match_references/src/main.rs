fn main() {
    let x = Some(5);
    // Matching against a reference to x
    match &x {
        Some(val) => println!("Value is {}", val),
        None => println!("No value"),
    }
}
