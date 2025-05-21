fn explicit_return() -> i32 {
    return 42;
}

fn implicit_return() -> i32 {
    42
}

fn main() {
    println!("Explicit return: {}", explicit_return());
    println!("Implicit return: {}", implicit_return());
}
