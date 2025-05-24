#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(2.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(3.0, 4.0);

    println!("Circle radius: {:#?}", circle);
    println!("Square side length: {:#?}", square);
    println!("Rectangle dimensions: {:#?}", rectangle);
    println!("Triangle dimensions: {:#?}", triangle);
}
