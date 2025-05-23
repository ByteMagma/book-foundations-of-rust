fn main() {
    let car = String::from("Honda Accord");
    print_car(car.clone());
    println!("Car: {}", car);

    let car = String::from("Honda Accord");
    print_car_two(&car);
    println!("Car: {}", car);
}

fn print_car(car: String) {
    println!("Car: {}", car);
}

fn print_car_two(car: &str) {
    println!("Car: {}", car);
}
