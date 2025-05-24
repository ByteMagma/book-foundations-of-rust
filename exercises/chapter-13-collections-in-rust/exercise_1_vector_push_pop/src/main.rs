fn main() {
    let mut animals = Vec::new();
    animals.push("Dog");
    animals.push("Cat");
    animals.push("Elephant");
    animals.push("Giraffe");

    while animals.len() > 0 {
        let animal = animals.pop().unwrap();
        println!("Animal: {}", animal);
    }
}
