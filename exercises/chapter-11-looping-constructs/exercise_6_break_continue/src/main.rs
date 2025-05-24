fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    for number in nums.iter() {
        if !number % 2 == 0 {
            continue;
        }

        if number > &10 {
            println!("Number is greater than 10: {}", number);
            break;
        }

        let doubled = number * 2; 
        println!("Doubled even number: {}", doubled);
    }   
}
