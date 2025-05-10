use generics_demo::swap;

fn main() {
  println!("Swapped u8: {:?}", swap(200, 80));
  println!("Swapped i32: {:?}", swap(10, 50));
  println!("Swapped f32: {:?}", swap(17.25, 32.42));
  println!("Swapped &str: {:?}", swap("world", "hello"));
}
