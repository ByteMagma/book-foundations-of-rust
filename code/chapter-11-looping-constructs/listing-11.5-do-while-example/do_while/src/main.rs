fn main() {
    let mut x = 5;
    loop {
        println!("Value: {}", x);
        x -= 1;
        if x <= 0 {
            break;
        }
    }
}
