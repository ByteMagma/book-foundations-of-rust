#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // Method that takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that compares with another Rectangle
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("Area of rect1: {} square pixels", rect1.area());
    // Output: Area of rect1: 1500 square pixels
}
