fn main() {
    let message = greeting("Frank", 28);
    println!("{message}");
}
fn greeting(name: &str, age: u8) -> String {
    let msg = format!("Yo {}, you are {} years old!", name, age);
    msg
}
