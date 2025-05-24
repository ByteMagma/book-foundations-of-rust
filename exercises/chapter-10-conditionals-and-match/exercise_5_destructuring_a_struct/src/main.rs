struct Shape {
    points: u8,
}

fn main() {
    let line = Shape { points: 2 };
    let triangle = Shape { points: 3 };
    let square = Shape { points: 4 };
    let pentagon = Shape { points: 5 };
    let something_else = Shape { points: 10 };

    print_points(line);
    print_points(triangle);
    print_points(square);
    print_points(pentagon);
    print_points(something_else);
}

fn print_points(shape: Shape) {
    match shape {
        Shape { points: 2 } => println!("Line"),
        Shape { points: 3 } => println!("Triangle"),
        Shape { points: 4 } => println!("Square"),
        Shape { points: 5 } => println!("Pentagon"),
        _ => println!("Unknown shape"),
    }
}
