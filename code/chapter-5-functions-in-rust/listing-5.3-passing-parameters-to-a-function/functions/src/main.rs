fn main() {
    greeting("Steve", 32);
}
fn greeting(name: &str, age: u8) {
    println!("Hello {}, you are {} years old!", name, age);
}
