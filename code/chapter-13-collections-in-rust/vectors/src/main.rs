// Basic vector collection example using push() to add elements.
// fn main() {
//     let mut todos = Vec::new();
//     todos.push("Buy milk");
//     todos.push("Call Alice");
//     todos.push("Write blog post");

//     for (i, task) in todos.iter().enumerate() {
//         println!("{}: {}", i + 1, task);
//     }
// }

// Using iterator sum() and vector len().
// fn main() {
//     let grades = vec![85, 92, 78, 90];
//     let sum: i32 = grades.iter().sum();
//     let average = sum as f32 / grades.len() as f32;
//     println!("{average}");
// }

// Filtering and collection on a vector.
// fn main() {
//     let emails = vec!["a@example.com", "invalid", "b@example.com"];
//     let valid: Vec<&str> =
//         emails
//             .into_iter()
//             .filter(|e| e.contains('@'))
//             .collect();
//     println!("{:?}", valid);
// }

// Filtering with retains()
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.retain(|&x| x % 2 == 0);
    // nums is now [2, 4
}

