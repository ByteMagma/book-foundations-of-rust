mod models;
use models::Item;
use models::Order;

fn main() {
    let item1 = Item {
        item_number: "ITM-001".to_string(),
        name: "Ferris PC".to_string(),
        price: 899.99,
    };

    let order = Order {
        order_number: "ORD-001".to_string(),
        items: vec![item1],
    };

    println!("Order created:\n{:#?}", order);
}
