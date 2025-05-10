fn main() {
    // booleans
    let active: bool = true;
    let expired: bool = false;
    // character
    let choice: char = 'Y';
    // signed integers
    let signed_small_number: i8 = 125;
    let signed_medium_number: i16 = -12_300;
    let signed_larger_number: i32 = 1_147_483_647;
    let signed_even_larger_number: i64 = 
        3_223_372_036_854_775_807;
    let signed_very_large_number: i128 = 
        100141183460469231731687303715884105727;
    let signed_pointer_size: isize = -500_000_000_000;
    // unsigned integers
    let unsigned_small_number: u8 = 200;
    let unsigned_medium_number: u16 = 45_339;
    let unsigned_larger_number: u32 = 3_294_967_295;
    let unsigned_even_larger_number = 
        12_446_744_073_709_551_615;
    let unsigned_very_large_number: u128 = 
        300282366920938463463374607431768211455;
    let unsigned_pointer_size: usize = 
        15_446_744_073_709_551_615;

    // floating point numbers
    let float_number: f32 = 3.1415926;
    let higher_precision_float_number: f64 = 3.141592653589793;

    // unit type
    let initialized: () = ();

    // string slice type
    let name: &str = "Bob";
    
    // compound types
    let name_age_tuple: (&str, u8) = ("Bob", 34);
    let scores: [u8; 20] = [0; 20];
}
