fn main() {
    let age = 18;
    match age {
        n if n < 18 => println!("Underage"),
        n if n == 18 => println!("Just turned 18!"),
        _ => println!("Adult"),
    }
}
