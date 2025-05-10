pub mod models;
pub mod utils;
pub mod inventory;
pub mod workflow;

pub use models::{Item, Order};
pub use utils::{generate_item_number, generate_order_number};
pub use inventory::{update_stock, check_stock};
pub use workflow::{place_order, build_item, receive_item_at_warehouse, ship_to_customer};
