#[derive(Debug)]
enum TrafficLight {
    Red(u8),
    Yellow(u8),
    Green(u8),
}

fn main() {
    let red_light = TrafficLight::Red(30);
    let yellow_light = TrafficLight::Yellow(5);
    let green_light = TrafficLight::Green(20);

    println!("{:?}", red_light);
    println!("{:?}", yellow_light);
    println!("{:?}", green_light);
}
