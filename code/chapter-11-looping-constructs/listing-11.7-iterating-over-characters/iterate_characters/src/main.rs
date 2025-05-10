fn main() {
    let text = "hello";
    for c in text.chars() {
        println!("{}", c);
    }
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("I like {}.", fruit);
    }
}
