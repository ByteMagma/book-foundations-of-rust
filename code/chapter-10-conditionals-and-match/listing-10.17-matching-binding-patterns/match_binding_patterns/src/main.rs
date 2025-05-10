fn main() {
    let num = Some(5);
    match num {
        Some(n @ 1..=10) => println!("Matched a small number: {}", n),
        Some(n) => println!("Matched something else: {}", n),
        None => println!("No match"),
    }
}
