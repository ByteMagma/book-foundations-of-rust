enum Shape {
    Line,
    Triangle,
    Square,
    Pentagon,
}

impl Shape {
    fn points(&self) -> u8 {
        match self {
            Shape::Line => 2,
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
        }
    }
}

fn main() {
    println!("A line has {} points.", Shape::Line.points());
    println!("A triangle has {} points.", Shape::Triangle.points());
    println!("A square has {} points.", Shape::Square.points());
    println!("A pentagon has {} points.", Shape::Pentagon.points());
}
