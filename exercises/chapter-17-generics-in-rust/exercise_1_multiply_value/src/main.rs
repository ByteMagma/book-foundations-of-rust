fn multiply_value<T: std::ops::Mul<Output = T>>(value: T, multiplier: T) -> T {
    value * multiplier
}

fn main() {
    println!("5 times 3 is: {}", multiply_value(5, 3));
    println!("5.0 times 3.0 is: {:.1}", multiply_value(5.0, 3.0));
}
