fn main() {
    let fruit_1 = String::from("apple");

    // We pass fruit_1 by value — this is a move
    display_a_fruit(fruit_1);

    // Error: fruit_1 is now invalid, because its ownership was moved
    println!("{fruit_1}");
}
fn display_a_fruit(fruit: String) {
    // fruit takes ownership of the String
    println!("The fruit: {}", fruit);
}
