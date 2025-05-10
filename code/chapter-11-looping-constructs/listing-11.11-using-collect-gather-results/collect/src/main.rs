fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter out only even numbers
    let even_numbers: Vec<i32> = numbers
        .iter()                    // Create an iterator over the vector
        .filter(|&n| n % 2 == 0)   // Keep only numbers divisible by 2
        .cloned()                  // Clone the filtered values
        .collect();                // Collect the results into a new Vec

    println!("Even numbers: {:?}", even_numbers);
}
