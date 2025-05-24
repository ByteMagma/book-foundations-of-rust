fn main() {
    let mut counter = 0;

    let final_value = loop {
        println!("Counter: {}", counter);
        counter += 1;

        if counter >= 10 {
            break counter;
        }
    };

    println!("Final value: {}", final_value);
}
