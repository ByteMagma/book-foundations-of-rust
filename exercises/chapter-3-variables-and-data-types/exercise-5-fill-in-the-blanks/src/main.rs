/* This is a simple Rust program that demonstrates the use of different data types.
    A temperature is defined as a floating-point number, the number of days in a 
    week at that temperature is an integer, and a boolean variable indicates whether 
    the weather is good for skiing.

    Note, the variable for the number of days in the week is a u8 because
     a week can only have 7 days, and a u8 can hold values from 0 to 255,
     so it is a good fit for this use case.
 */
fn main() {
    let temperature: f32 = 85.0;
    let number_of_days_in_week_at_that_temp: u8 = 5;
    let good_weather_for_skiing: bool = false;
}
