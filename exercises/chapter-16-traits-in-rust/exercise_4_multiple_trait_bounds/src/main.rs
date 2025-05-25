use std::fmt::Display;

fn print_and_clone_plus<T: Display + Clone>(item: T) {
    let cloned = item.clone();
    println!("[+ Syntax] Original: {}", item);
    println!("[+ Syntax] Cloned: {}", cloned);
}

fn print_and_clone_where<T>(item: T)
where
    T: Display + Clone,
{
    let cloned = item.clone();
    println!("[where Clause] Original: {}", item);
    println!("[where Clause] Cloned: {}", cloned);
}

fn main() {
    let text = String::from("Hello, world!");
    let number = 123;

    print_and_clone_plus(text);
    print_and_clone_where(number);
}
