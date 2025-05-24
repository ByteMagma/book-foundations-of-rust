fn main() {
    let discount = 10;

    match discount {
        amount if amount < 20 => println!("small discount applied"),
        amount if amount < 40 => println!("medium discount applied"),
        amount if amount < 60 => println!("large discount applied"),
        _ => println!("huge discount applied"),
    }
}
