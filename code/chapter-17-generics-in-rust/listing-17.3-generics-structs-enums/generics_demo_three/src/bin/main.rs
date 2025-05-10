use generics_demo_three::Container;
use generics_demo_three::Dimensions;

fn main() {
  let int_container = Container { value: 42 };
  let str_container = Container { value: "Ferris" };
  let float_container = Container { value: 3.14 };

  let fixed_size = Dimensions { width: 800, height: 600 };     // i32
  let scaled_size = Dimensions { width: 0.5, height: 0.75 };   // f64
}
