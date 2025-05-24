fn main() {
    let quantity = 10;

    for i in (0..quantity).rev() {
        println!("Remaining quantity: {}", i);
    }
}
