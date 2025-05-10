use crate::item::Item;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_number: String,
    pub items: Vec<Item>,
}
