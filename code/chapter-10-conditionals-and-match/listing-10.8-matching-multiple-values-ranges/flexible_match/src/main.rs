fn numbers(x: i32) {
    match x {
        1 | 2 => println!("One or Two"),
        3..6 => println!("Three to Five"),
        6..=8 => println!("Six to Eight"),
        _ => println!("Something else"),
    }
}
fn main() {
    numbers(1);
    numbers(2);
    numbers(4);
    numbers(7);
    numbers(10);
}
