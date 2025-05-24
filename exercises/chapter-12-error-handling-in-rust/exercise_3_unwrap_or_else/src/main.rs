fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let result = nums.get(10).unwrap_or(&0);
    println!("Result: {}", result);

    let result = nums.get(10).unwrap_or_else(|| {
        println!("Index out of bounds, returning default value");
        &0
    });
    println!("Result: {}", result);

}
