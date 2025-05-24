#[derive(Debug)]
enum DeviceType {
    Light,
    Thermostat
}

impl DeviceType {
    fn description(&self) -> &str {
        match self {
            DeviceType::Light => "a smart light that can be turned on or off",
            DeviceType::Thermostat => "a thermostat to control temperature",
        }
    }
}

struct SmartDevice {
    name: String,
    device_type: DeviceType,
    power_status: String,
}

impl SmartDevice {
    fn new(name: &str, device_type: DeviceType) -> Self {
        SmartDevice {
            name: name.to_string(),
            device_type,
            power_status: "Off".to_string(),
        }
    }

    fn power_on(&mut self) {
        self.power_status = "On".to_string();
        println!("{} is now ON.", self.name);
    }

    fn power_off(&mut self) {
        self.power_status = "Off".to_string();
        println!("{} is now OFF.", self.name);
    }

    fn status(&self) {
        println!(
            "{} ({:?}) - {}",
            self.name,
            self.device_type,
            self.power_status,
        );
    }

    fn describe(&self) {
        println!(
            "{} is {}.",
            self.name,
            self.device_type.description()
        );
    }
}

fn main() {
    let mut living_room_light = SmartDevice::new("Living Room Light", DeviceType::Light);
    let mut bedroom_thermo = SmartDevice::new("Bedroom Thermostat", DeviceType::Thermostat);

    println!("\n--- Device Descriptions ---");
    living_room_light.describe();
    bedroom_thermo.describe();

    println!("\n--- Initial Status ---");
    living_room_light.status();
    bedroom_thermo.status();

    println!("\n--- Power ON Devices ---");
    living_room_light.power_on();
    bedroom_thermo.power_on();

    println!("\n--- Power OFF Devices ---");
    living_room_light.power_off();
    bedroom_thermo.power_off();

    println!("\n--- Final Status ---");
    living_room_light.status();
    bedroom_thermo.status();
}
