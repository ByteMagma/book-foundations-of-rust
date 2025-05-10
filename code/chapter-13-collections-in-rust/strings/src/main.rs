// Iterating over characters in a String
// fn main() {
//     let mut s = String::from("hello");
//     for c in s.chars() {
//         println!("{}", c); // prints h e l l o
//     }
// }

// Iterating over bytes in a string slice
// fn main() {
//     let s = "world"; // type: &str
//     for b in s.bytes() {
//         println!("{}", b); // prints byte values
//     }
// }

// Filtering a string to have only letters
// fn main() {
//     let raw = String::from("User123!@#");
//     let letters_only: String = 
//         raw.chars().filter(|c| c.is_alphabetic()).collect();
//     println!("{}", letters_only); // Output: "User"
// }

// Checking if a string slice contains the word "error"
// fn main() {
//     let input = "error: file not found";
//     if input.contains("error") {
//         println!("Log contains an error");
//     }
// }

// Chaining methods on a string slice
// fn main() {
//     let word = "Greg";
//     let uppercased: String = 
//         word.chars().map(|c| c.to_ascii_uppercase()).collect();
//     println!("{}", uppercased); // Output: "GREG"
// }

// Splitting a string slice with a comma delimiter
// fn main() {
//     let csv = "apples,bananas,grapes";
//     let fruits: Vec<&str> = csv.split(',').collect();
//     for fruit in fruits {
//         println!("{}", fruit);
//     }
// }

// Adding to a string, similar to adding to a vector.
// fn main() {
//     let mut story = String::from("Once upon a time");
//     story.push_str(", there was a dev who learned Rust.");
//     println!("{}", story);  // Once upon a time, there was a dev who learned Rust.
// }

// Converting a string slice into a collection
fn main() {
    let name: &str = "Greg";
    let chars: Vec<char> = name.chars().collect();
    println!("{:?}", chars); // Output: ['G', 'r', 'e', 'g']
}
