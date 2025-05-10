fn main() {
    let fruit_1 = String::from("apple");

    // We borrow fruit_1 immutably, so ownership is not moved
    display_a_fruit(&fruit_1);

    // fruit_1 is still valid here, because we only borrowed it
    println!("{fruit_1}");
}

fn display_a_fruit(fruit: &String) {
    // We're borrowing the String immutably.
    // That means we can look at it, but we can't change it.
    println!("The fruit: {}", fruit);
}
