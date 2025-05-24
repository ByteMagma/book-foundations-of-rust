enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn get_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 3.14159 * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    let triangle = Shape::Triangle(4.0, 5.0);

    println!("Circle area: {:.2}", circle.get_area());
    println!("Square area: {:.2}", square.get_area());
    println!("Rectangle area: {:.2}", rectangle.get_area());
    println!("Triangle area: {:.2}", triangle.get_area());
}
