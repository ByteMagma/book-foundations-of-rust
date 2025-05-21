fn get_tax_rate() -> f32 {
    0.2
}

fn apply_tax(amount: f32) -> f32 {
    amount * (1.0 + get_tax_rate())
}

fn calculate_total(cost: f32, quantity: u32) -> f32 {
    let amount = cost * quantity as f32;
    apply_tax(amount)
}

fn main() {
    let price = 25.00;
    let quantity_purchased = 4;
    let total = calculate_total(price, quantity_purchased);
    println!("Total cost: ${:.2}", total);
}
