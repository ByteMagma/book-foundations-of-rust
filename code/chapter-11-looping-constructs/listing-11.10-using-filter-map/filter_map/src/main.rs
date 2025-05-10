fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    for num in numbers.iter().filter(|&&n| n % 2 == 0) {
        println!("Even number: {}", num);
    }
    for doubled in numbers.iter().map(|n| n * 2) {
        println!("Doubled: {}", doubled);
    }
}
