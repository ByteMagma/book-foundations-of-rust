fn main() {
    let point = (3, 5, 8);
    match point {
        (0, y, z) => println!("Point on the x-axis at y = {}, z = {}", y, z),
        (x, 0, z) => println!("Point on the y-axis at x = {}, z = {}", x, z),
        (x, y, 0) => println!("Point on the z-axis at x = {}, y = {}", x, y),
        (x, y, z) => println!("Point at ({}, {}, {})", x, y, z),
    }
}
