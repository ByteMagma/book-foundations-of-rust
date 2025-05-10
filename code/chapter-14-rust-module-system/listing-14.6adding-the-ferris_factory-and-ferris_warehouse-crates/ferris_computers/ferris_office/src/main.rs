use ferris_core::workflow::{place_order, build_item, receive_item_at_warehouse, ship_to_customer};
fn main() {
    println!("=== Ferris Computers Order Fulfillment Simulation ===");
    let order = place_order();
    let item = build_item(&order);
    receive_item_at_warehouse(&item);
    ship_to_customer(&order);
    println!("Simulation complete!");
}
