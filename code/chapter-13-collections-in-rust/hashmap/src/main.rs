// Determining word counts using HashMap
// use std::collections::HashMap;

// fn main() {
//     let text = "hello world wonderful world";
//     let mut word_count = HashMap::new();
//     for word in text.split_whitespace() {
//         // Increment count or insert with 1
//         let count = word_count.entry(word).or_insert(0);
//         *count += 1;
//     }
//     // Display the word frequencies
//     for (word, count) in &word_count {
//         println!("'{}': {}", word, count);
//     }
// }

// Tracking inventory
use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();

    // Add products
    inventory.insert("Apple", 50);
    inventory.insert("Banana", 30);
    inventory.insert("Orange", 20);
    // A sale is made
    let sold = "Apple";
    if let Some(stock) = inventory.get_mut(sold) {
        *stock -= 1;
        println!("Sold one {}. Remaining: {}", sold, stock);
    }
    // Display full inventory
    for (product, qty) in &inventory {
        println!("{}: {}", product, qty);
    }
}