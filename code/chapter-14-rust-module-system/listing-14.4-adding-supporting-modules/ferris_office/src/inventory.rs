use std::collections::HashMap;
pub type Inventory = HashMap<String, u32>;
pub fn update_stock(inventory: &mut Inventory, item_number: &str, delta: i32) {
    let count = inventory.entry(item_number.to_string()).or_insert(0);
    if delta.is_negative() {
        let remove = delta.abs() as u32;
        *count = count.saturating_sub(remove);
    } else {
        *count += delta as u32;
    }
}
pub fn check_stock(inventory: &Inventory, item_number: &str) -> u32 {
    *inventory.get(item_number).unwrap_or(&0)
}
