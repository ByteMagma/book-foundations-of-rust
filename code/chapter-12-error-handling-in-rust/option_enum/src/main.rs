// Example of using Option enum variants
// struct User {
//     name: String,
//     age: u32,
// }
// fn find_user<'a>(name: &'a str, users: &'a [User]) -> Option<&'a User> {
//     for user in users {
//         if user.name == name {
//             return Some(user);
//         }
//     }
//     None
// }
// fn main() {
//     let users = vec![
//         User { name: String::from("Alice"), age: 30 },
//         User { name: String::from("Bob"), age: 25 },
//     ];

//     match find_user("Alice", &users) {
//         Some(user) => println!("Found user: {} (Age: {})", user.name, user.age),
//         None => println!("User not found."),
//     }
// }

// Example of using Result and Option enums together
use std::fs::File;
use std::env;

fn get_config_file() -> Option<Result<File, std::io::Error>> {
    // Try to get CONFIG_PATH from the environment
    let config_path = env::var("CONFIG_PATH").ok();

    match config_path {
        Some(path) => Some(File::open(path)), // Could be Ok or Err
        None => None, // No config path specified
    }
}

fn main() {
    match get_config_file() {
        Some(Ok(file)) => println!("Config file opened successfully: {:?}", file),
        Some(Err(e)) => println!("Failed to open config file: {}", e),
        None => println!("No config path provided in environment."),
    }
}

