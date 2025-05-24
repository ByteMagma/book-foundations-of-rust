fn main() {
    let x = 100;
    let y = 200;
    let z = 300;

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }

    if y > z {
        println!("y is greater than z");
    } else if y < z {
        println!("y is less than z");
    } else {
        println!("y is equal to z");
    }
    if z > x {
        println!("z is greater than x");
    } else if z < x {
        println!("z is less than x");
    } else {
        println!("z is equal to x");
    }
}
