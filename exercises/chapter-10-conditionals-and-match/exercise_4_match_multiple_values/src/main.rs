fn main() {
    let x = 5;

    match x {
        5 | 6 => println!("x is 5 or 6"),
        3..=8 => println!("x is between 3 and 8"),
        _ => println!("x is something else"),
    }
}
