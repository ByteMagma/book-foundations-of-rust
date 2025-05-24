fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    for number in nums.iter().filter(|&x| x % 2 == 0).map(|x| x * 2) {
        println!("Doubled even number: {}", number);
    }   
}
