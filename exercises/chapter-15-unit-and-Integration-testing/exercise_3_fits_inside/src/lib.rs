/// Calculates the area of a shape.
/// 
/// # Examples
/// 
/// ```
/// enum Shape {
///     Square(f64),
///     Triangle(f64, f64),
/// }
///
/// impl Shape {
///     fn area(&self) -> f64 {
///         match self {
///             Shape::Square(side) => side * side,
///             Shape::Triangle(base, height) => 0.5 * base * height,
///         }
///     }
/// }
///
/// let square = Shape::Square(4.0);
/// assert_eq!(square.area(), 16.0);
///
/// let triangle = Shape::Triangle(3.0, 4.0);
/// assert_eq!(triangle.area(), 6.0);
/// ```

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

    fn can_fit_inside(&self, other: &Shape) -> bool {
        match (self, other) {
            (Shape::Square(inner), Shape::Square(outer)) => inner <= outer,
            (Shape::Triangle(inner_base, inner_height), Shape::Triangle(outer_base, outer_height)) => {
                inner_base <= outer_base && inner_height <= outer_height
            }
            _ => false, // can't compare different shape types
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

    #[test]
    fn test_square_fitting() {
        let small = Shape::Square(2.0);
        let big = Shape::Square(5.0);
        let equal = Shape::Square(2.0);

        assert!(small.can_fit_inside(&big));
        assert!(small.can_fit_inside(&equal));
        assert!(!big.can_fit_inside(&small));
    }

    #[test]
    fn test_triangle_fitting() {
        let small = Shape::Triangle(2.0, 3.0);
        let big = Shape::Triangle(5.0, 6.0);
        let equal = Shape::Triangle(2.0, 3.0);

        assert!(small.can_fit_inside(&big));
        assert!(small.can_fit_inside(&equal));
        assert!(!big.can_fit_inside(&small));
    }

    #[test]
    fn test_mixed_shapes_dont_fit() {
        let square = Shape::Square(3.0);
        let triangle = Shape::Triangle(4.0, 5.0);

        assert!(!square.can_fit_inside(&triangle));
        assert!(!triangle.can_fit_inside(&square));
    }    
}