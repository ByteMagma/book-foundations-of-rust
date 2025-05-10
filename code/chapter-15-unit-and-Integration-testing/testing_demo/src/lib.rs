/// Adds two unsigned 64-bit integers.
///
/// # Examples
///
/// ```
/// // Import your crate or use a fully-qualified path if testing externally
/// use testing_demo::add;
///
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    fn same_point(&self, other: &Point) -> bool {
        self.x == other.x || self.y == other.y
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        }        
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() -> Result<(), String> {
        let result = add(2, 6);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }    

    #[test]
    fn second_test() {
        panic!("Force this test to fail");
    }

    #[test]
    fn points_are_the_same() {
        let point_one = Point {
            x: 100,
            y: 50,
        };
        let point_two = Point {
            x: 100,
            y: 50,
        };
        assert!(point_one.same_point(&point_two));
    }

    #[test]
    fn points_are_not_the_same() {
        let point_one = Point {
            x: 100,
            y: 50,
        };
        let point_two = Point {
            x: 300,
            y: 50,
        };
        assert!(!point_one.same_point(&point_two));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }    
}
