use std::ops::Add;
use std::fmt::Display;

pub fn print_labeled_value<T: Display>(label: &str, value: T) {
    println!("{}: {}", label, value);
}

pub fn add_and_log<T: Add<Output = T> + Display>(a: T, b: T) {
    let result = a + b;
    println!("Result is: {}", result);
}

pub fn print_discounted_total<T, U>(product_id: T, price: U)
where
    T: Display,
    U: Add<Output = U> + Display + Clone,
{
    let total = price.clone() + price;
    println!("{} total (2 for 1 deal): {}", product_id, total);
}

