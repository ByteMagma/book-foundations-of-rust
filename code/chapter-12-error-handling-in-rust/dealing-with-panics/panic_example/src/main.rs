// Calling the panic!() macro to trigger a panic
// fn main() {
//     println!("Before panic...");
//     panic!("Something went terribly wrong!");
//     println!("This line will never be reached.");
// }

// Index out of bounds causing a panic
// fn main() {
//     let numbers = vec![1, 2, 3];
//     println!("{}", numbers[5]); // This will panic!
// }

// Access elements with vector get() method
// fn main() {
//     let numbers = vec![1, 2, 3];
//     match numbers.get(5) {
//         Some(value) => println!("Value: {}", value),
//         None => println!("Index out of bounds!"),
//     }
// }

// if-let construct
// fn main() {
//     let numbers = vec![1, 2, 3];
//     if let Some(value) = numbers.get(5) {
//         println!("Value: {}", value);
//     } else {
//         println!("Index out of bounds!");
//     }
// }

// unwrap() to panic on None
// fn main() {
//     let option: Option<i32> = None;
//     // Panics with "called `Option::unwrap()` on a `None` value"
//     println!("{}", option.unwrap());
// }

// if-let construct
// fn main() {
//     let option: Option<i32> = None;

//     if let Some(value) = option {
//         println!("Value: {}", value);
//     } else {
//         println!("No value present");
//     }
// }

// match construct
// fn main() {
//     let option: Option<i32> = None;

//     match option {
//         Some(value) => println!("Value: {}", value),
//         None => println!("No value present"),
//     }
// }

// unwrap_or() to provide a fallback default value
// fn main() {
//     let option: Option<i32> = None;
//     println!("Value: {}", option.unwrap_or(0)); // prints 0 if None
// }

// unwrap_or_else() - closure executes on None or Err
// fn main() {
//     let option: Option<i32> = None;
//     println!("Value: {}", option.unwrap_or_else(|| {
//         // Some expensive computation or logging
//         println!("Generating default...");
//         42
//     }));
// }

// map_or() - closure executes on Some, otherwise default value - 0 here
// fn main() {
//     let option: Option<i32> = None;
//     let doubled = option.map_or(0, |v| v * 2);
//     println!("Doubled value: {}", doubled);
// }

// Example of overflow
// fn main() {
//     let mut x: u8 = 255;
//     // Panics in debug mode, but wraps around in release mode.
//     x += 1;
// }

// wrapping_add() adds value or on overflow wraps to 0
// fn main() {
//     let mut x: u8 = 255;
//     x = x.wrapping_add(1); // x becomes 0
//     println!("{}", x);
// }

// checked_add() adds value or produces None on overflow
// fn main() {
//     let mut x: u8 = 255;
//     match x.checked_add(1) {
//         Some(result) => println!("Result: {}", result),
//         None => println!("Overflow detected!"),
//     }
// }

// saturating_add() adds value or caps value at max value for data type
// fn main() {
//     let mut x: u8 = 255;
//     x = x.saturating_add(1); // x stays at 255
//     println!("{}", x);
// }

// overflowing_add() adds value and returns tuple (result, did_overflow_boolean)
fn main() {
    let mut x: u8 = 255;
    let (result, overflowed) = x.overflowing_add(1);
    println!("Result: {}, Overflow occurred: {}", result, overflowed);
}
