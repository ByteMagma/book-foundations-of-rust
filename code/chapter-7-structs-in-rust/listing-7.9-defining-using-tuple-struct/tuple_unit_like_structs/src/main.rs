fn main() {
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    println!("{}, {}, {}", red.0, red.1, red.2);
    // Output:  255, 0, 0

    let origin = Coords(0, 0, 0);
    let dest = Coords(100, 200, 50);

    println!("{}, {}, {}", dest.0, dest.1, dest.2);
    // Output: 100, 200, 50
}
struct Color(u8, u8, u8);  // Represents RGB values
struct Coords(u8, u8, u8); // Represents 3D coordinates
