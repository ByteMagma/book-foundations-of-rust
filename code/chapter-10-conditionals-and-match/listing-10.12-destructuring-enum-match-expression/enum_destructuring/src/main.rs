enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}
fn main() {
    let shape = Shape::Rectangle { width: 10.0, height: 5.0 };
    match shape {
        Shape::Circle { radius } => 
            println!("Circle with radius {}", radius),
        Shape::Rectangle { width, height } => 
            println!("Rectangle with width {} and height {}", width, height),
        Shape::Triangle(a, b, c) => 
            println!("Triangle with sides {}, {}, and {}", a, b, c),
    }
}
