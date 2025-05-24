fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // let val = nums.get(10).unwrap();
    // println!("Value: {}", val);

    if let Some(value) = nums.get(10) {
        println!("Value: {}", value);
    } else {
        println!("No value found");
    }
}
