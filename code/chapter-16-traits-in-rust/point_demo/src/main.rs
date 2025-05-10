use point_demo::Point;

fn main() {
  let p = Point { x: 3, y: 7 };
  println!("Display: {}", p);        // Uses Display
  println!("Debug: {:?}", p);        // Uses Debug
  println!("Pretty Debug:\n{:#?}", p); // Pretty Debug output
}
