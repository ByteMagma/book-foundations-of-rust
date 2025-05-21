fn log(message: String) {
    println!("{}", message);
}
fn main() {
    let str_one = String::from("The meeting is at noon.");
    let str_two = str_one.clone();  // without clone() a move would occur
    log(str_one);
}
