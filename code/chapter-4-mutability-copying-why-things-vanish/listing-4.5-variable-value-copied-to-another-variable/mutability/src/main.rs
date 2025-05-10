fn print_value(val: i32) {
    println!("{}", val);
}
fn main() {
    let val_a = 5;
    let val_b = val_a; // val_a is copied into val_b, not moved
    print_value(val_a); // works fine
}
