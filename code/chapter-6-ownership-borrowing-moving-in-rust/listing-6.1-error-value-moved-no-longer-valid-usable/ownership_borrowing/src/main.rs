fn main() {
    let fruit_1 = String::from("apple");
    let fruit_2 = fruit_1;

    // fruit_1 no longer valid, its value moved to fruit_2
    println!("{fruit_1}");	
 }
