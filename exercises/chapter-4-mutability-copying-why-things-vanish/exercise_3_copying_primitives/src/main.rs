fn log(val: i32) {
    println!("{}", val);
}
fn main() {
    let mem_val_one = 60;
    let mem_val_two = mem_val_one; // mem_val_one is copied into mem_val_two, not moved
    log(mem_val_one); // works fine
}
