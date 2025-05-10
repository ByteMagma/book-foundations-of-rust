fn main() {
    let names = ["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }
}
