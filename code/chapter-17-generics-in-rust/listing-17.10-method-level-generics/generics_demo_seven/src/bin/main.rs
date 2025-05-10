use generics_demo_seven::Wrapper;

fn main() {
  let mut item = Wrapper::new(42);
  item.replace(100);

  // This works because i32 implements Display
  item.describe();

  let list = vec![1, 2, 3];
  let list_wrapper = Wrapper::new(list);

  // This won't compile unless Vec<i32> implements Display
  // Uncommenting this line would cause a compile error
  // list_wrapper.describe(); 

  let from_int = Wrapper::<String>::from_other("Rustacean".to_string());
  println!("Wrapped: {}", from_int.get());
}
