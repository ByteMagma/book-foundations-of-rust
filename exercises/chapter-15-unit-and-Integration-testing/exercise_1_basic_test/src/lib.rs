enum Shape {
    Square(f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Square(side) => side * side,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_area() {
        let square = Shape::Square(4.0);
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Shape::Triangle(3.0, 4.0);
        assert_eq!(triangle.area(), 6.0);
    }
}