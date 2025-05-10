use generics_demo_seven::Wrapper;

fn main() {
  let item = Wrapper::new(3.14);
  item.print(); // Wrapped value: 3.14

  let word = Wrapper::new("Rust");
  word.print(); // Wrapped value: Rust
}
