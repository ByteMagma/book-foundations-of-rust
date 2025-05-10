use generics_demo_seven::Wrapper;

fn main() {
  let mut int_wrapper = Wrapper::new(42);
  println!("Initial value: {}", int_wrapper.get());

  int_wrapper.set(100);
  println!("Updated value: {}", int_wrapper.get());

  let string_wrapper = Wrapper::new(String::from("Hello"));
  println!("String value: {}", string_wrapper.get());
}
