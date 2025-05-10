use crate::{Item, Order};
pub fn place_order() -> Order {
    let item = Item {
        item_number: crate::generate_item_number(),
        name: "Ferris Developer Laptop".to_string(),
        price: 1499.99,
    };
    println!("Office placed order for item: {:?}", item);
    Order {
        order_number: crate::generate_order_number(),
        items: vec![item],
    }
}
pub fn build_item(order: &Order) -> Item {
    let item = order.items[0].clone(); // Assume 1 item for now
    println!("Factory built item: {:?}", item);
    item
}
pub fn receive_item_at_warehouse(item: &Item) {
    println!(
        "Warehouse received item: {} ({}) — ready for fulfillment.",
        item.name, item.item_number
    );
}
pub fn ship_to_customer(order: &Order) {
    println!(
        "Warehouse shipped order {} to customer!",
        order.order_number
    );
}
