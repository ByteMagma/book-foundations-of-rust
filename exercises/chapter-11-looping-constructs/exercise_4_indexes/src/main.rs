fn main() {
    let fruits = ["apple", "banana", "cherry"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
}
