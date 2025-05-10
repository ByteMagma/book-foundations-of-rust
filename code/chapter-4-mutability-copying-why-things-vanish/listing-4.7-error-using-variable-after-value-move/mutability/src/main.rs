fn take_string(my_string: String) {
    println!("{}", my_string);
}
fn main() {
    let val_a = String::from("hello");
    let val_b = val_a;        // move occurs here
    println!("{}", val_a); // error: val_a is moved
    take_string(val_b);   // val_b is moved into function
}
