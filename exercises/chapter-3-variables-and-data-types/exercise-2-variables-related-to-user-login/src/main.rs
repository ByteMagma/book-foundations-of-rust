/* This is a simple Rust program involving a string slice and a boolean.

    We declare a string slice `user_name` and a boolean `is_logged_in`.
    Then we print the values of these variables using the `println!` macro.

    Now change the value of the user_name to something else and the is_logged_in to false,
    and run the program again with cargo run to see the updated output.
 */
fn main() {
    let user_name = "John Doe";
    let is_logged_in = true;

    println!("User: {} login status: {}", user_name, is_logged_in);
}
