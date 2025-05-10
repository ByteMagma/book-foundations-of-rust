use rand::{distributions::Alphanumeric, Rng};
pub fn generate_item_number() -> String {
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    format!("ITM-{}", suffix)
}
pub fn generate_order_number() -> String {
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    format!("ORD-{}", suffix)
}
