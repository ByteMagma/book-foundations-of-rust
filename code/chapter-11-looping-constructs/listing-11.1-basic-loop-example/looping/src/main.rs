fn main() {
    let mut x = 10;

    loop {
        println!("x is currently: {x}");
        x += 10;

        if x > 50 {
            break;
        }
    }
}
