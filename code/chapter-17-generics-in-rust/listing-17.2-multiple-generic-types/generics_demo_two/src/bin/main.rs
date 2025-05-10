use generics_demo_two::create_pair;

fn main() {
  let p1 = create_pair(10, "bananas");
  let p2 = create_pair(true, 3.14);

  println!("First: {}, Second: {}", p1.first, p1.second);
  println!("First: {}, Second: {}", p2.first, p2.second);
}
