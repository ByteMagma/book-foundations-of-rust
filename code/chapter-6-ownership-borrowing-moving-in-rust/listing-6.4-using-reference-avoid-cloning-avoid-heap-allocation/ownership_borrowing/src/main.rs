fn main() {
    let fruit_1 = String::from("apple");
    let fruit_2 = &fruit_1;

    // fruit_1 still valid, its value was borrowed, not moved
    println!("{fruit_1}");
 }
