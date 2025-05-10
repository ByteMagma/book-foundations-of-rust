fn main() {
    let fruit_1 = String::from("apple");
    let fruit_2 = fruit_1.clone();

    // fruit_1 still valid, its value was cloned, not moved
    println!("{fruit_1}");
 }
