#[derive(Debug)]
struct Customer {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    address_1: String,
    address_2: String,
    city: String,
    state: String,
    zip_code: String,
}

fn main() {
    let customer_one = Customer {
        first_name: String::from("Frank"),
        last_name: String::from("Smith"),
        email: String::from("frank@xyz.com"),
        phone: String::from("(123) 456-7890"),
        address_1: String::from("376 Main Street"),
        address_2: String::from("Apt 302"),
        city: String::from("West Springfield"),
        state: String::from("MA"),
        zip_code: String::from("01089"),
    };

    println!("{}", customer_one);
}
