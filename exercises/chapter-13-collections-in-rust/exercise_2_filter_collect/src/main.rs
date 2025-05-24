fn main() {
    let scores = vec![("Bob Smith", 92), ("Susan Carson", 94), ("Fred Wilson", 60)];
    let passing: Vec<(&str, i32)> = scores.into_iter().filter(|student| student.1 >= 65).collect();

    for (name, score) in passing {
        println!("{}: {}", name, score);
    }
}
