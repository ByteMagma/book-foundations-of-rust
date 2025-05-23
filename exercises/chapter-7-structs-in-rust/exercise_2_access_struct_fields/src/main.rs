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

    println!("Number of bedrooms: {}", house_one.bedrooms);
    println!("Number of bathrooms: {}", house_one.bathrooms);
    println!("Address: {}", house_one.address);
    println!("Year built: {}", house_one.year_built);
    println!("Has garage: {}", house_one.has_garage);
    println!("Lot size in square feet: {}", house_one.lot_sq_feet);
    println!("Price: ${:.2}", house_one.price);
}
