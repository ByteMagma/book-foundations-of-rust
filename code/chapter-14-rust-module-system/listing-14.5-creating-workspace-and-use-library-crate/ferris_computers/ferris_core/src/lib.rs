pub mod models;
pub mod utils;
pub mod inventory;
pub use models::{Item, Order};
pub use utils::{generate_item_number, generate_order_number};
pub use inventory::{update_stock, check_stock};
