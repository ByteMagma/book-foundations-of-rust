/* A simple Rust program that calculates the total cost of items
   including tax. The program defines a constant for the tax rate,
   takes the price and quantity of items, and computes the total cost.
   Finally, it prints the result to the console.

   Note, because total_cost is a floating-point number, we need to cast
    the quantity to a floating-point number before performing the
    multiplication. This is done using the `as` keyword in Rust.
    There are other ways to perform this operation, but this is the most
    straightforward way to do it in this case.

   Note, your choices for data types might vary slightly, but for things
    like the tax rate, price and total cost you should use floating point
    numbers. For the quantity, you should use an unsigned integer type.
*/
fn main() {
    const TAX_RATE: f32 = 0.08;
    let price: f32 = 20.00;
    let quantity: u16 = 5;
    let total_cost: f32 = price * (quantity as f32) * (1.0 + TAX_RATE);
    println!("The total cost is: ${:.2}", total_cost);
}
