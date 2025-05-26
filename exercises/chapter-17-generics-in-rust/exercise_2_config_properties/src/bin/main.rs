use exercise_2_config_properties::ConfigProperty;

fn main() {
  let log_level = ConfigProperty {
    name: "log_level",
    config_type: "String".to_string(),
    value: "debug",
  };

  let max_connections = ConfigProperty {
    name: "max_connections",
    config_type: "u32".to_string(),
    value: 100,
  };

  let timeout = ConfigProperty {
    name: "timeout",
    config_type: "u64".to_string(),
    value: 30,
  };

  println!("{}: {} ({})", log_level.name, log_level.value, log_level.config_type);
  println!("{}: {} ({})", max_connections.name, max_connections.value, max_connections.config_type);
  println!("{}: {} ({})", timeout.name, timeout.value, timeout.config_type);
}