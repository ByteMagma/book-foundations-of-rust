fn main() {
    let mut x = 10;

    let ending_value = loop {
        println!("x is currently: {x}");
        x += 10;

        if x > 50 {
            break x;
        }
    };
    println!("Ending value is: {:?}", ending_value);
}
