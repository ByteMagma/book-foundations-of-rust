#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // ARITHMETIC OPERATORS
    // Casting with the "as" operator
    let x:i32 = 5;
    let y: u32 = 10;
    // let z = x + y;           // mismatched types error
    let z = x + y as i32;    // works fine    

    // result is None due to overflow
    let x: u8 = 255;
    let result = x.checked_add(1);

    // wrapping produces result of 0
    let x: u8 = 255;
    let result = x.wrapping_add(1); 

    // saturation produces result of 255
    let x: u8 = 255;
    let result = x.saturating_add(1); // 255, saturated

    // modulus % examples
    let x: i32 = 10;
    let y: i32 = 3;
    let remainder = x % y; // 1
    let a: f64 = 10.5;
    let b: f64 = 4.2;
    let remainder_float = a % b; // 2.0999

    // ASSIGNMENT OPERATORS
    let mut x = 5; // assigns 5 to x.
    x += 3; // is the same as x = x + 3.
    x -= 2; // is the same as x = x - 2.
    x *= 4; // is the same as x = x * 4.
    x /= 2; // is the same as x = x / 2.
    x %= 3; // is the same as x = x % 3.

    // EQUALITY, INEQUALITY, COMPARISON OPERATORS
    5 == 5; // true
    5 == 3; // false
    5 != 3; // true
    5 != 5; // false

    "hello" == "hello"; // true - Valid comparison
    // 5 == "5";  // Invalid comparison - will not compile - Error: mismatched types
        
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 3, y: 4 };
    p1 == p2; // true

    let a = 5;
    let b = &a;
    a == *b; // true

    5 > 3;  // true
    3 > 5;  // false

    3 < 5;  // true
    5 < 3;  // false

    5 >= 5; // true
    5 >= 3; // true
    3 >= 5; // false
        
    5 <= 5; // true
    3 <= 5; // true
    5 <= 3; // false
    
    3.5 < 5.0; // true - valid comparison
    // 5 < "5";  // Invalid comparison - will not compile - Error: mismatched types

    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 3, y: 4 };
    p1 == p2; // true

    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 5, y: 6 };
    p1 < p2; // true

    let a = 5;
    let b = &a;
    a >= *b; // true

    // LOGICAL OPERATORS
    true && true;  // true
    true && false; // false
    false && true; // false
    false && false; // false

    true || false;  // true
    false || true;  // true
    false || false; // false
    true || true;   // true

    !true;  // false
    !false; // true
 
    let x = false;
    let result = x && {
        println!("This won't print.");
        true
    }; // false
    
    let x = true;
    let result = x || {
        println!("This won't print.");
        false
    };// true

    let (x, y) = (10, 5);
    if x > 5 && y < 10 {
        println!("Both conditions are true.");
    }
    if x < 5 || y < 10 {
        println!("At least one condition is true.");
    }
    if !(x == 10) {
        println!("x is not 10.");
    } else {
        println!("x is 10.");
    }

    // let x = 5 && 10; // This will not compile - Error: expected `bool`, found integer

    // BITWISE OPERATORS
    let a = 5;  // 0101 in binary
    let b = 3;  // 0011 in binary
    a & b; // 1 (0001 in binary)

    let a = 5;  // 0101 in binary
    let b = 3;  // 0011 in binary
    a | b; // 7 (0111 in binary)

    let a = 5;  // 0101 in binary
    let b = 3;  // 0011 in binary
    a ^ b; // 6 (0110 in binary)

    let a = 5;  // 0101 in binary
    !a; // -6

    let a = 5;  // 0101 in binary
    a << 1; // 10 (1010 in binary)
    a << 2; // 20 (10100 in binary)

    let a = 5;  // 0101 in binary
    a >> 1; // 2 (0010 in binary)
    let b: i32 = -5;
    b >> 1; // -3 (sign-extended)

    // BITWISE ASSIGNMENT OPERATORS
    let mut a = 5;
    a &= 3;         // a = a & 3 = 1
    a |= 2;         // a = a | 2 = 3
    a ^= 1;         // a = a ^ 1= 2
    a <<= 1;        // a = a << 1= 4
    a >>= 1;        // a = a >> 1= 2
}
