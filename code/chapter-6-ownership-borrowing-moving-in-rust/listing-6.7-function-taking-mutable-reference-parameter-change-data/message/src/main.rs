fn main() {
    // message is mutable, so we can change it
    let mut message = String::from("Hello");

    // We borrow message mutably, so ownership is not moved
    create_message(&mut message);

    // message is still valid here, because we only borrowed it
    println!("{message}");
}
// we take a mutable reference to a string
fn create_message(msg: &mut String) {
    // We're borrowing the String mutably.
    // That means we can look at it, and we can change it.
    msg.push_str(" world");
}
