fn main() {
    let movie = String::from("Good Will Hunting");

    for c in movie.chars() {
        println!("{}", c);
    }

    for s in movie.bytes() {
        println!("{}", s);
    }
}
