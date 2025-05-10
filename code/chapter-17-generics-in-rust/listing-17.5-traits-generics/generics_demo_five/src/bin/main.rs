use generics_demo_five::log;

fn main() {
  log("Welcome, Ferris!");   // &str implements Display
  log(404);                  // i32 implements Display
  //log([1, 2, 3]);          // doesn't implement Display
}
