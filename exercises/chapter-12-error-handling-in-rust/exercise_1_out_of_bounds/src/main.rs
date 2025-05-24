fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    match nums.get(10) {
        Some(val) => println!("Value: {}", val),
        None => println!("No value found at index 10"),
    }

}
