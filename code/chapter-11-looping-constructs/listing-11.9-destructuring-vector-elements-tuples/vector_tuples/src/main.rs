fn main() {
    let pairs = vec![(1, "one"), (2, "two")];
    for (num, word) in pairs {
        println!("{} is written as {}", num, word);
    }
}
