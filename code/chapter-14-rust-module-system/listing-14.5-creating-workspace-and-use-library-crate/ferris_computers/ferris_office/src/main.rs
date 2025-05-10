use ferris_core::{
    models::{Item, Order},
    utils::{generate_item_number, generate_order_number},
    inventory::{update_stock, check_stock, Inventory},
};

fn main() {
    let mut inventory: Inventory = Inventory::new();

    let item = Item {
        item_number: generate_item_number(),
        name: "Ferris Mini PC".to_string(),
        price: 599.99,
    };

    update_stock(&mut inventory, &item.item_number, 5);
    println!("Stock: {}", check_stock(&inventory, &item.item_number));

    let order = Order {
        order_number: generate_order_number(),
        items: vec![item.clone()],
    };

    println!("Created order:\n{:#?}", order);
}
