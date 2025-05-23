#[derive(Debug)]
struct House {
    bedrooms: u8,
    bathrooms: u8,
    address: String,
    year_built: u16,
    has_garage: bool,
    lot_sq_feet: u16,
    price: f32,
}

fn main() {
    let house_one = House {
        bedrooms: 3,
        bathrooms: 2,
        address: String::from("123 Main St"),
        year_built: 1990,
        has_garage: true,
        lot_sq_feet: 5000,
        price: 350000.00,
    };

    println!("House Details: {:#?}", house_one);
}
