fn main() {
    let nums = [1, 2, 3, 4, 5];
    match nums {
        [1, 2, .., 5] => println!("Starts with 1, 2 and ends with 5"),
        _ => println!("No match"),
    }
}
