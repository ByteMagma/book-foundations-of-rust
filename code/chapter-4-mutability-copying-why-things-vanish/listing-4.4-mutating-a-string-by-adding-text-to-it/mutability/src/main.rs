fn main() {
    let mut name = String::from("Greg");
    println!("Original name: {}", name);

    name.push_str(" the Rustacean");
    println!("Updated name: {}", name);
}
