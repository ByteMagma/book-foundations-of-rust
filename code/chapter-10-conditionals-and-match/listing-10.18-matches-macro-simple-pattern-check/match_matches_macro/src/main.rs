fn main() {
    let x = Some(5);
    if matches!(x, Some(5)) {
        println!("Matched 5!");
    }
}
